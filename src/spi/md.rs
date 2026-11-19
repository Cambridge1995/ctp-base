// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use crate::sys::convert::ConversionError;
use crate::{
    DepthMarketData, DisconnectReason, ForQuoteRsp, MulticastInstrument, RspInfo, RspUserLogin,
    SpecificInstrument, UserLogout,
};

/// 原生回调线程中的调用方 SPI；只需覆盖关心的方法。
/// 回调参数已经 owned 化，可直接保存或转移；慢 IO 和长时间阻塞应由调用方转交。
#[allow(unused_variables)]
pub trait MdSpi: Send {
    /// owned 转换失败时调用；失败的原回调不会继续分派。
    fn conversion_error(&mut self, error: ConversionError) {}
    fn connected(&mut self) {}
    fn disconnected(&mut self, reason: DisconnectReason) {}
    fn heartbeat(&mut self, time_lapse: i32) {}
    fn login_response(
        &mut self,
        rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn logout_response(
        &mut self,
        user_logout: Option<UserLogout>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn multicast_instrument_response(
        &mut self,
        multicast_instrument: Option<MulticastInstrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn error(&mut self, rsp_info: Option<RspInfo>, request_id: i32, is_last: bool) {}
    fn subscribe_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn unsubscribe_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn subscribe_quote_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn unsubscribe_quote_response(
        &mut self,
        specific_instrument: Option<SpecificInstrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn tick(&mut self, depth_market_data: Option<DepthMarketData>) {}
    fn for_quote_response(&mut self, for_quote_rsp: Option<ForQuoteRsp>) {}
}
