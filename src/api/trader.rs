use crate::sys::{
    convert::{ConversionError, ensure_no_nul, string_from_wire},
    events::trader::TraderSpiSink,
    ffi,
};
use crate::{
    FensUserInfo, FrontInfo, InputOrder, InputOrderAction, QryDepthMarketData, QryExchange,
    QryInstrument, QryInstrumentCommissionRate, QryInstrumentMarginRate,
    QryInstrumentOrderCommRate, QryInvestorPosition, QryInvestorPositionDetail, QryMaxOrderVolume,
    QryOrder, QryProduct, QrySettlementInfo, QrySettlementInfoConfirm, QryTrade, QryTradingAccount,
    QryTradingCode, ReqAuthenticate, ReqUserLogin, ResumeType, SettlementInfoConfirm, TextError,
    TraderSpi, UserLogout, sys,
};
use cxx::UniquePtr;
use std::{error::Error, fmt, ops::Deref};

#[derive(Debug, Clone)]
pub struct TraderConfig {
    pub flow_path: String,
    pub production_mode: bool,
}

impl TraderConfig {
    pub fn new(flow_path: impl Into<String>) -> Self {
        Self {
            flow_path: flow_path.into(),
            production_mode: true,
        }
    }
}

pub struct TraderApi {
    inner: UniquePtr<ffi::trader::TraderApiBridge>,
}

pub struct SharedTraderApi {
    inner: TraderApi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateTraderApiError {
    InvalidFlowPath,
    CreationFailed,
}

impl fmt::Display for CreateTraderApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFlowPath => formatter.write_str("流量目录包含 NUL 字节"),
            Self::CreationFailed => formatter.write_str("CTP 交易 API 创建失败"),
        }
    }
}

impl Error for CreateTraderApiError {}

// SAFETY: CTP 主动请求接口支持应用线程并发调用；调用方负责串行化配置与释放。
unsafe impl Send for SharedTraderApi {}
// SAFETY: 与 Send 相同，调用方不得与释放操作并发访问。
unsafe impl Sync for SharedTraderApi {}

impl Deref for SharedTraderApi {
    type Target = TraderApi;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// noinspection DuplicatedCode
impl TraderApi {
    /// 创建原生 API 并注册 SPI；必须在调用 `init` 前完成，SPI 随 API 持有并在原生释放后销毁。
    pub fn new(
        config: TraderConfig,
        spi: Box<dyn TraderSpi>,
    ) -> Result<Self, CreateTraderApiError> {
        if config.flow_path.as_bytes().contains(&0) {
            return Err(CreateTraderApiError::InvalidFlowPath);
        }
        let inner = ffi::trader::create_trader_api(
            Box::new(TraderSpiSink::new(spi)),
            &config.flow_path,
            false,
            false,
            config.production_mode,
        );
        if inner.is_null() {
            return Err(CreateTraderApiError::CreationFailed);
        }
        Ok(Self { inner })
    }

    pub fn front_info(&self) -> Result<FrontInfo, ConversionError> {
        let mut raw = sys::CThostFtdcFrontInfoField::default();
        // SAFETY: raw 是完整初始化的前置信息结构，指针仅在同步调用期间有效。
        unsafe {
            self.inner
                .front_info((&mut raw as *mut sys::CThostFtdcFrontInfoField).cast())
        };
        Ok(FrontInfo {
            front_address: string_from_wire(&raw.FrontAddr, "FrontAddr")?,
            qry_frequency: raw.QryFreq,
            ftd_pkg_frequency: raw.FTDPkgFreq,
        })
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

    pub fn subscribe_private(&self, resume: ResumeType) -> Result<(), ConversionError> {
        let code = resume
            .code()
            .ok_or_else(|| ConversionError::new("ResumeType", TextError::UnknownEnumValue))?;
        self.inner.subscribe_private(code as i32);
        Ok(())
    }

    pub fn subscribe_public(&self, resume: ResumeType) -> Result<(), ConversionError> {
        let code = resume
            .code()
            .ok_or_else(|| ConversionError::new("ResumeType", TextError::UnknownEnumValue))?;
        self.inner.subscribe_public(code as i32);
        Ok(())
    }

    pub fn authenticate(
        &self,
        request: &ReqAuthenticate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的认证请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.authenticate(
                (&mut raw as *mut sys::CThostFtdcReqAuthenticateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn login(&self, request: &ReqUserLogin, request_id: i32) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的登录请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.login(
                (&mut raw as *mut sys::CThostFtdcReqUserLoginField).cast(),
                request_id,
            )
        };
        Ok(result)
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

    pub fn confirm_settlement(
        &self,
        request: &SettlementInfoConfirm,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的结算确认，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.confirm_settlement(
                (&mut raw as *mut sys::CThostFtdcSettlementInfoConfirmField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn settlement_confirmation(
        &self,
        request: &QrySettlementInfoConfirm,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的结算状态查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.settlement_confirmation(
                (&mut raw as *mut sys::CThostFtdcQrySettlementInfoConfirmField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_settlement_info(
        &self,
        request: &QrySettlementInfo,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的结算单查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_settlement_info(
                (&mut raw as *mut sys::CThostFtdcQrySettlementInfoField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_position_detail(
        &self,
        request: &QryInvestorPositionDetail,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的持仓明细查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_position_detail(
                (&mut raw as *mut sys::CThostFtdcQryInvestorPositionDetailField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_product(
        &self,
        request: &QryProduct,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的产品查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_product(
                (&mut raw as *mut sys::CThostFtdcQryProductField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_exchange(
        &self,
        request: &QryExchange,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的交易所查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_exchange(
                (&mut raw as *mut sys::CThostFtdcQryExchangeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_trading_code(
        &self,
        request: &QryTradingCode,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的交易编码查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_trading_code(
                (&mut raw as *mut sys::CThostFtdcQryTradingCodeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_instrument_margin_rate(
        &self,
        request: &QryInstrumentMarginRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的保证金率查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_instrument_margin_rate(
                (&mut raw as *mut sys::CThostFtdcQryInstrumentMarginRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_instrument_commission_rate(
        &self,
        request: &QryInstrumentCommissionRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的手续费率查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_instrument_commission_rate(
                (&mut raw as *mut sys::CThostFtdcQryInstrumentCommissionRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_instrument_order_comm_rate(
        &self,
        request: &QryInstrumentOrderCommRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的报单手续费率查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_instrument_order_comm_rate(
                (&mut raw as *mut sys::CThostFtdcQryInstrumentOrderCommRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_max_order_volume(
        &self,
        request: &QryMaxOrderVolume,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的最大可报数量查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_max_order_volume(
                (&mut raw as *mut sys::CThostFtdcQryMaxOrderVolumeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_depth_market_data(
        &self,
        request: &QryDepthMarketData,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的行情快照查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_depth_market_data(
                (&mut raw as *mut sys::CThostFtdcQryDepthMarketDataField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_order(&self, request: &QryOrder, request_id: i32) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的报单查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_order(
                (&mut raw as *mut sys::CThostFtdcQryOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_trade(&self, request: &QryTrade, request_id: i32) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的成交查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_trade(
                (&mut raw as *mut sys::CThostFtdcQryTradeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_position(
        &self,
        request: &QryInvestorPosition,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的持仓查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_position(
                (&mut raw as *mut sys::CThostFtdcQryInvestorPositionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_trading_account(
        &self,
        request: &QryTradingAccount,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的资金查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_trading_account(
                (&mut raw as *mut sys::CThostFtdcQryTradingAccountField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_instrument(
        &self,
        request: &QryInstrument,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的合约查询，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_instrument(
                (&mut raw as *mut sys::CThostFtdcQryInstrumentField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn insert_order(
        &self,
        request: &InputOrder,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的报单请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.insert_order(
                (&mut raw as *mut sys::CThostFtdcInputOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn cancel_order(
        &self,
        request: &InputOrderAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的撤单请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.cancel_order(
                (&mut raw as *mut sys::CThostFtdcInputOrderActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    /// 释放原生 API；返回后不再调用 SPI。
    pub fn close(self) {}

    pub fn into_shared(self) -> SharedTraderApi {
        SharedTraderApi { inner: self }
    }
}
