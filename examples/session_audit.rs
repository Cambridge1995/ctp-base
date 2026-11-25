#[path = "common/config.rs"]
mod config;
#[path = "common/query.rs"]
mod query;

use ctp_base::{
    AccountId, ActionFlag, BizType, BusinessUnit, CombHedgeFlag, CombOffsetFlag,
    ContingentCondition, DateText, Direction, ForceCloseReason, HedgeFlag, InputOrder,
    InputOrderAction, Instrument, InstrumentId, MdConfig, MdSession, MdSessionEvent, OffsetFlag,
    OrderPriceType, OrderRef, OrderStatus, ProductClass, QryDepthMarketData, QryExchange,
    QryInstrument, QryInvestor, QryInvestorPosition, QryInvestorPositionDetail, QryOrder,
    QryProduct, QrySettlementInfo, QryTrade, QryTradingAccount, QueryRows, RequestId,
    TimeCondition, TraderConfig, TraderSession, TraderSessionEvent, VolumeCondition,
};
use std::error::Error;
use std::time::{Duration, Instant};

const WAIT: Duration = Duration::from_secs(30);
const QUERY_INTERVAL: Duration = Duration::from_millis(1100);

fn wait_md(
    session: &MdSession,
    description: &str,
    mut accept: impl FnMut(&MdSessionEvent) -> bool,
) -> Result<(), String> {
    let deadline = Instant::now() + WAIT;
    loop {
        if let Some(event) = session.try_event() {
            match &event {
                MdSessionEvent::Error { message, .. }
                | MdSessionEvent::ConversionFailed { message } => {
                    return Err(format!("{description}：{message}"));
                }
                MdSessionEvent::Disconnected { reason } => {
                    return Err(format!("{description}：断线 {reason}"));
                }
                _ => {}
            }
            if accept(&event) {
                return Ok(());
            }
        } else if Instant::now() >= deadline {
            return Err(format!("{description}：超时"));
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

fn run_md(ctp: &config::CtpConfig) -> Result<f64, Box<dyn Error>> {
    let session = MdSession::new(&MdConfig {
        broker_id: ctp.broker_id.clone(),
        user_id: ctp.user_id.clone(),
        fronts: ctp.market_fronts.clone(),
        flow_dir: format!("{}/session-audit-md", ctp.flow.trim_end_matches('/')),
        subscribe: vec![ctp.instrument.clone()],
        ..Default::default()
    })?;
    if session.trading_day().is_empty() {
        return Err("行情交易日为空".into());
    }
    println!("PASS MdSession::new / trading_day");

    wait_md(&session, "初始订阅", |event| {
        matches!(event, MdSessionEvent::Subscribed { instrument } if instrument == &ctp.instrument)
    })
    .map_err(|error| -> Box<dyn Error> { error.into() })?;
    println!("PASS MdSession 初始 subscribe / try_event");

    let mut reference_price = None;
    let deadline = Instant::now() + WAIT;
    while reference_price.is_none() && Instant::now() < deadline {
        if let Some(MdSessionEvent::Tick(tick)) = session.try_event() {
            let candidate = if tick.ask_price1.is_finite() && tick.ask_price1 > 0.0 {
                tick.ask_price1
            } else {
                tick.last_price
            };
            if candidate.is_finite() && candidate > 0.0 && candidate < f64::MAX {
                reference_price = Some(candidate);
            }
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    let reference_price = reference_price.ok_or("未收到有效行情，无法构造低风险测试单")?;
    println!("PASS MdSession Tick（参考价有效）");

    session.unsubscribe(&[&ctp.instrument])?;
    wait_md(&session, "退订", |event| {
        matches!(event, MdSessionEvent::Unsubscribed { instrument } if instrument == &ctp.instrument)
    })
    .map_err(|error| -> Box<dyn Error> { error.into() })?;
    println!("PASS MdSession::unsubscribe");

    session.subscribe(&[&ctp.instrument])?;
    wait_md(&session, "重新订阅", |event| {
        matches!(event, MdSessionEvent::Subscribed { instrument } if instrument == &ctp.instrument)
    })
    .map_err(|error| -> Box<dyn Error> { error.into() })?;
    println!("PASS MdSession::subscribe");

    if session.dropped_count() != 0 {
        return Err("行情测试期间出现丢弃事件".into());
    }
    session.exit();
    println!("PASS MdSession::dropped_count / exit");
    Ok(reference_price)
}

fn wait_query(
    session: &TraderSession,
    request_id: i32,
    name: &str,
    rows: QueryRows,
) -> Result<QueryRows, String> {
    let deadline = Instant::now() + WAIT;
    let mut rows = rows;
    loop {
        if let Some(event) = session.try_event() {
            match event {
                TraderSessionEvent::QueryRow {
                    request_id: id,
                    is_last,
                    error_id,
                    message,
                    row,
                } if id == request_id => {
                    if error_id != 0 {
                        return Err(format!("{name} 查询被拒 [{error_id}] {message}"));
                    }
                    if let Some(row) = row {
                        query::append_query_row(&mut rows, row)
                            .map_err(|error| format!("{name}：{error}"))?;
                    }
                    if is_last {
                        return Ok(rows);
                    }
                }
                TraderSessionEvent::QueryError {
                    request_id: id,
                    error_id,
                    message,
                } if id == request_id => {
                    return Err(format!("{name} 查询错误 [{error_id}] {message}"));
                }
                TraderSessionEvent::Error { error_id, message } => {
                    return Err(format!("{name} 查询错误 [{error_id}] {message}"));
                }
                TraderSessionEvent::ConversionFailed { message } => {
                    return Err(format!("{name} 查询转换失败：{message}"));
                }
                TraderSessionEvent::Disconnected { reason } => {
                    return Err(format!("{name} 查询期间断线：{reason}"));
                }
                _ => {}
            }
        } else if Instant::now() >= deadline {
            return Err(format!("{name} 查询超时"));
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

fn finish_query(
    session: &TraderSession,
    request_id: i32,
    name: &str,
    rows: QueryRows,
) -> Result<QueryRows, Box<dyn Error>> {
    let rows = wait_query(session, request_id, name, rows)?;
    println!("PASS TraderSession::{name}（error_id=0，完整批次）");
    std::thread::sleep(QUERY_INTERVAL);
    Ok(rows)
}

fn finish_non_empty_query(
    session: &TraderSession,
    request_id: i32,
    name: &str,
    rows: QueryRows,
) -> Result<QueryRows, Box<dyn Error>> {
    let rows = finish_query(session, request_id, name, rows)?;
    let is_empty = match &rows {
        QueryRows::TradingAccounts(rows) => rows.is_empty(),
        QueryRows::Investors(rows) => rows.is_empty(),
        QueryRows::Exchanges(rows) => rows.is_empty(),
        QueryRows::Products(rows) => rows.is_empty(),
        _ => return Err(format!("{name} 返回了非预期行集类型").into()),
    };
    if is_empty {
        return Err(format!("{name} 成功批次不应为空").into());
    }
    Ok(rows)
}

fn order_request(ctp: &config::CtpConfig, instrument: &Instrument, limit_price: f64) -> InputOrder {
    InputOrder {
        broker_id: Default::default(),
        investor_id: Default::default(),
        order_ref: OrderRef::default(),
        user_id: Default::default(),
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
        request_id: RequestId::default(),
        user_force_close: false,
        is_swap_order: false,
        exchange_id: instrument.exchange_id,
        invest_unit_id: Default::default(),
        account_id: AccountId::default(),
        currency_id: "CNY".to_owned(),
        client_id: ctp.user_id.clone(),
        mac_address: String::new(),
        instrument_id: instrument.instrument_id,
        ip_address: String::new(),
        order_memo: String::new(),
        session_req_seq: 0,
    }
}

fn cancel_request(instrument: &Instrument, reference: &str) -> InputOrderAction {
    InputOrderAction {
        broker_id: Default::default(),
        investor_id: Default::default(),
        order_action_ref: 0,
        order_ref: OrderRef::new(reference).expect("Session 返回的 OrderRef 应有效"),
        request_id: RequestId::default(),
        front_id: Default::default(),
        session_id: Default::default(),
        exchange_id: instrument.exchange_id,
        order_system_id: Default::default(),
        action_flag: ActionFlag::Delete,
        limit_price: 0.0,
        volume_change: 0,
        user_id: Default::default(),
        invest_unit_id: Default::default(),
        mac_address: String::new(),
        instrument_id: instrument.instrument_id,
        ip_address: String::new(),
        order_memo: String::new(),
        session_req_seq: 0,
    }
}

fn wait_order_ready(session: &TraderSession, reference: &str) -> Result<(), String> {
    let deadline = Instant::now() + WAIT;
    loop {
        if let Some(event) = session.try_event() {
            match event {
                TraderSessionEvent::OrderInsertResponse {
                    error_id, message, ..
                } => {
                    if error_id != 0 {
                        return Err(format!("报单被前置拒绝 [{error_id}] {message}"));
                    }
                }
                TraderSessionEvent::OrderInsertError {
                    error_id, message, ..
                } => {
                    return Err(format!("报单错误回报 [{error_id}] {message}"));
                }
                TraderSessionEvent::OrderUpdate(order) if order.order_ref.as_str() == reference => {
                    match order.order_status {
                        OrderStatus::NoTradeQueueing | OrderStatus::PartTradedQueueing => {
                            return Ok(());
                        }
                        OrderStatus::Canceled => {
                            return Err("报单在进入可撤状态前已撤销".to_owned());
                        }
                        OrderStatus::AllTraded => {
                            return Err("低风险测试单意外全部成交，未继续撤单测试".to_owned());
                        }
                        _ => {}
                    }
                }
                TraderSessionEvent::Error { error_id, message } => {
                    return Err(format!("报单过程错误 [{error_id}] {message}"));
                }
                TraderSessionEvent::Disconnected { reason } => {
                    return Err(format!("报单过程断线：{reason}"));
                }
                _ => {}
            }
        } else if Instant::now() >= deadline {
            return Err("等待报单进入可撤状态超时".to_owned());
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

fn wait_canceled(session: &TraderSession, reference: &str) -> Result<(), String> {
    let deadline = Instant::now() + WAIT;
    loop {
        if let Some(event) = session.try_event() {
            match event {
                TraderSessionEvent::OrderActionResponse {
                    error_id, message, ..
                } => {
                    if error_id != 0 {
                        return Err(format!("撤单被前置拒绝 [{error_id}] {message}"));
                    }
                }
                TraderSessionEvent::OrderActionError {
                    error_id, message, ..
                } => {
                    return Err(format!("撤单错误回报 [{error_id}] {message}"));
                }
                TraderSessionEvent::OrderUpdate(order)
                    if order.order_ref.as_str() == reference
                        && order.order_status == OrderStatus::Canceled =>
                {
                    return Ok(());
                }
                TraderSessionEvent::Disconnected { reason } => {
                    return Err(format!("撤单过程断线：{reason}"));
                }
                _ => {}
            }
        } else if Instant::now() >= deadline {
            return Err("等待撤单最终状态超时".to_owned());
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

fn run_trader(ctp: &config::CtpConfig, reference_price: f64) -> Result<(), Box<dyn Error>> {
    let session = TraderSession::new(&TraderConfig {
        broker_id: ctp.broker_id.clone(),
        user_id: ctp.user_id.clone(),
        password: ctp.password.clone(),
        app_id: ctp.app_id.clone(),
        auth_code: ctp.auth_code.clone(),
        fronts: ctp.trade_fronts.clone(),
        flow_dir: format!("{}/session-audit-trader", ctp.flow.trim_end_matches('/')),
        ..Default::default()
    })?;
    if session.front_id() <= 0 || session.session_id() == 0 || session.trading_day().is_empty() {
        return Err("交易会话身份无效".into());
    }
    println!("PASS TraderSession::new / front_id / session_id / trading_day");

    let instrument_query = QryInstrument {
        exchange_id: Default::default(),
        instrument_id: InstrumentId::new(&ctp.instrument)?,
        exchange_instrument_id: Default::default(),
        product_id: Default::default(),
    };
    let request_id = session.query_instrument(instrument_query)?;
    let rows = finish_query(
        &session,
        request_id,
        "query_instrument",
        QueryRows::Instruments(Vec::new()),
    )?;
    let instrument = match rows {
        QueryRows::Instruments(rows) => rows
            .into_iter()
            .find(|row| row.instrument_id.as_str() == ctp.instrument)
            .ok_or("合约查询成功但缺少测试合约")?,
        _ => return Err("合约查询批次类型错误".into()),
    };
    if instrument.price_tick <= 0.0 {
        return Err("合约最小变动价位无效".into());
    }

    let account_query = QryTradingAccount {
        broker_id: Default::default(),
        investor_id: Default::default(),
        currency_id: "CNY".to_owned(),
        biz_type: BizType::Future,
        account_id: Default::default(),
    };
    finish_non_empty_query(
        &session,
        session.query_trading_account(account_query)?,
        "query_trading_account",
        QueryRows::TradingAccounts(Vec::new()),
    )?;

    let position_query = QryInvestorPosition {
        broker_id: Default::default(),
        investor_id: Default::default(),
        exchange_id: instrument.exchange_id,
        invest_unit_id: Default::default(),
        instrument_id: instrument.instrument_id,
    };
    finish_query(
        &session,
        session.query_investor_position(position_query)?,
        "query_investor_position",
        QueryRows::Positions(Vec::new()),
    )?;

    let detail_query = QryInvestorPositionDetail {
        broker_id: Default::default(),
        investor_id: Default::default(),
        exchange_id: instrument.exchange_id,
        invest_unit_id: Default::default(),
        instrument_id: instrument.instrument_id,
    };
    finish_query(
        &session,
        session.query_investor_position_detail(detail_query)?,
        "query_investor_position_detail",
        QueryRows::PositionDetails(Vec::new()),
    )?;

    let order_query = QryOrder {
        broker_id: Default::default(),
        investor_id: Default::default(),
        exchange_id: instrument.exchange_id,
        order_system_id: Default::default(),
        insert_time_start: Default::default(),
        insert_time_end: Default::default(),
        invest_unit_id: Default::default(),
        instrument_id: instrument.instrument_id,
    };
    finish_query(
        &session,
        session.query_order(order_query)?,
        "query_order",
        QueryRows::Orders(Vec::new()),
    )?;

    let trade_query = QryTrade {
        broker_id: Default::default(),
        investor_id: Default::default(),
        exchange_id: instrument.exchange_id,
        trade_id: Default::default(),
        trade_time_start: Default::default(),
        trade_time_end: Default::default(),
        invest_unit_id: Default::default(),
        instrument_id: instrument.instrument_id,
    };
    finish_query(
        &session,
        session.query_trade(trade_query)?,
        "query_trade",
        QueryRows::Trades(Vec::new()),
    )?;

    let exchange_query = QryExchange {
        exchange_id: instrument.exchange_id,
    };
    finish_non_empty_query(
        &session,
        session.query_exchange(exchange_query)?,
        "query_exchange",
        QueryRows::Exchanges(Vec::new()),
    )?;

    let product_query = QryProduct {
        product_class: ProductClass::Futures,
        exchange_id: instrument.exchange_id,
        product_id: instrument.product_id,
    };
    finish_non_empty_query(
        &session,
        session.query_product(product_query)?,
        "query_product",
        QueryRows::Products(Vec::new()),
    )?;

    let depth_query = QryDepthMarketData {
        exchange_id: instrument.exchange_id,
        instrument_id: instrument.instrument_id,
        product_class: ProductClass::Futures,
    };
    finish_query(
        &session,
        session.query_depth_market_data(depth_query)?,
        "query_depth_market_data",
        QueryRows::DepthMarketData(Vec::new()),
    )?;

    let investor_query = QryInvestor {
        broker_id: Default::default(),
        investor_id: Default::default(),
    };
    finish_non_empty_query(
        &session,
        session.query_investor(investor_query)?,
        "query_investor",
        QueryRows::Investors(Vec::new()),
    )?;

    let settlement_query = QrySettlementInfo {
        broker_id: Default::default(),
        investor_id: Default::default(),
        trading_day: DateText::new(&session.trading_day())?,
        account_id: Default::default(),
        currency_id: "CNY".to_owned(),
    };
    finish_query(
        &session,
        session.query_settlement_info(settlement_query)?,
        "query_settlement_info",
        QueryRows::SettlementInfo(Vec::new()),
    )?;

    let limit_price =
        (reference_price * 0.9 / instrument.price_tick).floor() * instrument.price_tick;
    if !limit_price.is_finite() || limit_price <= 0.0 {
        return Err("测试单限价无效".into());
    }
    let reference = session.insert_order(order_request(ctp, &instrument, limit_price))?;
    if let Err(error) = wait_order_ready(&session, &reference) {
        let _ = session.cancel_order(cancel_request(&instrument, &reference));
        let _ = wait_canceled(&session, &reference);
        return Err(error.into());
    }
    println!("PASS TraderSession::insert_order（OrderUpdate=NoTradeQueueing）");

    session.cancel_order(cancel_request(&instrument, &reference))?;
    wait_canceled(&session, &reference)?;
    println!("PASS TraderSession::cancel_order（OrderUpdate=Canceled）");

    if session.dropped_count() != 0 {
        return Err("交易测试期间出现丢弃事件".into());
    }
    session.exit();
    println!("PASS TraderSession::try_event / dropped_count / exit");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let ctp = config::CtpConfig::load()?;
    std::fs::create_dir_all(&ctp.flow)?;
    let reference_price = run_md(&ctp)?;
    run_trader(&ctp, reference_price)?;
    println!("SESSION_AUDIT_OK");
    Ok(())
}
