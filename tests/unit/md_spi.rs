// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use super::md::test_md_spi;
use ctp_base::MdSpi;
use ctp_base::{
    DepthMarketData, DisconnectReason, ForQuoteRsp, MulticastInstrument, RspInfo, RspUserLogin,
    SpecificInstrument, UserLogout,
};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct RecordingSpi(Arc<Mutex<Vec<&'static str>>>);

impl RecordingSpi {
    fn push(&self, name: &'static str) {
        self.0.lock().unwrap().push(name);
    }
}

impl MdSpi for RecordingSpi {
    fn connected(&mut self) {
        self.push("connected");
    }
    fn disconnected(&mut self, _reason: DisconnectReason) {
        self.push("disconnected");
    }
    fn heartbeat(&mut self, _time_lapse: i32) {
        self.push("heartbeat");
    }
    fn login_response(
        &mut self,
        _rsp_user_login: Option<RspUserLogin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("login_response");
    }
    fn logout_response(
        &mut self,
        _user_logout: Option<UserLogout>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("logout_response");
    }
    fn multicast_instrument_response(
        &mut self,
        _multicast_instrument: Option<MulticastInstrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("multicast_instrument_response");
    }
    fn error(&mut self, _rsp_info: Option<RspInfo>, _request_id: i32, _is_last: bool) {
        self.push("error");
    }
    fn subscribe_response(
        &mut self,
        _specific_instrument: Option<SpecificInstrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("subscribe_response");
    }
    fn unsubscribe_response(
        &mut self,
        _specific_instrument: Option<SpecificInstrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("unsubscribe_response");
    }
    fn subscribe_quote_response(
        &mut self,
        _specific_instrument: Option<SpecificInstrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("subscribe_quote_response");
    }
    fn unsubscribe_quote_response(
        &mut self,
        _specific_instrument: Option<SpecificInstrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("unsubscribe_quote_response");
    }
    fn tick(&mut self, _depth_market_data: Option<DepthMarketData>) {
        self.push("tick");
    }
    fn for_quote_response(&mut self, _for_quote_rsp: Option<ForQuoteRsp>) {
        self.push("for_quote_response");
    }
}

#[test]
fn all_md_callbacks_reach_caller_spi() {
    let spi = RecordingSpi::default();
    let seen = spi.0.clone();
    let mut bridge = test_md_spi(spi);
    for index in 0..13 {
        bridge.emit_callback(index);
    }
    assert_eq!(
        *seen.lock().unwrap(),
        vec![
            "connected",
            "disconnected",
            "heartbeat",
            "login_response",
            "logout_response",
            "multicast_instrument_response",
            "error",
            "subscribe_response",
            "unsubscribe_response",
            "subscribe_quote_response",
            "unsubscribe_quote_response",
            "tick",
            "for_quote_response"
        ]
    );
}
