//! 行情托管会话：连接、登录、断线重连与重订阅的流程自动化。
//!
//! 模块路径 `session::md`；将来交易托管会话为 `session::trader`。
//!
//! - [`MdSession::new`] = 建 API + 连接 + 登录 + 初始订阅，全部成功才返回；
//! - 断线由 CTP 自动重连传输层，内部 loop 收到 `Connected` 自动重登录并重订阅；
//! - [`MdSession::exit`] / `Drop` = 关闭 loop 并销毁原生 API
//!   （CTP 的关闭语义是销毁实例；行情前置不支持登出，见FAQ 的 logout/重连条目）。
//!
//! 线程模型：SPI 回调线程只做 owned 转换与非阻塞入队；loop 线程独占 API
//! （`MdApi` 不是 `Send`，只能在创建它的线程中使用）。
//! tick 与询价数据由 SPI 直送事件通道，不经过 loop，行情洪流不会饿死控制命令。

use super::request::{emit_request_error, error_message};
use crate::MdApi;
use crate::session::enums::{
    Command, DEFAULT_QUEUE_CAPACITY, Input, LOGIN_TIMEOUT, MdEvent, MdSessionError, MdSessionEvent,
};
use crate::session::md_spi::MdSpiImpl;
use crate::structs::ReqUserLogin;
use crate::types::{BrokerId, UserId};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

// request_id 语义备忘（查证自 CTP 官方文档与社区实践）：
// - 它只是请求↔响应（OnRsp*）的回声标签，协议**不强制**递增；
//   强制递增的是 OrderRef/OrderActionRef（不递增报 ErrorID 22「不允许重复报单」）；
// - 回报类回调（OnRtnOrder/OnRtnTrade/OnErrRtn*）**不携带** request_id，
//   报单对账走 FrontID+SessionID+OrderRef 体系；
// - 本层统一自增分配（见 `MdLoop::next_request_id`），
//   调用方可据此关联请求与直接应答；业务聚合由调用方负责。

/// 行情会话配置。
///
/// 行情端登录默认没有必填项（官方文档：可用空结构体登录），
/// 但 CTP 6.6.2 起柜台可开启"行情身份校验"，届时 broker_id/user_id 必须正确
/// 且与交易端一致——因此保留这两个字段；密码与授权码行情端用不到，不在配置中。
#[derive(Clone)]
pub struct MdConfig {
    /// 经纪商代码。
    pub broker_id: String,
    /// 投资者代码。
    pub user_id: String,
    /// 行情前置地址列表；多个地址由 CTP 随机选择连接、断线自动切换。
    pub fronts: Vec<String>,
    /// CTP 流量文件目录。
    pub flow_dir: String,
    /// 登录后自动订阅的合约；断线重连后自动重新订阅。
    pub subscribe: Vec<String>,
    /// 事件与输入队列容量；0 表示使用 [`DEFAULT_QUEUE_CAPACITY`]。
    /// 队列满时事件被丢弃并计入 `dropped_count()`。
    pub queue_capacity: usize,
    /// 对应 CTP `CreateFtdcMdApi` 的 `bIsUsingUdp`，常规 TCP 订阅为 false。
    pub using_udp: bool,
    /// 对应 `bIsMulticast`（组播行情），常规 TCP 订阅为 false。
    pub multicast: bool,
    /// 对应 `bIsProductionMode`，仿真环境（simnow）为 false、实盘为 true。
    pub production_mode: bool,
}

impl Default for MdConfig {
    /// 默认：空凭据、无前置、无订阅、队列容量走默认值、TCP 非组播、生产模式。
    fn default() -> Self {
        Self {
            broker_id: String::new(),
            user_id: String::new(),
            fronts: Vec::new(),
            flow_dir: String::new(),
            subscribe: Vec::new(),
            queue_capacity: 0,
            using_udp: false,
            multicast: false,
            production_mode: true,
        }
    }
}

impl fmt::Debug for MdConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MdConfig")
            .field("broker_id", &self.broker_id)
            .field("user_id", &self.user_id)
            .field("fronts", &self.fronts)
            .field("flow_dir", &self.flow_dir)
            .field("subscribe", &self.subscribe)
            .field("queue_capacity", &self.queue_capacity)
            .finish()
    }
}

/// 首次登录握手通道的结果：成功为交易日（`yyyymmdd`）。
/// 行情端的 FrontID/SessionID 恒为 0（会话标识是交易端概念），不向上传递。
type Handshake = Result<String, MdSessionError>;

/// 行情托管会话：构造即已登录并完成初始订阅，exit 即关闭释放。
pub struct MdSession {
    /// 命令通道发送端：`subscribe`/`exit` 等调用经此投给 loop 线程。
    inputs: SyncSender<Input>,
    /// 事件通道接收端：`next_event`/`try_event` 从此取走 loop 产出的事件。
    events: Receiver<MdSessionEvent>,
    dropped: Arc<AtomicU64>,
    trading_day: String,
    /// loop 线程句柄：`exit`/`Drop` 时据此 join，保证资源释放完成。
    worker: Mutex<Option<JoinHandle<()>>>,
}

impl MdSession {
    /// 连接前置并完成登录与初始订阅。
    ///
    /// 失败（登录拒绝、断线、超时）返回错误，资源已释放。
    pub fn new(config: &MdConfig) -> Result<Self, MdSessionError> {
        let capacity = if config.queue_capacity == 0 {
            DEFAULT_QUEUE_CAPACITY
        } else {
            config.queue_capacity
        };
        let (event_sender, events) = std::sync::mpsc::sync_channel(capacity);
        let (input_sender, inputs) = std::sync::mpsc::sync_channel(capacity);
        let (handshake_sender, handshake) = std::sync::mpsc::sync_channel(1);
        let dropped = Arc::new(AtomicU64::new(0));
        let thread_dropped = dropped.clone();
        let spi_sender = input_sender.clone();
        let config = config.clone();
        let worker = std::thread::Builder::new()
            .name("ctp-md-loop".to_owned())
            .spawn(move || {
                match MdLoop::new(config, event_sender, inputs, spi_sender, thread_dropped) {
                    Ok(mut session_loop) => session_loop.run(handshake_sender),
                    Err(error) => {
                        let _ = handshake_sender.send(Err(error));
                    }
                }
            })
            .map_err(|error| MdSessionError::Create(format!("无法启动行情调度线程：{error}")))?;

        // 等待首次登录完成；失败时回收线程再返回错误
        let outcome = handshake
            .recv_timeout(LOGIN_TIMEOUT)
            .map_err(|_| MdSessionError::Timeout)?;
        match outcome {
            Ok(trading_day) => Ok(Self {
                inputs: input_sender,
                events,
                dropped,
                trading_day,
                worker: Mutex::new(Some(worker)),
            }),
            Err(error) => {
                // 阻塞 send：接收端存活时总能排空一个槽位；线程已死则返回 Err 直接放弃
                let _ = input_sender.send(Input::Command(Command::Shutdown));
                let _ = worker.join();
                Err(error)
            }
        }
    }

    /// 登录所得交易日（`yyyymmdd`）。
    pub fn trading_day(&self) -> &str {
        &self.trading_day
    }

    /// 追加订阅合约；断线重连后自动重订阅（含初始订阅与后续追加）。
    pub fn subscribe(&self, instruments: &[impl AsRef<str>]) -> Result<(), MdSessionError> {
        self.send_command(Command::Subscribe(
            instruments.iter().map(|s| s.as_ref().to_owned()).collect(),
        ))
    }

    /// 退订合约；退订后重连不再自动订阅该合约。
    pub fn unsubscribe(&self, instruments: &[impl AsRef<str>]) -> Result<(), MdSessionError> {
        self.send_command(Command::Unsubscribe(
            instruments.iter().map(|s| s.as_ref().to_owned()).collect(),
        ))
    }

    /// 订阅询价（期权做市相关）；断线重连后自动重订阅。
    pub fn subscribe_quote(&self, instruments: &[impl AsRef<str>]) -> Result<(), MdSessionError> {
        self.send_command(Command::SubscribeQuote(
            instruments.iter().map(|s| s.as_ref().to_owned()).collect(),
        ))
    }

    /// 退订询价。
    pub fn unsubscribe_quote(&self, instruments: &[impl AsRef<str>]) -> Result<(), MdSessionError> {
        self.send_command(Command::UnsubscribeQuote(
            instruments.iter().map(|s| s.as_ref().to_owned()).collect(),
        ))
    }

    /// 非阻塞发送命令。
    fn send_command(&self, command: Command) -> Result<(), MdSessionError> {
        match self.inputs.try_send(Input::Command(command)) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(_)) => Err(MdSessionError::QueueFull),
            Err(TrySendError::Disconnected(_)) => Err(MdSessionError::Stopped),
        }
    }

    /// 非阻塞取一个事件；无事件返回 `None`。
    pub fn try_event(&self) -> Option<MdSessionEvent> {
        self.events.try_recv().ok()
    }

    /// 阻塞等待下一个事件。
    pub fn next_event(&self) -> Result<MdSessionEvent, MdSessionError> {
        self.events
            .recv()
            .map_err(|_| MdSessionError::ChannelClosed)
    }

    /// 因队列满被丢弃的事件数（含 owned 转换失败丢弃的数据）。
    pub fn dropped_count(&self) -> u64 {
        self.dropped.load(Ordering::Acquire)
    }

    /// 关闭 loop 并释放原生资源。
    pub fn exit(mut self) {
        self.shutdown();
    }

    /// 通知 loop 退出并等待线程结束。
    fn shutdown(&mut self) {
        if let Some(worker) = self.worker.lock().unwrap_or_else(|e| e.into_inner()).take() {
            let _ = self.inputs.send(Input::Command(Command::Shutdown));
            let _ = worker.join();
        }
    }
}

impl Drop for MdSession {
    /// 未调用 `exit` 直接丢弃时同样关闭并回收线程。
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// loop 线程独占的运行时状态。
struct MdLoop {
    api: MdApi,
    inputs: Receiver<Input>,
    events: SyncSender<MdSessionEvent>,
    config: MdConfig,
    subscriptions: Vec<String>,
    quote_subscriptions: Vec<String>,
    handshake: Option<SyncSender<Handshake>>,
    /// 自增请求 ID 计数器（语义见文件顶部备忘）。
    next_request_id: i32,
    /// 丢弃计数（loop 侧与 SPI 侧共享）。
    dropped: Arc<AtomicU64>,
}

impl MdLoop {
    /// 在 loop 线程内创建 API 并注册前置（API 不是 `Send`，不得跨线程移动）。
    fn new(
        config: MdConfig,
        events: SyncSender<MdSessionEvent>,
        inputs: Receiver<Input>,
        spi_sender: SyncSender<Input>,
        dropped: Arc<AtomicU64>,
    ) -> Result<Self, MdSessionError> {
        if !config.flow_dir.is_empty() {
            std::fs::create_dir_all(&config.flow_dir)
                .map_err(|error| MdSessionError::Create(format!("创建流量目录失败：{error}")))?;
        }
        if config.fronts.is_empty() {
            return Err(MdSessionError::Create("前置地址列表不能为空".to_owned()));
        }
        let api = MdApi::new(
            Box::new(MdSpiImpl::new(spi_sender, events.clone(), dropped.clone())),
            &config.flow_dir,
            config.using_udp,
            config.multicast,
            config.production_mode,
        )
        .map_err(|error| MdSessionError::Create(error.to_string()))?;
        for front in &config.fronts {
            api.register_front(front)
                .map_err(|error| MdSessionError::Request(error.to_string()))?;
        }
        let subscriptions = config.subscribe.clone();
        Ok(Self {
            api,
            inputs,
            events,
            config,
            subscriptions,
            quote_subscriptions: Vec::new(),
            handshake: None,
            next_request_id: 1,
            dropped,
        })
    }

    /// 阻塞等待输入：有事件才醒，没有就睡死，零空转。
    fn run(&mut self, handshake: SyncSender<Handshake>) {
        self.handshake = Some(handshake);
        self.api.init();
        while let Ok(input) = self.inputs.recv() {
            match input {
                Input::Spi(event) => self.on_market_event(event),
                Input::Command(Command::Subscribe(list)) => self.subscribe(list),
                Input::Command(Command::Unsubscribe(list)) => self.unsubscribe(list),
                Input::Command(Command::SubscribeQuote(list)) => self.subscribe_quote(list),
                Input::Command(Command::UnsubscribeQuote(list)) => self.unsubscribe_quote(list),
                Input::Command(Command::Shutdown) => {
                    return;
                }
            }
        }
        // 所有发送端断开（MdSession 已丢弃）：loop 返回，原生 API 随字段析构释放
    }

    fn on_market_event(&mut self, event: MdEvent) {
        match event {
            MdEvent::Connected => {
                let request_id = self.next_request_id();
                let user = ReqUserLogin {
                    broker_id: BrokerId::new(&self.config.broker_id).unwrap_or_default(),
                    user_id: UserId::new(&self.config.user_id).unwrap_or_default(),
                    ..Default::default()
                };
                if let Some(message) = error_message(self.api.login(&user, request_id), "行情登录")
                {
                    let mut handshake = self.handshake.take();
                    report_md_request_failure(&mut handshake, message, |event| {
                        self.send_event(event)
                    });
                }
            }
            MdEvent::LoginOk { trading_day } => {
                let first_login = self.handshake.take();
                // 首登与重连都要（重）订阅行情与询价
                self.subscribe(self.subscriptions.clone());
                self.subscribe_quote(self.quote_subscriptions.clone());
                match first_login {
                    Some(handshake) => {
                        let _ = handshake.send(Ok(trading_day));
                    }
                    None => {
                        self.send_event(MdSessionEvent::Recovered { trading_day });
                    }
                }
            }
            MdEvent::LoginFailed { error_id, message } => {
                if let Some(handshake) = self.handshake.take() {
                    let _ = handshake.send(Err(MdSessionError::LoginRejected(error_id, message)));
                } else {
                    self.send_event(MdSessionEvent::Error { error_id, message });
                }
            }
            MdEvent::Disconnected(reason) => {
                self.send_event(MdSessionEvent::Disconnected { reason });
            }
            MdEvent::Subscribed(instrument) => {
                self.send_event(MdSessionEvent::Subscribed { instrument });
            }
            MdEvent::Unsubscribed(instrument) => {
                self.send_event(MdSessionEvent::Unsubscribed { instrument });
            }
            MdEvent::QuoteSubscribed(instrument) => {
                self.send_event(MdSessionEvent::QuoteSubscribed { instrument });
            }
            MdEvent::QuoteUnsubscribed(instrument) => {
                self.send_event(MdSessionEvent::QuoteUnsubscribed { instrument });
            }
            MdEvent::Error { error_id, message } => {
                self.send_event(MdSessionEvent::Error { error_id, message });
            }
            MdEvent::ConversionFailed(message) => {
                self.send_event(MdSessionEvent::ConversionFailed { message });
            }
        }
    }

    /// 合并订阅列表并调用 CTP 订阅。
    fn subscribe(&mut self, list: Vec<String>) {
        merge(&mut self.subscriptions, list);
        if self.subscriptions.is_empty() {
            return;
        }
        let refs: Vec<&str> = self.subscriptions.iter().map(String::as_str).collect();
        emit_request_error(self.api.subscribe(&refs), "订阅行情", |message| {
            self.send_event(MdSessionEvent::Error {
                error_id: -1,
                message,
            })
        });
    }

    /// 从订阅列表移除并调用 CTP 退订。
    fn unsubscribe(&mut self, list: Vec<String>) {
        self.subscriptions.retain(|item| !list.contains(item));
        let refs: Vec<&str> = list.iter().map(String::as_str).collect();
        if !refs.is_empty() {
            emit_request_error(self.api.unsubscribe(&refs), "退订行情", |message| {
                self.send_event(MdSessionEvent::Error {
                    error_id: -1,
                    message,
                })
            });
        }
    }

    /// 合并询价订阅列表并调用 CTP 询价订阅。
    fn subscribe_quote(&mut self, list: Vec<String>) {
        merge(&mut self.quote_subscriptions, list);
        if self.quote_subscriptions.is_empty() {
            return;
        }
        let refs: Vec<&str> = self
            .quote_subscriptions
            .iter()
            .map(String::as_str)
            .collect();
        emit_request_error(self.api.subscribe_quote(&refs), "订阅询价", |message| {
            self.send_event(MdSessionEvent::Error {
                error_id: -1,
                message,
            })
        });
    }

    /// 从询价订阅列表移除并调用 CTP 询价退订。
    fn unsubscribe_quote(&mut self, list: Vec<String>) {
        self.quote_subscriptions.retain(|item| !list.contains(item));
        let refs: Vec<&str> = list.iter().map(String::as_str).collect();
        if !refs.is_empty() {
            emit_request_error(
                self.api.unsubscribe_quote(&refs),
                "退订询价",
                |message| {
                    self.send_event(MdSessionEvent::Error {
                        error_id: -1,
                        message,
                    })
                },
            );
        }
    }

    /// 分配自增请求 ID；i32 耗尽后回到 1（无对账语义，回绕无害）。
    fn next_request_id(&mut self) -> i32 {
        let id = self.next_request_id;
        self.next_request_id = id.checked_add(1).unwrap_or(1);
        id
    }

    /// 非阻塞投递事件；队列满时丢弃并计数（tick 洪流下保护 loop 与回调线程）。
    fn send_event(&mut self, event: MdSessionEvent) {
        if self.events.try_send(event).is_err() {
            self.dropped.fetch_add(1, Ordering::AcqRel);
        }
    }
}

pub(super) fn report_md_request_failure(
    handshake: &mut Option<SyncSender<Handshake>>,
    message: String,
    emit: impl FnOnce(MdSessionEvent),
) {
    match handshake.take() {
        Some(handshake) => {
            let _ = handshake.send(Err(MdSessionError::Request(message)));
        }
        None => emit(MdSessionEvent::Error {
            error_id: -1,
            message,
        }),
    }
}

/// 合并订阅列表（去重追加）。
fn merge(subscriptions: &mut Vec<String>, list: Vec<String>) {
    for instrument in list {
        if !subscriptions.contains(&instrument) {
            subscriptions.push(instrument);
        }
    }
}
