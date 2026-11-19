#[path = "common/config.rs"]
mod config;

use ctp_base::{
    BrokerId, DateText, DepthMarketData, DisconnectReason, MdApi, MdConfig, MdSpi, ReqUserLogin,
    RspInfo, RspUserLogin, UserId,
};
use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(30);
const FLOW_PATH: &str = "./flow/";

#[derive(Default)]
struct ExampleState {
    connected: bool,
    logged_in: bool,
    ticks: usize,
    failure: Option<String>,
}

struct ExampleSpi {
    shared: Arc<(Mutex<ExampleState>, Condvar)>,
}

impl ExampleSpi {
    fn update(&self, update: impl FnOnce(&mut ExampleState)) {
        let (lock, changed) = &*self.shared;
        let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
        update(&mut state);
        changed.notify_all();
    }
}

impl MdSpi for ExampleSpi {
    fn connected(&mut self) {
        println!("行情前置已连接");
        self.update(|state| state.connected = true);
    }

    fn disconnected(&mut self, reason: DisconnectReason) {
        println!("行情前置已断开，原因：{reason}");
        self.update(|state| state.failure = Some(format!("行情前置断开：{reason}")));
    }

    fn login_response(
        &mut self,
        _rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.update(|state| match rsp_info {
            Some(info) if info.error_id == 0 => {
                println!("登录成功");
                state.logged_in = true;
            }
            Some(info) => {
                let message = format!("登录失败 {}：{}", info.error_id, info.error_msg);
                println!("{message}");
                state.failure = Some(message);
            }
            None => state.failure = Some("登录响应缺少错误信息".into()),
        });
    }

    fn error(&mut self, rsp_info: Option<RspInfo>, _request_id: i32, _is_last: bool) {
        if let Some(info) = rsp_info {
            eprintln!("CTP 错误 {}：{}", info.error_id, info.error_msg);
            self.update(|state| {
                state.failure = Some(format!("CTP 错误 {}：{}", info.error_id, info.error_msg));
            });
        }
    }

    fn subscribe_response(
        &mut self,
        _specific_instrument: Option<ctp_base::SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(info) = rsp_info.filter(|info| info.error_id != 0) {
            eprintln!("订阅失败 {}：{}", info.error_id, info.error_msg);
            self.update(|state| {
                state.failure = Some(format!("订阅失败 {}：{}", info.error_id, info.error_msg));
            });
        }
    }

    fn tick(&mut self, depth_market_data: Option<DepthMarketData>) {
        if let Some(data) = depth_market_data {
            println!(
                "{} {} 最新价={} 成交量={} 时间={}.{:03}",
                data.action_day,
                data.instrument_id,
                data.last_price,
                data.volume,
                data.update_time,
                data.update_millis
            );
            self.update(|state| state.ticks += 1);
        }
    }
}

fn wait_for(
    shared: &Arc<(Mutex<ExampleState>, Condvar)>,
    description: &str,
    mut ready: impl FnMut(&ExampleState) -> bool,
) -> Result<(), Box<dyn Error>> {
    let (lock, changed) = &**shared;
    let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if let Some(message) = state.failure.take() {
            return Err(format!("{description}失败：{message}").into());
        }
        if ready(&state) {
            return Ok(());
        }
        let now = Instant::now();
        if now >= deadline {
            return Err(format!("等待{description}超过 {} 秒", TIMEOUT.as_secs()).into());
        }
        let (next, timeout) = changed
            .wait_timeout(state, deadline - now)
            .unwrap_or_else(|error| error.into_inner());
        state = next;
        if timeout.timed_out() && !ready(&state) {
            return Err(format!("等待{description}超过 {} 秒", TIMEOUT.as_secs()).into());
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(FLOW_PATH)?;
    let ctp = config::CtpConfig::load()?;
    let shared = Arc::new((Mutex::new(ExampleState::default()), Condvar::new()));
    let api = MdApi::new(
        MdConfig::new(FLOW_PATH),
        Box::new(ExampleSpi {
            shared: shared.clone(),
        }),
    )?;

    let instrument = std::env::var("CTP_INSTRUMENT").unwrap_or_else(|_| ctp.instrument.clone());
    let tick_limit = std::env::var("CTP_TICK_LIMIT")
        .unwrap_or_else(|_| "10".into())
        .parse::<usize>()?;
    if tick_limit == 0 {
        return Err("CTP_TICK_LIMIT 必须大于 0".into());
    }

    println!("CTP API 版本：{}", api.version());
    let market_front =
        std::env::var("CTP_MARKET_FRONT").unwrap_or_else(|_| ctp.market_front.clone());
    println!("正在连接行情前置：{market_front}");
    api.register_front(&market_front)?;
    api.init();
    wait_for(&shared, "行情前置连接", |state| state.connected)?;

    let request = ReqUserLogin {
        trading_day: DateText::default(),
        broker_id: BrokerId::new(&ctp.broker_id)?,
        user_id: UserId::new(&ctp.user_id)?,
        password: ctp.password.clone(),
        user_product_info: String::new(),
        interface_product_info: String::new(),
        protocol_info: String::new(),
        mac_address: String::new(),
        one_time_password: String::new(),
        login_remark: String::new(),
        client_ip_port: 0,
        client_ip_address: String::new(),
        sms_code: String::new(),
    };
    let code = api.login(&request, 1)?;
    if code != 0 {
        return Err(format!("登录请求返回码：{code}").into());
    }
    wait_for(&shared, "行情登录", |state| state.logged_in)?;
    println!("当前交易日：{}", api.trading_day());

    let code = api.subscribe(&[instrument.as_str()])?;
    if code != 0 {
        return Err(format!("订阅请求返回码：{code}").into());
    }
    println!("已订阅 {instrument}，等待 {tick_limit} 笔行情……");
    wait_for(&shared, "行情", |state| state.ticks >= tick_limit)?;

    api.close();
    println!("行情 API 已关闭");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
