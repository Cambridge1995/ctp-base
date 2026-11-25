use super::md::test_md_spi;
use crate::config;
use ctp_base::sys;
use ctp_base::*;
use std::{
    os::raw::c_char,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

static NEXT_FLOW_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Default)]
struct RecordingSpi {
    ticks: Arc<Mutex<Vec<Option<DepthMarketData>>>>,
    errors: Arc<Mutex<Vec<ConversionError>>>,
}

impl MdSpi for RecordingSpi {
    fn conversion_error(&mut self, error: ConversionError) {
        self.errors.lock().unwrap().push(error);
    }

    fn tick(&mut self, depth_market_data: Option<DepthMarketData>) {
        self.ticks.lock().unwrap().push(depth_market_data);
    }
}

struct EmptySpi;

impl MdSpi for EmptySpi {}

fn test_api() -> (MdApi, PathBuf) {
    test_api_with_spi(Box::new(EmptySpi))
}

fn test_api_with_spi(spi: Box<dyn MdSpi>) -> (MdApi, PathBuf) {
    let id = NEXT_FLOW_ID.fetch_add(1, Ordering::Relaxed);
    let flow_dir = std::env::temp_dir().join(format!("ctp-base-md-{}-{id}", std::process::id()));
    std::fs::create_dir_all(&flow_dir).unwrap();
    let flow_path = format!("{}{}", flow_dir.display(), std::path::MAIN_SEPARATOR);
    let api = MdApi::new(spi, &flow_path, false, false, true).unwrap();
    (api, flow_dir)
}

fn close_api(api: MdApi, flow_dir: PathBuf) {
    drop(api);
    std::fs::remove_dir_all(flow_dir).unwrap();
}

fn sample_login(ctp: &config::CtpConfig) -> ReqUserLogin {
    ReqUserLogin {
        trading_day: DateText::new("20261117").unwrap(),
        broker_id: BrokerId::new(&ctp.broker_id).unwrap(),
        user_id: UserId::new(&ctp.user_id).unwrap(),
        password: ctp.password.clone(),
        user_product_info: "product".into(),
        interface_product_info: "interface".into(),
        protocol_info: "protocol".into(),
        mac_address: "00:00:00:00:00:00".into(),
        one_time_password: String::new(),
        login_remark: String::new(),
        client_ip_port: 0,
        client_ip_address: "127.0.0.1".into(),
        sms_code: String::new(),
    }
}

#[test]
fn null_market_data_is_not_an_error() {
    let spi = RecordingSpi::default();
    let ticks = spi.ticks.clone();
    let mut bridge = test_md_spi(spi);
    bridge.emit(None);
    assert!(ticks.lock().unwrap()[0].is_none());
}

#[test]
fn market_data_is_owned_before_callback_returns() {
    let spi = RecordingSpi::default();
    let ticks = spi.ticks.clone();
    let mut bridge = test_md_spi(spi);
    let mut raw = sys::CThostFtdcDepthMarketDataField::default();
    raw.TradingDay[0..8].copy_from_slice(&[b'2' as c_char; 8]);
    raw.LastPrice = 123.5;
    raw.Volume = 7;
    raw.InstrumentID[0] = b'I' as c_char;
    raw.InstrumentID[1] = b'F' as c_char;

    bridge.emit(Some(&mut raw));
    raw.LastPrice = 0.0;
    raw.Volume = 0;
    raw.InstrumentID[0] = 0;
    assert_eq!(raw.InstrumentID[0], 0);

    let ticks = ticks.lock().unwrap();
    let tick = ticks[0].as_ref().unwrap();
    assert_eq!(tick.last_price, 123.5);
    assert_eq!(tick.volume, 7);
    assert_eq!(tick.instrument_id, InstrumentId::new("IF").unwrap());
}

#[test]
fn invalid_identifier_is_reported() {
    let spi = RecordingSpi::default();
    let ticks = spi.ticks.clone();
    let errors = spi.errors.clone();
    let mut bridge = test_md_spi(spi);
    let mut raw = sys::CThostFtdcDepthMarketDataField::default();
    raw.InstrumentID[0] = 0x80u8 as c_char;
    bridge.emit(Some(&mut raw));
    assert!(ticks.lock().unwrap().is_empty());
    assert_eq!(
        errors.lock().unwrap().as_slice(),
        &[ConversionError::new(
            "InstrumentID",
            TextError::NonAscii { index: 0 },
        )]
    );
}

#[test]
fn response_text_may_fill_the_whole_wire_field() {
    let spi = RecordingSpi::default();
    let ticks = spi.ticks.clone();
    let mut bridge = test_md_spi(spi);
    let mut raw = sys::CThostFtdcDepthMarketDataField::default();
    raw.InstrumentID.fill(b'a' as c_char);
    bridge.emit(Some(&mut raw));
    let ticks = ticks.lock().unwrap();
    let tick = ticks[0].as_ref().unwrap();
    assert_eq!(tick.instrument_id.as_str().len(), raw.InstrumentID.len());
}

#[test]
fn caller_may_implement_only_selected_spi_methods() {
    let (api, flow_dir) = test_api();
    close_api(api, flow_dir);
}

#[test]
fn caller_panic_does_not_cross_ffi_or_stop_later_callbacks() {
    #[derive(Clone, Default)]
    struct PanicOnceSpi(Arc<AtomicUsize>);

    impl MdSpi for PanicOnceSpi {
        fn tick(&mut self, _depth_market_data: Option<DepthMarketData>) {
            if self.0.fetch_add(1, Ordering::Relaxed) == 0 {
                panic!("测试调用方 panic");
            }
        }
    }

    let spi = PanicOnceSpi::default();
    let calls = spi.0.clone();
    let mut bridge = test_md_spi(spi);
    bridge.emit(None);
    bridge.emit(None);
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

#[test]
fn api_releases_native_object_before_dropping_spi() {
    struct DropSpi(Arc<AtomicBool>);

    impl MdSpi for DropSpi {}

    impl Drop for DropSpi {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    let dropped = Arc::new(AtomicBool::new(false));
    let (api, flow_dir) = test_api_with_spi(Box::new(DropSpi(dropped.clone())));
    api.close();
    assert!(dropped.load(Ordering::Relaxed));
    std::fs::remove_dir_all(flow_dir).unwrap();
}

#[test]
fn login_request_checks_string_capacity() {
    let (api, flow_dir) = test_api();
    let ctp = config::CtpConfig::load().unwrap();
    let mut request = sample_login(&ctp);
    request.password = "p".repeat(40);
    assert!(api.login(&request, 1).is_ok());
    request.password.push('p');
    assert_eq!(
        api.login(&request, 1).unwrap_err(),
        ConversionError::new("Password", TextError::TooLong { maximum: 40 })
    );
    close_api(api, flow_dir);
}

#[test]
fn login_request_rejects_nul_and_non_ascii() {
    let (api, flow_dir) = test_api();
    let ctp = config::CtpConfig::load().unwrap();
    let mut request = sample_login(&ctp);
    request.password = "bad\0password".into();
    assert_eq!(
        api.login(&request, 1).unwrap_err(),
        ConversionError::new("Password", TextError::InteriorNul { index: 3 })
    );
    request.password = "密码".into();
    assert_eq!(
        api.login(&request, 1).unwrap_err(),
        ConversionError::new("Password", TextError::NonAscii { index: 0 })
    );
    close_api(api, flow_dir);
}
