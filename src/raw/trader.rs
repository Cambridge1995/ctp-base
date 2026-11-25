use crate::sys::{
    convert::{ConversionError, ensure_no_nul, string_from_wire},
    events::trader::TraderSpiSink,
    ffi,
};
use crate::{
    FensUserInfo, FrontInfo, InputBatchOrderAction, InputCombAction, InputExecOrder,
    InputExecOrderAction, InputForQuote, InputHedgeCfm, InputHedgeCfmAction, InputOffsetSetting,
    InputOptionSelfClose, InputOptionSelfCloseAction, InputOrder, InputOrderAction, InputQuote,
    InputQuoteAction, InputSpdApply, InputSpdApplyAction, ParkedOrder, ParkedOrderAction,
    QryAccountRegister, QryBrokerTradingAlgos, QryBrokerTradingParams, QryCFMMCTradingAccountKey,
    QryClassifiedInstrument, QryCombAction, QryCombInstrumentGuard, QryCombLeg,
    QryCombPromotionParam, QryContractBank, QryDepthMarketData, QryEWarrantOffset, QryExchange,
    QryExchangeMarginRate, QryExchangeMarginRateAdjust, QryExchangeRate, QryExecOrder, QryForQuote,
    QryHedgeCfm, QryInstrTradeLimit, QryInstrument, QryInstrumentCommissionRate,
    QryInstrumentMarginRate, QryInstrumentOrderCommRate, QryInvestUnit, QryInvestor,
    QryInvestorCommodityGroupSPMMMargin, QryInvestorCommoditySPMMMargin, QryInvestorInfoCommRec,
    QryInvestorPortfMarginRatio, QryInvestorPortfSetting, QryInvestorPosition,
    QryInvestorPositionCombineDetail, QryInvestorPositionDetail, QryInvestorProdRCAMSMargin,
    QryInvestorProdRULEMargin, QryInvestorProdSPBMDetail, QryInvestorProductGroupMargin,
    QryMMInstrumentCommissionRate, QryMMOptionInstrCommRate, QryMaxOrderVolume,
    QryMaxOrderVolumeWithPrice, QryNotice, QryOffsetSetting, QryOptionInstrCommRate,
    QryOptionInstrTradeCost, QryOptionSelfClose, QryOrder, QryParkedOrder, QryParkedOrderAction,
    QryProduct, QryProductExchRate, QryProductGroup, QryQuote, QryRCAMSCombProductInfo,
    QryRCAMSInstrParameter, QryRCAMSInterParameter, QryRCAMSIntraParameter,
    QryRCAMSInvestorCombPosition, QryRCAMSShortOptAdjustParam, QryRULEInstrParameter,
    QryRULEInterParameter, QryRULEIntraParameter, QryRiskSettleInvestPosition,
    QryRiskSettleProductStatus, QrySPBMAddOnInterParameter, QrySPBMFutureParameter,
    QrySPBMInterParameter, QrySPBMIntraParameter, QrySPBMInvestorPortfDef, QrySPBMOptionParameter,
    QrySPBMPortfDefinition, QrySPMMInstParam, QrySPMMProductParam, QrySecAgentACIDMap,
    QrySecAgentCheckMode, QrySecAgentTradeInfo, QrySettlementInfo, QrySettlementInfoConfirm,
    QrySpdApply, QryTrade, QryTraderOffer, QryTradingAccount, QryTradingCode, QryTradingNotice,
    QryTransferBank, QryTransferSerial, QryUserSession, QueryCFMMCTradingAccountToken,
    RemoveParkedOrder, RemoveParkedOrderAction, ReqAuthenticate, ReqGenSMSCode, ReqGenUserCaptcha,
    ReqGenUserText, ReqQueryAccount, ReqTransfer, ReqUserAuthMethod, ReqUserLogin,
    ReqUserLoginWithCaptcha, ReqUserLoginWithOTP, ReqUserLoginWithText, ResumeType,
    SettlementInfoConfirm, TextError, TraderSpi, TradingAccountPasswordUpdate, UserLogout,
    UserPasswordUpdate, UserSystemInfo, WechatUserSystemInfo, sys,
};

use cxx::UniquePtr;
use std::{error::Error, fmt, ops::Deref};

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
    ///
    /// 参数对应 CTP `CreateFtdcTraderApi(pszFlowPath)`（新版 SDK 增加 production_mode）；
    /// 常规使用建议走 `session` 托管层。
    pub fn new(
        spi: Box<dyn TraderSpi>,
        flow_path: &str,
        production_mode: bool,
    ) -> Result<Self, CreateTraderApiError> {
        if flow_path.as_bytes().contains(&0) {
            return Err(CreateTraderApiError::InvalidFlowPath);
        }
        let inner = ffi::trader::create_trader_api(
            Box::new(TraderSpiSink::new(spi)),
            flow_path,
            false,
            false,
            production_mode,
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

    pub fn register_user_system_info(
        &self,
        request: &UserSystemInfo,
    ) -> Result<(), ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的终端信息，指针仅在同步调用期间有效。
        unsafe {
            self.inner.register_user_system_info(
                (&mut raw as *mut sys::CThostFtdcUserSystemInfoField).cast(),
            )
        };
        Ok(())
    }

    pub fn submit_user_system_info(&self, request: &UserSystemInfo) -> Result<(), ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的终端信息，指针仅在同步调用期间有效。
        unsafe {
            self.inner.submit_user_system_info(
                (&mut raw as *mut sys::CThostFtdcUserSystemInfoField).cast(),
            )
        };
        Ok(())
    }

    pub fn register_wechat_user_system_info(
        &self,
        request: &WechatUserSystemInfo,
    ) -> Result<(), ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的微信终端信息，指针仅在同步调用期间有效。
        unsafe {
            self.inner.register_wechat_user_system_info(
                (&mut raw as *mut sys::CThostFtdcWechatUserSystemInfoField).cast(),
            )
        };
        Ok(())
    }

    pub fn submit_wechat_user_system_info(
        &self,
        request: &WechatUserSystemInfo,
    ) -> Result<(), ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的微信终端信息，指针仅在同步调用期间有效。
        unsafe {
            self.inner.submit_wechat_user_system_info(
                (&mut raw as *mut sys::CThostFtdcWechatUserSystemInfoField).cast(),
            )
        };
        Ok(())
    }

    pub fn user_password_update(
        &self,
        request: &UserPasswordUpdate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的口令更新请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.user_password_update(
                (&mut raw as *mut sys::CThostFtdcUserPasswordUpdateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn trading_account_password_update(
        &self,
        request: &TradingAccountPasswordUpdate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的资金账户口令更新请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.trading_account_password_update(
                (&mut raw as *mut sys::CThostFtdcTradingAccountPasswordUpdateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn user_auth_method(
        &self,
        request: &ReqUserAuthMethod,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的认证模式查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.user_auth_method(
                (&mut raw as *mut sys::CThostFtdcReqUserAuthMethodField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn gen_user_captcha(
        &self,
        request: &ReqGenUserCaptcha,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的图形验证码请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.gen_user_captcha(
                (&mut raw as *mut sys::CThostFtdcReqGenUserCaptchaField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn gen_user_text(
        &self,
        request: &ReqGenUserText,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的短信验证码请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.gen_user_text(
                (&mut raw as *mut sys::CThostFtdcReqGenUserTextField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn user_login_with_captcha(
        &self,
        request: &ReqUserLoginWithCaptcha,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的图形验证码登录请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.user_login_with_captcha(
                (&mut raw as *mut sys::CThostFtdcReqUserLoginWithCaptchaField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn user_login_with_text(
        &self,
        request: &ReqUserLoginWithText,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的短信验证码登录请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.user_login_with_text(
                (&mut raw as *mut sys::CThostFtdcReqUserLoginWithTextField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn user_login_with_otp(
        &self,
        request: &ReqUserLoginWithOTP,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的动态口令登录请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.user_login_with_otp(
                (&mut raw as *mut sys::CThostFtdcReqUserLoginWithOTPField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn parked_order_insert(
        &self,
        request: &ParkedOrder,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的预埋单请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.parked_order_insert(
                (&mut raw as *mut sys::CThostFtdcParkedOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn cancel_parked_order(
        &self,
        request: &ParkedOrderAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的预埋撤单请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.cancel_parked_order(
                (&mut raw as *mut sys::CThostFtdcParkedOrderActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn remove_parked_order(
        &self,
        request: &RemoveParkedOrder,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的预埋单删除请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.remove_parked_order(
                (&mut raw as *mut sys::CThostFtdcRemoveParkedOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn remove_parked_order_action(
        &self,
        request: &RemoveParkedOrderAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的预埋撤单删除请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.remove_parked_order_action(
                (&mut raw as *mut sys::CThostFtdcRemoveParkedOrderActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn exec_order_insert(
        &self,
        request: &InputExecOrder,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的执行宣告请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.exec_order_insert(
                (&mut raw as *mut sys::CThostFtdcInputExecOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn exec_order_action(
        &self,
        request: &InputExecOrderAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的执行宣告撤销请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.exec_order_action(
                (&mut raw as *mut sys::CThostFtdcInputExecOrderActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn for_quote_insert(
        &self,
        request: &InputForQuote,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的询价录入请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.for_quote_insert(
                (&mut raw as *mut sys::CThostFtdcInputForQuoteField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn quote_insert(
        &self,
        request: &InputQuote,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的报价录入请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.quote_insert(
                (&mut raw as *mut sys::CThostFtdcInputQuoteField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn quote_action(
        &self,
        request: &InputQuoteAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的报价操作请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.quote_action(
                (&mut raw as *mut sys::CThostFtdcInputQuoteActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn batch_order_action(
        &self,
        request: &InputBatchOrderAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的批量撤单请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.batch_order_action(
                (&mut raw as *mut sys::CThostFtdcInputBatchOrderActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn option_self_close_insert(
        &self,
        request: &InputOptionSelfClose,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的期权自对冲请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.option_self_close_insert(
                (&mut raw as *mut sys::CThostFtdcInputOptionSelfCloseField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn option_self_close_action(
        &self,
        request: &InputOptionSelfCloseAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的期权自对冲撤销请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.option_self_close_action(
                (&mut raw as *mut sys::CThostFtdcInputOptionSelfCloseActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn comb_action_insert(
        &self,
        request: &InputCombAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的组合申请请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.comb_action_insert(
                (&mut raw as *mut sys::CThostFtdcInputCombActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor(
        &self,
        request: &QryInvestor,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor(
                (&mut raw as *mut sys::CThostFtdcQryInvestorField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_user_session(
        &self,
        request: &QryUserSession,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_user_session(
                (&mut raw as *mut sys::CThostFtdcQryUserSessionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_trader_offer(
        &self,
        request: &QryTraderOffer,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_trader_offer(
                (&mut raw as *mut sys::CThostFtdcQryTraderOfferField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_transfer_bank(
        &self,
        request: &QryTransferBank,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_transfer_bank(
                (&mut raw as *mut sys::CThostFtdcQryTransferBankField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_notice(
        &self,
        request: &QryNotice,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_notice(
                (&mut raw as *mut sys::CThostFtdcQryNoticeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_position_combine_detail(
        &self,
        request: &QryInvestorPositionCombineDetail,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_position_combine_detail(
                (&mut raw as *mut sys::CThostFtdcQryInvestorPositionCombineDetailField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_cfmmc_trading_account_key(
        &self,
        request: &QryCFMMCTradingAccountKey,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_cfmmc_trading_account_key(
                (&mut raw as *mut sys::CThostFtdcQryCFMMCTradingAccountKeyField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_ewarrant_offset(
        &self,
        request: &QryEWarrantOffset,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_ewarrant_offset(
                (&mut raw as *mut sys::CThostFtdcQryEWarrantOffsetField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_exchange_rate(
        &self,
        request: &QryExchangeRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_exchange_rate(
                (&mut raw as *mut sys::CThostFtdcQryExchangeRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_product_exch_rate(
        &self,
        request: &QryProductExchRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_product_exch_rate(
                (&mut raw as *mut sys::CThostFtdcQryProductExchRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_product_group(
        &self,
        request: &QryProductGroup,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_product_group(
                (&mut raw as *mut sys::CThostFtdcQryProductGroupField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_exec_order(
        &self,
        request: &QryExecOrder,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_exec_order(
                (&mut raw as *mut sys::CThostFtdcQryExecOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_for_quote(
        &self,
        request: &QryForQuote,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_for_quote(
                (&mut raw as *mut sys::CThostFtdcQryForQuoteField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_quote(&self, request: &QryQuote, request_id: i32) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_quote(
                (&mut raw as *mut sys::CThostFtdcQryQuoteField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_option_self_close(
        &self,
        request: &QryOptionSelfClose,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_option_self_close(
                (&mut raw as *mut sys::CThostFtdcQryOptionSelfCloseField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_invest_unit(
        &self,
        request: &QryInvestUnit,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_invest_unit(
                (&mut raw as *mut sys::CThostFtdcQryInvestUnitField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_comb_instrument_guard(
        &self,
        request: &QryCombInstrumentGuard,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_comb_instrument_guard(
                (&mut raw as *mut sys::CThostFtdcQryCombInstrumentGuardField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_comb_action(
        &self,
        request: &QryCombAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_comb_action(
                (&mut raw as *mut sys::CThostFtdcQryCombActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_transfer_serial(
        &self,
        request: &QryTransferSerial,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_transfer_serial(
                (&mut raw as *mut sys::CThostFtdcQryTransferSerialField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_account_register(
        &self,
        request: &QryAccountRegister,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_account_register(
                (&mut raw as *mut sys::CThostFtdcQryAccountregisterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_contract_bank(
        &self,
        request: &QryContractBank,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_contract_bank(
                (&mut raw as *mut sys::CThostFtdcQryContractBankField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_parked_order(
        &self,
        request: &QryParkedOrder,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_parked_order(
                (&mut raw as *mut sys::CThostFtdcQryParkedOrderField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_parked_actions(
        &self,
        request: &QryParkedOrderAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_parked_actions(
                (&mut raw as *mut sys::CThostFtdcQryParkedOrderActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn trading_notices(
        &self,
        request: &QryTradingNotice,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.trading_notices(
                (&mut raw as *mut sys::CThostFtdcQryTradingNoticeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_cfmmc_trading_account_token(
        &self,
        request: &QueryCFMMCTradingAccountToken,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_cfmmc_trading_account_token(
                (&mut raw as *mut sys::CThostFtdcQueryCFMMCTradingAccountTokenField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_classified_instrument(
        &self,
        request: &QryClassifiedInstrument,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_classified_instrument(
                (&mut raw as *mut sys::CThostFtdcQryClassifiedInstrumentField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_comb_promotion_param(
        &self,
        request: &QryCombPromotionParam,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_comb_promotion_param(
                (&mut raw as *mut sys::CThostFtdcQryCombPromotionParamField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_product_group_margin(
        &self,
        request: &QryInvestorProductGroupMargin,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_product_group_margin(
                (&mut raw as *mut sys::CThostFtdcQryInvestorProductGroupMarginField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_exchange_margin_rate(
        &self,
        request: &QryExchangeMarginRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_exchange_margin_rate(
                (&mut raw as *mut sys::CThostFtdcQryExchangeMarginRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_exchange_margin_rate_adjust(
        &self,
        request: &QryExchangeMarginRateAdjust,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_exchange_margin_rate_adjust(
                (&mut raw as *mut sys::CThostFtdcQryExchangeMarginRateAdjustField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_mm_instrument_commission_rate(
        &self,
        request: &QryMMInstrumentCommissionRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_mm_instrument_commission_rate(
                (&mut raw as *mut sys::CThostFtdcQryMMInstrumentCommissionRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_mm_option_instr_comm_rate(
        &self,
        request: &QryMMOptionInstrCommRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_mm_option_instr_comm_rate(
                (&mut raw as *mut sys::CThostFtdcQryMMOptionInstrCommRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_option_instr_trade_cost(
        &self,
        request: &QryOptionInstrTradeCost,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_option_instr_trade_cost(
                (&mut raw as *mut sys::CThostFtdcQryOptionInstrTradeCostField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_option_instr_comm_rate(
        &self,
        request: &QryOptionInstrCommRate,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_option_instr_comm_rate(
                (&mut raw as *mut sys::CThostFtdcQryOptionInstrCommRateField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_risk_settle_invest_position(
        &self,
        request: &QryRiskSettleInvestPosition,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_risk_settle_invest_position(
                (&mut raw as *mut sys::CThostFtdcQryRiskSettleInvstPositionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_risk_settle_product_status(
        &self,
        request: &QryRiskSettleProductStatus,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_risk_settle_product_status(
                (&mut raw as *mut sys::CThostFtdcQryRiskSettleProductStatusField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_future_parameter(
        &self,
        request: &QrySPBMFutureParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_future_parameter(
                (&mut raw as *mut sys::CThostFtdcQrySPBMFutureParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_option_parameter(
        &self,
        request: &QrySPBMOptionParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_option_parameter(
                (&mut raw as *mut sys::CThostFtdcQrySPBMOptionParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_intra_parameter(
        &self,
        request: &QrySPBMIntraParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_intra_parameter(
                (&mut raw as *mut sys::CThostFtdcQrySPBMIntraParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_inter_parameter(
        &self,
        request: &QrySPBMInterParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_inter_parameter(
                (&mut raw as *mut sys::CThostFtdcQrySPBMInterParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_portf_definition(
        &self,
        request: &QrySPBMPortfDefinition,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_portf_definition(
                (&mut raw as *mut sys::CThostFtdcQrySPBMPortfDefinitionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_investor_portf_def(
        &self,
        request: &QrySPBMInvestorPortfDef,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_investor_portf_def(
                (&mut raw as *mut sys::CThostFtdcQrySPBMInvestorPortfDefField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_portf_margin_ratio(
        &self,
        request: &QryInvestorPortfMarginRatio,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_portf_margin_ratio(
                (&mut raw as *mut sys::CThostFtdcQryInvestorPortfMarginRatioField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_prod_spbm_detail(
        &self,
        request: &QryInvestorProdSPBMDetail,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_prod_spbm_detail(
                (&mut raw as *mut sys::CThostFtdcQryInvestorProdSPBMDetailField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_commodity_spmm_margin(
        &self,
        request: &QryInvestorCommoditySPMMMargin,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_commodity_spmm_margin(
                (&mut raw as *mut sys::CThostFtdcQryInvestorCommoditySPMMMarginField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_commodity_group_spmm_margin(
        &self,
        request: &QryInvestorCommodityGroupSPMMMargin,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_commodity_group_spmm_margin(
                (&mut raw as *mut sys::CThostFtdcQryInvestorCommodityGroupSPMMMarginField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spmm_inst_param(
        &self,
        request: &QrySPMMInstParam,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spmm_inst_param(
                (&mut raw as *mut sys::CThostFtdcQrySPMMInstParamField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spmm_product_param(
        &self,
        request: &QrySPMMProductParam,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spmm_product_param(
                (&mut raw as *mut sys::CThostFtdcQrySPMMProductParamField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spbm_add_on_inter_parameter(
        &self,
        request: &QrySPBMAddOnInterParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spbm_add_on_inter_parameter(
                (&mut raw as *mut sys::CThostFtdcQrySPBMAddOnInterParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rcams_comb_product_info(
        &self,
        request: &QryRCAMSCombProductInfo,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rcams_comb_product_info(
                (&mut raw as *mut sys::CThostFtdcQryRCAMSCombProductInfoField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rcams_instr_parameter(
        &self,
        request: &QryRCAMSInstrParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rcams_instr_parameter(
                (&mut raw as *mut sys::CThostFtdcQryRCAMSInstrParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rcams_intra_parameter(
        &self,
        request: &QryRCAMSIntraParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rcams_intra_parameter(
                (&mut raw as *mut sys::CThostFtdcQryRCAMSIntraParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rcams_inter_parameter(
        &self,
        request: &QryRCAMSInterParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rcams_inter_parameter(
                (&mut raw as *mut sys::CThostFtdcQryRCAMSInterParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rcams_short_opt_adjust_param(
        &self,
        request: &QryRCAMSShortOptAdjustParam,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rcams_short_opt_adjust_param(
                (&mut raw as *mut sys::CThostFtdcQryRCAMSShortOptAdjustParamField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rcams_investor_comb_position(
        &self,
        request: &QryRCAMSInvestorCombPosition,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rcams_investor_comb_position(
                (&mut raw as *mut sys::CThostFtdcQryRCAMSInvestorCombPositionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_prod_rcams_margin(
        &self,
        request: &QryInvestorProdRCAMSMargin,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_prod_rcams_margin(
                (&mut raw as *mut sys::CThostFtdcQryInvestorProdRCAMSMarginField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rule_instr_parameter(
        &self,
        request: &QryRULEInstrParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rule_instr_parameter(
                (&mut raw as *mut sys::CThostFtdcQryRULEInstrParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rule_intra_parameter(
        &self,
        request: &QryRULEIntraParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rule_intra_parameter(
                (&mut raw as *mut sys::CThostFtdcQryRULEIntraParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_rule_inter_parameter(
        &self,
        request: &QryRULEInterParameter,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_rule_inter_parameter(
                (&mut raw as *mut sys::CThostFtdcQryRULEInterParameterField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_prod_rule_margin(
        &self,
        request: &QryInvestorProdRULEMargin,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_prod_rule_margin(
                (&mut raw as *mut sys::CThostFtdcQryInvestorProdRULEMarginField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_portf_setting(
        &self,
        request: &QryInvestorPortfSetting,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_portf_setting(
                (&mut raw as *mut sys::CThostFtdcQryInvestorPortfSettingField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn bank_to_future(
        &self,
        request: &ReqTransfer,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.bank_to_future(
                (&mut raw as *mut sys::CThostFtdcReqTransferField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn future_to_bank(
        &self,
        request: &ReqTransfer,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.future_to_bank(
                (&mut raw as *mut sys::CThostFtdcReqTransferField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_bank_balance(
        &self,
        request: &ReqQueryAccount,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_bank_balance(
                (&mut raw as *mut sys::CThostFtdcReqQueryAccountField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_sec_agent_acid_map(
        &self,
        request: &QrySecAgentACIDMap,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_sec_agent_acid_map(
                (&mut raw as *mut sys::CThostFtdcQrySecAgentACIDMapField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_sec_agent_trading_account(
        &self,
        request: &QryTradingAccount,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_sec_agent_trading_account(
                (&mut raw as *mut sys::CThostFtdcQryTradingAccountField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_sec_agent_check_mode(
        &self,
        request: &QrySecAgentCheckMode,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_sec_agent_check_mode(
                (&mut raw as *mut sys::CThostFtdcQrySecAgentCheckModeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_sec_agent_trade_info(
        &self,
        request: &QrySecAgentTradeInfo,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_sec_agent_trade_info(
                (&mut raw as *mut sys::CThostFtdcQrySecAgentTradeInfoField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_broker_trading_params(
        &self,
        request: &QryBrokerTradingParams,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_broker_trading_params(
                (&mut raw as *mut sys::CThostFtdcQryBrokerTradingParamsField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_broker_trading_algos(
        &self,
        request: &QryBrokerTradingAlgos,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_broker_trading_algos(
                (&mut raw as *mut sys::CThostFtdcQryBrokerTradingAlgosField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn set_offset(
        &self,
        request: &InputOffsetSetting,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.set_offset(
                (&mut raw as *mut sys::CThostFtdcInputOffsetSettingField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn cancel_offset_setting(
        &self,
        request: &InputOffsetSetting,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.cancel_offset_setting(
                (&mut raw as *mut sys::CThostFtdcInputOffsetSettingField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn offset_settings(
        &self,
        request: &QryOffsetSetting,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.offset_settings(
                (&mut raw as *mut sys::CThostFtdcQryOffsetSettingField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn gen_sms_code(
        &self,
        request: &ReqGenSMSCode,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.gen_sms_code(
                (&mut raw as *mut sys::CThostFtdcReqGenSMSCodeField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn hedge_cfm(
        &self,
        request: &InputHedgeCfm,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.hedge_cfm(
                (&mut raw as *mut sys::CThostFtdcInputHedgeCfmField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn hedge_cfm_action(
        &self,
        request: &InputHedgeCfmAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.hedge_cfm_action(
                (&mut raw as *mut sys::CThostFtdcInputHedgeCfmActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_hedge_cfm(
        &self,
        request: &QryHedgeCfm,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_hedge_cfm(
                (&mut raw as *mut sys::CThostFtdcQryHedgeCfmField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_spd_apply(
        &self,
        request: &QrySpdApply,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_spd_apply(
                (&mut raw as *mut sys::CThostFtdcQrySpdApplyField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn spd_apply(
        &self,
        request: &InputSpdApply,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.spd_apply(
                (&mut raw as *mut sys::CThostFtdcInputSpdApplyField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn spd_apply_action(
        &self,
        request: &InputSpdApplyAction,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.spd_apply_action(
                (&mut raw as *mut sys::CThostFtdcInputSpdApplyActionField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_investor_info_comm_rec(
        &self,
        request: &QryInvestorInfoCommRec,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_investor_info_comm_rec(
                (&mut raw as *mut sys::CThostFtdcQryInvestorInfoCommRecField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_comb_leg(
        &self,
        request: &QryCombLeg,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_comb_leg(
                (&mut raw as *mut sys::CThostFtdcQryCombLegField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_instr_trade_limit(
        &self,
        request: &QryInstrTradeLimit,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_instr_trade_limit(
                (&mut raw as *mut sys::CThostFtdcQryInstrTradeLimitField).cast(),
                request_id,
            )
        };
        Ok(result)
    }

    pub fn query_max_order_volume_with_price(
        &self,
        request: &QryMaxOrderVolumeWithPrice,
        request_id: i32,
    ) -> Result<i32, ConversionError> {
        let mut raw = request.to_sys()?;
        // SAFETY: raw 是完整初始化的查询请求，指针仅在同步请求调用期间有效。
        let result = unsafe {
            self.inner.query_max_order_volume_with_price(
                (&mut raw as *mut sys::CThostFtdcQryMaxOrderVolumeWithPriceField).cast(),
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
