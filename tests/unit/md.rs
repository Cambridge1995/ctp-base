use ctp_base::sys::{self, events::md::MdSpiSink, ffi};
use ctp_base::{MdSpi, RspInfo};
use cxx::UniquePtr;
use std::{
    os::raw::c_char,
    sync::{Arc, Mutex},
};

pub(crate) struct TestMdSpi {
    inner: UniquePtr<ffi::md::MdSpiBridge>,
}

pub(crate) fn test_md_spi(spi: impl MdSpi + 'static) -> TestMdSpi {
    TestMdSpi {
        inner: ffi::md::create_md_spi(Box::new(MdSpiSink::new(Box::new(spi)))),
    }
}

impl TestMdSpi {
    pub(crate) fn emit(&mut self, data: Option<&mut sys::CThostFtdcDepthMarketDataField>) {
        let pointer = data.map_or(std::ptr::null_mut(), |raw| {
            (raw as *mut sys::CThostFtdcDepthMarketDataField).cast::<u8>()
        });
        // SAFETY: pointer 为空或来自有效期覆盖整个同步调用的官方行情结构。
        unsafe { ffi::md::emit_depth_market_data(self.inner.as_mut().unwrap(), pointer) };
    }

    pub(crate) fn emit_callback(&mut self, callback_index: u32) {
        // SAFETY: 测试注入的两个空指针由 Rust 回调显式处理，不用于解引用。
        unsafe {
            ffi::md::emit_md_callback(
                self.inner.as_mut().unwrap(),
                callback_index,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                7,
                true,
            )
        };
    }

    fn emit_error(
        &mut self,
        info: &mut sys::CThostFtdcRspInfoField,
        request_id: i32,
        is_last: bool,
    ) {
        // SAFETY: 第一个指针指向有效错误结构，第二个空指针会被忽略；索引 6 对应 OnRspError。
        unsafe {
            ffi::md::emit_md_callback(
                self.inner.as_mut().unwrap(),
                6,
                (info as *mut sys::CThostFtdcRspInfoField).cast(),
                std::ptr::null_mut(),
                request_id,
                is_last,
            )
        };
    }
}

type Responses = Arc<Mutex<Vec<(Option<RspInfo>, i32, bool)>>>;

#[derive(Clone, Default)]
struct RecordingSpi(Responses);

impl MdSpi for RecordingSpi {
    fn error(&mut self, rsp_info: Option<RspInfo>, request_id: i32, is_last: bool) {
        self.0.lock().unwrap().push((rsp_info, request_id, is_last));
    }
}

#[test]
fn response_preserves_metadata_and_gbk_text() {
    let spi = RecordingSpi::default();
    let seen = spi.0.clone();
    let mut bridge = test_md_spi(spi);
    let mut raw = sys::CThostFtdcRspInfoField {
        ErrorID: 7,
        ..Default::default()
    };
    raw.ErrorMsg[..4].copy_from_slice(&[0xB4, 0xED, 0xCE, 0xF3].map(|byte| byte as c_char));

    bridge.emit_error(&mut raw, 42, true);
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    let (info, request_id, is_last) = &seen[0];
    let info = info.as_ref().unwrap();
    assert_eq!(info.error_id, 7);
    assert_eq!(info.error_msg, "错误");
    assert_eq!(*request_id, 42);
    assert!(*is_last);
}
