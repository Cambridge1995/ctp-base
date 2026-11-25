//! 交易托管会话：连接、认证、登录、结算确认的流程自动化。
//!
//! 模块路径 `session::trader`，与行情侧 `session::md` 对称。
//!
//! - [`TraderSession::new`] = 建 API + 连接 + 认证 + 登录 + 结算确认，全部成功才返回；
//! - 断线由 CTP 自动重连传输层，内部 loop 收到 `Connected` 自动重走认证链；
//! - [`TraderSession::exit`] / `Drop` = 关闭 loop 并销毁原生 API
//!   （CTP 的关闭语义是销毁实例；登出语义反常，见FAQ 的 logout/重连条目）。
//!
//! 线程模型与行情侧一致：SPI 回调线程只做 owned 转换与非阻塞入队，
//! loop 线程独占 API（`TraderApi` 不是 `Send`，只能在创建它的线程中使用）。
//!
//! 批次一：会话链（认证→登录→结算确认→就绪）。
//! 批次二：交易指令（报撤单 + 订单/成交/错单回报）：
//!
//! - 报单对账走 FrontID + SessionID + OrderRef，不用 request_id（回声标签）；
//! - OrderRef / OrderActionRef 由 loop 自动分配、严格递增，断线重连（新会话）后归零重计；
//! - CTP 拒单（`OrderInsertResponse` 错误 + `OrderInsertError`）与交易所拒单
//!   （`OrderUpdate` 被动撤单）是两条不同路径，分别上报。
//! - [`TraderSession::reconnect`] 提供主动重连（网络不稳定/假死时），
//!   利用官方记载的"登出=断开重连"行为（见FAQ 的 logout/重连条目）。
//!
//! 批次三：核心查询（报单/成交/持仓/资金/合约等 13 类查询 + 合约状态推送）：
//!
//! - 查询方法返回 request_id；每个查询响应立即以 [`TraderSessionEvent::QueryRow`] 公开；
//! - 空载荷行同样公开，调用方按 request_id + is_last 自行聚合和结束批次；
//! - 查询流不被柜台维护：断线与 `OnRspError` 原样上报，调用方负责结束自己的批次。
//!
//! 批次四：高级指令（行权/期权自对冲/组合/报价询价/预埋单，34 个回调）：
//!
//! - 本批引用命名空间：报单/预埋单/行权/自对冲/组合/报价/询价共用 `next_order_ref`
//!   单条递增序列——各家族子序列天然各自递增，且从构造上杜绝跨家族撞号；
//!   各操作引用（OrderActionRef/ExecOrderActionRef 等）共用 `next_action_ref` 序列；
//! - 预埋单的 ParkedOrderID/ParkedOrderActionID 由 CTP 分配，loop 发送前清空入参残留，
//!   删除按 ID 定位；
//! - 新查询行集复用批次三的逐行事件，调用方自行构造视图。
//!
//! 批次五：银期转账（26 个回调）：
//!
//! - 期商主动操作：`transfer_to_future`（入金）/`transfer_to_bank`（出金）/
//!   `query_bank_balance`（查银行余额），业务功能码与身份字段由 loop 填充；
//! - 应答（带 request_id）只表示受理，处理结果看对应的 `*Report` 事件——
//!   受理≠成功，资金动作必须以结果回报为准；
//! - 银行端发起的转账/开销户结果也以回报事件上报；
//! - 冲正（Repeal）原始层未封装请求方法（柜台/人工场景），本层只对接冲正回报与错误。
//!
//! 批次六：保证金、费率与风险监控查询（44 类）：
//! - 复用统一查询发送、身份填充与逐行事件；调用方负责聚合和错误清理；
//! - 交易所参数不虚构账户字段，投资者参数覆盖 BrokerID/InvestorID；
//! - 保留完整原始行，不在绑定层推导组合保证金、申报费或覆盖响应过滤字段。
//!
//! 批次七：对冲设置、SPD/套保确认与新增查询（12 个主动接口、21 个回调）：
//! - 对冲设置保留 RequestID 与原撤销维度；SPD/套保申请共享 OrderRef 序列；
//! - 撤销保留原 OrderRef/OrderSysID，只覆盖身份和当前 FrontID/SessionID；
//! - 直接应答、错误回报、状态通知和六类查询分别映射，不混淆业务终态。

use super::request::error_message;
pub(super) use crate::session::enums::{
    DEFAULT_QUEUE_CAPACITY, LOGIN_TIMEOUT, QueryKind, QueryRow, TraderCommand, TraderEvent,
    TraderInput, TraderSessionEvent,
};
use crate::session::trader_spi::TraderSpiImpl;
pub(super) use crate::structs::{
    InputBatchOrderAction, InputCombAction, InputExecOrder, InputExecOrderAction, InputForQuote,
    InputOptionSelfClose, InputOptionSelfCloseAction, InputOrder, InputOrderAction, InputQuote,
    InputQuoteAction, ParkedOrder, ParkedOrderAction, QryAccountRegister, QryCombAction,
    QryCombLeg, QryContractBank, QryDepthMarketData, QryExchange, QryExecOrder, QryForQuote,
    QryInstrument, QryInvestor, QryInvestorPosition, QryInvestorPositionCombineDetail,
    QryInvestorPositionDetail, QryOptionSelfClose, QryOrder, QryParkedOrder, QryParkedOrderAction,
    QryProduct, QryQuote, QrySettlementInfo, QryTrade, QryTradingAccount, QryTransferBank,
    QryTransferSerial, RemoveParkedOrder, RemoveParkedOrderAction, ReqAuthenticate,
    ReqQueryAccount, ReqTransfer, ReqUserLogin, SettlementInfoConfirm, UserLogout,
};
pub(super) use crate::structs::{
    InputHedgeCfm, InputHedgeCfmAction, InputOffsetSetting, InputSpdApply, InputSpdApplyAction,
    QryClassifiedInstrument, QryCombPromotionParam, QryHedgeCfm, QryInstrTradeLimit,
    QryOffsetSetting, QrySpdApply,
};
pub(super) use crate::structs::{
    QryCombInstrumentGuard, QryExchangeMarginRate, QryExchangeMarginRateAdjust, QryExchangeRate,
    QryInstrumentCommissionRate, QryInstrumentMarginRate, QryInstrumentOrderCommRate,
    QryInvestorCommodityGroupSPMMMargin, QryInvestorCommoditySPMMMargin, QryInvestorInfoCommRec,
    QryInvestorPortfMarginRatio, QryInvestorPortfSetting, QryInvestorProdRCAMSMargin,
    QryInvestorProdRULEMargin, QryInvestorProdSPBMDetail, QryInvestorProductGroupMargin,
    QryMMInstrumentCommissionRate, QryMMOptionInstrCommRate, QryMaxOrderVolume,
    QryMaxOrderVolumeWithPrice, QryOptionInstrCommRate, QryOptionInstrTradeCost,
    QryProductExchRate, QryProductGroup, QryRCAMSCombProductInfo, QryRCAMSInstrParameter,
    QryRCAMSInterParameter, QryRCAMSIntraParameter, QryRCAMSInvestorCombPosition,
    QryRCAMSShortOptAdjustParam, QryRULEInstrParameter, QryRULEInterParameter,
    QryRULEIntraParameter, QryRiskSettleInvestPosition, QryRiskSettleProductStatus,
    QrySPBMAddOnInterParameter, QrySPBMFutureParameter, QrySPBMInterParameter,
    QrySPBMIntraParameter, QrySPBMInvestorPortfDef, QrySPBMOptionParameter, QrySPBMPortfDefinition,
    QrySPMMInstParam, QrySPMMProductParam,
};
pub(super) use crate::types::codes;
pub(super) use crate::types::{
    AccountId, BrokerId, DateText, FrontId, InvestorId, OrderRef, SessionId, TimeText, UserId,
};
pub(super) use crate::{ResumeType, TraderApi};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// 交易会话配置。
#[derive(Clone)]
pub struct TraderConfig {
    /// 经纪商代码。
    pub broker_id: String,
    /// 投资者代码。
    pub user_id: String,
    /// 登录密码。
    pub password: String,
    /// 应用标识（强制终端认证需要）。
    pub app_id: String,
    /// 授权码（强制终端认证需要）。
    pub auth_code: String,
    /// 交易前置地址列表；多个地址由 CTP 随机选择连接、断线自动切换。
    pub fronts: Vec<String>,
    /// CTP 流量文件目录；空字符串表示 CTP 默认（当前目录）。
    pub flow_dir: String,
    /// 事件与输入队列容量；0 表示使用 [`DEFAULT_QUEUE_CAPACITY`]。
    pub queue_capacity: usize,
    /// 对应 CTP 创建参数的生产模式：仿真环境为 false、实盘为 true。
    pub production_mode: bool,
}

impl Default for TraderConfig {
    /// 默认：空凭据、无前置、队列容量走默认值、生产模式。
    fn default() -> Self {
        Self {
            broker_id: String::new(),
            user_id: String::new(),
            password: String::new(),
            app_id: String::new(),
            auth_code: String::new(),
            fronts: Vec::new(),
            flow_dir: String::new(),
            queue_capacity: 0,
            production_mode: true,
        }
    }
}

impl fmt::Debug for TraderConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TraderConfig")
            .field("broker_id", &self.broker_id)
            .field("user_id", &self.user_id)
            .field("password", &"<redacted>")
            .field("app_id", &self.app_id)
            .field("auth_code", &"<redacted>")
            .field("fronts", &self.fronts)
            .field("flow_dir", &self.flow_dir)
            .field("queue_capacity", &self.queue_capacity)
            .field("production_mode", &self.production_mode)
            .finish()
    }
}

/// 就绪握手通道的结果：成功为 (FrontID, SessionID, 交易日)。
type Handshake = Result<(i32, i32, String), TraderSessionError>;

/// 交易托管会话错误。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraderSessionError {
    /// 创建交易 API 失败。
    Create(String),
    /// CTP 请求发送失败。
    Request(String),
    /// 认证、登录或结算确认被前置拒绝（错误码 + 描述）。
    Rejected(i32, String),
    /// 连接至就绪超时。
    Timeout,
    /// 事件通道已断开。
    ChannelClosed,
    /// 调度线程已停止。
    Stopped,
    /// 命令队列已满。
    QueueFull,
    /// 会话未就绪（未登录或已断线），交易指令被拒绝。
    NotReady,
}

impl fmt::Display for TraderSessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(message) => write!(f, "创建交易 API 失败：{message}"),
            Self::Request(message) => write!(f, "CTP 请求发送失败：{message}"),
            Self::Rejected(error_id, message) => {
                write!(f, "会话流程被前置拒绝：[{error_id}] {message}")
            }
            Self::Timeout => write!(f, "连接至就绪超时（{} 秒）", LOGIN_TIMEOUT.as_secs()),
            Self::ChannelClosed => write!(f, "事件通道已断开"),
            Self::Stopped => write!(f, "交易调度线程已停止"),
            Self::QueueFull => write!(f, "命令队列已满，请稍后重试"),
            Self::NotReady => write!(f, "会话未就绪（未登录或已断线），交易指令被拒绝"),
        }
    }
}

impl std::error::Error for TraderSessionError {}

/// 交易托管会话：构造即已就绪（认证 + 登录 + 结算确认完成），exit 即关闭释放。
pub struct TraderSession {
    /// 命令通道发送端：调用经此投给 loop 线程。
    pub(super) inputs: SyncSender<TraderInput>,
    /// 事件通道接收端：`next_event`/`try_event` 从此取走事件。
    pub(super) events: Receiver<TraderSessionEvent>,
    pub(super) dropped: Arc<AtomicU64>,
    /// 当前会话身份：首登写入，断线重连就绪后由 loop 刷新，读取即当前值。
    pub(super) identity: Arc<Mutex<Identity>>,
    /// loop 线程句柄：`exit`/`Drop` 时据此 join，保证资源释放完成。
    pub(super) worker: Mutex<Option<JoinHandle<()>>>,
}

/// 会话身份：报单对账主键（FrontID/SessionID）与交易日。
pub(super) struct Identity {
    pub(super) front_id: i32,
    pub(super) session_id: i32,
    pub(super) trading_day: String,
}

impl TraderSession {
    /// 连接前置并完成认证、登录与结算确认。
    ///
    /// 失败（拒绝、断线、超时）返回错误，资源已释放。
    pub fn new(config: &TraderConfig) -> Result<Self, TraderSessionError> {
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
        let identity = Arc::new(Mutex::new(Identity {
            front_id: 0,
            session_id: 0,
            trading_day: String::new(),
        }));
        let thread_identity = identity.clone();
        let spi_sender = input_sender.clone();
        let config = config.clone();
        let worker = std::thread::Builder::new()
            .name("ctp-trader-loop".to_owned())
            .spawn(move || {
                match TraderLoop::new(
                    config,
                    event_sender,
                    inputs,
                    spi_sender,
                    thread_dropped,
                    thread_identity,
                ) {
                    Ok(mut session_loop) => session_loop.run(handshake_sender),
                    Err(error) => {
                        let _ = handshake_sender.send(Err(error));
                    }
                }
            })
            .map_err(|error| {
                TraderSessionError::Create(format!("无法启动交易调度线程：{error}"))
            })?;

        // 等待首次就绪；失败时回收线程再返回错误
        let outcome = match handshake.recv_timeout(LOGIN_TIMEOUT) {
            Ok(outcome) => outcome,
            Err(_) => {
                // 超时也须回收 loop 线程与原生 API，不得泄漏（与下方拒绝路径同一套收尾）
                let _ = input_sender.send(TraderInput::Command(TraderCommand::Shutdown));
                let _ = worker.join();
                return Err(TraderSessionError::Timeout);
            }
        };
        match outcome {
            Ok(_) => Ok(Self {
                inputs: input_sender,
                events,
                dropped,
                identity,
                worker: Mutex::new(Some(worker)),
            }),
            Err(error) => {
                let _ = input_sender.send(TraderInput::Command(TraderCommand::Shutdown));
                let _ = worker.join();
                Err(error)
            }
        }
    }

    /// 当前会话的前置编号（报单对账标识之一）；断线重连就绪后由 loop 刷新，读取即当前值。
    pub fn front_id(&self) -> i32 {
        self.identity
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .front_id
    }

    /// 当前会话的会话编号（报单对账标识之一）；断线重连就绪后由 loop 刷新，读取即当前值。
    pub fn session_id(&self) -> i32 {
        self.identity
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .session_id
    }

    /// 当前交易日（`yyyymmdd`）；断线重连就绪后由 loop 刷新，读取即当前值。
    pub fn trading_day(&self) -> String {
        self.identity
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .trading_day
            .clone()
    }

    /// 非阻塞取一个事件；无事件返回 `None`。
    pub fn try_event(&self) -> Option<TraderSessionEvent> {
        self.events.try_recv().ok()
    }

    /// 阻塞等待下一个事件。
    pub fn next_event(&self) -> Result<TraderSessionEvent, TraderSessionError> {
        self.events
            .recv()
            .map_err(|_| TraderSessionError::ChannelClosed)
    }

    /// 因队列满被丢弃的事件数（含 owned 转换失败丢弃的数据）。
    pub fn dropped_count(&self) -> u64 {
        self.dropped.load(Ordering::Acquire)
    }

    /// 报单。OrderRef 由会话自动分配（会话内严格递增）并返回，供回报对账；
    /// `order` 的 `order_ref`、`broker_id`、`investor_id`、`user_id` 会被会话身份覆盖。
    ///
    /// 会话未就绪（未登录或已断线）时拒绝执行；CTP 流控拒绝（返回码非 0）直接报错。
    pub fn insert_order(&self, order: InputOrder) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::InsertOrder {
            order: Box::new(order),
            reply,
        })
    }

    /// 撤单：按 `action` 中的 OrderRef 或 OrderSysID 撤掉既有报单。
    /// OrderActionRef、FrontID、SessionID 由会话自动填充，返回分配的 OrderActionRef。
    ///
    /// 注意：FrontID/SessionID 恒为当前会话，因此按 OrderRef 只能撤本会话的报单；
    /// 跨会话（重连前）的挂单须填 `order_system_id` + `exchange_id` 走交易所编号定位。
    pub fn cancel_order(&self, action: InputOrderAction) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::CancelOrder {
            action: Box::new(action),
            reply,
        })
    }

    /// 批量撤单。与单笔撤单共用同一 OrderActionRef 递增序列，返回分配的 OrderActionRef。
    pub fn batch_order_action(
        &self,
        action: InputBatchOrderAction,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::BatchOrderAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 行权申报（期权行权/放弃）。ExecOrderRef 由会话分配并返回。
    pub fn exec_order_insert(&self, order: InputExecOrder) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::ExecOrderInsert {
            order: Box::new(order),
            reply,
        })
    }

    /// 行权操作（撤销行权申报）。返回分配的 ExecOrderActionRef。
    pub fn exec_order_action(
        &self,
        action: InputExecOrderAction,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::ExecOrderAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 期权自对冲申报。OptionSelfCloseRef 由会话分配并返回。
    pub fn option_self_close_insert(
        &self,
        order: InputOptionSelfClose,
    ) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::OptionSelfCloseInsert {
            order: Box::new(order),
            reply,
        })
    }

    /// 期权自对冲操作（撤销申报）。返回分配的 OptionSelfCloseActionRef。
    pub fn option_self_close_action(
        &self,
        action: InputOptionSelfCloseAction,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::OptionSelfCloseAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 组合申请（申请组合/解除组合持仓）。CombActionRef 由会话分配并返回。
    pub fn comb_action_insert(
        &self,
        action: InputCombAction,
    ) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::CombActionInsert {
            action: Box::new(action),
            reply,
        })
    }

    /// 报价申报（期权做市）。QuoteRef 由会话分配并返回。
    pub fn quote_insert(&self, quote: InputQuote) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::QuoteInsert {
            quote: Box::new(quote),
            reply,
        })
    }

    /// 报价操作（撤销报价）。返回分配的 QuoteActionRef。
    pub fn quote_action(&self, action: InputQuoteAction) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::QuoteAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 询价申报（请求对手方报价）。ForQuoteRef 由会话分配并返回。
    pub fn for_quote_insert(&self, for_quote: InputForQuote) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::ForQuoteInsert {
            for_quote: Box::new(for_quote),
            reply,
        })
    }

    /// 预埋单申报（挂单待触发）。OrderRef 由会话分配（与报单共用递增序列）并返回；
    /// `order_ref`、身份字段与 `parked_order_id`（CTP 分配，残留值会被清空）会被覆盖。
    pub fn parked_order_insert(&self, order: ParkedOrder) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::ParkedOrderInsert {
            order: Box::new(order),
            reply,
        })
    }

    /// 预埋撤单。返回分配的 OrderActionRef；
    /// `order_action_ref`、`front_id`、`session_id`、身份字段与 `parked_order_action_id`
    /// （CTP 分配，残留值会被清空）会被覆盖。
    pub fn parked_order_action(
        &self,
        action: ParkedOrderAction,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::ParkedOrderAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 删除预埋单（按 CTP 分配的 ParkedOrderID）。返回分配的 request_id。
    pub fn remove_parked_order(
        &self,
        removed: RemoveParkedOrder,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::RemoveParkedOrder {
            removed: Box::new(removed),
            reply,
        })
    }

    /// 删除预埋撤单（按 CTP 分配的 ParkedOrderActionID）。返回分配的 request_id。
    pub fn remove_parked_order_action(
        &self,
        removed: RemoveParkedOrderAction,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::RemoveParkedOrderAction {
            removed: Box::new(removed),
            reply,
        })
    }

    pub(super) fn call<T>(
        &self,
        command: impl FnOnce(SyncSender<Result<T, TraderSessionError>>) -> TraderCommand,
    ) -> Result<T, TraderSessionError> {
        let (reply, answer) = std::sync::mpsc::sync_channel(1);
        self.send_command(command(reply))?;
        answer
            .recv()
            .map_err(|_| TraderSessionError::ChannelClosed)?
    }

    /// 非阻塞投递命令；队列满或通道断开时立即报错。
    fn send_command(&self, command: TraderCommand) -> Result<(), TraderSessionError> {
        match self.inputs.try_send(TraderInput::Command(command)) {
            Ok(()) => Ok(()),
            Err(std::sync::mpsc::TrySendError::Full(_)) => Err(TraderSessionError::QueueFull),
            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                Err(TraderSessionError::ChannelClosed)
            }
        }
    }

    /// 发送查询并返回批次 request_id；结果以 `QueryRow` 事件逐行返回。
    fn send_query(&self, kind: QueryKind) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::Query {
            kind: Box::new(kind),
            reply,
        })
    }

    /// 查询报单；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_order(&self, request: QryOrder) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Order(request))
    }

    /// 查询成交；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_trade(&self, request: QryTrade) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Trade(request))
    }

    /// 查询持仓；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_position(
        &self,
        request: QryInvestorPosition,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorPosition(request))
    }

    /// 查询持仓明细；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_position_detail(
        &self,
        request: QryInvestorPositionDetail,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorPositionDetail(request))
    }

    /// 查询资金账户；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_trading_account(
        &self,
        request: QryTradingAccount,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::TradingAccount(request))
    }

    /// 查询合约；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_instrument(&self, request: QryInstrument) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Instrument(request))
    }

    /// 查询交易所；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_exchange(&self, request: QryExchange) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Exchange(request))
    }

    /// 查询产品；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_product(&self, request: QryProduct) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Product(request))
    }

    /// 查询深度行情；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_depth_market_data(
        &self,
        request: QryDepthMarketData,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::DepthMarketData(request))
    }

    /// 查询组合持仓明细；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_position_combine_detail(
        &self,
        request: QryInvestorPositionCombineDetail,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorPositionCombineDetail(request))
    }

    /// 查询组合合约腿；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_comb_leg(&self, request: QryCombLeg) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::CombLeg(request))
    }

    /// 查询投资者信息；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor(&self, request: QryInvestor) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Investor(request))
    }

    /// 查询结算单；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_settlement_info(
        &self,
        request: QrySettlementInfo,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SettlementInfo(request))
    }

    /// 查询行权；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_exec_order(&self, request: QryExecOrder) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ExecOrder(request))
    }

    /// 查询期权自对冲；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_option_self_close(
        &self,
        request: QryOptionSelfClose,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::OptionSelfClose(request))
    }

    /// 查询组合申请；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_comb_action(&self, request: QryCombAction) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::CombAction(request))
    }

    /// 查询报价；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_quote(&self, request: QryQuote) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::Quote(request))
    }

    /// 查询询价；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_for_quote(&self, request: QryForQuote) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ForQuote(request))
    }

    /// 查询预埋单；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_parked_order(&self, request: QryParkedOrder) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ParkedOrder(request))
    }

    /// 查询预埋撤单；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_parked_order_action(
        &self,
        request: QryParkedOrderAction,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ParkedOrderAction(request))
    }

    /// 查询转帐银行；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_transfer_bank(&self, request: QryTransferBank) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::TransferBank(request))
    }

    /// 查询转账流水；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_transfer_serial(
        &self,
        request: QryTransferSerial,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::TransferSerial(request))
    }

    /// 查询银期账户登记；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_account_register(
        &self,
        request: QryAccountRegister,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::AccountRegister(request))
    }

    /// 查询签约银行；返回批次 request_id，结果以 `QueryRow` 事件逐行返回。
    pub fn query_contract_bank(&self, request: QryContractBank) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ContractBank(request))
    }

    /// 查询最大报单数量；结果以 `QueryRow` 事件逐行返回。
    pub fn query_max_order_volume(
        &self,
        request: QryMaxOrderVolume,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::MaxOrderVolume(request))
    }

    /// 查询合约保证金率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_instrument_margin_rate(
        &self,
        request: QryInstrumentMarginRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InstrumentMarginRate(request))
    }

    /// 查询合约手续费率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_instrument_commission_rate(
        &self,
        request: QryInstrumentCommissionRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InstrumentCommissionRate(request))
    }

    /// 查询做市商合约手续费率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_mm_instrument_commission_rate(
        &self,
        request: QryMMInstrumentCommissionRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::MMInstrumentCommissionRate(request))
    }

    /// 查询做市商期权合约手续费率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_mm_option_instr_comm_rate(
        &self,
        request: QryMMOptionInstrCommRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::MMOptionInstrCommRate(request))
    }

    /// 查询报单手续费率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_instrument_order_comm_rate(
        &self,
        request: QryInstrumentOrderCommRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InstrumentOrderCommRate(request))
    }

    /// 查询期权交易成本；结果以 `QueryRow` 事件逐行返回。
    pub fn query_option_instr_trade_cost(
        &self,
        request: QryOptionInstrTradeCost,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::OptionInstrTradeCost(request))
    }

    /// 查询期权合约手续费率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_option_instr_comm_rate(
        &self,
        request: QryOptionInstrCommRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::OptionInstrCommRate(request))
    }

    /// 查询交易所保证金率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_exchange_margin_rate(
        &self,
        request: QryExchangeMarginRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ExchangeMarginRate(request))
    }

    /// 查询交易所调整保证金率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_exchange_margin_rate_adjust(
        &self,
        request: QryExchangeMarginRateAdjust,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ExchangeMarginRateAdjust(request))
    }

    /// 查询汇率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_exchange_rate(&self, request: QryExchangeRate) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ExchangeRate(request))
    }

    /// 查询产品报价汇率；结果以 `QueryRow` 事件逐行返回。
    pub fn query_product_exch_rate(
        &self,
        request: QryProductExchRate,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ProductExchRate(request))
    }

    /// 查询产品组；结果以 `QueryRow` 事件逐行返回。
    pub fn query_product_group(&self, request: QryProductGroup) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ProductGroup(request))
    }

    /// 查询投资者品种/跨品种保证金；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_product_group_margin(
        &self,
        request: QryInvestorProductGroupMargin,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorProductGroupMargin(request))
    }

    /// 查询组合合约安全系数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_comb_instrument_guard(
        &self,
        request: QryCombInstrumentGuard,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::CombInstrumentGuard(request))
    }

    /// 根据价格查询最大报单数量；结果以 `QueryRow` 事件逐行返回。
    pub fn query_max_order_volume_with_price(
        &self,
        request: QryMaxOrderVolumeWithPrice,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::MaxOrderVolumeWithPrice(request))
    }

    /// 查询 SPBM 期货合约参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_future_parameter(
        &self,
        request: QrySPBMFutureParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMFutureParameter(request))
    }

    /// 查询 SPBM 期权合约参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_option_parameter(
        &self,
        request: QrySPBMOptionParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMOptionParameter(request))
    }

    /// 查询 SPBM 品种内参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_intra_parameter(
        &self,
        request: QrySPBMIntraParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMIntraParameter(request))
    }

    /// 查询 SPBM 跨品种参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_inter_parameter(
        &self,
        request: QrySPBMInterParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMInterParameter(request))
    }

    /// 查询 SPBM 组合保证金套餐；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_portf_definition(
        &self,
        request: QrySPBMPortfDefinition,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMPortfDefinition(request))
    }

    /// 查询投资者 SPBM 套餐选择；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_investor_portf_def(
        &self,
        request: QrySPBMInvestorPortfDef,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMInvestorPortfDef(request))
    }

    /// 查询 SPBM 附加跨品种抵扣参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spbm_add_on_inter_parameter(
        &self,
        request: QrySPBMAddOnInterParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPBMAddOnInterParameter(request))
    }

    /// 查询投资者组合保证金系数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_portf_margin_ratio(
        &self,
        request: QryInvestorPortfMarginRatio,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorPortfMarginRatio(request))
    }

    /// 查询投资者产品 SPBM 明细；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_prod_spbm_detail(
        &self,
        request: QryInvestorProdSPBMDetail,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorProdSPBMDetail(request))
    }

    /// 查询 SPMM 合约参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spmm_inst_param(
        &self,
        request: QrySPMMInstParam,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPMMInstParam(request))
    }

    /// 查询 SPMM 品种参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spmm_product_param(
        &self,
        request: QrySPMMProductParam,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SPMMProductParam(request))
    }

    /// 查询投资者商品组 SPMM 保证金；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_commodity_spmm_margin(
        &self,
        request: QryInvestorCommoditySPMMMargin,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorCommoditySPMMMargin(request))
    }

    /// 查询投资者商品群 SPMM 保证金；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_commodity_group_spmm_margin(
        &self,
        request: QryInvestorCommodityGroupSPMMMargin,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorCommodityGroupSPMMMargin(request))
    }

    /// 查询 RCAMS 组合产品信息；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rcams_comb_product_info(
        &self,
        request: QryRCAMSCombProductInfo,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RCAMSCombProductInfo(request))
    }

    /// 查询 RCAMS 合约参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rcams_instr_parameter(
        &self,
        request: QryRCAMSInstrParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RCAMSInstrParameter(request))
    }

    /// 查询 RCAMS 品种内参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rcams_intra_parameter(
        &self,
        request: QryRCAMSIntraParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RCAMSIntraParameter(request))
    }

    /// 查询 RCAMS 跨品种参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rcams_inter_parameter(
        &self,
        request: QryRCAMSInterParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RCAMSInterParameter(request))
    }

    /// 查询 RCAMS 空头期权风险调整参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rcams_short_opt_adjust_param(
        &self,
        request: QryRCAMSShortOptAdjustParam,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RCAMSShortOptAdjustParam(request))
    }

    /// 查询 RCAMS 策略组合持仓；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rcams_investor_comb_position(
        &self,
        request: QryRCAMSInvestorCombPosition,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RCAMSInvestorCombPosition(request))
    }

    /// 查询投资者品种 RCAMS 保证金；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_prod_rcams_margin(
        &self,
        request: QryInvestorProdRCAMSMargin,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorProdRCAMSMargin(request))
    }

    /// 查询 RULE 合约参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rule_instr_parameter(
        &self,
        request: QryRULEInstrParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RULEInstrParameter(request))
    }

    /// 查询 RULE 品种内参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rule_intra_parameter(
        &self,
        request: QryRULEIntraParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RULEIntraParameter(request))
    }

    /// 查询 RULE 跨品种参数；结果以 `QueryRow` 事件逐行返回。
    pub fn query_rule_inter_parameter(
        &self,
        request: QryRULEInterParameter,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RULEInterParameter(request))
    }

    /// 查询投资者产品 RULE 保证金；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_prod_rule_margin(
        &self,
        request: QryInvestorProdRULEMargin,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorProdRULEMargin(request))
    }

    /// 查询投资者组合保证金开关；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_portf_setting(
        &self,
        request: QryInvestorPortfSetting,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorPortfSetting(request))
    }

    /// 查询投资者风险结算持仓；结果以 `QueryRow` 事件逐行返回。
    pub fn query_risk_settle_invest_position(
        &self,
        request: QryRiskSettleInvestPosition,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RiskSettleInvestPosition(request))
    }

    /// 查询风险结算产品状态；结果以 `QueryRow` 事件逐行返回。
    pub fn query_risk_settle_product_status(
        &self,
        request: QryRiskSettleProductStatus,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::RiskSettleProductStatus(request))
    }

    /// 查询投资者申报费阶梯收取记录；结果以 `QueryRow` 事件逐行返回。
    pub fn query_investor_info_comm_rec(
        &self,
        request: QryInvestorInfoCommRec,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InvestorInfoCommRec(request))
    }

    /// 设置大商所对冲；返回外层 request_id，结构体内 RequestID 保持调用方值。
    pub fn set_offset(&self, setting: InputOffsetSetting) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::SetOffset {
            setting: Box::new(setting),
            reply,
        })
    }

    /// 撤销大商所对冲设置；保留原设置维度，返回外层 request_id。
    pub fn cancel_offset_setting(
        &self,
        setting: InputOffsetSetting,
    ) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::CancelOffsetSetting {
            setting: Box::new(setting),
            reply,
        })
    }

    /// 提交 SPD 套利确认；返回 Session 分配的 OrderRef。
    pub fn spd_apply(&self, apply: InputSpdApply) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::SpdApply {
            apply: Box::new(apply),
            reply,
        })
    }

    /// 撤销 SPD 套利确认；保留原 OrderRef/OrderSysID，返回外层 request_id。
    pub fn spd_apply_action(&self, action: InputSpdApplyAction) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::SpdApplyAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 提交套保确认；返回 Session 分配的 OrderRef。
    pub fn hedge_cfm(&self, apply: InputHedgeCfm) -> Result<String, TraderSessionError> {
        self.call(|reply| TraderCommand::HedgeCfm {
            apply: Box::new(apply),
            reply,
        })
    }

    /// 撤销套保确认；保留原 OrderRef/OrderSysID，返回外层 request_id。
    pub fn hedge_cfm_action(&self, action: InputHedgeCfmAction) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::HedgeCfmAction {
            action: Box::new(action),
            reply,
        })
    }

    /// 查询投资者对冲设置；结果以 `QueryRow` 事件逐行返回。
    pub fn query_offset_settings(
        &self,
        request: QryOffsetSetting,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::OffsetSetting(request))
    }

    /// 查询 SPD 套利确认；结果以 `QueryRow` 事件逐行返回。
    pub fn query_spd_apply(&self, request: QrySpdApply) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::SpdApply(request))
    }

    /// 查询套保确认；结果以 `QueryRow` 事件逐行返回。
    pub fn query_hedge_cfm(&self, request: QryHedgeCfm) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::HedgeCfm(request))
    }

    /// 查询合约交易限制；结果以 `QueryRow` 事件逐行返回。
    pub fn query_instr_trade_limit(
        &self,
        request: QryInstrTradeLimit,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::InstrTradeLimit(request))
    }

    /// 查询分类合约；结果以 `QueryRow` 事件逐行返回。
    pub fn query_classified_instrument(
        &self,
        request: QryClassifiedInstrument,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::ClassifiedInstrument(request))
    }

    /// 查询组合优惠比例；结果以 `QueryRow` 事件逐行返回。
    pub fn query_comb_promotion_param(
        &self,
        request: QryCombPromotionParam,
    ) -> Result<i32, TraderSessionError> {
        self.send_query(QueryKind::CombPromotionParam(request))
    }

    /// 期商发起入金（银行→期货）。业务功能码与身份字段由会话填充，返回分配的 request_id；
    /// 受理结果看 `BankToFutureResponse`，处理结果看 `BankToFutureByFutureReport`。
    ///
    /// 资金密码必须手动传入（`password` 字段）：为空时本地直接拒绝，不流向柜台。
    ///
    /// 断线后在途应答可能丢失（对话流重置）：资金状态一律以 `query_transfer_serial`
    /// 流水查询对账（重连恢复 `Recovered` 后重查），不得假设应答必达。
    pub fn transfer_to_future(&self, transfer: ReqTransfer) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::BankToFuture {
            transfer: Box::new(transfer),
            reply,
        })
    }

    /// 期商发起出金（期货→银行）。业务功能码与身份字段由会话填充，返回分配的 request_id；
    /// 受理结果看 `FutureToBankResponse`，处理结果看 `FutureToBankByFutureReport`。
    ///
    /// 资金密码必须手动传入（`password` 字段）：为空时本地直接拒绝，不流向柜台。
    ///
    /// 断线后在途应答可能丢失（对话流重置）：资金状态一律以 `query_transfer_serial`
    /// 流水查询对账（重连恢复 `Recovered` 后重查），不得假设应答必达。
    pub fn transfer_to_bank(&self, transfer: ReqTransfer) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::FutureToBank {
            transfer: Box::new(transfer),
            reply,
        })
    }

    /// 查询银行余额。身份字段由会话填充，返回分配的 request_id；
    /// 结果看 `QueryBankBalanceReport`。
    ///
    /// 资金密码必须手动传入（`password` 字段）：为空时本地直接拒绝，不流向柜台。
    ///
    /// 断线后在途应答可能丢失（对话流重置）：必要时重发查询，
    /// 或以 `query_transfer_serial` 流水查询对账。
    pub fn query_bank_balance(&self, query: ReqQueryAccount) -> Result<i32, TraderSessionError> {
        self.call(|reply| TraderCommand::QueryBankBalance {
            query: Box::new(query),
            reply,
        })
    }

    /// 主动重连：网络不稳定或怀疑连接假死时调用。
    ///
    /// 利用官方记载的"登出=断开重连"行为触发链路重建（FAQ 的 logout/重连条目）；
    /// 随后会依次收到 `Logout`、`Disconnected`、`Recovered` 事件，
    /// 重连完成（`Recovered`）前交易指令返回 [`TraderSessionError::NotReady`]。
    ///
    /// 注意：该行为出自官方开发指南的行为描述而非协议承诺；
    /// 若未来 CTP 版本将登出改为优雅登出，此接口需改为销毁重建。
    pub fn reconnect(&self) -> Result<(), TraderSessionError> {
        self.send_command(TraderCommand::Reconnect)
    }

    /// 关闭 loop 并释放原生资源。
    pub fn exit(mut self) {
        self.shutdown();
    }

    /// 通知 loop 退出并等待线程结束。
    fn shutdown(&mut self) {
        if let Some(worker) = self.worker.lock().unwrap_or_else(|e| e.into_inner()).take() {
            let _ = self
                .inputs
                .send(TraderInput::Command(TraderCommand::Shutdown));
            let _ = worker.join();
        }
    }
}

impl Drop for TraderSession {
    /// 未调用 `exit` 直接丢弃时同样关闭并回收线程。
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// loop 线程独占的运行时状态。
pub(super) struct TraderLoop {
    api: TraderApi,
    inputs: Receiver<TraderInput>,
    events: SyncSender<TraderSessionEvent>,
    config: TraderConfig,
    handshake: Option<SyncSender<Handshake>>,
    /// 登录成功但结算确认未完成的暂存（结算确认到达后转为 Ready/Recovered）。
    pending_ready: Option<(i32, i32, String)>,
    /// 当前就绪会话的 (FrontID, SessionID)；未就绪为 None，交易指令被拒绝。
    session: Option<(i32, i32)>,
    /// 与 TraderSession 共享的会话身份，就绪时刷新（调用方 getter 读取即当前值）。
    identity: Arc<Mutex<Identity>>,
    /// 自增请求 ID 计数器（语义见 md.rs 顶部备忘）。
    next_request_id: i32,
    /// 下一个 OrderRef（报单/预埋单/行权/自对冲/组合/报价/询价/SPD/套保确认共用
    /// 同一递增序列，断线重连后归零重计；单序列从构造上杜绝跨家族撞号）。
    next_order_ref: u64,
    /// 下一个 OrderActionRef（报单与批量撤单共用同一递增序列）。
    next_action_ref: i32,
    /// 丢弃计数（loop 侧与 SPI 侧共享）。
    dropped: Arc<AtomicU64>,
}

pub(super) fn into_session_query_row(
    request_id: i32,
    is_last: bool,
    error_id: i32,
    message: String,
    row: Option<QueryRow>,
) -> TraderSessionEvent {
    TraderSessionEvent::QueryRow {
        request_id,
        is_last,
        error_id,
        message,
        row,
    }
}

pub(super) fn into_session_query_error(
    request_id: i32,
    error_id: i32,
    message: String,
) -> TraderSessionEvent {
    TraderSessionEvent::QueryError {
        request_id,
        error_id,
        message,
    }
}

impl TraderLoop {
    /// 在 loop 线程内创建 API 并注册前置（API 不是 `Send`，不得跨线程移动）。
    ///
    /// 私有流与公共流在 `init` 前订阅为 `Quick`，避免默认恢复语义重放历史消息。
    fn new(
        config: TraderConfig,
        events: SyncSender<TraderSessionEvent>,
        inputs: Receiver<TraderInput>,
        spi_sender: SyncSender<TraderInput>,
        dropped: Arc<AtomicU64>,
        identity: Arc<Mutex<Identity>>,
    ) -> Result<Self, TraderSessionError> {
        if !config.flow_dir.is_empty() {
            std::fs::create_dir_all(&config.flow_dir).map_err(|error| {
                TraderSessionError::Create(format!("创建流量目录失败：{error}"))
            })?;
        }
        if config.fronts.is_empty() {
            return Err(TraderSessionError::Create(
                "前置地址列表不能为空".to_owned(),
            ));
        }
        let api = TraderApi::new(
            Box::new(TraderSpiImpl::new(spi_sender, dropped.clone())),
            &config.flow_dir,
            config.production_mode,
        )
        .map_err(|error| TraderSessionError::Create(error.to_string()))?;
        for front in &config.fronts {
            api.register_front(front)
                .map_err(|error| TraderSessionError::Request(error.to_string()))?;
        }
        api.subscribe_private(ResumeType::Quick)
            .map_err(|error| TraderSessionError::Request(error.to_string()))?;
        api.subscribe_public(ResumeType::Quick)
            .map_err(|error| TraderSessionError::Request(error.to_string()))?;
        Ok(Self {
            api,
            inputs,
            events,
            config,
            handshake: None,
            pending_ready: None,
            session: None,
            identity,
            next_request_id: 1,
            next_order_ref: 1,
            next_action_ref: 1,
            dropped,
        })
    }

    /// 阻塞等待输入：有事件才醒，没有就睡死，零空转。
    fn run(&mut self, handshake: SyncSender<Handshake>) {
        self.handshake = Some(handshake);
        self.api.init();
        while let Ok(input) = self.inputs.recv() {
            match input {
                TraderInput::Spi(event) => self.on_trader_event(event),
                TraderInput::Command(TraderCommand::Shutdown) => {
                    return;
                }
                TraderInput::Command(TraderCommand::Query { kind, reply }) => {
                    let _ = reply.send(self.submit_query(*kind));
                }
                TraderInput::Command(TraderCommand::Reconnect) => self.reconnect(),
                TraderInput::Command(TraderCommand::InsertOrder { mut order, reply }) => {
                    let _ = reply.send(self.submit_insert_order(&mut order));
                }
                TraderInput::Command(TraderCommand::CancelOrder { mut action, reply }) => {
                    let _ = reply.send(self.submit_cancel_order(&mut action));
                }
                TraderInput::Command(TraderCommand::BatchOrderAction { mut action, reply }) => {
                    let _ = reply.send(self.submit_batch_action(&mut action));
                }
                TraderInput::Command(TraderCommand::ExecOrderInsert { mut order, reply }) => {
                    let _ = reply.send(self.submit_exec_order_insert(&mut order));
                }
                TraderInput::Command(TraderCommand::ExecOrderAction { mut action, reply }) => {
                    let _ = reply.send(self.submit_exec_order_action(&mut action));
                }
                TraderInput::Command(TraderCommand::OptionSelfCloseInsert { mut order, reply }) => {
                    let _ = reply.send(self.submit_option_self_close_insert(&mut order));
                }
                TraderInput::Command(TraderCommand::OptionSelfCloseAction {
                    mut action,
                    reply,
                }) => {
                    let _ = reply.send(self.submit_option_self_close_action(&mut action));
                }
                TraderInput::Command(TraderCommand::CombActionInsert { mut action, reply }) => {
                    let _ = reply.send(self.submit_comb_action_insert(&mut action));
                }
                TraderInput::Command(TraderCommand::QuoteInsert { mut quote, reply }) => {
                    let _ = reply.send(self.submit_quote_insert(&mut quote));
                }
                TraderInput::Command(TraderCommand::QuoteAction { mut action, reply }) => {
                    let _ = reply.send(self.submit_quote_action(&mut action));
                }
                TraderInput::Command(TraderCommand::ForQuoteInsert {
                    mut for_quote,
                    reply,
                }) => {
                    let _ = reply.send(self.submit_for_quote_insert(&mut for_quote));
                }
                TraderInput::Command(TraderCommand::ParkedOrderInsert { mut order, reply }) => {
                    let _ = reply.send(self.submit_parked_order_insert(&mut order));
                }
                TraderInput::Command(TraderCommand::ParkedOrderAction { mut action, reply }) => {
                    let _ = reply.send(self.submit_parked_order_action(&mut action));
                }
                TraderInput::Command(TraderCommand::RemoveParkedOrder { mut removed, reply }) => {
                    let _ = reply.send(self.submit_remove_parked_order(&mut removed));
                }
                TraderInput::Command(TraderCommand::RemoveParkedOrderAction {
                    mut removed,
                    reply,
                }) => {
                    let _ = reply.send(self.submit_remove_parked_order_action(&mut removed));
                }
                TraderInput::Command(TraderCommand::SetOffset { mut setting, reply }) => {
                    let _ = reply.send(self.submit_set_offset(&mut setting));
                }
                TraderInput::Command(TraderCommand::CancelOffsetSetting { mut setting, reply }) => {
                    let _ = reply.send(self.submit_cancel_offset_setting(&mut setting));
                }
                TraderInput::Command(TraderCommand::SpdApply { mut apply, reply }) => {
                    let _ = reply.send(self.submit_spd_apply(&mut apply));
                }
                TraderInput::Command(TraderCommand::SpdApplyAction { mut action, reply }) => {
                    let _ = reply.send(self.submit_spd_apply_action(&mut action));
                }
                TraderInput::Command(TraderCommand::HedgeCfm { mut apply, reply }) => {
                    let _ = reply.send(self.submit_hedge_cfm(&mut apply));
                }
                TraderInput::Command(TraderCommand::HedgeCfmAction { mut action, reply }) => {
                    let _ = reply.send(self.submit_hedge_cfm_action(&mut action));
                }
                TraderInput::Command(TraderCommand::BankToFuture {
                    mut transfer,
                    reply,
                }) => {
                    let _ = reply
                        .send(self.submit_transfer(TransferDirection::ToFuture, &mut transfer));
                }
                TraderInput::Command(TraderCommand::FutureToBank {
                    mut transfer,
                    reply,
                }) => {
                    let _ =
                        reply.send(self.submit_transfer(TransferDirection::ToBank, &mut transfer));
                }
                TraderInput::Command(TraderCommand::QueryBankBalance { mut query, reply }) => {
                    let _ = reply.send(self.submit_bank_balance_query(&mut query));
                }
            }
        }
        // 所有发送端断开（TraderSession 已丢弃）：loop 返回，原生 API 随字段析构释放
    }

    fn on_trader_event(&mut self, event: TraderEvent) {
        match event {
            TraderEvent::Connected => {
                self.send_request("认证", |api, config, request_id| {
                    api.authenticate(
                        &ReqAuthenticate {
                            broker_id: BrokerId::new(&config.broker_id).unwrap_or_default(),
                            user_id: UserId::new(&config.user_id).unwrap_or_default(),
                            user_product_info: String::new(),
                            auth_code: config.auth_code.clone(),
                            app_id: config.app_id.clone(),
                        },
                        request_id,
                    )
                });
            }
            TraderEvent::AuthenticateOk => {
                self.send_request("登录", |api, config, request_id| {
                    api.login(&login_request(config), request_id)
                });
            }
            TraderEvent::LoginOk {
                front_id,
                session_id,
                trading_day,
            } => {
                // 新会话：OrderRef/OrderActionRef 命名空间刷新，归零重计
                //（FAQ 的断线重连条目）
                self.session = None;
                self.next_order_ref = 1;
                self.next_action_ref = 1;
                self.send_request("结算确认", |api, config, request_id| {
                    api.confirm_settlement(
                        &SettlementInfoConfirm {
                            broker_id: BrokerId::new(&config.broker_id).unwrap_or_default(),
                            investor_id: InvestorId::new(&config.user_id).unwrap_or_default(),
                            confirm_date: DateText::default(),
                            confirm_time: TimeText::default(),
                            settlement_id: 0,
                            account_id: AccountId::new(&config.user_id).unwrap_or_default(),
                            currency_id: "CNY".to_owned(),
                        },
                        request_id,
                    )
                });
                self.pending_ready = Some((front_id, session_id, trading_day));
            }
            TraderEvent::SettlementConfirmed => {
                match (self.handshake.take(), self.pending_ready.take()) {
                    // 首次就绪：结束握手
                    (Some(handshake), Some(ready)) => {
                        self.mark_ready(ready.0, ready.1, &ready.2);
                        let _ = handshake.send(Ok(ready));
                    }
                    // 断线重连后就绪：上报恢复事件
                    (None, Some((front_id, session_id, trading_day))) => {
                        self.mark_ready(front_id, session_id, &trading_day);
                        self.send_event(TraderSessionEvent::Recovered {
                            front_id,
                            session_id,
                            trading_day,
                        });
                    }
                    // 防御死角：握手未结束却没有在途的登录结果，作答失败避免 new() 干等到超时
                    (Some(handshake), None) => {
                        let _ = handshake.send(Err(TraderSessionError::Rejected(
                            -1,
                            "结算确认先于登录回报到达，会话流程异常".to_owned(),
                        )));
                    }
                    (None, None) => {}
                }
            }
            TraderEvent::Failed { error_id, message } => {
                if let Some(handshake) = self.handshake.take() {
                    let _ = handshake.send(Err(TraderSessionError::Rejected(error_id, message)));
                } else {
                    self.send_event(TraderSessionEvent::Error { error_id, message });
                }
            }
            TraderEvent::Disconnected(reason) => {
                self.session = None;
                self.send_event(TraderSessionEvent::Disconnected { reason });
            }
            TraderEvent::Error {
                request_id,
                error_id,
                message,
            } => self.send_event(into_session_query_error(request_id, error_id, message)),
            TraderEvent::ConversionFailed(message) => {
                self.send_event(TraderSessionEvent::ConversionFailed { message });
            }
            TraderEvent::Logout => {
                // 交易端登出语义是断开重连（注意事项 2）：即刻视为未就绪，Disconnected 随后到达
                self.session = None;
                self.send_event(TraderSessionEvent::Logout);
            }
            // 批次二：交易指令回报，无需 loop 决策，原样透传
            TraderEvent::Report(report) => self.send_event(report),
            TraderEvent::QueryRow {
                request_id,
                is_last,
                error_id,
                message,
                row,
            } => self.send_event(into_session_query_row(
                request_id, is_last, error_id, message, row,
            )),
        }
    }

    /// 就绪：记录当前会话并刷新共享身份（getter 读取即当前值，重连后不残留旧会话标识）。
    fn mark_ready(&mut self, front_id: i32, session_id: i32, trading_day: &str) {
        self.session = Some((front_id, session_id));
        let mut identity = self
            .identity
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        identity.front_id = front_id;
        identity.session_id = session_id;
        identity.trading_day.clear();
        identity.trading_day.push_str(trading_day);
    }

    /// 报单：填充会话身份与递增 OrderRef 后发送；未就绪或本地发送失败直接报错。
    fn submit_insert_order(
        &mut self,
        order: &mut InputOrder,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let reference = prepare_insert_order(&self.config, &mut self.next_order_ref, order)?;
        Self::send_checked(&mut self.next_request_id, "报单", |request_id| {
            self.api.insert_order(order, request_id)
        })?;
        Ok(reference)
    }

    /// 撤单：填充会话身份、当前 FrontID/SessionID 与递增 OrderActionRef 后发送。
    fn submit_cancel_order(
        &mut self,
        action: &mut InputOrderAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let assigned =
            prepare_cancel_order(&self.config, session, &mut self.next_action_ref, action);
        Self::send_checked(&mut self.next_request_id, "撤单", |request_id| {
            self.api.cancel_order(action, request_id)
        })?;
        Ok(assigned)
    }

    /// 批量撤单：与单笔撤单共用同一 OrderActionRef 递增序列。
    fn submit_batch_action(
        &mut self,
        action: &mut InputBatchOrderAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let assigned =
            prepare_batch_action(&self.config, session, &mut self.next_action_ref, action);
        Self::send_checked(&mut self.next_request_id, "批量撤单", |request_id| {
            self.api.batch_order_action(action, request_id)
        })?;
        Ok(assigned)
    }

    /// 行权申报：填充身份并分配 ExecOrderRef 后发送。
    fn submit_exec_order_insert(
        &mut self,
        order: &mut InputExecOrder,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        order.broker_id = broker_id;
        order.investor_id = investor_id;
        order.user_id = user_id;
        let reference = take_order_ref(&mut self.next_order_ref)?;
        order.exec_order_ref = reference;
        Self::send_checked(&mut self.next_request_id, "行权申报", |request_id| {
            self.api.exec_order_insert(order, request_id)
        })?;
        Ok(reference.as_str().to_owned())
    }

    /// 行权操作（撤销）：填充身份、当前 FrontID/SessionID 并分配递增的 ExecOrderActionRef。
    fn submit_exec_order_action(
        &mut self,
        action: &mut InputExecOrderAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        action.broker_id = broker_id;
        action.investor_id = investor_id;
        action.user_id = user_id;
        action.front_id = FrontId::new(session.0);
        action.session_id = SessionId::new(session.1);
        let assigned = take_action_ref(&mut self.next_action_ref);
        action.exec_order_action_ref = assigned;
        Self::send_checked(&mut self.next_request_id, "行权操作", |request_id| {
            self.api.exec_order_action(action, request_id)
        })?;
        Ok(assigned)
    }

    /// 期权自对冲申报：填充身份并分配 OptionSelfCloseRef 后发送。
    fn submit_option_self_close_insert(
        &mut self,
        order: &mut InputOptionSelfClose,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        order.broker_id = broker_id;
        order.investor_id = investor_id;
        order.user_id = user_id;
        let reference = take_order_ref(&mut self.next_order_ref)?;
        order.option_self_close_ref = reference;
        Self::send_checked(
            &mut self.next_request_id,
            "期权自对冲申报",
            |request_id| self.api.option_self_close_insert(order, request_id),
        )?;
        Ok(reference.as_str().to_owned())
    }

    /// 期权自对冲操作（撤销）：填充身份、当前 FrontID/SessionID 并分配递增的 OptionSelfCloseActionRef。
    fn submit_option_self_close_action(
        &mut self,
        action: &mut InputOptionSelfCloseAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        action.broker_id = broker_id;
        action.investor_id = investor_id;
        action.user_id = user_id;
        action.front_id = FrontId::new(session.0);
        action.session_id = SessionId::new(session.1);
        let assigned = take_action_ref(&mut self.next_action_ref);
        action.option_self_close_action_ref = assigned;
        Self::send_checked(
            &mut self.next_request_id,
            "期权自对冲操作",
            |request_id| self.api.option_self_close_action(action, request_id),
        )?;
        Ok(assigned)
    }

    /// 组合申请：填充身份、当前 FrontID/SessionID 并分配 CombActionRef 后发送。
    fn submit_comb_action_insert(
        &mut self,
        action: &mut InputCombAction,
    ) -> Result<String, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        action.broker_id = broker_id;
        action.investor_id = investor_id;
        action.user_id = user_id;
        action.front_id = FrontId::new(session.0);
        action.session_id = SessionId::new(session.1);
        let reference = take_order_ref(&mut self.next_order_ref)?;
        action.comb_action_ref = reference;
        Self::send_checked(&mut self.next_request_id, "组合申请", |request_id| {
            self.api.comb_action_insert(action, request_id)
        })?;
        Ok(reference.as_str().to_owned())
    }

    /// 报价申报：填充身份并分配 QuoteRef 后发送。
    fn submit_quote_insert(
        &mut self,
        quote: &mut InputQuote,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        quote.broker_id = broker_id;
        quote.investor_id = investor_id;
        quote.user_id = user_id;
        let reference = take_order_ref(&mut self.next_order_ref)?;
        quote.quote_ref = reference;
        Self::send_checked(&mut self.next_request_id, "报价申报", |request_id| {
            self.api.quote_insert(quote, request_id)
        })?;
        Ok(reference.as_str().to_owned())
    }

    /// 报价操作（撤销）：填充身份、当前 FrontID/SessionID 并分配递增的 QuoteActionRef。
    fn submit_quote_action(
        &mut self,
        action: &mut InputQuoteAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        action.broker_id = broker_id;
        action.investor_id = investor_id;
        action.user_id = user_id;
        action.front_id = FrontId::new(session.0);
        action.session_id = SessionId::new(session.1);
        let assigned = take_action_ref(&mut self.next_action_ref);
        action.quote_action_ref = assigned;
        Self::send_checked(&mut self.next_request_id, "报价操作", |request_id| {
            self.api.quote_action(action, request_id)
        })?;
        Ok(assigned)
    }

    /// 询价申报：填充身份并分配 ForQuoteRef 后发送。
    fn submit_for_quote_insert(
        &mut self,
        for_quote: &mut InputForQuote,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        for_quote.broker_id = broker_id;
        for_quote.investor_id = investor_id;
        for_quote.user_id = user_id;
        let reference = take_order_ref(&mut self.next_order_ref)?;
        for_quote.for_quote_ref = reference;
        Self::send_checked(&mut self.next_request_id, "询价申报", |request_id| {
            self.api.for_quote_insert(for_quote, request_id)
        })?;
        Ok(reference.as_str().to_owned())
    }

    /// 预埋单申报：填充身份并分配 OrderRef（与报单共用递增序列）后发送。
    fn submit_parked_order_insert(
        &mut self,
        order: &mut ParkedOrder,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        order.broker_id = broker_id;
        order.investor_id = investor_id;
        order.user_id = user_id;
        // ParkedOrderID 由 CTP 分配（注意事项 14），入参残留的旧值必须清空
        order.parked_order_id.clear();
        let reference = take_order_ref(&mut self.next_order_ref)?;
        order.order_ref = reference;
        Self::send_checked(&mut self.next_request_id, "预埋单申报", |request_id| {
            self.api.parked_order_insert(order, request_id)
        })?;
        Ok(reference.as_str().to_owned())
    }

    /// 预埋撤单：填充身份、当前 FrontID/SessionID 并分配递增的 OrderActionRef。
    fn submit_parked_order_action(
        &mut self,
        action: &mut ParkedOrderAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        let (broker_id, investor_id, user_id) = identity_fields(&self.config);
        action.broker_id = broker_id;
        action.investor_id = investor_id;
        action.user_id = user_id;
        // ParkedOrderActionID 由 CTP 分配（注意事项 14），入参残留的旧值必须清空
        action.parked_order_action_id.clear();
        action.front_id = FrontId::new(session.0);
        action.session_id = SessionId::new(session.1);
        let assigned = take_action_ref(&mut self.next_action_ref);
        action.order_action_ref = assigned;
        Self::send_checked(&mut self.next_request_id, "预埋撤单", |request_id| {
            self.api.cancel_parked_order(action, request_id)
        })?;
        Ok(assigned)
    }

    /// 删除预埋单：填充身份后发送，返回分配的 request_id（应答回声）。
    fn submit_remove_parked_order(
        &mut self,
        removed: &mut RemoveParkedOrder,
    ) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, _) = identity_fields(&self.config);
        removed.broker_id = broker_id;
        removed.investor_id = investor_id;
        Self::send_checked(&mut self.next_request_id, "删除预埋单", |request_id| {
            self.api.remove_parked_order(removed, request_id)
        })
    }

    /// 删除预埋撤单：填充身份后发送，返回分配的 request_id（应答回声）。
    fn submit_remove_parked_order_action(
        &mut self,
        removed: &mut RemoveParkedOrderAction,
    ) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let (broker_id, investor_id, _) = identity_fields(&self.config);
        removed.broker_id = broker_id;
        removed.investor_id = investor_id;
        Self::send_checked(
            &mut self.next_request_id,
            "删除预埋撤单",
            |request_id| self.api.remove_parked_order_action(removed, request_id),
        )
    }

    fn submit_set_offset(
        &mut self,
        setting: &mut InputOffsetSetting,
    ) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        prepare_offset_setting(&self.config, setting);
        Self::send_checked(&mut self.next_request_id, "对冲设置", |request_id| {
            self.api.set_offset(setting, request_id)
        })
    }

    fn submit_cancel_offset_setting(
        &mut self,
        setting: &mut InputOffsetSetting,
    ) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        prepare_offset_setting(&self.config, setting);
        Self::send_checked(
            &mut self.next_request_id,
            "撤销对冲设置",
            |request_id| self.api.cancel_offset_setting(setting, request_id),
        )
    }

    fn submit_spd_apply(
        &mut self,
        apply: &mut InputSpdApply,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let reference = prepare_spd_apply(&self.config, &mut self.next_order_ref, apply)?;
        Self::send_checked(&mut self.next_request_id, "SPD套利确认", |request_id| {
            self.api.spd_apply(apply, request_id)
        })?;
        Ok(reference)
    }

    fn submit_spd_apply_action(
        &mut self,
        action: &mut InputSpdApplyAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        prepare_spd_apply_action(&self.config, session, action);
        Self::send_checked(
            &mut self.next_request_id,
            "撤销SPD套利确认",
            |request_id| self.api.spd_apply_action(action, request_id),
        )
    }

    fn submit_hedge_cfm(
        &mut self,
        apply: &mut InputHedgeCfm,
    ) -> Result<String, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        let reference = prepare_hedge_cfm(&self.config, &mut self.next_order_ref, apply)?;
        Self::send_checked(&mut self.next_request_id, "套保确认", |request_id| {
            self.api.hedge_cfm(apply, request_id)
        })?;
        Ok(reference)
    }

    fn submit_hedge_cfm_action(
        &mut self,
        action: &mut InputHedgeCfmAction,
    ) -> Result<i32, TraderSessionError> {
        let session = self.session.ok_or(TraderSessionError::NotReady)?;
        prepare_hedge_cfm_action(&self.config, session, action);
        Self::send_checked(
            &mut self.next_request_id,
            "撤销套保确认",
            |request_id| self.api.hedge_cfm_action(action, request_id),
        )
    }

    /// 银期转账：填充业务功能码与身份后发送，返回分配的 request_id（应答回声）。
    fn submit_transfer(
        &mut self,
        direction: TransferDirection,
        transfer: &mut ReqTransfer,
    ) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        prepare_transfer(&self.config, direction, transfer)?;
        Self::send_checked(
            &mut self.next_request_id,
            "银期转账",
            |request_id| match direction {
                TransferDirection::ToFuture => self.api.bank_to_future(transfer, request_id),
                TransferDirection::ToBank => self.api.future_to_bank(transfer, request_id),
            },
        )
    }

    /// 查询银行余额：填充身份后发送，返回分配的 request_id（应答回声）。
    fn submit_bank_balance_query(
        &mut self,
        query: &mut ReqQueryAccount,
    ) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        prepare_query_account(&self.config, query)?;
        Self::send_checked(
            &mut self.next_request_id,
            "查询银行余额",
            |request_id| self.api.query_bank_balance(query, request_id),
        )
    }

    /// 查询：填充会话身份、分配 request_id 并发送；响应逐行公开，由调用方聚合。
    fn submit_query(&mut self, mut kind: QueryKind) -> Result<i32, TraderSessionError> {
        if self.session.is_none() {
            return Err(TraderSessionError::NotReady);
        }
        fill_query_identity(&self.config, &mut kind);
        let request_id = self.next_request_id();
        let result = match &kind {
            QueryKind::Order(request) => self.api.query_order(request, request_id),
            QueryKind::Trade(request) => self.api.query_trade(request, request_id),
            QueryKind::InvestorPosition(request) => {
                self.api.query_investor_position(request, request_id)
            }
            QueryKind::InvestorPositionDetail(request) => {
                self.api.query_investor_position_detail(request, request_id)
            }
            QueryKind::TradingAccount(request) => {
                self.api.query_trading_account(request, request_id)
            }
            QueryKind::Instrument(request) => self.api.query_instrument(request, request_id),
            QueryKind::Exchange(request) => self.api.query_exchange(request, request_id),
            QueryKind::Product(request) => self.api.query_product(request, request_id),
            QueryKind::DepthMarketData(request) => {
                self.api.query_depth_market_data(request, request_id)
            }
            QueryKind::InvestorPositionCombineDetail(request) => self
                .api
                .query_investor_position_combine_detail(request, request_id),
            QueryKind::CombLeg(request) => self.api.query_comb_leg(request, request_id),
            QueryKind::Investor(request) => self.api.query_investor(request, request_id),
            QueryKind::SettlementInfo(request) => {
                self.api.query_settlement_info(request, request_id)
            }
            QueryKind::ExecOrder(request) => self.api.query_exec_order(request, request_id),
            QueryKind::OptionSelfClose(request) => {
                self.api.query_option_self_close(request, request_id)
            }
            QueryKind::CombAction(request) => self.api.query_comb_action(request, request_id),
            QueryKind::Quote(request) => self.api.query_quote(request, request_id),
            QueryKind::ForQuote(request) => self.api.query_for_quote(request, request_id),
            QueryKind::ParkedOrder(request) => self.api.query_parked_order(request, request_id),
            QueryKind::ParkedOrderAction(request) => {
                self.api.query_parked_actions(request, request_id)
            }
            QueryKind::TransferBank(request) => self.api.query_transfer_bank(request, request_id),
            QueryKind::TransferSerial(request) => {
                self.api.query_transfer_serial(request, request_id)
            }
            QueryKind::AccountRegister(request) => {
                self.api.query_account_register(request, request_id)
            }
            QueryKind::ContractBank(request) => self.api.query_contract_bank(request, request_id),
            QueryKind::MaxOrderVolume(request) => {
                self.api.query_max_order_volume(request, request_id)
            }
            QueryKind::InstrumentMarginRate(request) => {
                self.api.query_instrument_margin_rate(request, request_id)
            }
            QueryKind::InstrumentCommissionRate(request) => self
                .api
                .query_instrument_commission_rate(request, request_id),
            QueryKind::MMInstrumentCommissionRate(request) => self
                .api
                .query_mm_instrument_commission_rate(request, request_id),
            QueryKind::MMOptionInstrCommRate(request) => self
                .api
                .query_mm_option_instr_comm_rate(request, request_id),
            QueryKind::InstrumentOrderCommRate(request) => self
                .api
                .query_instrument_order_comm_rate(request, request_id),
            QueryKind::OptionInstrTradeCost(request) => {
                self.api.query_option_instr_trade_cost(request, request_id)
            }
            QueryKind::OptionInstrCommRate(request) => {
                self.api.query_option_instr_comm_rate(request, request_id)
            }
            QueryKind::ExchangeMarginRate(request) => {
                self.api.query_exchange_margin_rate(request, request_id)
            }
            QueryKind::ExchangeMarginRateAdjust(request) => self
                .api
                .query_exchange_margin_rate_adjust(request, request_id),
            QueryKind::ExchangeRate(request) => self.api.query_exchange_rate(request, request_id),
            QueryKind::ProductExchRate(request) => {
                self.api.query_product_exch_rate(request, request_id)
            }
            QueryKind::ProductGroup(request) => self.api.query_product_group(request, request_id),
            QueryKind::InvestorProductGroupMargin(request) => self
                .api
                .query_investor_product_group_margin(request, request_id),
            QueryKind::CombInstrumentGuard(request) => {
                self.api.query_comb_instrument_guard(request, request_id)
            }
            QueryKind::MaxOrderVolumeWithPrice(request) => self
                .api
                .query_max_order_volume_with_price(request, request_id),
            QueryKind::SPBMFutureParameter(request) => {
                self.api.query_spbm_future_parameter(request, request_id)
            }
            QueryKind::SPBMOptionParameter(request) => {
                self.api.query_spbm_option_parameter(request, request_id)
            }
            QueryKind::SPBMIntraParameter(request) => {
                self.api.query_spbm_intra_parameter(request, request_id)
            }
            QueryKind::SPBMInterParameter(request) => {
                self.api.query_spbm_inter_parameter(request, request_id)
            }
            QueryKind::SPBMPortfDefinition(request) => {
                self.api.query_spbm_portf_definition(request, request_id)
            }
            QueryKind::SPBMInvestorPortfDef(request) => {
                self.api.query_spbm_investor_portf_def(request, request_id)
            }
            QueryKind::SPBMAddOnInterParameter(request) => self
                .api
                .query_spbm_add_on_inter_parameter(request, request_id),
            QueryKind::InvestorPortfMarginRatio(request) => self
                .api
                .query_investor_portf_margin_ratio(request, request_id),
            QueryKind::InvestorProdSPBMDetail(request) => self
                .api
                .query_investor_prod_spbm_detail(request, request_id),
            QueryKind::SPMMInstParam(request) => {
                self.api.query_spmm_inst_param(request, request_id)
            }
            QueryKind::SPMMProductParam(request) => {
                self.api.query_spmm_product_param(request, request_id)
            }
            QueryKind::InvestorCommoditySPMMMargin(request) => self
                .api
                .query_investor_commodity_spmm_margin(request, request_id),
            QueryKind::InvestorCommodityGroupSPMMMargin(request) => self
                .api
                .query_investor_commodity_group_spmm_margin(request, request_id),
            QueryKind::RCAMSCombProductInfo(request) => {
                self.api.query_rcams_comb_product_info(request, request_id)
            }
            QueryKind::RCAMSInstrParameter(request) => {
                self.api.query_rcams_instr_parameter(request, request_id)
            }
            QueryKind::RCAMSIntraParameter(request) => {
                self.api.query_rcams_intra_parameter(request, request_id)
            }
            QueryKind::RCAMSInterParameter(request) => {
                self.api.query_rcams_inter_parameter(request, request_id)
            }
            QueryKind::RCAMSShortOptAdjustParam(request) => self
                .api
                .query_rcams_short_opt_adjust_param(request, request_id),
            QueryKind::RCAMSInvestorCombPosition(request) => self
                .api
                .query_rcams_investor_comb_position(request, request_id),
            QueryKind::InvestorProdRCAMSMargin(request) => self
                .api
                .query_investor_prod_rcams_margin(request, request_id),
            QueryKind::RULEInstrParameter(request) => {
                self.api.query_rule_instr_parameter(request, request_id)
            }
            QueryKind::RULEIntraParameter(request) => {
                self.api.query_rule_intra_parameter(request, request_id)
            }
            QueryKind::RULEInterParameter(request) => {
                self.api.query_rule_inter_parameter(request, request_id)
            }
            QueryKind::InvestorProdRULEMargin(request) => self
                .api
                .query_investor_prod_rule_margin(request, request_id),
            QueryKind::InvestorPortfSetting(request) => {
                self.api.query_investor_portf_setting(request, request_id)
            }
            QueryKind::RiskSettleInvestPosition(request) => self
                .api
                .query_risk_settle_invest_position(request, request_id),
            QueryKind::RiskSettleProductStatus(request) => self
                .api
                .query_risk_settle_product_status(request, request_id),
            QueryKind::InvestorInfoCommRec(request) => {
                self.api.query_investor_info_comm_rec(request, request_id)
            }
            QueryKind::OffsetSetting(request) => self.api.offset_settings(request, request_id),
            QueryKind::SpdApply(request) => self.api.query_spd_apply(request, request_id),
            QueryKind::HedgeCfm(request) => self.api.query_hedge_cfm(request, request_id),
            QueryKind::InstrTradeLimit(request) => {
                self.api.query_instr_trade_limit(request, request_id)
            }
            QueryKind::ClassifiedInstrument(request) => {
                self.api.query_classified_instrument(request, request_id)
            }
            QueryKind::CombPromotionParam(request) => {
                self.api.query_comb_promotion_param(request, request_id)
            }
        };
        let code = result.map_err(|error| TraderSessionError::Request(error.to_string()))?;
        check_send_code(code, "查询")?;
        Ok(request_id)
    }

    /// 发送一个流程请求；失败时首登结束握手、重连上报错误事件（error_id -1 表示本地发送失败）。
    fn send_request(
        &mut self,
        action: &str,
        request: impl FnOnce(&TraderApi, &TraderConfig, i32) -> Result<i32, crate::ConversionError>,
    ) {
        let request_id = self.next_request_id();
        if let Some(message) = error_message(request(&self.api, &self.config, request_id), action) {
            let mut handshake = self.handshake.take();
            report_request_failure(&mut handshake, message, |event| self.send_event(event));
        }
    }

    pub(super) fn send_checked(
        next_request_id: &mut i32,
        what: &str,
        request: impl FnOnce(i32) -> Result<i32, crate::ConversionError>,
    ) -> Result<i32, TraderSessionError> {
        let request_id = *next_request_id;
        *next_request_id = request_id.checked_add(1).unwrap_or(1);
        let code =
            request(request_id).map_err(|error| TraderSessionError::Request(error.to_string()))?;
        check_send_code(code, what)?;
        Ok(request_id)
    }

    /// 分配自增请求 ID；i32 耗尽后回到 1（无对账语义，回绕无害）。
    fn next_request_id(&mut self) -> i32 {
        let id = self.next_request_id;
        self.next_request_id = id.checked_add(1).unwrap_or(1);
        id
    }

    /// 主动重连：利用官方记载的"登出=断开重连"行为触发链路重建。
    /// 随后链路自动走 Disconnected → Connected → 认证链 → Recovered（FAQ 的 logout/重连条目）。
    /// 本地发送失败不静默，转成错误事件上报。
    fn reconnect(&mut self) {
        let request = UserLogout {
            broker_id: BrokerId::new(&self.config.broker_id).unwrap_or_default(),
            user_id: UserId::new(&self.config.user_id).unwrap_or_default(),
        };
        let request_id = self.next_request_id();
        let failure = match self.api.logout(&request, request_id) {
            Ok(0) => None,
            Ok(code) => Some(format!("主动重连请求发送失败，CTP 返回码 {code}")),
            Err(error) => Some(format!("主动重连请求转换失败：{error}")),
        };
        if let Some(message) = failure {
            self.send_event(TraderSessionEvent::Error {
                error_id: -1,
                message,
            });
        }
    }

    /// 非阻塞投递事件；队列满时丢弃并计数。
    fn send_event(&mut self, event: TraderSessionEvent) {
        if self.events.try_send(event).is_err() {
            self.dropped.fetch_add(1, Ordering::AcqRel);
        }
    }
}

/// 构造登录请求（交易端校验凭据，密码必填）。
fn login_request(config: &TraderConfig) -> ReqUserLogin {
    ReqUserLogin {
        broker_id: BrokerId::new(&config.broker_id).unwrap_or_default(),
        user_id: UserId::new(&config.user_id).unwrap_or_default(),
        password: config.password.clone(),
        ..Default::default()
    }
}

/// 从配置构造会话身份字段；无效值降级为空（登录阶段前置已校验过凭据）。
pub(super) fn identity_fields(config: &TraderConfig) -> (BrokerId, InvestorId, UserId) {
    (
        BrokerId::new(&config.broker_id).unwrap_or_default(),
        InvestorId::new(&config.user_id).unwrap_or_default(),
        UserId::new(&config.user_id).unwrap_or_default(),
    )
}

pub(super) fn prepare_offset_setting(config: &TraderConfig, setting: &mut InputOffsetSetting) {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    setting.broker_id = broker_id;
    setting.investor_id = investor_id;
    setting.user_id = user_id;
}

pub(super) fn prepare_spd_apply(
    config: &TraderConfig,
    next_ref: &mut u64,
    apply: &mut InputSpdApply,
) -> Result<String, TraderSessionError> {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    apply.broker_id = broker_id;
    apply.investor_id = investor_id;
    apply.user_id = user_id;
    let reference = take_order_ref(next_ref)?;
    apply.order_ref = reference;
    Ok(reference.as_str().to_owned())
}

pub(super) fn prepare_hedge_cfm(
    config: &TraderConfig,
    next_ref: &mut u64,
    apply: &mut InputHedgeCfm,
) -> Result<String, TraderSessionError> {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    apply.broker_id = broker_id;
    apply.investor_id = investor_id;
    apply.user_id = user_id;
    let reference = take_order_ref(next_ref)?;
    apply.order_ref = reference;
    Ok(reference.as_str().to_owned())
}

pub(super) fn prepare_spd_apply_action(
    config: &TraderConfig,
    session: (i32, i32),
    action: &mut InputSpdApplyAction,
) {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    action.broker_id = broker_id;
    action.investor_id = investor_id;
    action.user_id = user_id;
    action.front_id = FrontId::new(session.0);
    action.session_id = SessionId::new(session.1);
}

pub(super) fn prepare_hedge_cfm_action(
    config: &TraderConfig,
    session: (i32, i32),
    action: &mut InputHedgeCfmAction,
) {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    action.broker_id = broker_id;
    action.investor_id = investor_id;
    action.user_id = user_id;
    action.front_id = FrontId::new(session.0);
    action.session_id = SessionId::new(session.1);
}

/// 取下一个 OrderRef：严格递增的十进制文本（CTP 要求会话内递增，否则 ErrorID 22「不允许重复报单」）。
pub(super) fn take_order_ref(next_ref: &mut u64) -> Result<OrderRef, TraderSessionError> {
    let value = *next_ref;
    *next_ref += 1;
    OrderRef::new(&value.to_string())
        .map_err(|error| TraderSessionError::Request(format!("分配报单引用失败：{error}")))
}

/// 为报单填充会话身份并分配严格递增的 OrderRef；返回分配的引用文本。
pub(super) fn prepare_insert_order(
    config: &TraderConfig,
    next_ref: &mut u64,
    order: &mut InputOrder,
) -> Result<String, TraderSessionError> {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    order.broker_id = broker_id;
    order.investor_id = investor_id;
    order.user_id = user_id;
    let reference = take_order_ref(next_ref)?;
    order.order_ref = reference;
    Ok(reference.as_str().to_owned())
}

/// 取下一个 OrderActionRef：i32 递增，耗尽后回到 1（会话内不会打满）。
pub(super) fn take_action_ref(next_ref: &mut i32) -> i32 {
    let value = *next_ref;
    *next_ref = value.checked_add(1).unwrap_or(1);
    value
}

/// 为撤单填充会话身份、当前 (FrontID, SessionID) 并分配递增的 OrderActionRef；返回分配的引用。
pub(super) fn prepare_cancel_order(
    config: &TraderConfig,
    session: (i32, i32),
    next_ref: &mut i32,
    action: &mut InputOrderAction,
) -> i32 {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    action.broker_id = broker_id;
    action.investor_id = investor_id;
    action.user_id = user_id;
    action.front_id = FrontId::new(session.0);
    action.session_id = SessionId::new(session.1);
    let assigned = take_action_ref(next_ref);
    action.order_action_ref = assigned;
    assigned
}

/// 为批量撤单填充会话身份、当前 (FrontID, SessionID) 并分配递增的 OrderActionRef；返回分配的引用。
pub(super) fn prepare_batch_action(
    config: &TraderConfig,
    session: (i32, i32),
    next_ref: &mut i32,
    action: &mut InputBatchOrderAction,
) -> i32 {
    let (broker_id, investor_id, user_id) = identity_fields(config);
    action.broker_id = broker_id;
    action.investor_id = investor_id;
    action.user_id = user_id;
    action.front_id = FrontId::new(session.0);
    action.session_id = SessionId::new(session.1);
    let assigned = take_action_ref(next_ref);
    action.order_action_ref = assigned;
    assigned
}

pub(super) fn report_request_failure(
    handshake: &mut Option<SyncSender<Handshake>>,
    message: String,
    emit: impl FnOnce(TraderSessionEvent),
) {
    match handshake.take() {
        Some(handshake) => {
            let _ = handshake.send(Err(TraderSessionError::Request(message)));
        }
        None => emit(TraderSessionEvent::Error {
            error_id: -1,
            message,
        }),
    }
}

/// CTP 请求返回码：0 为发送成功；负值为本地失败（-1 网络连接失败 / -2 未处理请求超限 / -3 流控拒绝）。
fn check_send_code(code: i32, what: &str) -> Result<(), TraderSessionError> {
    if code == 0 {
        Ok(())
    } else {
        Err(TraderSessionError::Request(format!(
            "{what}发送失败，CTP 返回码 {code}"
        )))
    }
}

/// 为查询请求填充会话身份（broker_id/investor_id 覆盖入参，与批次二报单同一约定）。
pub(super) fn fill_query_identity(config: &TraderConfig, kind: &mut QueryKind) {
    let (broker_id, investor_id, _) = identity_fields(config);
    match kind {
        QueryKind::Order(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::Trade(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorPosition(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorPositionDetail(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::TradingAccount(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
            request.account_id = AccountId::new(&config.user_id).unwrap_or_default();
        }
        QueryKind::InvestorPositionCombineDetail(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::Investor(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::SettlementInfo(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::ExecOrder(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::OptionSelfClose(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::CombAction(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::Quote(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::ForQuote(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::ParkedOrder(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::ParkedOrderAction(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::TransferSerial(request) => {
            request.broker_id = broker_id;
            request.account_id = AccountId::new(&config.user_id).unwrap_or_default();
        }
        QueryKind::AccountRegister(request) => {
            request.broker_id = broker_id;
            request.account_id = AccountId::new(&config.user_id).unwrap_or_default();
        }
        QueryKind::ContractBank(request) => {
            request.broker_id = broker_id;
        }
        QueryKind::MaxOrderVolume(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InstrumentMarginRate(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InstrumentCommissionRate(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::MMInstrumentCommissionRate(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::MMOptionInstrCommRate(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InstrumentOrderCommRate(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::OptionInstrTradeCost(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::OptionInstrCommRate(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::ExchangeMarginRate(request) => {
            request.broker_id = broker_id;
        }
        QueryKind::ExchangeMarginRateAdjust(request) => {
            request.broker_id = broker_id;
        }
        QueryKind::ExchangeRate(request) => {
            request.broker_id = broker_id;
        }
        QueryKind::InvestorProductGroupMargin(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::CombInstrumentGuard(request) => {
            request.broker_id = broker_id;
        }
        QueryKind::MaxOrderVolumeWithPrice(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::SPBMInvestorPortfDef(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorPortfMarginRatio(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorProdSPBMDetail(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorCommoditySPMMMargin(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorCommodityGroupSPMMMargin(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::RCAMSInvestorCombPosition(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorProdRCAMSMargin(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorProdRULEMargin(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorPortfSetting(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::RiskSettleInvestPosition(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::InvestorInfoCommRec(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::OffsetSetting(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::SpdApply(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        QueryKind::HedgeCfm(request) => {
            request.broker_id = broker_id;
            request.investor_id = investor_id;
        }
        // 其余查询不含身份字段
        QueryKind::Instrument(_)
        | QueryKind::Exchange(_)
        | QueryKind::Product(_)
        | QueryKind::DepthMarketData(_)
        | QueryKind::CombLeg(_)
        | QueryKind::TransferBank(_)
        | QueryKind::ProductExchRate(_)
        | QueryKind::ProductGroup(_)
        | QueryKind::SPBMFutureParameter(_)
        | QueryKind::SPBMOptionParameter(_)
        | QueryKind::SPBMIntraParameter(_)
        | QueryKind::SPBMInterParameter(_)
        | QueryKind::SPBMPortfDefinition(_)
        | QueryKind::SPBMAddOnInterParameter(_)
        | QueryKind::SPMMInstParam(_)
        | QueryKind::SPMMProductParam(_)
        | QueryKind::RCAMSCombProductInfo(_)
        | QueryKind::RCAMSInstrParameter(_)
        | QueryKind::RCAMSIntraParameter(_)
        | QueryKind::RCAMSInterParameter(_)
        | QueryKind::RCAMSShortOptAdjustParam(_)
        | QueryKind::RULEInstrParameter(_)
        | QueryKind::RULEIntraParameter(_)
        | QueryKind::RULEInterParameter(_)
        | QueryKind::RiskSettleProductStatus(_)
        | QueryKind::InstrTradeLimit(_)
        | QueryKind::ClassifiedInstrument(_)
        | QueryKind::CombPromotionParam(_) => {}
    }
}

/// 银期转账方向（决定业务功能码）。
#[derive(Clone, Copy)]
pub(super) enum TransferDirection {
    /// 银行转期货（入金），业务功能码 202001。
    ToFuture,
    /// 期货转银行（出金），业务功能码 202002。
    ToBank,
}

/// 为转账请求填充业务功能码与会话身份（`broker_id`/`account_id`/`user_id` 覆盖入参）。
/// 资金密码必须手动传入：空密码在本地直接拒绝（不经柜台）。
pub(super) fn prepare_transfer(
    config: &TraderConfig,
    direction: TransferDirection,
    transfer: &mut ReqTransfer,
) -> Result<(), TraderSessionError> {
    if transfer.password.is_empty() {
        return Err(TraderSessionError::Request(
            "资金密码必须手动传入（password 字段为空）".to_owned(),
        ));
    }
    transfer.trade_code = match direction {
        TransferDirection::ToFuture => codes::VIREMENT_FUTURE_BANK_TO_FUTURE.to_owned(),
        TransferDirection::ToBank => codes::VIREMENT_FUTURE_FUTURE_TO_BANK.to_owned(),
    };
    transfer.broker_id = BrokerId::new(&config.broker_id).unwrap_or_default();
    transfer.account_id = AccountId::new(&config.user_id).unwrap_or_default();
    transfer.user_id = UserId::new(&config.user_id).unwrap_or_default();
    Ok(())
}

/// 为银行余额查询填充会话身份（`broker_id`/`account_id`/`user_id` 覆盖入参）。
/// 资金密码必须手动传入：空密码在本地直接拒绝（不经柜台）。
pub(super) fn prepare_query_account(
    config: &TraderConfig,
    query: &mut ReqQueryAccount,
) -> Result<(), TraderSessionError> {
    if query.password.is_empty() {
        return Err(TraderSessionError::Request(
            "资金密码必须手动传入（password 字段为空）".to_owned(),
        ));
    }
    query.broker_id = BrokerId::new(&config.broker_id).unwrap_or_default();
    query.account_id = AccountId::new(&config.user_id).unwrap_or_default();
    query.user_id = UserId::new(&config.user_id).unwrap_or_default();
    Ok(())
}
