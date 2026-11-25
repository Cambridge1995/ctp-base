// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use crate::sys::convert::owned_params;
use crate::{
    DepthMarketData, ForQuoteRsp, MdSpi, MulticastInstrument, RspInfo, RspUserLogin,
    SpecificInstrument, UserLogout,
};

pub struct MdSpiSink {
    spi: Box<dyn MdSpi>,
}

impl MdSpiSink {
    pub fn new(spi: Box<dyn MdSpi>) -> Self {
        Self { spi }
    }
}

#[inline]
fn invoke(callback: impl FnOnce()) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback));
}

pub(crate) fn connected(sink: &mut MdSpiSink) {
    invoke(|| sink.spi.connected());
}

pub(crate) fn disconnected(sink: &mut MdSpiSink, reason: i32) {
    invoke(|| sink.spi.disconnected(reason.into()));
}

pub(crate) fn heartbeat(sink: &mut MdSpiSink, time_lapse: i32) {
    invoke(|| sink.spi.heartbeat(time_lapse));
}

pub(crate) unsafe fn login_response(
    sink: &mut MdSpiSink,
    rsp_user_login: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_user_login: RspUserLogin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .login_response(rsp_user_login, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn logout_response(
    sink: &mut MdSpiSink,
    user_logout: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, user_logout: UserLogout,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .logout_response(user_logout, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn multicast_instrument_response(
    sink: &mut MdSpiSink,
    multicast_instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, multicast_instrument: MulticastInstrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .multicast_instrument_response(multicast_instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn error(
    sink: &mut MdSpiSink,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_info: RspInfo);
    invoke(|| sink.spi.error(rsp_info, request_id, is_last));
}

pub(crate) unsafe fn subscribe_response(
    sink: &mut MdSpiSink,
    specific_instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, specific_instrument: SpecificInstrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .subscribe_response(specific_instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn unsubscribe_response(
    sink: &mut MdSpiSink,
    specific_instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, specific_instrument: SpecificInstrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .unsubscribe_response(specific_instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn subscribe_quote_response(
    sink: &mut MdSpiSink,
    specific_instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, specific_instrument: SpecificInstrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .subscribe_quote_response(specific_instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn unsubscribe_quote_response(
    sink: &mut MdSpiSink,
    specific_instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, specific_instrument: SpecificInstrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .unsubscribe_quote_response(specific_instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn tick(sink: &mut MdSpiSink, depth_market_data: *const u8) {
    owned_params!(sink, depth_market_data: DepthMarketData);
    invoke(|| sink.spi.tick(depth_market_data));
}

pub(crate) unsafe fn for_quote_response(sink: &mut MdSpiSink, for_quote_rsp: *const u8) {
    owned_params!(sink, for_quote_rsp: ForQuoteRsp);
    invoke(|| sink.spi.for_quote_response(for_quote_rsp));
}
