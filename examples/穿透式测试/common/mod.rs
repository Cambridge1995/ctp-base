// 每个独立 example 只使用公共模块的一部分，保留完整测试工具链。
#![allow(dead_code)]

#[path = "../../common/query.rs"]
mod query;

use crate::config::CtpConfig;
use ctp_base::{
    ActionFlag, CombHedgeFlag, CombOffsetFlag, ContingentCondition, DepthMarketData, Direction,
    ForceCloseReason, HedgeFlag, InputBatchOrderAction, InputOrder, InputOrderAction, Instrument,
    InstrumentId, InvestorPosition, MdConfig, MdSession, MdSessionEvent, OffsetFlag,
    OrderPriceType, OrderRef, OrderStatus, QryInstrument, QryInvestorPosition, QryTradingAccount,
    QueryRows, TimeCondition, TraderConfig, TraderSession, TraderSessionEvent, VolumeCondition,
};
use std::error::Error;
use std::time::{Duration, Instant};

pub const WAIT: Duration = Duration::from_secs(30);
pub const QUERY_INTERVAL: Duration = Duration::from_millis(1100);

#[derive(Clone, Debug, Default)]
pub struct RiskStats {
    pub order_count: usize,
    pub cancel_count: usize,
    pub repeat_intent: usize,
    pub trade_volume: i32,
    pub messages: usize,
}

pub struct TestContext {
    pub ctp: CtpConfig,
    pub instrument: Instrument,
    pub session: TraderSession,
    pub stats: RiskStats,
}

fn wait_md(session: &MdSession) -> Result<DepthMarketData, Box<dyn Error>> {
    let deadline = Instant::now() + WAIT;
    loop {
        if let Some(event) = session.try_event() {
            match event {
                MdSessionEvent::Tick(tick) => return Ok(*tick),
                MdSessionEvent::Error { message, .. }
                | MdSessionEvent::ConversionFailed { message } => return Err(message.into()),
                MdSessionEvent::Disconnected { reason } => {
                    return Err(format!("行情断线：{reason}").into());
                }
                _ => {}
            }
        } else if Instant::now() >= deadline {
            return Err("等待行情超时".into());
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

pub fn market_price(ctp: &CtpConfig) -> Result<DepthMarketData, Box<dyn Error>> {
    let session = MdSession::new(&MdConfig {
        broker_id: ctp.broker_id.clone(),
        user_id: ctp.user_id.clone(),
        fronts: ctp.market_fronts.clone(),
        flow_dir: format!("{}/penetration-md", ctp.flow.trim_end_matches('/')),
        subscribe: vec![ctp.instrument.clone()],
        ..Default::default()
    })?;
    let tick = wait_md(&session)?;
    session.exit();
    Ok(tick)
}

fn payload_matches<T>(
    payload: Option<&T>,
    reference: &str,
    reference_of: impl FnOnce(&T) -> &str,
) -> bool {
    payload.is_some_and(|payload| reference_of(payload) == reference)
}

fn wait_query(
    session: &TraderSession,
    request_id: i32,
    rows: QueryRows,
) -> Result<QueryRows, Box<dyn Error>> {
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
                        return Err(format!("查询被拒 [{error_id}] {message}").into());
                    }
                    if let Some(row) = row {
                        query::append_query_row(&mut rows, row)?;
                    }
                    if is_last {
                        std::thread::sleep(QUERY_INTERVAL);
                        return Ok(rows);
                    }
                }
                TraderSessionEvent::QueryError {
                    request_id: id,
                    error_id,
                    message,
                } if id == request_id => {
                    return Err(format!("查询被拒 [{error_id}] {message}").into());
                }
                TraderSessionEvent::Error { message, .. }
                | TraderSessionEvent::ConversionFailed { message } => {
                    return Err(message.into());
                }
                TraderSessionEvent::Disconnected { reason } => {
                    return Err(format!("查询期间断线：{reason}").into());
                }
                _ => {}
            }
        } else if Instant::now() >= deadline {
            return Err("查询超时".into());
        } else {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

impl TestContext {
    pub fn connect() -> Result<Self, Box<dyn Error>> {
        let ctp = CtpConfig::load()?;
        std::fs::create_dir_all(&ctp.flow)?;
        let session = TraderSession::new(&TraderConfig {
            broker_id: ctp.broker_id.clone(),
            user_id: ctp.user_id.clone(),
            password: ctp.password.clone(),
            app_id: ctp.app_id.clone(),
            auth_code: ctp.auth_code.clone(),
            fronts: ctp.trade_fronts.clone(),
            flow_dir: format!("{}/penetration-trader", ctp.flow.trim_end_matches('/')),
            production_mode: true,
            ..Default::default()
        })?;
        let query = QryInstrument {
            instrument_id: InstrumentId::new(&ctp.instrument)?,
            ..Default::default()
        };
        let request_id = session.query_instrument(query)?;
        let instrument = match wait_query(&session, request_id, QueryRows::Instruments(Vec::new()))?
        {
            QueryRows::Instruments(mut rows) if !rows.is_empty() => rows.remove(0),
            QueryRows::Instruments(_) => return Err("合约查询成功但没有目标合约".into()),
            _ => return Err("合约查询返回了错误行集".into()),
        };
        Ok(Self {
            ctp,
            instrument,
            session,
            stats: RiskStats::default(),
        })
    }

    pub fn account(&self) -> Result<ctp_base::TradingAccount, Box<dyn Error>> {
        let query = QryTradingAccount {
            currency_id: "CNY".to_owned(),
            biz_type: ctp_base::BizType::Future,
            ..Default::default()
        };
        let request_id = self.session.query_trading_account(query)?;
        match wait_query(
            &self.session,
            request_id,
            QueryRows::TradingAccounts(Vec::new()),
        )? {
            QueryRows::TradingAccounts(mut rows) if !rows.is_empty() => Ok(rows.remove(0)),
            QueryRows::TradingAccounts(_) => Err("资金查询成功但批次为空".into()),
            _ => Err("资金查询返回了错误行集".into()),
        }
    }

    pub fn positions(&self) -> Result<Vec<InvestorPosition>, Box<dyn Error>> {
        let query = QryInvestorPosition {
            exchange_id: self.instrument.exchange_id,
            instrument_id: self.instrument.instrument_id,
            ..Default::default()
        };
        let request_id = self.session.query_investor_position(query)?;
        match wait_query(&self.session, request_id, QueryRows::Positions(Vec::new()))? {
            QueryRows::Positions(rows) => Ok(rows),
            _ => Err("持仓查询返回了错误行集".into()),
        }
    }

    pub fn print_positions(&self) -> Result<(), Box<dyn Error>> {
        let positions = self.positions()?;
        if positions.is_empty() {
            println!("当前持仓：0 手");
            return Ok(());
        }
        for position in positions {
            println!(
                "当前持仓：合约={}，方向={:?}，HedgeFlag={:?}，PositionDate={:?}，InvestUnitID={}，总持仓={}，今仓={}，昨仓={}",
                position.instrument_id.as_str(),
                position.position_direction,
                position.hedge_flag,
                position.position_date,
                position.invest_unit_id.as_str(),
                position.position,
                position.today_position,
                position.yd_position
            );
        }
        Ok(())
    }

    pub fn print_snapshot(&self) -> Result<(), Box<dyn Error>> {
        let account = self.account()?;
        let positions = self.positions()?;
        println!(
            "合约={}，最小跳动={}，限价单手数范围={}~{}，可用资金={:.2}，持仓行数={}",
            self.instrument.instrument_id.as_str(),
            self.instrument.price_tick,
            self.instrument.min_limit_order_volume,
            self.instrument.max_limit_order_volume,
            account.available,
            positions.len()
        );
        Ok(())
    }

    pub fn close_offset(&self) -> OffsetFlag {
        match self.instrument.exchange_id.as_str() {
            "SHFE" | "INE" => OffsetFlag::CloseToday,
            _ => OffsetFlag::Close,
        }
    }

    pub fn order(
        &self,
        direction: Direction,
        offset: OffsetFlag,
        price: f64,
        volume: i32,
    ) -> InputOrder {
        InputOrder {
            order_price_type: OrderPriceType::LimitPrice,
            direction,
            comb_offset_flag: CombOffsetFlag::new(offset),
            comb_hedge_flag: CombHedgeFlag::new(HedgeFlag::Speculation),
            limit_price: price,
            volume_total_original: volume,
            time_condition: TimeCondition::GFD,
            volume_condition: VolumeCondition::AV,
            min_volume: 1,
            contingent_condition: ContingentCondition::Immediately,
            force_close_reason: ForceCloseReason::NotForceClose,
            exchange_id: self.instrument.exchange_id,
            instrument_id: self.instrument.instrument_id,
            ..Default::default()
        }
    }

    pub fn submit(&mut self, order: InputOrder) -> Result<String, Box<dyn Error>> {
        self.stats.order_count += 1;
        self.stats.messages += 1;
        Ok(self.session.insert_order(order)?)
    }

    fn cancel_request(&self, reference: &str) -> Result<InputOrderAction, Box<dyn Error>> {
        Ok(InputOrderAction {
            order_ref: OrderRef::new(reference)?,
            exchange_id: self.instrument.exchange_id,
            action_flag: ActionFlag::Delete,
            instrument_id: self.instrument.instrument_id,
            ..Default::default()
        })
    }

    pub fn wait_final(
        &mut self,
        reference: &str,
        expected_volume: Option<i32>,
    ) -> Result<OrderStatus, Box<dyn Error>> {
        let deadline = Instant::now() + WAIT;
        let start_trade_volume = self.stats.trade_volume;
        let mut final_status = None;
        loop {
            if let Some(event) = self.session.try_event() {
                match event {
                    TraderSessionEvent::OrderUpdate(order)
                        if order.order_ref.as_str() == reference =>
                    {
                        if let status @ (OrderStatus::AllTraded | OrderStatus::Canceled) =
                            order.order_status
                        {
                            final_status = Some(status);
                        }
                    }
                    TraderSessionEvent::Execution(trade)
                        if trade.order_ref.as_str() == reference =>
                    {
                        self.stats.trade_volume += trade.volume;
                        self.stats.messages += 1;
                    }
                    TraderSessionEvent::OrderInsertResponse {
                        order,
                        error_id,
                        message,
                        ..
                    } if error_id != 0
                        && payload_matches(order.as_deref(), reference, |order| {
                            order.order_ref.as_str()
                        }) =>
                    {
                        return Err(format!("指令被拒 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::OrderActionResponse {
                        action,
                        error_id,
                        message,
                        ..
                    } if error_id != 0
                        && payload_matches(action.as_deref(), reference, |action| {
                            action.order_ref.as_str()
                        }) =>
                    {
                        return Err(format!("指令被拒 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::OrderInsertError {
                        order,
                        error_id,
                        message,
                        ..
                    } if payload_matches(order.as_deref(), reference, |order| {
                        order.order_ref.as_str()
                    }) =>
                    {
                        return Err(format!("指令错误回报 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::OrderActionError {
                        action,
                        error_id,
                        message,
                        ..
                    } if payload_matches(action.as_deref(), reference, |action| {
                        action.order_ref.as_str()
                    }) =>
                    {
                        return Err(format!("指令错误回报 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::Error { message, .. }
                    | TraderSessionEvent::ConversionFailed { message } => {
                        return Err(message.into());
                    }
                    TraderSessionEvent::Disconnected { reason } => {
                        return Err(format!("交易断线：{reason}").into());
                    }
                    _ => {}
                }
                if let Some(status) = final_status {
                    let execution_incomplete = expected_volume.is_some_and(|expected| {
                        status == OrderStatus::AllTraded
                            && self.stats.trade_volume - start_trade_volume < expected
                    });
                    if !execution_incomplete {
                        return Ok(status);
                    }
                }
            } else if Instant::now() >= deadline {
                return Err("等待指令最终状态超时".into());
            } else {
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }

    pub fn submit_wait(&mut self, order: InputOrder) -> Result<OrderStatus, Box<dyn Error>> {
        let expected_volume = order.volume_total_original;
        let reference = self.submit(order)?;
        self.wait_final(&reference, Some(expected_volume))
    }

    pub fn wait_queueing(&self, reference: &str) -> Result<(), Box<dyn Error>> {
        let deadline = Instant::now() + WAIT;
        loop {
            if let Some(event) = self.session.try_event() {
                match event {
                    TraderSessionEvent::OrderUpdate(order)
                        if order.order_ref.as_str() == reference
                            && matches!(
                                order.order_status,
                                OrderStatus::NoTradeQueueing | OrderStatus::PartTradedQueueing
                            ) =>
                    {
                        return Ok(());
                    }
                    TraderSessionEvent::OrderUpdate(order)
                        if order.order_ref.as_str() == reference
                            && matches!(
                                order.order_status,
                                OrderStatus::AllTraded | OrderStatus::Canceled
                            ) =>
                    {
                        return Err(format!("报单未进入可撤状态：{:?}", order.order_status).into());
                    }
                    TraderSessionEvent::OrderInsertResponse {
                        order,
                        error_id,
                        message,
                        ..
                    } if error_id != 0
                        && payload_matches(order.as_deref(), reference, |order| {
                            order.order_ref.as_str()
                        }) =>
                    {
                        return Err(format!("报单被拒 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::OrderInsertError {
                        order,
                        error_id,
                        message,
                        ..
                    } if payload_matches(order.as_deref(), reference, |order| {
                        order.order_ref.as_str()
                    }) =>
                    {
                        return Err(format!("报单错误回报 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::Error { message, .. }
                    | TraderSessionEvent::ConversionFailed { message } => {
                        return Err(message.into());
                    }
                    TraderSessionEvent::Disconnected { reason } => {
                        return Err(format!("报单期间断线：{reason}").into());
                    }
                    _ => {}
                }
            } else if Instant::now() >= deadline {
                return Err("等待报单进入队列超时".into());
            } else {
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }

    pub fn cancel(&mut self, reference: &str) -> Result<OrderStatus, Box<dyn Error>> {
        self.stats.cancel_count += 1;
        self.stats.messages += 1;
        self.session.cancel_order(self.cancel_request(reference)?)?;
        self.wait_final(reference, None)
    }

    pub fn submit_and_cancel(
        &mut self,
        direction: Direction,
        offset: OffsetFlag,
        low_price: f64,
        volume: i32,
    ) -> Result<String, Box<dyn Error>> {
        let reference = self.submit(self.order(direction, offset, low_price, volume))?;
        self.wait_queueing(&reference)?;
        let status = self.cancel(&reference)?;
        if status != OrderStatus::Canceled {
            return Err(format!("撤单最终状态不是 Canceled：{status:?}").into());
        }
        Ok(reference)
    }

    pub fn batch_cancel(&mut self, references: &[String]) -> Result<(), Box<dyn Error>> {
        self.stats.cancel_count += 1;
        self.stats.messages += 1;
        let request = InputBatchOrderAction {
            exchange_id: self.instrument.exchange_id,
            ..Default::default()
        };
        self.session.batch_order_action(request)?;
        let mut pending: std::collections::HashSet<&str> =
            references.iter().map(String::as_str).collect();
        let deadline = Instant::now() + WAIT;
        while !pending.is_empty() {
            if let Some(event) = self.session.try_event() {
                match event {
                    TraderSessionEvent::OrderUpdate(order) => {
                        let reference = order.order_ref.as_str();
                        if pending.contains(reference) {
                            match order.order_status {
                                OrderStatus::Canceled => {
                                    pending.remove(reference);
                                }
                                OrderStatus::AllTraded => {
                                    return Err(
                                        format!("批量撤单前报单 {reference} 已全成交").into()
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    TraderSessionEvent::BatchOrderActionResponse {
                        error_id, message, ..
                    } if error_id != 0 => {
                        return Err(format!("批量撤单被拒 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::BatchOrderActionError {
                        error_id, message, ..
                    } => {
                        return Err(format!("批量撤单错误回报 [{error_id}] {message}").into());
                    }
                    TraderSessionEvent::Error { message, .. }
                    | TraderSessionEvent::ConversionFailed { message } => {
                        return Err(message.into());
                    }
                    TraderSessionEvent::Disconnected { reason } => {
                        return Err(format!("批量撤单期间断线：{reason}").into());
                    }
                    _ => {}
                }
            } else if Instant::now() >= deadline {
                return Err(format!("批量撤单尚有 {} 笔未确认 Canceled", pending.len()).into());
            } else {
                std::thread::sleep(Duration::from_millis(20));
            }
        }
        Ok(())
    }

    pub fn expect_reject(
        &mut self,
        order: InputOrder,
        name: &str,
    ) -> Result<String, Box<dyn Error>> {
        self.stats.order_count += 1;
        self.stats.messages += 1;
        let reference = match self.session.insert_order(order) {
            Ok(reference) => reference,
            Err(error) => {
                println!("EXPECTED_REJECT {name}：{error}");
                return Ok(error.to_string());
            }
        };
        let deadline = Instant::now() + WAIT;
        loop {
            if let Some(event) = self.session.try_event() {
                match event {
                    TraderSessionEvent::OrderInsertResponse {
                        order,
                        error_id,
                        message,
                        ..
                    } if error_id != 0
                        && payload_matches(order.as_deref(), &reference, |order| {
                            order.order_ref.as_str()
                        }) =>
                    {
                        let text = format!("[{error_id}] {message}");
                        println!("EXPECTED_REJECT {name}：{text}");
                        return Ok(text);
                    }
                    TraderSessionEvent::OrderInsertError {
                        order,
                        error_id,
                        message,
                        ..
                    } if payload_matches(order.as_deref(), &reference, |order| {
                        order.order_ref.as_str()
                    }) =>
                    {
                        let text = format!("[{error_id}] {message}");
                        println!("EXPECTED_REJECT {name}：{text}");
                        return Ok(text);
                    }
                    TraderSessionEvent::OrderUpdate(order)
                        if order.order_ref.as_str() == reference
                            && order.order_status == OrderStatus::Canceled =>
                    {
                        if order.volume_traded != 0 {
                            return Err(format!("{name} 已有成交，不能视为干净拒绝").into());
                        }
                        let text = order.status_msg.clone();
                        println!("EXPECTED_REJECT {name}：{text}");
                        return Ok(text);
                    }
                    TraderSessionEvent::OrderUpdate(order)
                        if order.order_ref.as_str() == reference
                            && matches!(
                                order.order_status,
                                OrderStatus::NoTradeQueueing
                                    | OrderStatus::PartTradedQueueing
                                    | OrderStatus::AllTraded
                            ) =>
                    {
                        let _ = self.cancel(&reference);
                        return Err(
                            format!("{name} 未被拒绝，状态={:?}", order.order_status).into()
                        );
                    }
                    TraderSessionEvent::Error { message, .. }
                    | TraderSessionEvent::ConversionFailed { message } => {
                        return Err(
                            format!("{name} 测试出现非本单证据的会话错误：{message}").into()
                        );
                    }
                    TraderSessionEvent::Disconnected { reason } => {
                        return Err(format!("{name} 测试期间断线：{reason}").into());
                    }
                    _ => {}
                }
            } else if Instant::now() >= deadline {
                return Err(format!("{name} 未收到明确结果").into());
            } else {
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }

    pub fn reconnect(&self) -> Result<(), Box<dyn Error>> {
        self.session.reconnect()?;
        let deadline = Instant::now() + WAIT;
        loop {
            if let Some(event) = self.session.try_event() {
                match event {
                    TraderSessionEvent::Recovered { .. } => return Ok(()),
                    TraderSessionEvent::Error { message, .. }
                    | TraderSessionEvent::ConversionFailed { message } => {
                        return Err(message.into());
                    }
                    _ => {}
                }
            } else if Instant::now() >= deadline {
                return Err("等待 Recovered 超时".into());
            } else {
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }

    pub fn print_stats(&self) {
        println!(
            "本地统计：提交尝试={}，撤单尝试={}，重复业务意图={}，成交手数={}，消息={}",
            self.stats.order_count,
            self.stats.cancel_count,
            self.stats.repeat_intent,
            self.stats.trade_volume,
            self.stats.messages
        );
    }

    pub fn exit(self) {
        self.session.exit();
    }
}
