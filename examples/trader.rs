#[path = "common/config.rs"]
mod config;

use ctp_base::{
    AccountId, ActionFlag, BrokerId, BusinessUnit, CombHedgeFlag, CombOffsetFlag,
    ContingentCondition, ConversionError, DateText, DepthMarketData, Direction, DisconnectReason,
    ExchangeId, ExchangeInstrumentId, ForceCloseReason, FrontId, HedgeFlag, InputOrder,
    InputOrderAction, Instrument, InstrumentId, InvestUnitId, InvestorId, MdApi, MdConfig, MdSpi,
    OffsetFlag, Order, OrderAction, OrderPriceType, OrderRef, OrderStatus, OrderSysId,
    QryInstrument, ReqAuthenticate, ReqUserLogin, RequestId, ResumeType, RspAuthenticate, RspInfo,
    RspUserLogin, SessionId, SettlementInfoConfirm, TimeCondition, TimeText, Trade, TraderApi,
    TraderConfig, TraderSpi, UserId, VolumeCondition,
};
use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const TIMEOUT: Duration = Duration::from_secs(30);
const FLOW_PATH: &str = "./flow/";

#[derive(Default)]
struct TraderState {
    connected: bool,
    authenticated: bool,
    login: Option<(FrontId, SessionId)>,
    settlement_confirmed: bool,
    orders: Vec<Order>,
    executions: usize,
    order_action_failure: Option<String>,
    instrument: Option<Instrument>,
    failure: Option<String>,
}

struct SimnowTraderSpi {
    shared: Arc<(Mutex<TraderState>, Condvar)>,
    order_ref: OrderRef,
}

impl SimnowTraderSpi {
    fn update(&self, update: impl FnOnce(&mut TraderState)) {
        let (lock, changed) = &*self.shared;
        let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
        update(&mut state);
        changed.notify_all();
    }

    fn success(info: Option<RspInfo>) -> Result<(), String> {
        match info {
            Some(info) if info.error_id == 0 => Ok(()),
            Some(info) => Err(info.error_msg),
            None => Err("CTP 返回未知错误".into()),
        }
    }
}

impl TraderSpi for SimnowTraderSpi {
    fn connected(&mut self) {
        self.update(|state| state.connected = true);
    }

    fn disconnected(&mut self, reason: DisconnectReason) {
        self.update(|state| state.failure = Some(format!("交易前置断开：{reason}")));
    }

    fn conversion_error(&mut self, error: ConversionError) {
        self.update(|state| state.failure = Some(format!("回调转换失败：{error}")));
    }

    fn authenticate_response(
        &mut self,
        _rsp_authenticate: Option<RspAuthenticate>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match Self::success(rsp_info) {
            Ok(()) => self.update(|state| state.authenticated = true),
            Err(message) => self.update(|state| state.failure = Some(message)),
        }
    }

    fn login_response(
        &mut self,
        rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match (rsp_user_login, rsp_info) {
            (Some(login), Some(info)) if info.error_id == 0 => {
                self.update(|state| state.login = Some((login.front_id, login.session_id)));
            }
            (_, info) => self.update(|state| {
                state.failure = Some(info.map_or_else(|| "登录失败".into(), |info| info.error_msg));
            }),
        }
    }

    fn settlement_confirm_response(
        &mut self,
        _settlement_info_confirm: Option<SettlementInfoConfirm>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match Self::success(rsp_info) {
            Ok(()) => self.update(|state| state.settlement_confirmed = true),
            Err(message) => self.update(|state| state.failure = Some(message)),
        }
    }

    fn settlement_status(
        &mut self,
        settlement_info_confirm: Option<SettlementInfoConfirm>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match (settlement_info_confirm, rsp_info) {
            (Some(_), Some(info)) if info.error_id == 0 => {
                self.update(|state| state.settlement_confirmed = true);
            }
            (_, Some(info)) => self.update(|state| {
                state.failure = Some(format!(
                    "结算状态错误 {}：{}",
                    info.error_id, info.error_msg
                ));
            }),
            (_, None) => self.update(|state| {
                state.failure = Some("结算状态未确认或返回为空".into());
            }),
        }
    }

    fn order_insert_error(&mut self, _input_order: Option<InputOrder>, rsp_info: Option<RspInfo>) {
        self.update(|state| {
            state.failure =
                Some(rsp_info.map_or_else(|| "下单被拒绝".into(), |info| info.error_msg));
        });
    }

    fn order_action_error(
        &mut self,
        _order_action: Option<OrderAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.update(|state| {
            state.order_action_failure =
                Some(rsp_info.map_or_else(|| "撤单被拒绝".into(), |info| info.error_msg));
        });
    }

    fn order_response(
        &mut self,
        _order: Option<Order>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        is_last: bool,
    ) {
        if is_last && let Some(info) = rsp_info.filter(|info| info.error_id != 0) {
            self.update(|state| state.failure = Some(info.error_msg));
        }
    }

    fn trade_response(
        &mut self,
        _trade: Option<Trade>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        is_last: bool,
    ) {
        if is_last && let Some(info) = rsp_info.filter(|info| info.error_id != 0) {
            self.update(|state| state.failure = Some(info.error_msg));
        }
    }

    fn instrument_response(
        &mut self,
        instrument: Option<Instrument>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(instrument) = instrument {
            self.update(|state| state.instrument = Some(instrument));
        } else {
            self.update(|state| {
                state.failure =
                    Some(rsp_info.map_or_else(|| "合约查询失败".into(), |info| info.error_msg));
            });
        }
    }

    fn order_update(&mut self, order: Option<Order>) {
        if let Some(order) = order.filter(|order| order.order_ref == self.order_ref) {
            println!(
                "报单状态：{:?}，OrderRef={}",
                order.order_status, order.order_ref
            );
            self.update(|state| state.orders.push(order));
        }
    }

    fn execution(&mut self, trade: Option<Trade>) {
        if let Some(trade) = trade.filter(|trade| trade.order_ref == self.order_ref) {
            println!(
                "成交回报：{} 价格={} 数量={}",
                trade.instrument_id, trade.price, trade.volume
            );
            self.update(|state| state.executions += 1);
        }
    }

    fn error(&mut self, rsp_info: Option<RspInfo>, _request_id: i32, _is_last: bool) {
        self.update(|state| {
            state.failure =
                Some(rsp_info.map_or_else(|| "CTP 返回未知错误".into(), |info| info.error_msg));
        });
    }
}

fn wait_for(
    shared: &Arc<(Mutex<TraderState>, Condvar)>,
    description: &str,
    mut predicate: impl FnMut(&TraderState) -> bool,
) -> Result<(), String> {
    let (lock, changed) = &**shared;
    let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if let Some(message) = state.failure.take() {
            return Err(format!("{description}失败：{message}"));
        }
        if predicate(&state) {
            return Ok(());
        }
        let now = Instant::now();
        if now >= deadline {
            return Err(format!("{description}超时"));
        }
        let (next, _) = changed
            .wait_timeout(state, deadline - now)
            .unwrap_or_else(|error| error.into_inner());
        state = next;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderOutcome {
    AllTraded,
    Canceled,
}

fn wait_for_order(
    shared: &Arc<(Mutex<TraderState>, Condvar)>,
    description: &str,
    canceling: bool,
) -> Result<OrderOutcome, String> {
    let (lock, changed) = &**shared;
    let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if let Some(order) = state.orders.iter().find(|order| {
            matches!(
                order.order_status,
                OrderStatus::AllTraded | OrderStatus::Canceled
            )
        }) {
            return Ok(if order.order_status == OrderStatus::AllTraded {
                OrderOutcome::AllTraded
            } else {
                OrderOutcome::Canceled
            });
        }
        if let Some(message) = state.order_action_failure.take() {
            if canceling && message.contains("已全成交或已撤销") {
                println!("撤单未执行，等待订单最终状态：{message}");
            } else {
                return Err(format!("{description}失败：{message}"));
            }
        }
        if let Some(message) = state.failure.take() {
            return Err(format!("{description}失败：{message}"));
        }
        let now = Instant::now();
        if now >= deadline {
            return Err(format!("{description}超时"));
        }
        let (next, _) = changed
            .wait_timeout(state, deadline - now)
            .unwrap_or_else(|error| error.into_inner());
        state = next;
    }
}

fn login_request(ctp: &config::CtpConfig) -> ReqUserLogin {
    ReqUserLogin {
        trading_day: DateText::default(),
        broker_id: BrokerId::new(&ctp.broker_id).unwrap(),
        user_id: UserId::new(&ctp.user_id).unwrap(),
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
    }
}

#[derive(Default)]
struct MarketState {
    connected: bool,
    logged_in: bool,
    tick: Option<DepthMarketData>,
    failure: Option<String>,
}

struct MarketDepthSpi {
    shared: Arc<(Mutex<MarketState>, Condvar)>,
}

impl MarketDepthSpi {
    fn update(&self, update: impl FnOnce(&mut MarketState)) {
        let (lock, changed) = &*self.shared;
        let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
        update(&mut state);
        changed.notify_all();
    }
}

impl MdSpi for MarketDepthSpi {
    fn connected(&mut self) {
        self.update(|state| state.connected = true);
    }

    fn disconnected(&mut self, reason: DisconnectReason) {
        self.update(|state| state.failure = Some(format!("行情前置断开：{reason}")));
    }

    fn login_response(
        &mut self,
        _rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if rsp_info.as_ref().is_some_and(|info| info.error_id == 0) {
            self.update(|state| state.logged_in = true);
        } else {
            self.update(|state| {
                state.failure =
                    Some(rsp_info.map_or_else(|| "行情登录失败".into(), |info| info.error_msg));
            });
        }
    }

    fn error(&mut self, rsp_info: Option<RspInfo>, _request_id: i32, _is_last: bool) {
        self.update(|state| {
            state.failure = Some(rsp_info.map_or_else(|| "行情错误".into(), |info| info.error_msg));
        });
    }

    fn tick(&mut self, data: Option<DepthMarketData>) {
        if let Some(data) = data.filter(|data| {
            data.ask_price1.is_finite() && data.ask_price1 > 0.0 && data.ask_price1 < f64::MAX
        }) {
            self.update(|state| state.tick = Some(data));
        }
    }
}

fn wait_market(
    shared: &Arc<(Mutex<MarketState>, Condvar)>,
    description: &str,
    mut predicate: impl FnMut(&MarketState) -> bool,
) -> Result<(), String> {
    let (lock, changed) = &**shared;
    let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if let Some(message) = state.failure.take() {
            return Err(message);
        }
        if predicate(&state) {
            return Ok(());
        }
        let now = Instant::now();
        if now >= deadline {
            return Err(format!("{description}超时"));
        }
        let (next, _) = changed
            .wait_timeout(state, deadline - now)
            .unwrap_or_else(|error| error.into_inner());
        state = next;
    }
}

fn load_market_data(
    market_front: &str,
    ctp: &config::CtpConfig,
) -> Result<DepthMarketData, String> {
    std::fs::create_dir_all(FLOW_PATH).map_err(|error| error.to_string())?;
    let shared = Arc::new((Mutex::new(MarketState::default()), Condvar::new()));
    let api = MdApi::new(
        MdConfig::new(FLOW_PATH),
        Box::new(MarketDepthSpi {
            shared: shared.clone(),
        }),
    )
    .map_err(|error| error.to_string())?;

    println!("正在连接行情前置：{market_front}");
    api.register_front(market_front)
        .map_err(|error| error.to_string())?;
    api.init();
    wait_market(&shared, "等待行情连接", |state| state.connected)?;
    let code = api
        .login(&login_request(ctp), 1)
        .map_err(|error| error.to_string())?;
    if code != 0 {
        return Err(format!("行情登录返回码：{code}"));
    }
    wait_market(&shared, "等待行情登录", |state| state.logged_in)?;
    let code = api
        .subscribe(&[ctp.instrument.as_str()])
        .map_err(|error| error.to_string())?;
    if code != 0 {
        return Err(format!("行情订阅返回码：{code}"));
    }
    wait_market(&shared, "等待盘口", |state| state.tick.is_some())?;
    let tick = {
        let (lock, _) = &*shared;
        let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
        state.tick.take().unwrap()
    };
    println!("盘口卖一价：{}", tick.ask_price1);
    api.close();
    Ok(tick)
}

fn new_order_ref() -> OrderRef {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    OrderRef::new(&timestamp.to_string()).unwrap()
}

fn order_request(limit_price: f64, order_ref: OrderRef, ctp: &config::CtpConfig) -> InputOrder {
    InputOrder {
        broker_id: BrokerId::new(&ctp.broker_id).unwrap(),
        investor_id: InvestorId::new(&ctp.user_id).unwrap(),
        order_ref,
        user_id: UserId::new(&ctp.user_id).unwrap(),
        order_price_type: OrderPriceType::LimitPrice,
        direction: Direction::Buy,
        comb_offset_flag: CombOffsetFlag::new(OffsetFlag::Open),
        comb_hedge_flag: CombHedgeFlag::new(HedgeFlag::Speculation),
        limit_price,
        volume_total_original: 1,
        time_condition: TimeCondition::GFD,
        gtd_date: DateText::default(),
        volume_condition: VolumeCondition::AV,
        min_volume: 1,
        contingent_condition: ContingentCondition::Immediately,
        stop_price: 0.0,
        force_close_reason: ForceCloseReason::NotForceClose,
        is_auto_suspend: false,
        business_unit: BusinessUnit::default(),
        request_id: RequestId::new(4),
        user_force_close: false,
        is_swap_order: false,
        exchange_id: ExchangeId::new("SHFE").unwrap(),
        invest_unit_id: InvestUnitId::default(),
        account_id: AccountId::new(&ctp.user_id).unwrap(),
        currency_id: "CNY".into(),
        client_id: ctp.user_id.clone(),
        mac_address: String::new(),
        instrument_id: InstrumentId::new(&ctp.instrument).unwrap(),
        ip_address: String::new(),
        order_memo: String::new(),
        session_req_seq: 0,
    }
}

fn cancel_request(
    front_id: FrontId,
    session_id: SessionId,
    limit_price: f64,
    order_ref: OrderRef,
    ctp: &config::CtpConfig,
) -> InputOrderAction {
    InputOrderAction {
        broker_id: BrokerId::new(&ctp.broker_id).unwrap(),
        investor_id: InvestorId::new(&ctp.user_id).unwrap(),
        order_action_ref: 1,
        order_ref,
        request_id: RequestId::new(5),
        front_id,
        session_id,
        exchange_id: ExchangeId::new("SHFE").unwrap(),
        order_system_id: OrderSysId::default(),
        action_flag: ActionFlag::Delete,
        limit_price,
        volume_change: 0,
        user_id: UserId::new(&ctp.user_id).unwrap(),
        invest_unit_id: InvestUnitId::default(),
        mac_address: String::new(),
        instrument_id: InstrumentId::new(&ctp.instrument).unwrap(),
        ip_address: String::new(),
        order_memo: String::new(),
        session_req_seq: 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(FLOW_PATH).unwrap();
    let ctp = config::CtpConfig::load()?;
    let order_ref = new_order_ref();
    let shared = Arc::new((Mutex::new(TraderState::default()), Condvar::new()));
    let api = TraderApi::new(
        TraderConfig::new(FLOW_PATH),
        Box::new(SimnowTraderSpi {
            shared: shared.clone(),
            order_ref,
        }),
    )
    .unwrap();

    let result = (|| -> Result<(), String> {
        let market_front =
            std::env::var("CTP_MARKET_FRONT").unwrap_or_else(|_| ctp.market_front.clone());
        let trade_front =
            std::env::var("CTP_TRADE_FRONT").unwrap_or_else(|_| ctp.trade_front.clone());
        let market_data = load_market_data(&market_front, &ctp)?;
        let limit_price = market_data.ask_price1;
        println!("正在连接交易前置：{trade_front}");
        api.register_front(&trade_front)
            .map_err(|error| error.to_string())?;
        api.init();
        wait_for(&shared, "连接交易前置", |state| state.connected)?;
        println!("交易前置已连接");

        let authenticate = ReqAuthenticate {
            broker_id: BrokerId::new(&ctp.broker_id).unwrap(),
            user_id: UserId::new(&ctp.user_id).unwrap(),
            user_product_info: String::new(),
            auth_code: ctp.auth_code.clone(),
            app_id: ctp.app_id.clone(),
        };
        let code = api
            .authenticate(&authenticate, 1)
            .map_err(|error| error.to_string())?;
        if code != 0 {
            return Err(format!("认证请求返回码：{code}"));
        }
        wait_for(&shared, "认证", |state| state.authenticated)?;
        println!("认证成功");

        let code = api
            .login(&login_request(&ctp), 2)
            .map_err(|error| error.to_string())?;
        if code != 0 {
            return Err(format!("登录请求返回码：{code}"));
        }
        wait_for(&shared, "登录", |state| state.login.is_some())?;
        let (front_id, session_id) = {
            let (lock, _) = &*shared;
            let state = lock.lock().unwrap_or_else(|error| error.into_inner());
            state.login.unwrap()
        };
        println!(
            "登录成功：FrontID={}，SessionID={}",
            front_id.get(),
            session_id.get()
        );
        api.subscribe_private(ResumeType::Quick)
            .map_err(|error| error.to_string())?;
        api.subscribe_public(ResumeType::Quick)
            .map_err(|error| error.to_string())?;
        println!("私有流、公共流订阅完成");

        let settlement = SettlementInfoConfirm {
            broker_id: BrokerId::new(&ctp.broker_id).unwrap(),
            investor_id: InvestorId::new(&ctp.user_id).unwrap(),
            confirm_date: DateText::default(),
            confirm_time: TimeText::default(),
            settlement_id: 0,
            account_id: AccountId::new(&ctp.user_id).unwrap(),
            currency_id: "CNY".into(),
        };
        let code = api
            .confirm_settlement(&settlement, 3)
            .map_err(|error| error.to_string())?;
        if code != 0 {
            return Err(format!("结算确认返回码：{code}"));
        }
        println!(
            "结算确认完成，以 {limit_price} 买入 {} 1 手",
            ctp.instrument
        );
        let code = api
            .insert_order(&order_request(limit_price, order_ref, &ctp), 4)
            .map_err(|error| error.to_string())?;
        if code != 0 {
            return Err(format!("下单请求返回码：{code}"));
        }
        let outcome = match wait_for_order(&shared, "快速成交", false) {
            Ok(outcome) => outcome,
            Err(error) if error.contains("超时") => {
                let code = api
                    .cancel_order(
                        &cancel_request(front_id, session_id, limit_price, order_ref, &ctp),
                        5,
                    )
                    .map_err(|error| error.to_string())?;
                if code != 0 {
                    return Err(format!("撤单请求返回码：{code}"));
                }
                wait_for_order(&shared, "撤单状态", true)?
            }
            Err(error) => return Err(error),
        };
        match outcome {
            OrderOutcome::AllTraded => {
                wait_for(&shared, "成交回报", |state| state.executions > 0)?;
                println!("订单已全部成交");
            }
            OrderOutcome::Canceled => println!("订单已撤销，未全部成交"),
        }

        let query_instrument = QryInstrument {
            exchange_id: ExchangeId::default(),
            instrument_id: InstrumentId::new(&ctp.instrument).unwrap(),
            exchange_instrument_id: ExchangeInstrumentId::default(),
            product_id: InstrumentId::default(),
        };
        let code = api
            .query_instrument(&query_instrument, 8)
            .map_err(|error| error.to_string())?;
        if code != 0 {
            return Err(format!("合约查询返回码：{code}"));
        }
        wait_for(&shared, "合约查询", |state| state.instrument.is_some())?;
        let instrument = {
            let (lock, _) = &*shared;
            let mut state = lock.lock().unwrap_or_else(|error| error.into_inner());
            state.instrument.take().unwrap()
        };
        if instrument.price_tick <= 0.0 || instrument.volume_multiple <= 0 {
            return Err("合约参数无效".into());
        }
        println!(
            "合约参数：最小变动价位={}，合约乘数={}",
            instrument.price_tick, instrument.volume_multiple
        );
        Ok(())
    })();

    api.close();
    result?;
    println!("交易 API 已关闭");
    Ok(())
}
