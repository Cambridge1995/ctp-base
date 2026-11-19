use ctp_base::sys::{self, events::trader::TraderSpiSink, ffi};
use ctp_base::{Direction, OffsetFlag, Order, TraderSpi};
use cxx::UniquePtr;
use std::sync::{Arc, Mutex};

pub(crate) struct TestTraderSpi {
    inner: UniquePtr<ffi::trader::TraderSpiBridge>,
}

pub(crate) fn test_trader_spi(spi: impl TraderSpi + 'static) -> TestTraderSpi {
    TestTraderSpi {
        inner: ffi::trader::create_trader_spi(Box::new(TraderSpiSink::new(Box::new(spi)))),
    }
}

impl TestTraderSpi {
    pub(crate) fn emit_order(&mut self, data: Option<&mut sys::CThostFtdcOrderField>) {
        let pointer = data.map_or(std::ptr::null_mut(), |raw| {
            (raw as *mut sys::CThostFtdcOrderField).cast::<u8>()
        });
        // SAFETY: pointer 为空或来自有效期覆盖整个同步调用的官方报单结构。
        unsafe { ffi::trader::emit_trader_order(self.inner.as_mut().unwrap(), pointer) };
    }

    pub(crate) fn emit_callback(&mut self, callback_index: u32) {
        // SAFETY: 测试注入的两个空指针由 Rust 回调显式处理，不用于解引用。
        unsafe {
            ffi::trader::emit_trader_callback(
                self.inner.as_mut().unwrap(),
                callback_index,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                7,
                true,
            )
        };
    }
}

#[derive(Clone, Default)]
struct RecordingSpi(Arc<Mutex<Vec<Option<Order>>>>);

impl TraderSpi for RecordingSpi {
    fn order_update(&mut self, order: Option<Order>) {
        self.0.lock().unwrap().push(order);
    }
}

#[test]
fn trader_order_callback_preserves_null() {
    let spi = RecordingSpi::default();
    let seen = spi.0.clone();
    let mut bridge = test_trader_spi(spi);
    bridge.emit_order(None);
    assert!(seen.lock().unwrap()[0].is_none());
}

#[test]
fn trader_order_preserves_unknown_values_and_comb_tail() {
    let spi = RecordingSpi::default();
    let seen = spi.0.clone();
    let mut bridge = test_trader_spi(spi);
    let mut raw = sys::CThostFtdcOrderField {
        Direction: 0x7F,
        ..Default::default()
    };
    raw.CombOffsetFlag
        .copy_from_slice(&[0xFF, 1, 2, 3, 4].map(|byte| byte as c_char));
    bridge.emit_order(Some(&mut raw));

    let seen = seen.lock().unwrap();
    let order = seen[0].as_ref().unwrap();
    assert_eq!(order.direction, Direction::UnknownValue(0x7F));
    assert_eq!(
        order.comb_offset_flag.first(),
        OffsetFlag::UnknownValue(0xFF)
    );
    assert_eq!(order.comb_offset_flag.tail(), &[1, 2, 3, 4]);
}

use std::os::raw::c_char;
