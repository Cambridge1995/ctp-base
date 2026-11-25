use crate::session::enums::{Input, MdEvent, MdSessionEvent};
use crate::{
    ConversionError, DepthMarketData, DisconnectReason, ForQuoteRsp, MdSpi, RspInfo, RspUserLogin,
    SpecificInstrument,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::SyncSender;

/// 行情 SPI 适配器：只转换并入队，不做任何决策。
/// tick 与询价直送事件通道，控制信号进 loop 输入队列。
pub(crate) struct MdSpiImpl {
    inputs: SyncSender<Input>,
    events: SyncSender<MdSessionEvent>,
    dropped: Arc<AtomicU64>,
}

impl MdSpiImpl {
    /// `dropped` 为丢弃计数（队列满或转换失败时递增）。
    pub(crate) fn new(
        inputs: SyncSender<Input>,
        events: SyncSender<MdSessionEvent>,
        dropped: Arc<AtomicU64>,
    ) -> Self {
        Self {
            inputs,
            events,
            dropped,
        }
    }

    /// 非阻塞投递控制事件；队列满时丢弃并计数（回调线程绝不阻塞）。
    fn send_control(&mut self, event: MdEvent) {
        if self.inputs.try_send(Input::Spi(event)).is_err() {
            self.dropped.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// 非阻塞直送数据事件；队列满时丢弃并计数。
    fn send_data(&mut self, event: MdSessionEvent) {
        if self.events.try_send(event).is_err() {
            self.dropped.fetch_add(1, Ordering::AcqRel);
        }
    }
}

impl MdSpi for MdSpiImpl {
    // heartbeat：目前 CTP C++ 端暂不支持该方法，session 层不予实现。
    // multicast_instrument_response：组播行情地址查询需要托管在期货商服务器中，
    // 服务于高频交易，session 层不予实现；若希望进行高频交易，可用 CTP mini。

    /// owned 转换失败：数据损坏被丢弃，计入丢弃数并上报事件，不得无声无息。
    fn conversion_error(&mut self, error: ConversionError) {
        self.dropped.fetch_add(1, Ordering::AcqRel);
        self.send_control(MdEvent::ConversionFailed(format!("{error:?}")));
    }

    fn connected(&mut self) {
        self.send_control(MdEvent::Connected);
    }

    fn disconnected(&mut self, reason: DisconnectReason) {
        self.send_control(MdEvent::Disconnected(reason));
    }

    fn login_response(
        &mut self,
        rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match rsp_info {
            Some(info) if info.error_id != 0 => self.send_control(MdEvent::LoginFailed {
                error_id: info.error_id,
                message: info.error_msg,
            }),
            _ => match rsp_user_login {
                Some(login) => self.send_control(MdEvent::LoginOk {
                    trading_day: login.trading_day.as_str().to_owned(),
                }),
                None => self.send_control(MdEvent::LoginOk {
                    trading_day: String::new(),
                }),
            },
        }
    }
    // logout_response：行情前置不支持登出（错误 77「无此功能」），
    // Session 层不对接该回调，走 trait 默认空实现（FAQ 的 logout/重连条目）。

    /// 通用请求级错误（如流控拒绝）：转成控制事件上报。
    fn error(&mut self, rsp_info: Option<RspInfo>, _request_id: i32, _is_last: bool) {
        if let Some(info) = rsp_info
            && info.error_id != 0
        {
            self.send_control(MdEvent::Error {
                error_id: info.error_id,
                message: info.error_msg,
            });
        }
    }

    fn subscribe_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(event) = instrument_event(specific_instrument, rsp_info, MdEvent::Subscribed) {
            self.send_control(event);
        }
    }

    fn unsubscribe_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(event) = instrument_event(specific_instrument, rsp_info, MdEvent::Unsubscribed)
        {
            self.send_control(event);
        }
    }

    fn subscribe_quote_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(event) =
            instrument_event(specific_instrument, rsp_info, MdEvent::QuoteSubscribed)
        {
            self.send_control(event);
        }
    }

    fn unsubscribe_quote_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        if let Some(event) =
            instrument_event(specific_instrument, rsp_info, MdEvent::QuoteUnsubscribed)
        {
            self.send_control(event);
        }
    }

    /// tick 直送事件通道：不经过 loop，行情洪流不会占用控制队列。
    fn tick(&mut self, depth_market_data: Option<DepthMarketData>) {
        if let Some(data) = depth_market_data {
            self.send_data(MdSessionEvent::Tick(Box::new(data)));
        }
    }

    /// 询价回报直送事件通道（期权做市相关）。
    fn for_quote_response(&mut self, for_quote_rsp: Option<ForQuoteRsp>) {
        if let Some(quote) = for_quote_rsp {
            self.send_data(MdSessionEvent::Quote(Box::new(quote)));
        }
    }
}

/// 从响应中提取合约代码；错误响应（error_id 非 0）转为错误控制事件。
fn instrument_event(
    specific_instrument: Option<SpecificInstrument>,
    rsp_info: Option<RspInfo>,
    success: impl FnOnce(String) -> MdEvent,
) -> Option<MdEvent> {
    if let Some(info) = rsp_info
        && info.error_id != 0
    {
        return Some(MdEvent::Error {
            error_id: info.error_id,
            message: info.error_msg,
        });
    }
    specific_instrument.map(|instrument| success(instrument.instrument_id.as_str().to_owned()))
}
