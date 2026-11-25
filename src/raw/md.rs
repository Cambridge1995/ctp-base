use crate::sys::{
    convert::{ConversionError, ensure_no_nul},
    events::md::MdSpiSink,
    ffi,
};
use crate::{
    FensUserInfo, InstrumentId, MdSpi, QryMulticastInstrument, ReqUserLogin, UserLogout, sys,
};
use cxx::UniquePtr;
use std::{error::Error, fmt};

pub struct MdApi {
    inner: UniquePtr<ffi::md::MdApiBridge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateMdApiError {
    InvalidFlowPath,
    CreationFailed,
}

impl fmt::Display for CreateMdApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFlowPath => formatter.write_str("流量目录包含 NUL 字节"),
            Self::CreationFailed => formatter.write_str("CTP 行情 API 创建失败"),
        }
    }
}

impl Error for CreateMdApiError {}

impl MdApi {
    /// 创建原生 API 并注册 SPI；必须在调用 `init` 前完成，SPI 随 API 持有并在原生释放后销毁。
    ///
    /// 参数与 CTP `CreateFtdcMdApi(pszFlowPath, bIsUsingUdp, bIsMulticast, bIsProductionMode)`
    /// 一一对应；常规使用建议走 `session` 托管层（带默认值的配置）。
    pub fn new(
        spi: Box<dyn MdSpi>,
        flow_path: &str,
        using_udp: bool,
        multicast: bool,
        production_mode: bool,
    ) -> Result<Self, CreateMdApiError> {
        if flow_path.as_bytes().contains(&0) {
            return Err(CreateMdApiError::InvalidFlowPath);
        }
        let inner = ffi::md::create_md_api(
            Box::new(MdSpiSink::new(spi)),
            flow_path,
            using_udp,
            multicast,
            production_mode,
        );
        if inner.is_null() {
            return Err(CreateMdApiError::CreationFailed);
        }
        Ok(Self { inner })
    }

    pub fn login(&self, request: &ReqUserLogin, request_id: i32) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是按官方布局生成且已完整初始化的登录请求；指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.req_user_login(
                (&mut raw as *mut sys::CThostFtdcReqUserLoginField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn version(&self) -> String {
        self.inner.version()
    }

    pub fn init(&self) {
        self.inner.init();
    }

    pub fn join(&self) -> i32 {
        self.inner.join()
    }

    pub fn trading_day(&self) -> String {
        self.inner.trading_day()
    }

    pub fn register_front(&self, address: &str) -> Result<(), ConversionError> {
        ensure_no_nul(address, "FrontAddress")?;
        self.inner.register_front(address);
        Ok(())
    }

    pub fn register_name_server(&self, address: &str) -> Result<(), ConversionError> {
        ensure_no_nul(address, "NameServerAddress")?;
        self.inner.register_name_server(address);
        Ok(())
    }

    pub fn register_fens_user_info(&self, request: &FensUserInfo) -> Result<(), ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的 Fens 请求，指针仅在同步调用期间有效。
        unsafe {
            self.inner.register_fens_user_info(
                (&mut raw as *mut sys::CThostFtdcFensUserInfoField).cast(),
            );
        }
        Ok(())
    }

    pub fn subscribe(&self, instruments: &[&str]) -> Result<i32, ConversionError> {
        let instruments = instrument_ids(instruments)?;
        Ok(self.inner.subscribe(instruments))
    }

    pub fn unsubscribe(&self, instruments: &[&str]) -> Result<i32, ConversionError> {
        let instruments = instrument_ids(instruments)?;
        Ok(self.inner.unsubscribe(instruments))
    }

    pub fn subscribe_quote(&self, instruments: &[&str]) -> Result<i32, ConversionError> {
        let instruments = instrument_ids(instruments)?;
        Ok(self.inner.subscribe_quote(instruments))
    }

    pub fn unsubscribe_quote(&self, instruments: &[&str]) -> Result<i32, ConversionError> {
        let instruments = instrument_ids(instruments)?;
        Ok(self.inner.unsubscribe_quote(instruments))
    }

    pub fn logout(&self, request: &UserLogout, request_id: i32) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的登出请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.logout(
                (&mut raw as *mut sys::CThostFtdcUserLogoutField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_multicast_instrument(
        &self,
        request: &QryMulticastInstrument,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的组播合约查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_multicast_instrument(
                (&mut raw as *mut sys::CThostFtdcQryMulticastInstrumentField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    /// 释放原生 API；返回后不再调用 SPI。
    pub fn close(self) {}
}

fn instrument_ids(instruments: &[&str]) -> Result<Vec<String>, ConversionError> {
    instruments
        .iter()
        .map(|value| {
            InstrumentId::new(value)
                .map(|id| id.as_str().to_owned())
                .map_err(|source| ConversionError::new("InstrumentID", source))
        })
        .collect()
}
