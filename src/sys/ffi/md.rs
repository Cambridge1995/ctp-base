// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use crate::sys::events::md::*;

#[allow(dead_code, clippy::missing_safety_doc)]
#[cxx::bridge(namespace = "ctp_base::md")]
mod bridge {
    unsafe extern "C++" {
        include!("cpp/bridge.h");

        type MdApiBridge;
        type MdSpiBridge;

        fn create_md_api(
            sink: Box<MdSpiSink>,
            flow_path: &str,
            using_udp: bool,
            multicast: bool,
            production_mode: bool,
        ) -> UniquePtr<MdApiBridge>;
        fn create_md_spi(sink: Box<MdSpiSink>) -> UniquePtr<MdSpiBridge>;
        fn version(self: &MdApiBridge) -> String;
        fn init(self: &MdApiBridge);
        fn join(self: &MdApiBridge) -> i32;
        fn trading_day(self: &MdApiBridge) -> String;
        fn register_front(self: &MdApiBridge, address: &str);
        fn register_name_server(self: &MdApiBridge, address: &str);
        unsafe fn register_fens_user_info(self: &MdApiBridge, request: *mut u8);
        fn subscribe(self: &MdApiBridge, instruments: Vec<String>) -> i32;
        fn unsubscribe(self: &MdApiBridge, instruments: Vec<String>) -> i32;
        fn subscribe_quote(self: &MdApiBridge, instruments: Vec<String>) -> i32;
        fn unsubscribe_quote(self: &MdApiBridge, instruments: Vec<String>) -> i32;
        unsafe fn req_user_login(self: &MdApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn logout(self: &MdApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_multicast_instrument(
            self: &MdApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn emit_depth_market_data(spi: Pin<&mut MdSpiBridge>, data: *mut u8);
        fn emit_md_front_connected(spi: Pin<&mut MdSpiBridge>);
        unsafe fn emit_md_callback(
            spi: Pin<&mut MdSpiBridge>,
            callback_index: u32,
            first: *mut u8,
            second: *mut u8,
            request_id: i32,
            last: bool,
        );
    }

    extern "Rust" {
        type MdSpiSink;

        fn connected(sink: &mut MdSpiSink);
        fn disconnected(sink: &mut MdSpiSink, reason: i32);
        fn heartbeat(sink: &mut MdSpiSink, time_lapse: i32);
        unsafe fn login_response(
            sink: &mut MdSpiSink,
            rsp_user_login: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn logout_response(
            sink: &mut MdSpiSink,
            user_logout: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn multicast_instrument_response(
            sink: &mut MdSpiSink,
            multicast_instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn error(sink: &mut MdSpiSink, rsp_info: *const u8, request_id: i32, is_last: bool);
        unsafe fn subscribe_response(
            sink: &mut MdSpiSink,
            specific_instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn unsubscribe_response(
            sink: &mut MdSpiSink,
            specific_instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn subscribe_quote_response(
            sink: &mut MdSpiSink,
            specific_instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn unsubscribe_quote_response(
            sink: &mut MdSpiSink,
            specific_instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn tick(sink: &mut MdSpiSink, depth_market_data: *const u8);
        unsafe fn for_quote_response(sink: &mut MdSpiSink, for_quote_rsp: *const u8);
    }
}

pub use bridge::*;
