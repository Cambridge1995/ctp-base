// 由 ctp-generator 根据审核表生成，用于类型与原生桥接。
// 请勿手工编辑；正式映射以当前审核表为准。

use core::fmt;

/// CTP 类型：`TThostFtdcAMLCheckStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AMLCheckStatus {
    /// 未复核。CTP：`THOST_FTDC_AMLCHS_Init = '0'`。
    Init,
    /// 复核中。CTP：`THOST_FTDC_AMLCHS_Checking = '1'`。
    Checking,
    /// 已复核。CTP：`THOST_FTDC_AMLCHS_Checked = '2'`。
    Checked,
    /// 拒绝上报。CTP：`THOST_FTDC_AMLCHS_RefuseReport = '3'`。
    RefuseReport,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AMLCheckStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Init,
            49 => Self::Checking,
            50 => Self::Checked,
            51 => Self::RefuseReport,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Init => Some(48),
            Self::Checking => Some(49),
            Self::Checked => Some(50),
            Self::RefuseReport => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AMLCheckStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAMLGenStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AMLGenStatus {
    /// 程序生成。CTP：`THOST_FTDC_GEN_Program = '0'`。
    Program,
    /// 人工生成。CTP：`THOST_FTDC_GEN_HandWork = '1'`。
    HandWork,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AMLGenStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Program,
            49 => Self::HandWork,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Program => Some(48),
            Self::HandWork => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AMLGenStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAPIProductClassType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum APIProductClass {
    /// 期货单一合约。CTP：`THOST_FTDC_APC_FutureSingle = '1'`。
    FutureSingle,
    /// 期权单一合约。CTP：`THOST_FTDC_APC_OptionSingle = '2'`。
    OptionSingle,
    /// 可交易期货(含期货组合和期货单一合约)。CTP：`THOST_FTDC_APC_Futures = '3'`。
    Futures,
    /// 可交易期权(含期权组合和期权单一合约)。CTP：`THOST_FTDC_APC_Options = '4'`。
    Options,
    /// 可下单套利组合。CTP：`THOST_FTDC_APC_TradingComb = '5'`。
    TradingComb,
    /// 可申请的组合（可以申请的组合合约 包含可以交易的合约）。CTP：`THOST_FTDC_APC_UnTradingComb = '6'`。
    UnTradingComb,
    /// 所有可以交易合约。CTP：`THOST_FTDC_APC_AllTrading = '7'`。
    AllTrading,
    /// 所有合约（包含不能交易合约 慎用）。CTP：`THOST_FTDC_APC_All = '8'`。
    All,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl APIProductClass {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::FutureSingle,
            50 => Self::OptionSingle,
            51 => Self::Futures,
            52 => Self::Options,
            53 => Self::TradingComb,
            54 => Self::UnTradingComb,
            55 => Self::AllTrading,
            56 => Self::All,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FutureSingle => Some(49),
            Self::OptionSingle => Some(50),
            Self::Futures => Some(51),
            Self::Options => Some(52),
            Self::TradingComb => Some(53),
            Self::UnTradingComb => Some(54),
            Self::AllTrading => Some(55),
            Self::All => Some(56),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for APIProductClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAccountSettlementParamIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountSettlementParamID {
    /// 基础保证金。CTP：`THOST_FTDC_ASPI_BaseMargin = '1'`。
    BaseMargin,
    /// 最低权益标准。CTP：`THOST_FTDC_ASPI_LowestInterest = '2'`。
    LowestInterest,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AccountSettlementParamID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::BaseMargin,
            50 => Self::LowestInterest,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BaseMargin => Some(49),
            Self::LowestInterest => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AccountSettlementParamID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAccountSourceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountSourceType {
    /// 银期同步。CTP：`THOST_FTDC_AST_FBTransfer = '0'`。
    FBTransfer,
    /// 手工录入。CTP：`THOST_FTDC_AST_ManualEntry = '1'`。
    ManualEntry,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AccountSourceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::FBTransfer,
            49 => Self::ManualEntry,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FBTransfer => Some(48),
            Self::ManualEntry => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AccountSourceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcActionDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionDirection {
    /// 增加。CTP：`THOST_FTDC_ACD_Add = '1'`。
    Add,
    /// 删除。CTP：`THOST_FTDC_ACD_Del = '2'`。
    Del,
    /// 更新。CTP：`THOST_FTDC_ACD_Upd = '3'`。
    Upd,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ActionDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Add,
            50 => Self::Del,
            51 => Self::Upd,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Add => Some(49),
            Self::Del => Some(50),
            Self::Upd => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ActionDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcActionFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionFlag {
    /// 删除。CTP：`THOST_FTDC_AF_Delete = '0'`。
    Delete,
    /// 修改。CTP：`THOST_FTDC_AF_Modify = '3'`。
    Modify,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ActionFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Delete,
            51 => Self::Modify,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Delete => Some(48),
            Self::Modify => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ActionFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcActionTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// 执行。CTP：`THOST_FTDC_ACTP_Exec = '1'`。
    Exec,
    /// 放弃。CTP：`THOST_FTDC_ACTP_Abandon = '2'`。
    Abandon,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ActionType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Exec,
            50 => Self::Abandon,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Exec => Some(49),
            Self::Abandon => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcActiveTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveType {
    /// 仅当日生效。CTP：`THOST_FTDC_ACT_Intraday = '1'`。
    Intraday,
    /// 长期生效。CTP：`THOST_FTDC_ACT_Long = '2'`。
    Long,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ActiveType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Intraday,
            50 => Self::Long,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Intraday => Some(49),
            Self::Long => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ActiveType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAddrSrvModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddrSrvMode {
    /// 交易地址。CTP：`THOST_FTDC_ASM_Trade = '0'`。
    Trade,
    /// 行情地址。CTP：`THOST_FTDC_ASM_MarketData = '1'`。
    MarketData,
    /// 其他。CTP：`THOST_FTDC_ASM_Other = '2'`。
    Other,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AddrSrvMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Trade,
            49 => Self::MarketData,
            50 => Self::Other,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Trade => Some(48),
            Self::MarketData => Some(49),
            Self::Other => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AddrSrvMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAddrVerType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddrVer {
    /// IPV4。CTP：`THOST_FTDC_ADV_V4 = '0'`。
    V4,
    /// IPV6。CTP：`THOST_FTDC_ADV_V6 = '1'`。
    V6,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AddrVer {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::V4,
            49 => Self::V6,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::V4 => Some(48),
            Self::V6 => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AddrVer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAlgoTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgoType {
    /// 持仓处理算法。CTP：`THOST_FTDC_AT_HandlePositionAlgo = '1'`。
    HandlePositionAlgo,
    /// 寻找保证金率算法。CTP：`THOST_FTDC_AT_FindMarginRateAlgo = '2'`。
    FindMarginRateAlgo,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AlgoType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::HandlePositionAlgo,
            50 => Self::FindMarginRateAlgo,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::HandlePositionAlgo => Some(49),
            Self::FindMarginRateAlgo => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AlgoType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAlgorithmType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    /// 浮盈浮亏都计算。CTP：`THOST_FTDC_AG_All = '1'`。
    All,
    /// 浮盈不计，浮亏计。CTP：`THOST_FTDC_AG_OnlyLost = '2'`。
    OnlyLost,
    /// 浮盈计，浮亏不计。CTP：`THOST_FTDC_AG_OnlyGain = '3'`。
    OnlyGain,
    /// 浮盈浮亏都不计算。CTP：`THOST_FTDC_AG_None = '4'`。
    None,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl Algorithm {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::OnlyLost,
            51 => Self::OnlyGain,
            52 => Self::None,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::OnlyLost => Some(50),
            Self::OnlyGain => Some(51),
            Self::None => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAllWithoutTradeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllWithoutTrade {
    /// 无仓无成交不受可提比例限制。CTP：`THOST_FTDC_AWT_Enable = '0'`。
    Enable,
    /// 受可提比例限制。CTP：`THOST_FTDC_AWT_Disable = '2'`。
    Disable,
    /// 无仓不受可提比例限制。CTP：`THOST_FTDC_AWT_NoHoldEnable = '3'`。
    NoHoldEnable,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AllWithoutTrade {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Enable,
            50 => Self::Disable,
            51 => Self::NoHoldEnable,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Enable => Some(48),
            Self::Disable => Some(50),
            Self::NoHoldEnable => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AllWithoutTrade {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAmTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmType {
    /// 银行。CTP：`THOST_FTDC_AMT_Bank = '1'`。
    Bank,
    /// 证券公司。CTP：`THOST_FTDC_AMT_Securities = '2'`。
    Securities,
    /// 基金公司。CTP：`THOST_FTDC_AMT_Fund = '3'`。
    Fund,
    /// 保险公司。CTP：`THOST_FTDC_AMT_Insurance = '4'`。
    Insurance,
    /// 信托公司。CTP：`THOST_FTDC_AMT_Trust = '5'`。
    Trust,
    /// 其他。CTP：`THOST_FTDC_AMT_Other = '9'`。
    Other,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AmType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Bank,
            50 => Self::Securities,
            51 => Self::Fund,
            52 => Self::Insurance,
            53 => Self::Trust,
            57 => Self::Other,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Bank => Some(49),
            Self::Securities => Some(50),
            Self::Fund => Some(51),
            Self::Insurance => Some(52),
            Self::Trust => Some(53),
            Self::Other => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AmType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAmlCheckLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmlCheckLevel {
    /// 零级审核。CTP：`THOST_FTDC_AMLCL_CheckLevel0 = '0'`。
    CheckLevel0,
    /// 一级审核。CTP：`THOST_FTDC_AMLCL_CheckLevel1 = '1'`。
    CheckLevel1,
    /// 二级审核。CTP：`THOST_FTDC_AMLCL_CheckLevel2 = '2'`。
    CheckLevel2,
    /// 三级审核。CTP：`THOST_FTDC_AMLCL_CheckLevel3 = '3'`。
    CheckLevel3,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AmlCheckLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::CheckLevel0,
            49 => Self::CheckLevel1,
            50 => Self::CheckLevel2,
            51 => Self::CheckLevel3,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CheckLevel0 => Some(48),
            Self::CheckLevel1 => Some(49),
            Self::CheckLevel2 => Some(50),
            Self::CheckLevel3 => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AmlCheckLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAmlDateTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmlDateType {
    /// 检查日期。CTP：`THOST_FTDC_AMLDT_DrawDay = '0'`。
    DrawDay,
    /// 发生日期。CTP：`THOST_FTDC_AMLDT_TouchDay = '1'`。
    TouchDay,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AmlDateType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::DrawDay,
            49 => Self::TouchDay,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::DrawDay => Some(48),
            Self::TouchDay => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AmlDateType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAppTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppType {
    /// 直连的投资者。CTP：`THOST_FTDC_APP_TYPE_Investor = '1'`。
    Investor,
    /// 为每个投资者都创建连接的中继。CTP：`THOST_FTDC_APP_TYPE_InvestorRelay = '2'`。
    InvestorRelay,
    /// 所有投资者共享一个操作员连接的中继。CTP：`THOST_FTDC_APP_TYPE_OperatorRelay = '3'`。
    OperatorRelay,
    /// 未知。CTP：`THOST_FTDC_APP_TYPE_UnKnown = '4'`。
    UnKnown,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AppType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Investor,
            50 => Self::InvestorRelay,
            51 => Self::OperatorRelay,
            52 => Self::UnKnown,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Investor => Some(49),
            Self::InvestorRelay => Some(50),
            Self::OperatorRelay => Some(51),
            Self::UnKnown => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AppType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcApplyOperateIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplyOperateID {
    /// 开户。CTP：`THOST_FTDC_AOID_OpenInvestor = '1'`。
    OpenInvestor,
    /// 修改身份信息。CTP：`THOST_FTDC_AOID_ModifyIDCard = '2'`。
    ModifyIDCard,
    /// 修改一般信息。CTP：`THOST_FTDC_AOID_ModifyNoIDCard = '3'`。
    ModifyNoIDCard,
    /// 申请交易编码。CTP：`THOST_FTDC_AOID_ApplyTradingCode = '4'`。
    ApplyTradingCode,
    /// 撤销交易编码。CTP：`THOST_FTDC_AOID_CancelTradingCode = '5'`。
    CancelTradingCode,
    /// 销户。CTP：`THOST_FTDC_AOID_CancelInvestor = '6'`。
    CancelInvestor,
    /// 账户休眠。CTP：`THOST_FTDC_AOID_FreezeAccount = '8'`。
    FreezeAccount,
    /// 激活休眠账户。CTP：`THOST_FTDC_AOID_ActiveFreezeAccount = '9'`。
    ActiveFreezeAccount,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ApplyOperateID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::OpenInvestor,
            50 => Self::ModifyIDCard,
            51 => Self::ModifyNoIDCard,
            52 => Self::ApplyTradingCode,
            53 => Self::CancelTradingCode,
            54 => Self::CancelInvestor,
            56 => Self::FreezeAccount,
            57 => Self::ActiveFreezeAccount,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::OpenInvestor => Some(49),
            Self::ModifyIDCard => Some(50),
            Self::ModifyNoIDCard => Some(51),
            Self::ApplyTradingCode => Some(52),
            Self::CancelTradingCode => Some(53),
            Self::CancelInvestor => Some(54),
            Self::FreezeAccount => Some(56),
            Self::ActiveFreezeAccount => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ApplyOperateID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcApplySrcType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplySrc {
    /// 交易。CTP：`THOST_FTDC_AS_Trade = '0'`。
    Trade,
    /// 会服。CTP：`THOST_FTDC_AS_Member = '1'`。
    Member,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ApplySrc {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Trade,
            49 => Self::Member,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Trade => Some(48),
            Self::Member => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ApplySrc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcApplyStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplyStatus {
    /// 未知。CTP：`THOST_FTDC_AS2_Unknown = 'a'`。
    Unknown,
    /// 已撤销。CTP：`THOST_FTDC_AS2_Canceled = '0'`。
    Canceled,
    /// 已挂起。CTP：`THOST_FTDC_AS2_Suspended = '1'`。
    Suspended,
    /// 申请成功。CTP：`THOST_FTDC_AS2_Accepted = '3'`。
    Accepted,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ApplyStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            97 => Self::Unknown,
            48 => Self::Canceled,
            49 => Self::Suspended,
            51 => Self::Accepted,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Unknown => Some(97),
            Self::Canceled => Some(48),
            Self::Suspended => Some(49),
            Self::Accepted => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ApplyStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcApplyStatusIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplyStatusID {
    /// 未补全。CTP：`THOST_FTDC_ASID_NoComplete = '1'`。
    NoComplete,
    /// 已提交。线值：`'2'`；名称已规范化，原始常量见审核表。
    Submitted,
    /// 已审核。CTP：`THOST_FTDC_ASID_Checked = '3'`。
    Checked,
    /// 已拒绝。CTP：`THOST_FTDC_ASID_Refused = '4'`。
    Refused,
    /// 已删除。CTP：`THOST_FTDC_ASID_Deleted = '5'`。
    Deleted,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ApplyStatusID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NoComplete,
            50 => Self::Submitted,
            51 => Self::Checked,
            52 => Self::Refused,
            53 => Self::Deleted,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoComplete => Some(49),
            Self::Submitted => Some(50),
            Self::Checked => Some(51),
            Self::Refused => Some(52),
            Self::Deleted => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ApplyStatusID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcApplyTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplyType {
    /// 不执行数量。CTP：`THOST_FTDC_APPT_NotStrikeNum = '4'`。
    NotStrikeNum,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ApplyType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            52 => Self::NotStrikeNum,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotStrikeNum => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ApplyType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetManagerClientType {
    /// 个人资管客户。CTP：`THOST_FTDC_AMCT_Person = '1'`。
    Person,
    /// 单位资管客户。CTP：`THOST_FTDC_AMCT_Organ = '2'`。
    Organ,
    /// 特殊单位资管客户。CTP：`THOST_FTDC_AMCT_SpecialOrgan = '4'`。
    SpecialOrgan,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AssetManagerClientType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Person,
            50 => Self::Organ,
            52 => Self::SpecialOrgan,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Person => Some(49),
            Self::Organ => Some(50),
            Self::SpecialOrgan => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AssetManagerClientType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetManagerType {
    /// 期货类。CTP：`THOST_FTDC_ASST_Futures = '3'`。
    Futures,
    /// 综合类。CTP：`THOST_FTDC_ASST_SpecialOrgan = '4'`。
    SpecialOrgan,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AssetManagerType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            51 => Self::Futures,
            52 => Self::SpecialOrgan,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Futures => Some(51),
            Self::SpecialOrgan => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AssetManagerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAuthTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthType {
    /// 白名单校验。CTP：`THOST_FTDC_AU_WHITE = '0'`。
    WHITE,
    /// 黑名单校验。CTP：`THOST_FTDC_AU_BLACK = '1'`。
    BLACK,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AuthType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::WHITE,
            49 => Self::BLACK,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::WHITE => Some(48),
            Self::BLACK => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AuthType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcAvailabilityFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvailabilityFlag {
    /// 未确认。CTP：`THOST_FTDC_AVAF_Invalid = '0'`。
    Invalid,
    /// 有效。CTP：`THOST_FTDC_AVAF_Valid = '1'`。
    Valid,
    /// 冲正。CTP：`THOST_FTDC_AVAF_Repeal = '2'`。
    Repeal,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl AvailabilityFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Invalid,
            49 => Self::Valid,
            50 => Self::Repeal,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Invalid => Some(48),
            Self::Valid => Some(49),
            Self::Repeal => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for AvailabilityFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBackUpStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackUpStatus {
    /// 未生成备份数据。CTP：`THOST_FTDC_BUS_UnBak = '0'`。
    UnBak,
    /// 备份数据生成中。CTP：`THOST_FTDC_BUS_BakUp = '1'`。
    BakUp,
    /// 已生成备份数据。线值：`'2'`；名称已规范化，原始常量见审核表。
    BackedUp,
    /// 备份数据失败。CTP：`THOST_FTDC_BUS_BakFail = '3'`。
    BakFail,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BackUpStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnBak,
            49 => Self::BakUp,
            50 => Self::BackedUp,
            51 => Self::BakFail,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnBak => Some(48),
            Self::BakUp => Some(49),
            Self::BackedUp => Some(50),
            Self::BakFail => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BackUpStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBalanceAlgorithmType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BalanceAlgorithm {
    /// 不计算期权市值盈亏。CTP：`THOST_FTDC_BLAG_Default = '1'`。
    Default,
    /// 计算期权市值亏损。CTP：`THOST_FTDC_BLAG_IncludeOptValLost = '2'`。
    IncludeOptValLost,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BalanceAlgorithm {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Default,
            50 => Self::IncludeOptValLost,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Default => Some(49),
            Self::IncludeOptValLost => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BalanceAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BalanceType {
    /// 当前余额。CTP：`THOST_FTDC_BLT_CurrentMoney = '0'`。
    CurrentMoney,
    /// 可用余额。CTP：`THOST_FTDC_BLT_UsableMoney = '1'`。
    UsableMoney,
    /// 可取余额。CTP：`THOST_FTDC_BLT_FetchableMoney = '2'`。
    FetchableMoney,
    /// 冻结余额。CTP：`THOST_FTDC_BLT_FreezeMoney = '3'`。
    FreezeMoney,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BalanceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::CurrentMoney,
            49 => Self::UsableMoney,
            50 => Self::FetchableMoney,
            51 => Self::FreezeMoney,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CurrentMoney => Some(48),
            Self::UsableMoney => Some(49),
            Self::FetchableMoney => Some(50),
            Self::FreezeMoney => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BalanceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBankAccStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BankAccStatus {
    /// 正常。CTP：`THOST_FTDC_BAS_Normal = '0'`。
    Normal,
    /// 冻结。CTP：`THOST_FTDC_BAS_Freeze = '1'`。
    Freeze,
    /// 挂失。CTP：`THOST_FTDC_BAS_ReportLoss = '2'`。
    ReportLoss,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BankAccStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::Freeze,
            50 => Self::ReportLoss,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::Freeze => Some(49),
            Self::ReportLoss => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BankAccStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBankAccTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BankAccType {
    /// 银行存折。CTP：`THOST_FTDC_BAT_BankBook = '1'`。
    BankBook,
    /// 储蓄卡。CTP：`THOST_FTDC_BAT_SavingCard = '2'`。
    SavingCard,
    /// 信用卡。CTP：`THOST_FTDC_BAT_CreditCard = '3'`。
    CreditCard,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BankAccType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::BankBook,
            50 => Self::SavingCard,
            51 => Self::CreditCard,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BankBook => Some(49),
            Self::SavingCard => Some(50),
            Self::CreditCard => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BankAccType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BankAccountOrigin {
    /// 手工录入。CTP：`THOST_FTDC_BAO_ByAccProperty = '0'`。
    ByAccProperty,
    /// 银期转账。CTP：`THOST_FTDC_BAO_ByFBTransfer = '1'`。
    ByFBTransfer,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BankAccountOrigin {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ByAccProperty,
            49 => Self::ByFBTransfer,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByAccProperty => Some(48),
            Self::ByFBTransfer => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BankAccountOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBankRepealFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BankRepealFlag {
    /// 银行无需自动冲正。CTP：`THOST_FTDC_BRF_BankNotNeedRepeal = '0'`。
    BankNotNeedRepeal,
    /// 银行待自动冲正。CTP：`THOST_FTDC_BRF_BankWaitingRepeal = '1'`。
    BankWaitingRepeal,
    /// 银行已自动冲正。CTP：`THOST_FTDC_BRF_BankBeenRepealed = '2'`。
    BankBeenRepealed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BankRepealFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::BankNotNeedRepeal,
            49 => Self::BankWaitingRepeal,
            50 => Self::BankBeenRepealed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BankNotNeedRepeal => Some(48),
            Self::BankWaitingRepeal => Some(49),
            Self::BankBeenRepealed => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BankRepealFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBasisPriceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasisPriceType {
    /// 上一合约结算价。CTP：`THOST_FTDC_IPT_LastSettlement = '1'`。
    LastSettlement,
    /// 上一合约收盘价。线值：`'2'`；名称已规范化，原始常量见审核表。
    LastClose,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BasisPriceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::LastSettlement,
            50 => Self::LastClose,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::LastSettlement => Some(49),
            Self::LastClose => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BasisPriceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBatchStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BatchStatus {
    /// 未上传。CTP：`THOST_FTDC_BS_NoUpload = '1'`。
    NoUpload,
    /// 已上传。CTP：`THOST_FTDC_BS_Uploaded = '2'`。
    Uploaded,
    /// 审核失败。CTP：`THOST_FTDC_BS_Failed = '3'`。
    Failed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BatchStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NoUpload,
            50 => Self::Uploaded,
            51 => Self::Failed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoUpload => Some(49),
            Self::Uploaded => Some(50),
            Self::Failed => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BatchStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBillGenStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BillGenStatus {
    /// 未生成。CTP：`THOST_FTDC_BGS_None = '0'`。
    None,
    /// 生成中。CTP：`THOST_FTDC_BGS_NoGenerated = '1'`。
    NoGenerated,
    /// 已生成。CTP：`THOST_FTDC_BGS_Generated = '2'`。
    Generated,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BillGenStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::NoGenerated,
            50 => Self::Generated,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::NoGenerated => Some(49),
            Self::Generated => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BillGenStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBillHedgeFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BillHedgeFlag {
    /// 投机。CTP：`THOST_FTDC_BHF_Speculation = '1'`。
    Speculation,
    /// 套利。CTP：`THOST_FTDC_BHF_Arbitrage = '2'`。
    Arbitrage,
    /// 套保。CTP：`THOST_FTDC_BHF_Hedge = '3'`。
    Hedge,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BillHedgeFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Speculation,
            50 => Self::Arbitrage,
            51 => Self::Hedge,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Speculation => Some(49),
            Self::Arbitrage => Some(50),
            Self::Hedge => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BillHedgeFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBizTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BizType {
    /// 期货。CTP：`THOST_FTDC_BZTP_Future = '1'`。
    Future,
    /// 证券。CTP：`THOST_FTDC_BZTP_Stock = '2'`。
    Stock,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BizType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Future,
            50 => Self::Stock,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Future => Some(49),
            Self::Stock => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BizType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBrokerDataSyncStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrokerDataSyncStatus {
    /// 已同步。CTP：`THOST_FTDC_BDS_Synchronized = '1'`。
    Synchronized,
    /// 同步中。CTP：`THOST_FTDC_BDS_Synchronizing = '2'`。
    Synchronizing,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BrokerDataSyncStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Synchronized,
            50 => Self::Synchronizing,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Synchronized => Some(49),
            Self::Synchronizing => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BrokerDataSyncStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBrokerFunctionCodeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrokerFunctionCode {
    /// 强制用户登出。CTP：`THOST_FTDC_BFC_ForceUserLogout = '1'`。
    ForceUserLogout,
    /// 变更用户口令。CTP：`THOST_FTDC_BFC_UserPasswordUpdate = '2'`。
    UserPasswordUpdate,
    /// 同步经纪公司数据。CTP：`THOST_FTDC_BFC_SyncBrokerData = '3'`。
    SyncBrokerData,
    /// 批量同步经纪公司数据。线值：`'4'`；名称已规范化，原始常量见审核表。
    BatchSyncBrokerData,
    /// 报单插入。CTP：`THOST_FTDC_BFC_OrderInsert = '5'`。
    OrderInsert,
    /// 报单操作。CTP：`THOST_FTDC_BFC_OrderAction = '6'`。
    OrderAction,
    /// 全部查询。CTP：`THOST_FTDC_BFC_AllQuery = '7'`。
    AllQuery,
    /// 系统功能：登入/登出/修改密码等。线值：`'a'`；名称已规范化，原始常量见审核表。
    Log,
    /// 基本查询：查询基础数据，如合约，交易所等常量。CTP：`THOST_FTDC_BFC_BaseQry = 'b'`。
    BaseQry,
    /// 交易查询：如查成交，委托。CTP：`THOST_FTDC_BFC_TradeQry = 'c'`。
    TradeQry,
    /// 交易功能：报单，撤单。CTP：`THOST_FTDC_BFC_Trade = 'd'`。
    Trade,
    /// 银期转账。CTP：`THOST_FTDC_BFC_Virement = 'e'`。
    Virement,
    /// 风险监控。CTP：`THOST_FTDC_BFC_Risk = 'f'`。
    Risk,
    /// 查询/管理：查询会话，踢人等。CTP：`THOST_FTDC_BFC_Session = 'g'`。
    Session,
    /// 风控通知控制。CTP：`THOST_FTDC_BFC_RiskNoticeCtl = 'h'`。
    RiskNoticeCtl,
    /// 风控通知发送。CTP：`THOST_FTDC_BFC_RiskNotice = 'i'`。
    RiskNotice,
    /// 察看经纪公司资金权限。CTP：`THOST_FTDC_BFC_BrokerDeposit = 'j'`。
    BrokerDeposit,
    /// 资金查询。CTP：`THOST_FTDC_BFC_QueryFund = 'k'`。
    QueryFund,
    /// 报单查询。CTP：`THOST_FTDC_BFC_QueryOrder = 'l'`。
    QueryOrder,
    /// 成交查询。CTP：`THOST_FTDC_BFC_QueryTrade = 'm'`。
    QueryTrade,
    /// 持仓查询。CTP：`THOST_FTDC_BFC_QueryPosition = 'n'`。
    QueryPosition,
    /// 行情查询。CTP：`THOST_FTDC_BFC_QueryMarketData = 'o'`。
    QueryMarketData,
    /// 用户事件查询。CTP：`THOST_FTDC_BFC_QueryUserEvent = 'p'`。
    QueryUserEvent,
    /// 风险通知查询。CTP：`THOST_FTDC_BFC_QueryRiskNotify = 'q'`。
    QueryRiskNotify,
    /// 出入金查询。CTP：`THOST_FTDC_BFC_QueryFundChange = 'r'`。
    QueryFundChange,
    /// 投资者信息查询。CTP：`THOST_FTDC_BFC_QueryInvestor = 's'`。
    QueryInvestor,
    /// 交易编码查询。CTP：`THOST_FTDC_BFC_QueryTradingCode = 't'`。
    QueryTradingCode,
    /// 强平。CTP：`THOST_FTDC_BFC_ForceClose = 'u'`。
    ForceClose,
    /// 压力测试。CTP：`THOST_FTDC_BFC_PressTest = 'v'`。
    PressTest,
    /// 权益反算。CTP：`THOST_FTDC_BFC_RemainCalc = 'w'`。
    RemainCalc,
    /// 净持仓保证金指标。CTP：`THOST_FTDC_BFC_NetPositionInd = 'x'`。
    NetPositionInd,
    /// 风险预算。CTP：`THOST_FTDC_BFC_RiskPredict = 'y'`。
    RiskPredict,
    /// 数据导出。CTP：`THOST_FTDC_BFC_DataExport = 'z'`。
    DataExport,
    /// 风控指标设置。CTP：`THOST_FTDC_BFC_RiskTargetSetup = 'A'`。
    RiskTargetSetup,
    /// 行情预警。CTP：`THOST_FTDC_BFC_MarketDataWarn = 'B'`。
    MarketDataWarn,
    /// 业务通知查询。CTP：`THOST_FTDC_BFC_QryBizNotice = 'C'`。
    QryBizNotice,
    /// 业务通知模板设置。CTP：`THOST_FTDC_BFC_CfgBizNotice = 'D'`。
    CfgBizNotice,
    /// 同步动态令牌。CTP：`THOST_FTDC_BFC_SyncOTP = 'E'`。
    SyncOTP,
    /// 发送业务通知。CTP：`THOST_FTDC_BFC_SendBizNotice = 'F'`。
    SendBizNotice,
    /// 风险级别标准设置。CTP：`THOST_FTDC_BFC_CfgRiskLevelStd = 'G'`。
    CfgRiskLevelStd,
    /// 交易终端应急功能。CTP：`THOST_FTDC_BFC_TbCommand = 'H'`。
    TbCommand,
    /// 删除未知单。CTP：`THOST_FTDC_BFC_DeleteOrder = 'J'`。
    DeleteOrder,
    /// 预埋报单插入。CTP：`THOST_FTDC_BFC_ParkedOrderInsert = 'K'`。
    ParkedOrderInsert,
    /// 预埋报单操作。CTP：`THOST_FTDC_BFC_ParkedOrderAction = 'L'`。
    ParkedOrderAction,
    /// 资金不够仍允许行权。CTP：`THOST_FTDC_BFC_ExecOrderNoCheck = 'M'`。
    ExecOrderNoCheck,
    /// 指定。CTP：`THOST_FTDC_BFC_Designate = 'N'`。
    Designate,
    /// 证券处置。CTP：`THOST_FTDC_BFC_StockDisposal = 'O'`。
    StockDisposal,
    /// 席位资金预警。CTP：`THOST_FTDC_BFC_BrokerDepositWarn = 'Q'`。
    BrokerDepositWarn,
    /// 备兑不足预警。CTP：`THOST_FTDC_BFC_CoverWarn = 'S'`。
    CoverWarn,
    /// 行权试算。CTP：`THOST_FTDC_BFC_PreExecOrder = 'T'`。
    PreExecOrder,
    /// 行权交收风险。CTP：`THOST_FTDC_BFC_ExecOrderRisk = 'P'`。
    ExecOrderRisk,
    /// 持仓限额预警。CTP：`THOST_FTDC_BFC_PosiLimitWarn = 'U'`。
    PosiLimitWarn,
    /// 持仓限额查询。CTP：`THOST_FTDC_BFC_QryPosiLimit = 'V'`。
    QryPosiLimit,
    /// 银期签到签退。CTP：`THOST_FTDC_BFC_FBSign = 'W'`。
    FBSign,
    /// 银期签约解约。CTP：`THOST_FTDC_BFC_FBAccount = 'X'`。
    FBAccount,
    /// 接收短信验证码。CTP：`THOST_FTDC_BFC_RcvSMSCode = 'Y'`。
    RcvSMSCode,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BrokerFunctionCode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ForceUserLogout,
            50 => Self::UserPasswordUpdate,
            51 => Self::SyncBrokerData,
            52 => Self::BatchSyncBrokerData,
            53 => Self::OrderInsert,
            54 => Self::OrderAction,
            55 => Self::AllQuery,
            97 => Self::Log,
            98 => Self::BaseQry,
            99 => Self::TradeQry,
            100 => Self::Trade,
            101 => Self::Virement,
            102 => Self::Risk,
            103 => Self::Session,
            104 => Self::RiskNoticeCtl,
            105 => Self::RiskNotice,
            106 => Self::BrokerDeposit,
            107 => Self::QueryFund,
            108 => Self::QueryOrder,
            109 => Self::QueryTrade,
            110 => Self::QueryPosition,
            111 => Self::QueryMarketData,
            112 => Self::QueryUserEvent,
            113 => Self::QueryRiskNotify,
            114 => Self::QueryFundChange,
            115 => Self::QueryInvestor,
            116 => Self::QueryTradingCode,
            117 => Self::ForceClose,
            118 => Self::PressTest,
            119 => Self::RemainCalc,
            120 => Self::NetPositionInd,
            121 => Self::RiskPredict,
            122 => Self::DataExport,
            65 => Self::RiskTargetSetup,
            66 => Self::MarketDataWarn,
            67 => Self::QryBizNotice,
            68 => Self::CfgBizNotice,
            69 => Self::SyncOTP,
            70 => Self::SendBizNotice,
            71 => Self::CfgRiskLevelStd,
            72 => Self::TbCommand,
            74 => Self::DeleteOrder,
            75 => Self::ParkedOrderInsert,
            76 => Self::ParkedOrderAction,
            77 => Self::ExecOrderNoCheck,
            78 => Self::Designate,
            79 => Self::StockDisposal,
            81 => Self::BrokerDepositWarn,
            83 => Self::CoverWarn,
            84 => Self::PreExecOrder,
            80 => Self::ExecOrderRisk,
            85 => Self::PosiLimitWarn,
            86 => Self::QryPosiLimit,
            87 => Self::FBSign,
            88 => Self::FBAccount,
            89 => Self::RcvSMSCode,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ForceUserLogout => Some(49),
            Self::UserPasswordUpdate => Some(50),
            Self::SyncBrokerData => Some(51),
            Self::BatchSyncBrokerData => Some(52),
            Self::OrderInsert => Some(53),
            Self::OrderAction => Some(54),
            Self::AllQuery => Some(55),
            Self::Log => Some(97),
            Self::BaseQry => Some(98),
            Self::TradeQry => Some(99),
            Self::Trade => Some(100),
            Self::Virement => Some(101),
            Self::Risk => Some(102),
            Self::Session => Some(103),
            Self::RiskNoticeCtl => Some(104),
            Self::RiskNotice => Some(105),
            Self::BrokerDeposit => Some(106),
            Self::QueryFund => Some(107),
            Self::QueryOrder => Some(108),
            Self::QueryTrade => Some(109),
            Self::QueryPosition => Some(110),
            Self::QueryMarketData => Some(111),
            Self::QueryUserEvent => Some(112),
            Self::QueryRiskNotify => Some(113),
            Self::QueryFundChange => Some(114),
            Self::QueryInvestor => Some(115),
            Self::QueryTradingCode => Some(116),
            Self::ForceClose => Some(117),
            Self::PressTest => Some(118),
            Self::RemainCalc => Some(119),
            Self::NetPositionInd => Some(120),
            Self::RiskPredict => Some(121),
            Self::DataExport => Some(122),
            Self::RiskTargetSetup => Some(65),
            Self::MarketDataWarn => Some(66),
            Self::QryBizNotice => Some(67),
            Self::CfgBizNotice => Some(68),
            Self::SyncOTP => Some(69),
            Self::SendBizNotice => Some(70),
            Self::CfgRiskLevelStd => Some(71),
            Self::TbCommand => Some(72),
            Self::DeleteOrder => Some(74),
            Self::ParkedOrderInsert => Some(75),
            Self::ParkedOrderAction => Some(76),
            Self::ExecOrderNoCheck => Some(77),
            Self::Designate => Some(78),
            Self::StockDisposal => Some(79),
            Self::BrokerDepositWarn => Some(81),
            Self::CoverWarn => Some(83),
            Self::PreExecOrder => Some(84),
            Self::ExecOrderRisk => Some(80),
            Self::PosiLimitWarn => Some(85),
            Self::QryPosiLimit => Some(86),
            Self::FBSign => Some(87),
            Self::FBAccount => Some(88),
            Self::RcvSMSCode => Some(89),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BrokerFunctionCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBrokerRepealFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrokerRepealFlag {
    /// 期商无需自动冲正。CTP：`THOST_FTDC_BRORF_BrokerNotNeedRepeal = '0'`。
    BrokerNotNeedRepeal,
    /// 期商待自动冲正。CTP：`THOST_FTDC_BRORF_BrokerWaitingRepeal = '1'`。
    BrokerWaitingRepeal,
    /// 期商已自动冲正。CTP：`THOST_FTDC_BRORF_BrokerBeenRepealed = '2'`。
    BrokerBeenRepealed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BrokerRepealFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::BrokerNotNeedRepeal,
            49 => Self::BrokerWaitingRepeal,
            50 => Self::BrokerBeenRepealed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BrokerNotNeedRepeal => Some(48),
            Self::BrokerWaitingRepeal => Some(49),
            Self::BrokerBeenRepealed => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BrokerRepealFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBrokerTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrokerType {
    /// 交易会员。CTP：`THOST_FTDC_BT_Trade = '0'`。
    Trade,
    /// 交易结算会员。CTP：`THOST_FTDC_BT_TradeSettle = '1'`。
    TradeSettle,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BrokerType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Trade,
            49 => Self::TradeSettle,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Trade => Some(48),
            Self::TradeSettle => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BrokerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBrokerUserTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrokerUserType {
    /// 投资者。CTP：`THOST_FTDC_BUT_Investor = '1'`。
    Investor,
    /// 操作员。CTP：`THOST_FTDC_BUT_BrokerUser = '2'`。
    BrokerUser,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BrokerUserType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Investor,
            50 => Self::BrokerUser,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Investor => Some(49),
            Self::BrokerUser => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BrokerUserType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBusinessClassType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BusinessClass {
    /// 盈利。CTP：`THOST_FTDC_BT_Profit = '0'`。
    Profit,
    /// 亏损。CTP：`THOST_FTDC_BT_Loss = '1'`。
    Loss,
    /// 其他。CTP：`THOST_FTDC_BT_Other = 'Z'`。
    Other,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BusinessClass {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Profit,
            49 => Self::Loss,
            90 => Self::Other,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Profit => Some(48),
            Self::Loss => Some(49),
            Self::Other => Some(90),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BusinessClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcBusinessTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BusinessType {
    /// 请求。CTP：`THOST_FTDC_BT_Request = '1'`。
    Request,
    /// 应答。CTP：`THOST_FTDC_BT_Response = '2'`。
    Response,
    /// 通知。CTP：`THOST_FTDC_BT_Notice = '3'`。
    Notice,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl BusinessType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Request,
            50 => Self::Response,
            51 => Self::Notice,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Request => Some(49),
            Self::Response => Some(50),
            Self::Notice => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for BusinessType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcByGroupType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByGroup {
    /// 按投资者统计。CTP：`THOST_FTDC_BG_Investor = '2'`。
    Investor,
    /// 按类统计。CTP：`THOST_FTDC_BG_Group = '1'`。
    Group,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ByGroup {
    pub const fn from_code(code: u8) -> Self {
        match code {
            50 => Self::Investor,
            49 => Self::Group,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Investor => Some(50),
            Self::Group => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ByGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcByInvestorRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByInvestorRange {
    /// 属性统计。CTP：`THOST_FTDC_BIR_Property = '1'`。
    Property,
    /// 统计所有。CTP：`THOST_FTDC_BIR_All = '2'`。
    All,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ByInvestorRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Property,
            50 => Self::All,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Property => Some(49),
            Self::All => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ByInvestorRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCCBFeeModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CCBFeeMode {
    /// 按金额扣收。CTP：`THOST_FTDC_CCBFM_ByAmount = '1'`。
    ByAmount,
    /// 按月扣收。CTP：`THOST_FTDC_CCBFM_ByMonth = '2'`。
    ByMonth,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CCBFeeMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ByAmount,
            50 => Self::ByMonth,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByAmount => Some(49),
            Self::ByMonth => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CCBFeeMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCFFEXUploadFileNameType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CFFEXUploadFileName {
    /// ^\d{4}_SG\d{1}_\d{8}_\d{1}_Trade。CTP：`THOST_FTDC_CFUFN_SUFN_T = 'T'`。
    T,
    /// ^\d{4}_SG\d{1}_\d{8}_\d{1}_SettlementDetail。CTP：`THOST_FTDC_CFUFN_SUFN_P = 'P'`。
    P,
    /// ^\d{4}_SG\d{1}_\d{8}_\d{1}_Capital。CTP：`THOST_FTDC_CFUFN_SUFN_F = 'F'`。
    F,
    /// ^\d{4}_SG\d{1}_\d{8}_\d{1}_OptionExec。CTP：`THOST_FTDC_CFUFN_SUFN_S = 'S'`。
    S,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CFFEXUploadFileName {
    pub const fn from_code(code: u8) -> Self {
        match code {
            84 => Self::T,
            80 => Self::P,
            70 => Self::F,
            83 => Self::S,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::T => Some(84),
            Self::P => Some(80),
            Self::F => Some(70),
            Self::S => Some(83),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CFFEXUploadFileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCFMMCKeyKindType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CFMMCKeyKind {
    /// 主动请求更新。CTP：`THOST_FTDC_CFMMCKK_REQUEST = 'R'`。
    REQUEST,
    /// CFMMC自动更新。CTP：`THOST_FTDC_CFMMCKK_AUTO = 'A'`。
    AUTO,
    /// CFMMC手动更新。CTP：`THOST_FTDC_CFMMCKK_MANUAL = 'M'`。
    MANUAL,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CFMMCKeyKind {
    pub const fn from_code(code: u8) -> Self {
        match code {
            82 => Self::REQUEST,
            65 => Self::AUTO,
            77 => Self::MANUAL,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::REQUEST => Some(82),
            Self::AUTO => Some(65),
            Self::MANUAL => Some(77),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CFMMCKeyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CSRCDataQueryType {
    /// 查询当前交易日报送的数据。CTP：`THOST_FTDC_CSRCQ_Current = '0'`。
    Current,
    /// 查询历史报送的代理经纪公司的数据。CTP：`THOST_FTDC_CSRCQ_History = '1'`。
    History,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CSRCDataQueryType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Current,
            49 => Self::History,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Current => Some(48),
            Self::History => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CSRCDataQueryType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCSRCFundIOTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CSRCFundIOType {
    /// 出入金。CTP：`THOST_FTDC_CFIOT_FundIO = '0'`。
    FundIO,
    /// 银期换汇。CTP：`THOST_FTDC_CFIOT_SwapCurrency = '1'`。
    SwapCurrency,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CSRCFundIOType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::FundIO,
            49 => Self::SwapCurrency,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FundIO => Some(48),
            Self::SwapCurrency => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CSRCFundIOType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCTPTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CTPType {
    /// 未知类型。线值：`'0'`；名称已规范化，原始常量见审核表。
    Unknown,
    /// 主中心。CTP：`THOST_FTDC_CTPT_MainCenter = '1'`。
    MainCenter,
    /// 备中心。CTP：`THOST_FTDC_CTPT_BackUp = '2'`。
    BackUp,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CTPType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Unknown,
            49 => Self::MainCenter,
            50 => Self::BackUp,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Unknown => Some(48),
            Self::MainCenter => Some(49),
            Self::BackUp => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CTPType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCZCEUploadFileNameType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CZCEUploadFileName {
    /// ^\d{8}_zz_\d{4}。CTP：`THOST_FTDC_CUFN_CUFN_O = 'O'`。
    O,
    /// ^\d{8}成交表。CTP：`THOST_FTDC_CUFN_CUFN_T = 'T'`。
    T,
    /// ^\d{8}单腿持仓表new。CTP：`THOST_FTDC_CUFN_CUFN_P = 'P'`。
    P,
    /// ^\d{8}非平仓了结表。CTP：`THOST_FTDC_CUFN_CUFN_N = 'N'`。
    N,
    /// ^\d{8}平仓表。CTP：`THOST_FTDC_CUFN_CUFN_L = 'L'`。
    L,
    /// ^\d{8}资金表。CTP：`THOST_FTDC_CUFN_CUFN_F = 'F'`。
    F,
    /// ^\d{8}组合持仓表。CTP：`THOST_FTDC_CUFN_CUFN_C = 'C'`。
    C,
    /// ^\d{8}保证金参数表。CTP：`THOST_FTDC_CUFN_CUFN_M = 'M'`。
    M,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CZCEUploadFileName {
    pub const fn from_code(code: u8) -> Self {
        match code {
            79 => Self::O,
            84 => Self::T,
            80 => Self::P,
            78 => Self::N,
            76 => Self::L,
            70 => Self::F,
            67 => Self::C,
            77 => Self::M,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::O => Some(79),
            Self::T => Some(84),
            Self::P => Some(80),
            Self::N => Some(78),
            Self::L => Some(76),
            Self::F => Some(70),
            Self::C => Some(67),
            Self::M => Some(77),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CZCEUploadFileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCashExchangeCodeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CashExchangeCode {
    /// 汇。CTP：`THOST_FTDC_CEC_Exchange = '1'`。
    Exchange,
    /// 钞。CTP：`THOST_FTDC_CEC_Cash = '2'`。
    Cash,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CashExchangeCode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Exchange,
            50 => Self::Cash,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Exchange => Some(49),
            Self::Cash => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CashExchangeCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCertificationTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CertificationType {
    /// 身份证。CTP：`THOST_FTDC_CFT_IDCard = '0'`。
    IDCard,
    /// 护照。CTP：`THOST_FTDC_CFT_Passport = '1'`。
    Passport,
    /// 军官证。CTP：`THOST_FTDC_CFT_OfficerIDCard = '2'`。
    OfficerIDCard,
    /// 士兵证。CTP：`THOST_FTDC_CFT_SoldierIDCard = '3'`。
    SoldierIDCard,
    /// 回乡证。CTP：`THOST_FTDC_CFT_HomeComingCard = '4'`。
    HomeComingCard,
    /// 户口簿。CTP：`THOST_FTDC_CFT_HouseholdRegister = '5'`。
    HouseholdRegister,
    /// 营业执照号。CTP：`THOST_FTDC_CFT_LicenseNo = '6'`。
    LicenseNo,
    /// 组织机构代码证。CTP：`THOST_FTDC_CFT_InstitutionCodeCard = '7'`。
    InstitutionCodeCard,
    /// 临时营业执照号。CTP：`THOST_FTDC_CFT_TempLicenseNo = '8'`。
    TempLicenseNo,
    /// 民办非企业登记证书。CTP：`THOST_FTDC_CFT_NoEnterpriseLicenseNo = '9'`。
    NoEnterpriseLicenseNo,
    /// 其他证件。CTP：`THOST_FTDC_CFT_OtherCard = 'x'`。
    OtherCard,
    /// 主管部门批文。CTP：`THOST_FTDC_CFT_SuperDepAgree = 'a'`。
    SuperDepAgree,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CertificationType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::IDCard,
            49 => Self::Passport,
            50 => Self::OfficerIDCard,
            51 => Self::SoldierIDCard,
            52 => Self::HomeComingCard,
            53 => Self::HouseholdRegister,
            54 => Self::LicenseNo,
            55 => Self::InstitutionCodeCard,
            56 => Self::TempLicenseNo,
            57 => Self::NoEnterpriseLicenseNo,
            120 => Self::OtherCard,
            97 => Self::SuperDepAgree,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::IDCard => Some(48),
            Self::Passport => Some(49),
            Self::OfficerIDCard => Some(50),
            Self::SoldierIDCard => Some(51),
            Self::HomeComingCard => Some(52),
            Self::HouseholdRegister => Some(53),
            Self::LicenseNo => Some(54),
            Self::InstitutionCodeCard => Some(55),
            Self::TempLicenseNo => Some(56),
            Self::NoEnterpriseLicenseNo => Some(57),
            Self::OtherCard => Some(120),
            Self::SuperDepAgree => Some(97),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CertificationType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCfmmcReturnCodeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CfmmcReturnCode {
    /// 成功。CTP：`THOST_FTDC_CRC_Success = '0'`。
    Success,
    /// 该客户已经有流程在处理中。CTP：`THOST_FTDC_CRC_Working = '1'`。
    Working,
    /// 监控中客户资料检查失败。CTP：`THOST_FTDC_CRC_InfoFail = '2'`。
    InfoFail,
    /// 监控中实名制检查失败。CTP：`THOST_FTDC_CRC_IDCardFail = '3'`。
    IDCardFail,
    /// 其他错误。CTP：`THOST_FTDC_CRC_OtherFail = '4'`。
    OtherFail,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CfmmcReturnCode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Success,
            49 => Self::Working,
            50 => Self::InfoFail,
            51 => Self::IDCardFail,
            52 => Self::OtherFail,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Success => Some(48),
            Self::Working => Some(49),
            Self::InfoFail => Some(50),
            Self::IDCardFail => Some(51),
            Self::OtherFail => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CfmmcReturnCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCheckInstrTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckInstrType {
    /// 合约交易所不存在。CTP：`THOST_FTDC_CIT_HasExch = '0'`。
    HasExch,
    /// 合约本系统不存在。CTP：`THOST_FTDC_CIT_HasATP = '1'`。
    HasATP,
    /// 合约比较不一致。CTP：`THOST_FTDC_CIT_HasDiff = '2'`。
    HasDiff,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CheckInstrType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::HasExch,
            49 => Self::HasATP,
            50 => Self::HasDiff,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::HasExch => Some(48),
            Self::HasATP => Some(49),
            Self::HasDiff => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CheckInstrType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCheckLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckLevel {
    /// 零级复核。CTP：`THOST_FTDC_CL_Zero = '0'`。
    Zero,
    /// 一级复核。CTP：`THOST_FTDC_CL_One = '1'`。
    One,
    /// 二级复核。CTP：`THOST_FTDC_CL_Two = '2'`。
    Two,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CheckLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Zero,
            49 => Self::One,
            50 => Self::Two,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Zero => Some(48),
            Self::One => Some(49),
            Self::Two => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CheckLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCheckStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckStatus {
    /// 未复核。CTP：`THOST_FTDC_CHS_Init = '0'`。
    Init,
    /// 复核中。CTP：`THOST_FTDC_CHS_Checking = '1'`。
    Checking,
    /// 已复核。CTP：`THOST_FTDC_CHS_Checked = '2'`。
    Checked,
    /// 拒绝。CTP：`THOST_FTDC_CHS_Refuse = '3'`。
    Refuse,
    /// 作废。CTP：`THOST_FTDC_CHS_Cancel = '4'`。
    Cancel,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CheckStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Init,
            49 => Self::Checking,
            50 => Self::Checked,
            51 => Self::Refuse,
            52 => Self::Cancel,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Init => Some(48),
            Self::Checking => Some(49),
            Self::Checked => Some(50),
            Self::Refuse => Some(51),
            Self::Cancel => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CheckStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcClassTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassType {
    /// 所有合约。CTP：`THOST_FTDC_INS_ALL = '0'`。
    ALL,
    /// 期货、即期、期转现、Tas、金属指数合约。CTP：`THOST_FTDC_INS_FUTURE = '1'`。
    FUTURE,
    /// 期货、现货期权合约。CTP：`THOST_FTDC_INS_OPTION = '2'`。
    OPTION,
    /// 组合合约。CTP：`THOST_FTDC_INS_COMB = '3'`。
    COMB,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ClassType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ALL,
            49 => Self::FUTURE,
            50 => Self::OPTION,
            51 => Self::COMB,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ALL => Some(48),
            Self::FUTURE => Some(49),
            Self::OPTION => Some(50),
            Self::COMB => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ClassType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcClientIDStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientIDStatus {
    /// 未申请。CTP：`THOST_FTDC_UOACS_NoApply = '1'`。
    NoApply,
    /// 已提交申请。线值：`'2'`；名称已规范化，原始常量见审核表。
    Submitted,
    /// 已发送申请。线值：`'3'`；名称已规范化，原始常量见审核表。
    Sent,
    /// 完成。CTP：`THOST_FTDC_UOACS_Success = '4'`。
    Success,
    /// 拒绝。CTP：`THOST_FTDC_UOACS_Refuse = '5'`。
    Refuse,
    /// 已撤销编码。CTP：`THOST_FTDC_UOACS_Cancel = '6'`。
    Cancel,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ClientIDStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NoApply,
            50 => Self::Submitted,
            51 => Self::Sent,
            52 => Self::Success,
            53 => Self::Refuse,
            54 => Self::Cancel,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoApply => Some(49),
            Self::Submitted => Some(50),
            Self::Sent => Some(51),
            Self::Success => Some(52),
            Self::Refuse => Some(53),
            Self::Cancel => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ClientIDStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcClientIDTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientIDType {
    /// 投机。CTP：`THOST_FTDC_CIDT_Speculation = '1'`。
    Speculation,
    /// 套利。CTP：`THOST_FTDC_CIDT_Arbitrage = '2'`。
    Arbitrage,
    /// 套保。CTP：`THOST_FTDC_CIDT_Hedge = '3'`。
    Hedge,
    /// 做市商。CTP：`THOST_FTDC_CIDT_MarketMaker = '5'`。
    MarketMaker,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ClientIDType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Speculation,
            50 => Self::Arbitrage,
            51 => Self::Hedge,
            53 => Self::MarketMaker,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Speculation => Some(49),
            Self::Arbitrage => Some(50),
            Self::Hedge => Some(51),
            Self::MarketMaker => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ClientIDType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcClientRegionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientRegion {
    /// 国内客户。CTP：`THOST_FTDC_CR_Domestic = '1'`。
    Domestic,
    /// 港澳台客户。CTP：`THOST_FTDC_CR_GMT = '2'`。
    GMT,
    /// 国外客户。CTP：`THOST_FTDC_CR_Foreign = '3'`。
    Foreign,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ClientRegion {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Domestic,
            50 => Self::GMT,
            51 => Self::Foreign,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Domestic => Some(49),
            Self::GMT => Some(50),
            Self::Foreign => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ClientRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcClientTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientType {
    /// 所有。CTP：`THOST_FTDC_CfMMCCT_All = '0'`。
    All,
    /// 个人。CTP：`THOST_FTDC_CfMMCCT_Person = '1'`。
    Person,
    /// 单位。CTP：`THOST_FTDC_CfMMCCT_Company = '2'`。
    Company,
    /// 其他。CTP：`THOST_FTDC_CfMMCCT_Other = '3'`。
    Other,
    /// 特殊法人。CTP：`THOST_FTDC_CfMMCCT_SpecialOrgan = '4'`。
    SpecialOrgan,
    /// 资管户。CTP：`THOST_FTDC_CfMMCCT_Asset = '5'`。
    Asset,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ClientType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::All,
            49 => Self::Person,
            50 => Self::Company,
            51 => Self::Other,
            52 => Self::SpecialOrgan,
            53 => Self::Asset,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(48),
            Self::Person => Some(49),
            Self::Company => Some(50),
            Self::Other => Some(51),
            Self::SpecialOrgan => Some(52),
            Self::Asset => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCloseDealTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloseDealType {
    /// 正常。CTP：`THOST_FTDC_CDT_Normal = '0'`。
    Normal,
    /// 投机平仓优先。CTP：`THOST_FTDC_CDT_SpecFirst = '1'`。
    SpecFirst,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CloseDealType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::SpecFirst,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::SpecFirst => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CloseDealType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCloseStyleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloseStyle {
    /// 先开先平。CTP：`THOST_FTDC_ICS_Close = '0'`。
    Close,
    /// 先平今再平昨。CTP：`THOST_FTDC_ICS_CloseToday = '1'`。
    CloseToday,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CloseStyle {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Close,
            49 => Self::CloseToday,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Close => Some(48),
            Self::CloseToday => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CloseStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCmbTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmbType {
    /// SPZ。CTP：`THOST_FTDC_ZCECT_SPZ = '0'`。
    SPZ,
    /// SPD。CTP：`THOST_FTDC_ZCECT_SPD = '1'`。
    SPD,
    /// IPS。CTP：`THOST_FTDC_ZCECT_IPS = '2'`。
    IPS,
    /// BUL。CTP：`THOST_FTDC_ZCECT_BUL = '3'`。
    BUL,
    /// BER。CTP：`THOST_FTDC_ZCECT_BER = '4'`。
    BER,
    /// BLT。CTP：`THOST_FTDC_ZCECT_BLT = '5'`。
    BLT,
    /// BRT。CTP：`THOST_FTDC_ZCECT_BRT = '6'`。
    BRT,
    /// STD。CTP：`THOST_FTDC_ZCECT_STD = '7'`。
    STD,
    /// STG。CTP：`THOST_FTDC_ZCECT_STG = '8'`。
    STG,
    /// PRT。CTP：`THOST_FTDC_ZCECT_PRT = '9'`。
    PRT,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CmbType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::SPZ,
            49 => Self::SPD,
            50 => Self::IPS,
            51 => Self::BUL,
            52 => Self::BER,
            53 => Self::BLT,
            54 => Self::BRT,
            55 => Self::STD,
            56 => Self::STG,
            57 => Self::PRT,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SPZ => Some(48),
            Self::SPD => Some(49),
            Self::IPS => Some(50),
            Self::BUL => Some(51),
            Self::BER => Some(52),
            Self::BLT => Some(53),
            Self::BRT => Some(54),
            Self::STD => Some(55),
            Self::STG => Some(56),
            Self::PRT => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CmbType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCodeSourceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodeSourceType {
    /// 统一开户(已规范)。CTP：`THOST_FTDC_CST_UnifyAccount = '0'`。
    UnifyAccount,
    /// 手工录入(未规范)。CTP：`THOST_FTDC_CST_ManualEntry = '1'`。
    ManualEntry,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CodeSourceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnifyAccount,
            49 => Self::ManualEntry,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnifyAccount => Some(48),
            Self::ManualEntry => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CodeSourceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCombDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombDirection {
    /// 申请组合。CTP：`THOST_FTDC_CMDR_Comb = '0'`。
    Comb,
    /// 申请拆分。CTP：`THOST_FTDC_CMDR_UnComb = '1'`。
    UnComb,
    /// 操作员删组合单。CTP：`THOST_FTDC_CMDR_DelComb = '2'`。
    DelComb,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CombDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Comb,
            49 => Self::UnComb,
            50 => Self::DelComb,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Comb => Some(48),
            Self::UnComb => Some(49),
            Self::DelComb => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CombDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCombinationTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombinationType {
    /// 期货组合。CTP：`THOST_FTDC_COMBT_Future = '0'`。
    Future,
    /// 垂直价差BUL。CTP：`THOST_FTDC_COMBT_BUL = '1'`。
    BUL,
    /// 垂直价差BER。CTP：`THOST_FTDC_COMBT_BER = '2'`。
    BER,
    /// 跨式组合。CTP：`THOST_FTDC_COMBT_STD = '3'`。
    STD,
    /// 宽跨式组合。CTP：`THOST_FTDC_COMBT_STG = '4'`。
    STG,
    /// 备兑组合。CTP：`THOST_FTDC_COMBT_PRT = '5'`。
    PRT,
    /// 时间价差组合。CTP：`THOST_FTDC_COMBT_CAS = '6'`。
    CAS,
    /// 期权对锁组合。CTP：`THOST_FTDC_COMBT_OPL = '7'`。
    OPL,
    /// 买备兑组合。CTP：`THOST_FTDC_COMBT_BFO = '8'`。
    BFO,
    /// 买入期权垂直价差组合。CTP：`THOST_FTDC_COMBT_BLS = '9'`。
    BLS,
    /// 卖出期权垂直价差组合。CTP：`THOST_FTDC_COMBT_BES = 'a'`。
    BES,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CombinationType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Future,
            49 => Self::BUL,
            50 => Self::BER,
            51 => Self::STD,
            52 => Self::STG,
            53 => Self::PRT,
            54 => Self::CAS,
            55 => Self::OPL,
            56 => Self::BFO,
            57 => Self::BLS,
            97 => Self::BES,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Future => Some(48),
            Self::BUL => Some(49),
            Self::BER => Some(50),
            Self::STD => Some(51),
            Self::STG => Some(52),
            Self::PRT => Some(53),
            Self::CAS => Some(54),
            Self::OPL => Some(55),
            Self::BFO => Some(56),
            Self::BLS => Some(57),
            Self::BES => Some(97),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CombinationType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCommApiTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommApiType {
    /// 客户端。CTP：`THOST_FTDC_CAPIT_Client = '1'`。
    Client,
    /// 服务端。CTP：`THOST_FTDC_CAPIT_Server = '2'`。
    Server,
    /// 交易系统的UserApi。CTP：`THOST_FTDC_CAPIT_UserApi = '3'`。
    UserApi,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CommApiType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Client,
            50 => Self::Server,
            51 => Self::UserApi,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Client => Some(49),
            Self::Server => Some(50),
            Self::UserApi => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CommApiType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcConditionalOrderSortTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionalOrderSortType {
    /// 使用最新价升序。CTP：`THOST_FTDC_COST_LastPriceAsc = '0'`。
    LastPriceAsc,
    /// 使用最新价降序。CTP：`THOST_FTDC_COST_LastPriceDesc = '1'`。
    LastPriceDesc,
    /// 使用卖价升序。CTP：`THOST_FTDC_COST_AskPriceAsc = '2'`。
    AskPriceAsc,
    /// 使用卖价降序。CTP：`THOST_FTDC_COST_AskPriceDesc = '3'`。
    AskPriceDesc,
    /// 使用买价升序。CTP：`THOST_FTDC_COST_BidPriceAsc = '4'`。
    BidPriceAsc,
    /// 使用买价降序。CTP：`THOST_FTDC_COST_BidPriceDesc = '5'`。
    BidPriceDesc,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ConditionalOrderSortType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::LastPriceAsc,
            49 => Self::LastPriceDesc,
            50 => Self::AskPriceAsc,
            51 => Self::AskPriceDesc,
            52 => Self::BidPriceAsc,
            53 => Self::BidPriceDesc,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::LastPriceAsc => Some(48),
            Self::LastPriceDesc => Some(49),
            Self::AskPriceAsc => Some(50),
            Self::AskPriceDesc => Some(51),
            Self::BidPriceAsc => Some(52),
            Self::BidPriceDesc => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ConditionalOrderSortType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcConnectModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectMode {
    /// 短连接。CTP：`THOST_FTDC_CM_ShortConnect = '0'`。
    ShortConnect,
    /// 长连接。CTP：`THOST_FTDC_CM_LongConnect = '1'`。
    LongConnect,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ConnectMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ShortConnect,
            49 => Self::LongConnect,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ShortConnect => Some(48),
            Self::LongConnect => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ConnectMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcContingentConditionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContingentCondition {
    /// 立即。CTP：`THOST_FTDC_CC_Immediately = '1'`。
    Immediately,
    /// 止损。CTP：`THOST_FTDC_CC_Touch = '2'`。
    Touch,
    /// 止赢。CTP：`THOST_FTDC_CC_TouchProfit = '3'`。
    TouchProfit,
    /// 预埋单。CTP：`THOST_FTDC_CC_ParkedOrder = '4'`。
    ParkedOrder,
    /// 最新价大于条件价。CTP：`THOST_FTDC_CC_LastPriceGreaterThanStopPrice = '5'`。
    LastPriceGreaterThanStopPrice,
    /// 最新价大于等于条件价。CTP：`THOST_FTDC_CC_LastPriceGreaterEqualStopPrice = '6'`。
    LastPriceGreaterEqualStopPrice,
    /// 最新价小于条件价。CTP：`THOST_FTDC_CC_LastPriceLesserThanStopPrice = '7'`。
    LastPriceLesserThanStopPrice,
    /// 最新价小于等于条件价。CTP：`THOST_FTDC_CC_LastPriceLesserEqualStopPrice = '8'`。
    LastPriceLesserEqualStopPrice,
    /// 卖一价大于条件价。CTP：`THOST_FTDC_CC_AskPriceGreaterThanStopPrice = '9'`。
    AskPriceGreaterThanStopPrice,
    /// 卖一价大于等于条件价。CTP：`THOST_FTDC_CC_AskPriceGreaterEqualStopPrice = 'A'`。
    AskPriceGreaterEqualStopPrice,
    /// 卖一价小于条件价。CTP：`THOST_FTDC_CC_AskPriceLesserThanStopPrice = 'B'`。
    AskPriceLesserThanStopPrice,
    /// 卖一价小于等于条件价。CTP：`THOST_FTDC_CC_AskPriceLesserEqualStopPrice = 'C'`。
    AskPriceLesserEqualStopPrice,
    /// 买一价大于条件价。CTP：`THOST_FTDC_CC_BidPriceGreaterThanStopPrice = 'D'`。
    BidPriceGreaterThanStopPrice,
    /// 买一价大于等于条件价。CTP：`THOST_FTDC_CC_BidPriceGreaterEqualStopPrice = 'E'`。
    BidPriceGreaterEqualStopPrice,
    /// 买一价小于条件价。CTP：`THOST_FTDC_CC_BidPriceLesserThanStopPrice = 'F'`。
    BidPriceLesserThanStopPrice,
    /// 买一价小于等于条件价。CTP：`THOST_FTDC_CC_BidPriceLesserEqualStopPrice = 'H'`。
    BidPriceLesserEqualStopPrice,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ContingentCondition {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Immediately,
            50 => Self::Touch,
            51 => Self::TouchProfit,
            52 => Self::ParkedOrder,
            53 => Self::LastPriceGreaterThanStopPrice,
            54 => Self::LastPriceGreaterEqualStopPrice,
            55 => Self::LastPriceLesserThanStopPrice,
            56 => Self::LastPriceLesserEqualStopPrice,
            57 => Self::AskPriceGreaterThanStopPrice,
            65 => Self::AskPriceGreaterEqualStopPrice,
            66 => Self::AskPriceLesserThanStopPrice,
            67 => Self::AskPriceLesserEqualStopPrice,
            68 => Self::BidPriceGreaterThanStopPrice,
            69 => Self::BidPriceGreaterEqualStopPrice,
            70 => Self::BidPriceLesserThanStopPrice,
            72 => Self::BidPriceLesserEqualStopPrice,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Immediately => Some(49),
            Self::Touch => Some(50),
            Self::TouchProfit => Some(51),
            Self::ParkedOrder => Some(52),
            Self::LastPriceGreaterThanStopPrice => Some(53),
            Self::LastPriceGreaterEqualStopPrice => Some(54),
            Self::LastPriceLesserThanStopPrice => Some(55),
            Self::LastPriceLesserEqualStopPrice => Some(56),
            Self::AskPriceGreaterThanStopPrice => Some(57),
            Self::AskPriceGreaterEqualStopPrice => Some(65),
            Self::AskPriceLesserThanStopPrice => Some(66),
            Self::AskPriceLesserEqualStopPrice => Some(67),
            Self::BidPriceGreaterThanStopPrice => Some(68),
            Self::BidPriceGreaterEqualStopPrice => Some(69),
            Self::BidPriceLesserThanStopPrice => Some(70),
            Self::BidPriceLesserEqualStopPrice => Some(72),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ContingentCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCurrExDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrExDirection {
    /// 结汇。CTP：`THOST_FTDC_CED_Settlement = '0'`。
    Settlement,
    /// 售汇。CTP：`THOST_FTDC_CED_Sale = '1'`。
    Sale,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CurrExDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Settlement,
            49 => Self::Sale,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Settlement => Some(48),
            Self::Sale => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CurrExDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCurrencySwapStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrencySwapStatus {
    /// 已录入。CTP：`THOST_FTDC_CSS_Entry = '1'`。
    Entry,
    /// 已审核。CTP：`THOST_FTDC_CSS_Approve = '2'`。
    Approve,
    /// 已拒绝。CTP：`THOST_FTDC_CSS_Refuse = '3'`。
    Refuse,
    /// 已撤销。CTP：`THOST_FTDC_CSS_Revoke = '4'`。
    Revoke,
    /// 已发送。CTP：`THOST_FTDC_CSS_Send = '5'`。
    Send,
    /// 换汇成功。CTP：`THOST_FTDC_CSS_Success = '6'`。
    Success,
    /// 换汇失败。CTP：`THOST_FTDC_CSS_Failure = '7'`。
    Failure,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CurrencySwapStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Entry,
            50 => Self::Approve,
            51 => Self::Refuse,
            52 => Self::Revoke,
            53 => Self::Send,
            54 => Self::Success,
            55 => Self::Failure,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Entry => Some(49),
            Self::Approve => Some(50),
            Self::Refuse => Some(51),
            Self::Revoke => Some(52),
            Self::Send => Some(53),
            Self::Success => Some(54),
            Self::Failure => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CurrencySwapStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCusAccountTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CusAccountType {
    /// 期货结算账户。CTP：`THOST_FTDC_CAT_Futures = '1'`。
    Futures,
    /// 纯期货资管业务下的资管结算账户。线值：`'2'`；名称已规范化，原始常量见审核表。
    AssetManagerFuture,
    /// 综合类资管业务下的期货资管托管账户。线值：`'3'`；名称已规范化，原始常量见审核表。
    AssetManagerTrustee,
    /// 综合类资管业务下的资金中转账户。线值：`'4'`；名称已规范化，原始常量见审核表。
    AssetManagerTransfer,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CusAccountType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Futures,
            50 => Self::AssetManagerFuture,
            51 => Self::AssetManagerTrustee,
            52 => Self::AssetManagerTransfer,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Futures => Some(49),
            Self::AssetManagerFuture => Some(50),
            Self::AssetManagerTrustee => Some(51),
            Self::AssetManagerTransfer => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CusAccountType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcCustTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CustType {
    /// 自然人。CTP：`THOST_FTDC_CUSTT_Person = '0'`。
    Person,
    /// 机构户。CTP：`THOST_FTDC_CUSTT_Institution = '1'`。
    Institution,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl CustType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Person,
            49 => Self::Institution,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Person => Some(48),
            Self::Institution => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for CustType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDAClientTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DAClientType {
    /// 自然人。CTP：`THOST_FTDC_CACT_Person = '0'`。
    Person,
    /// 法人。CTP：`THOST_FTDC_CACT_Company = '1'`。
    Company,
    /// 其他。CTP：`THOST_FTDC_CACT_Other = '2'`。
    Other,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DAClientType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Person,
            49 => Self::Company,
            50 => Self::Other,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Person => Some(48),
            Self::Company => Some(49),
            Self::Other => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DAClientType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDBOperationType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DBOperation {
    /// 插入。CTP：`THOST_FTDC_DBOP_Insert = '0'`。
    Insert,
    /// 更新。CTP：`THOST_FTDC_DBOP_Update = '1'`。
    Update,
    /// 删除。CTP：`THOST_FTDC_DBOP_Delete = '2'`。
    Delete,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DBOperation {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Insert,
            49 => Self::Update,
            50 => Self::Delete,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Insert => Some(48),
            Self::Update => Some(49),
            Self::Delete => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DBOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDCEUploadFileNameType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DCEUploadFileName {
    /// ^\d{8}_dl_\d{3}。CTP：`THOST_FTDC_DUFN_DUFN_O = 'O'`。
    O,
    /// ^\d{8}_成交表。CTP：`THOST_FTDC_DUFN_DUFN_T = 'T'`。
    T,
    /// ^\d{8}_持仓表。CTP：`THOST_FTDC_DUFN_DUFN_P = 'P'`。
    P,
    /// ^\d{8}_资金结算表。CTP：`THOST_FTDC_DUFN_DUFN_F = 'F'`。
    F,
    /// ^\d{8}_优惠组合持仓明细表。CTP：`THOST_FTDC_DUFN_DUFN_C = 'C'`。
    C,
    /// ^\d{8}_持仓明细表。CTP：`THOST_FTDC_DUFN_DUFN_D = 'D'`。
    D,
    /// ^\d{8}_保证金参数表。CTP：`THOST_FTDC_DUFN_DUFN_M = 'M'`。
    M,
    /// ^\d{8}_期权执行表。CTP：`THOST_FTDC_DUFN_DUFN_S = 'S'`。
    S,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DCEUploadFileName {
    pub const fn from_code(code: u8) -> Self {
        match code {
            79 => Self::O,
            84 => Self::T,
            80 => Self::P,
            70 => Self::F,
            67 => Self::C,
            68 => Self::D,
            77 => Self::M,
            83 => Self::S,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::O => Some(79),
            Self::T => Some(84),
            Self::P => Some(80),
            Self::F => Some(70),
            Self::C => Some(67),
            Self::D => Some(68),
            Self::M => Some(77),
            Self::S => Some(83),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DCEUploadFileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDataResourceType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataResource {
    /// 本系统。CTP：`THOST_FTDC_DAR_Settle = '1'`。
    Settle,
    /// 交易所。CTP：`THOST_FTDC_DAR_Exchange = '2'`。
    Exchange,
    /// 报送数据。CTP：`THOST_FTDC_DAR_CSRC = '3'`。
    CSRC,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DataResource {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Settle,
            50 => Self::Exchange,
            51 => Self::CSRC,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Settle => Some(49),
            Self::Exchange => Some(50),
            Self::CSRC => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DataResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDataStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataStatus {
    /// 正常。CTP：`THOST_FTDC_AMLDS_Normal = '0'`。
    Normal,
    /// 已删除。CTP：`THOST_FTDC_AMLDS_Deleted = '1'`。
    Deleted,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DataStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::Deleted,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::Deleted => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DataStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDataSyncStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataSyncStatus {
    /// 未同步。CTP：`THOST_FTDC_DS_Asynchronous = '1'`。
    Asynchronous,
    /// 同步中。CTP：`THOST_FTDC_DS_Synchronizing = '2'`。
    Synchronizing,
    /// 已同步。CTP：`THOST_FTDC_DS_Synchronized = '3'`。
    Synchronized,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DataSyncStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Asynchronous,
            50 => Self::Synchronizing,
            51 => Self::Synchronized,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Asynchronous => Some(49),
            Self::Synchronizing => Some(50),
            Self::Synchronized => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DataSyncStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDceCombinationTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DceCombinationType {
    /// 期货对锁组合。CTP：`THOST_FTDC_DCECOMBT_SPL = '0'`。
    SPL,
    /// 期权对锁组合。CTP：`THOST_FTDC_DCECOMBT_OPL = '1'`。
    OPL,
    /// 期货跨期组合。CTP：`THOST_FTDC_DCECOMBT_SP = '2'`。
    SP,
    /// 期货跨品种组合。CTP：`THOST_FTDC_DCECOMBT_SPC = '3'`。
    SPC,
    /// 买入期权垂直价差组合。CTP：`THOST_FTDC_DCECOMBT_BLS = '4'`。
    BLS,
    /// 卖出期权垂直价差组合。CTP：`THOST_FTDC_DCECOMBT_BES = '5'`。
    BES,
    /// 期权日历价差组合。CTP：`THOST_FTDC_DCECOMBT_CAS = '6'`。
    CAS,
    /// 期权跨式组合。CTP：`THOST_FTDC_DCECOMBT_STD = '7'`。
    STD,
    /// 期权宽跨式组合。CTP：`THOST_FTDC_DCECOMBT_STG = '8'`。
    STG,
    /// 买入期货期权组合。CTP：`THOST_FTDC_DCECOMBT_BFO = '9'`。
    BFO,
    /// 卖出期货期权组合。CTP：`THOST_FTDC_DCECOMBT_SFO = 'a'`。
    SFO,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DceCombinationType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::SPL,
            49 => Self::OPL,
            50 => Self::SP,
            51 => Self::SPC,
            52 => Self::BLS,
            53 => Self::BES,
            54 => Self::CAS,
            55 => Self::STD,
            56 => Self::STG,
            57 => Self::BFO,
            97 => Self::SFO,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SPL => Some(48),
            Self::OPL => Some(49),
            Self::SP => Some(50),
            Self::SPC => Some(51),
            Self::BLS => Some(52),
            Self::BES => Some(53),
            Self::CAS => Some(54),
            Self::STD => Some(55),
            Self::STG => Some(56),
            Self::BFO => Some(57),
            Self::SFO => Some(97),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DceCombinationType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDeliveryModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryMode {
    /// 现金交割。线值：`'1'`；名称已规范化，原始常量见审核表。
    CashDelivery,
    /// 实物交割。线值：`'2'`；名称已规范化，原始常量见审核表。
    CommodityDelivery,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DeliveryMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::CashDelivery,
            50 => Self::CommodityDelivery,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CashDelivery => Some(49),
            Self::CommodityDelivery => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DeliveryMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDeliveryTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryType {
    /// 手工交割。线值：`'1'`；名称已规范化，原始常量见审核表。
    ManualDelivery,
    /// 到期交割。线值：`'2'`；名称已规范化，原始常量见审核表。
    PersonDelivery,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DeliveryType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ManualDelivery,
            50 => Self::PersonDelivery,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ManualDelivery => Some(49),
            Self::PersonDelivery => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DeliveryType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDepartmentRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepartmentRange {
    /// 所有。CTP：`THOST_FTDC_DR_All = '1'`。
    All,
    /// 组织架构。CTP：`THOST_FTDC_DR_Group = '2'`。
    Group,
    /// 单一投资者。CTP：`THOST_FTDC_DR_Single = '3'`。
    Single,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DepartmentRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::Group,
            51 => Self::Single,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::Group => Some(50),
            Self::Single => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DepartmentRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// 买。CTP：`THOST_FTDC_D_Buy = '0'`。
    Buy,
    /// 卖。CTP：`THOST_FTDC_D_Sell = '1'`。
    Sell,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl Direction {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Buy,
            49 => Self::Sell,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Buy => Some(48),
            Self::Sell => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcDirectionEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionEn {
    /// Buy。CTP：`THOST_FTDC_DEN_Buy = '0'`。
    Buy,
    /// Sell。CTP：`THOST_FTDC_DEN_Sell = '1'`。
    Sell,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl DirectionEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Buy,
            49 => Self::Sell,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Buy => Some(48),
            Self::Sell => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for DirectionEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcEnumBoolType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumBool {
    /// false。线值：`'0'`；名称已规范化，原始常量见审核表。
    FalseValue,
    /// true。线值：`'1'`；名称已规范化，原始常量见审核表。
    TrueValue,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl EnumBool {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::FalseValue,
            49 => Self::TrueValue,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FalseValue => Some(48),
            Self::TrueValue => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for EnumBool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcEventModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventMode {
    /// 增加。CTP：`THOST_FTDC_EvM_ADD = '1'`。
    ADD,
    /// 修改。CTP：`THOST_FTDC_EvM_UPDATE = '2'`。
    UPDATE,
    /// 删除。CTP：`THOST_FTDC_EvM_DELETE = '3'`。
    DELETE,
    /// 复核。CTP：`THOST_FTDC_EvM_CHECK = '4'`。
    CHECK,
    /// 复制。CTP：`THOST_FTDC_EvM_COPY = '5'`。
    COPY,
    /// 注销。CTP：`THOST_FTDC_EvM_CANCEL = '6'`。
    CANCEL,
    /// 冲销。CTP：`THOST_FTDC_EvM_Reverse = '7'`。
    Reverse,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl EventMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ADD,
            50 => Self::UPDATE,
            51 => Self::DELETE,
            52 => Self::CHECK,
            53 => Self::COPY,
            54 => Self::CANCEL,
            55 => Self::Reverse,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ADD => Some(49),
            Self::UPDATE => Some(50),
            Self::DELETE => Some(51),
            Self::CHECK => Some(52),
            Self::COPY => Some(53),
            Self::CANCEL => Some(54),
            Self::Reverse => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for EventMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExClientIDTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExClientIDType {
    /// 套保。CTP：`THOST_FTDC_ECIDT_Hedge = '1'`。
    Hedge,
    /// 套利。CTP：`THOST_FTDC_ECIDT_Arbitrage = '2'`。
    Arbitrage,
    /// 投机。CTP：`THOST_FTDC_ECIDT_Speculation = '3'`。
    Speculation,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExClientIDType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Hedge,
            50 => Self::Arbitrage,
            51 => Self::Speculation,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Hedge => Some(49),
            Self::Arbitrage => Some(50),
            Self::Speculation => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExClientIDType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExDirection {
    /// 结汇。CTP：`THOST_FTDC_FBEDIR_Settlement = '0'`。
    Settlement,
    /// 售汇。CTP：`THOST_FTDC_FBEDIR_Sale = '1'`。
    Sale,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Settlement,
            49 => Self::Sale,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Settlement => Some(48),
            Self::Sale => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExStatus {
    /// 修改前。CTP：`THOST_FTDC_EXS_Before = '0'`。
    Before,
    /// 修改后。CTP：`THOST_FTDC_EXS_After = '1'`。
    After,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Before,
            49 => Self::After,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Before => Some(48),
            Self::After => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExchangeConnectStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeConnectStatus {
    /// 没有任何连接。CTP：`THOST_FTDC_ECS_NoConnection = '1'`。
    NoConnection,
    /// 已经发出合约查询请求。CTP：`THOST_FTDC_ECS_QryInstrumentSent = '2'`。
    QryInstrumentSent,
    /// 已经获取信息。CTP：`THOST_FTDC_ECS_GotInformation = '9'`。
    GotInformation,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExchangeConnectStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NoConnection,
            50 => Self::QryInstrumentSent,
            57 => Self::GotInformation,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoConnection => Some(49),
            Self::QryInstrumentSent => Some(50),
            Self::GotInformation => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExchangeConnectStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExchangeIDTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeIDType {
    /// 上海期货交易所。CTP：`THOST_FTDC_EIDT_SHFE = 'S'`。
    SHFE,
    /// 郑州商品交易所。CTP：`THOST_FTDC_EIDT_CZCE = 'Z'`。
    CZCE,
    /// 大连商品交易所。CTP：`THOST_FTDC_EIDT_DCE = 'D'`。
    DCE,
    /// 中国金融期货交易所。CTP：`THOST_FTDC_EIDT_CFFEX = 'J'`。
    CFFEX,
    /// 上海国际能源交易中心股份有限公司。CTP：`THOST_FTDC_EIDT_INE = 'N'`。
    INE,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExchangeIDType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            83 => Self::SHFE,
            90 => Self::CZCE,
            68 => Self::DCE,
            74 => Self::CFFEX,
            78 => Self::INE,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SHFE => Some(83),
            Self::CZCE => Some(90),
            Self::DCE => Some(68),
            Self::CFFEX => Some(74),
            Self::INE => Some(78),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExchangeIDType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExchangePropertyType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeProperty {
    /// 正常。CTP：`THOST_FTDC_EXP_Normal = '0'`。
    Normal,
    /// 根据成交生成报单。CTP：`THOST_FTDC_EXP_GenOrderByTrade = '1'`。
    GenOrderByTrade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExchangeProperty {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::GenOrderByTrade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::GenOrderByTrade => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExchangeProperty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExchangeSettlementParamIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeSettlementParamID {
    /// 质押比例。CTP：`THOST_FTDC_ESPI_MortgageRatio = '1'`。
    MortgageRatio,
    /// 分项资金导入项。CTP：`THOST_FTDC_ESPI_OtherFundItem = '2'`。
    OtherFundItem,
    /// 分项资金入交易所出入金。CTP：`THOST_FTDC_ESPI_OtherFundImport = '3'`。
    OtherFundImport,
    /// 中金所开户最低可用金额。线值：`'6'`；名称已规范化，原始常量见审核表。
    CFFEXMinPrepay,
    /// 郑商所结算方式。CTP：`THOST_FTDC_ESPI_CZCESettlementType = '7'`。
    CZCESettlementType,
    /// 交易所交割手续费收取方式。线值：`'9'`；名称已规范化，原始常量见审核表。
    ExchangeDeliveryFeeMode,
    /// 投资者交割手续费收取方式。线值：`'0'`；名称已规范化，原始常量见审核表。
    DeliveryFeeMode,
    /// 郑商所组合持仓保证金收取方式。CTP：`THOST_FTDC_ESPI_CZCEComMarginType = 'A'`。
    CZCEComMarginType,
    /// 大商所套利保证金是否优惠。CTP：`THOST_FTDC_ESPI_DceComMarginType = 'B'`。
    DceComMarginType,
    /// 虚值期权保证金优惠比率。CTP：`THOST_FTDC_ESPI_OptOutDisCountRate = 'a'`。
    OptOutDisCountRate,
    /// 最低保障系数。CTP：`THOST_FTDC_ESPI_OptMiniGuarantee = 'b'`。
    OptMiniGuarantee,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExchangeSettlementParamID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::MortgageRatio,
            50 => Self::OtherFundItem,
            51 => Self::OtherFundImport,
            54 => Self::CFFEXMinPrepay,
            55 => Self::CZCESettlementType,
            57 => Self::ExchangeDeliveryFeeMode,
            48 => Self::DeliveryFeeMode,
            65 => Self::CZCEComMarginType,
            66 => Self::DceComMarginType,
            97 => Self::OptOutDisCountRate,
            98 => Self::OptMiniGuarantee,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::MortgageRatio => Some(49),
            Self::OtherFundItem => Some(50),
            Self::OtherFundImport => Some(51),
            Self::CFFEXMinPrepay => Some(54),
            Self::CZCESettlementType => Some(55),
            Self::ExchangeDeliveryFeeMode => Some(57),
            Self::DeliveryFeeMode => Some(48),
            Self::CZCEComMarginType => Some(65),
            Self::DceComMarginType => Some(66),
            Self::OptOutDisCountRate => Some(97),
            Self::OptMiniGuarantee => Some(98),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExchangeSettlementParamID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExecOrderCloseFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecOrderCloseFlag {
    /// 自动平仓。CTP：`THOST_FTDC_EOCF_AutoClose = '0'`。
    AutoClose,
    /// 免于自动平仓。CTP：`THOST_FTDC_EOCF_NotToClose = '1'`。
    NotToClose,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExecOrderCloseFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::AutoClose,
            49 => Self::NotToClose,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::AutoClose => Some(48),
            Self::NotToClose => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExecOrderCloseFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExecOrderPositionFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecOrderPositionFlag {
    /// 保留。CTP：`THOST_FTDC_EOPF_Reserve = '0'`。
    Reserve,
    /// 不保留。CTP：`THOST_FTDC_EOPF_UnReserve = '1'`。
    UnReserve,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExecOrderPositionFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Reserve,
            49 => Self::UnReserve,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Reserve => Some(48),
            Self::UnReserve => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExecOrderPositionFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExecResultType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecResult {
    /// 没有执行。CTP：`THOST_FTDC_OER_NoExec = 'n'`。
    NoExec,
    /// 已经取消。CTP：`THOST_FTDC_OER_Canceled = 'c'`。
    Canceled,
    /// 执行成功。CTP：`THOST_FTDC_OER_OK = '0'`。
    OK,
    /// 期权持仓不够。CTP：`THOST_FTDC_OER_NoPosition = '1'`。
    NoPosition,
    /// 资金不够。CTP：`THOST_FTDC_OER_NoDeposit = '2'`。
    NoDeposit,
    /// 会员不存在。CTP：`THOST_FTDC_OER_NoParticipant = '3'`。
    NoParticipant,
    /// 客户不存在。CTP：`THOST_FTDC_OER_NoClient = '4'`。
    NoClient,
    /// 合约不存在。CTP：`THOST_FTDC_OER_NoInstrument = '6'`。
    NoInstrument,
    /// 没有执行权限。CTP：`THOST_FTDC_OER_NoRight = '7'`。
    NoRight,
    /// 不合理的数量。CTP：`THOST_FTDC_OER_InvalidVolume = '8'`。
    InvalidVolume,
    /// 没有足够的历史成交。CTP：`THOST_FTDC_OER_NoEnoughHistoryTrade = '9'`。
    NoEnoughHistoryTrade,
    /// 未知。CTP：`THOST_FTDC_OER_Unknown = 'a'`。
    Unknown,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExecResult {
    pub const fn from_code(code: u8) -> Self {
        match code {
            110 => Self::NoExec,
            99 => Self::Canceled,
            48 => Self::OK,
            49 => Self::NoPosition,
            50 => Self::NoDeposit,
            51 => Self::NoParticipant,
            52 => Self::NoClient,
            54 => Self::NoInstrument,
            55 => Self::NoRight,
            56 => Self::InvalidVolume,
            57 => Self::NoEnoughHistoryTrade,
            97 => Self::Unknown,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoExec => Some(110),
            Self::Canceled => Some(99),
            Self::OK => Some(48),
            Self::NoPosition => Some(49),
            Self::NoDeposit => Some(50),
            Self::NoParticipant => Some(51),
            Self::NoClient => Some(52),
            Self::NoInstrument => Some(54),
            Self::NoRight => Some(55),
            Self::InvalidVolume => Some(56),
            Self::NoEnoughHistoryTrade => Some(57),
            Self::Unknown => Some(97),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExecResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExportFileTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportFileType {
    /// CSV。CTP：`THOST_FTDC_EFT_CSV = '0'`。
    CSV,
    /// Excel。CTP：`THOST_FTDC_EFT_EXCEL = '1'`。
    EXCEL,
    /// DBF。CTP：`THOST_FTDC_EFT_DBF = '2'`。
    DBF,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExportFileType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::CSV,
            49 => Self::EXCEL,
            50 => Self::DBF,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CSV => Some(48),
            Self::EXCEL => Some(49),
            Self::DBF => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExportFileType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcExprSetModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExprSetMode {
    /// 相对已有规则设置。CTP：`THOST_FTDC_ESM_Relative = '1'`。
    Relative,
    /// 典型设置。CTP：`THOST_FTDC_ESM_Typical = '2'`。
    Typical,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ExprSetMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Relative,
            50 => Self::Typical,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Relative => Some(49),
            Self::Typical => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ExprSetMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBEAlreadyTradeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBEAlreadyTrade {
    /// 未交易。CTP：`THOST_FTDC_FBEAT_NotTrade = '0'`。
    NotTrade,
    /// 已交易。CTP：`THOST_FTDC_FBEAT_Trade = '1'`。
    Trade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBEAlreadyTrade {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NotTrade,
            49 => Self::Trade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotTrade => Some(48),
            Self::Trade => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBEAlreadyTrade {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBEExchStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBEExchStatus {
    /// 正常。CTP：`THOST_FTDC_FBEES_Normal = '0'`。
    Normal,
    /// 交易重发。CTP：`THOST_FTDC_FBEES_ReExchange = '1'`。
    ReExchange,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBEExchStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::ReExchange,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::ReExchange => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBEExchStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBEFileFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBEFileFlag {
    /// 数据包。CTP：`THOST_FTDC_FBEFG_DataPackage = '0'`。
    DataPackage,
    /// 文件。CTP：`THOST_FTDC_FBEFG_File = '1'`。
    File,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBEFileFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::DataPackage,
            49 => Self::File,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::DataPackage => Some(48),
            Self::File => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBEFileFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBEReqFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBEReqFlag {
    /// 未处理。CTP：`THOST_FTDC_FBERF_UnProcessed = '0'`。
    UnProcessed,
    /// 等待发送。CTP：`THOST_FTDC_FBERF_WaitSend = '1'`。
    WaitSend,
    /// 发送成功。CTP：`THOST_FTDC_FBERF_SendSuccess = '2'`。
    SendSuccess,
    /// 发送失败。CTP：`THOST_FTDC_FBERF_SendFailed = '3'`。
    SendFailed,
    /// 等待重发。CTP：`THOST_FTDC_FBERF_WaitReSend = '4'`。
    WaitReSend,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBEReqFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnProcessed,
            49 => Self::WaitSend,
            50 => Self::SendSuccess,
            51 => Self::SendFailed,
            52 => Self::WaitReSend,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnProcessed => Some(48),
            Self::WaitSend => Some(49),
            Self::SendSuccess => Some(50),
            Self::SendFailed => Some(51),
            Self::WaitReSend => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBEReqFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBEResultFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBEResultFlag {
    /// 成功。CTP：`THOST_FTDC_FBERES_Success = '0'`。
    Success,
    /// 账户余额不足。CTP：`THOST_FTDC_FBERES_InsufficientBalance = '1'`。
    InsufficientBalance,
    /// 交易结果未知。CTP：`THOST_FTDC_FBERES_UnknownTrading = '8'`。
    UnknownTrading,
    /// 失败。CTP：`THOST_FTDC_FBERES_Fail = 'x'`。
    Fail,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBEResultFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Success,
            49 => Self::InsufficientBalance,
            56 => Self::UnknownTrading,
            120 => Self::Fail,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Success => Some(48),
            Self::InsufficientBalance => Some(49),
            Self::UnknownTrading => Some(56),
            Self::Fail => Some(120),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBEResultFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBEUserEventTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBEUserEventType {
    /// 签到。CTP：`THOST_FTDC_FBEUET_SignIn = '0'`。
    SignIn,
    /// 换汇。CTP：`THOST_FTDC_FBEUET_Exchange = '1'`。
    Exchange,
    /// 换汇重发。CTP：`THOST_FTDC_FBEUET_ReExchange = '2'`。
    ReExchange,
    /// 银行账户查询。CTP：`THOST_FTDC_FBEUET_QueryBankAccount = '3'`。
    QueryBankAccount,
    /// 换汇明细查询。线值：`'4'`；名称已规范化，原始常量见审核表。
    QueryExchangeDetail,
    /// 换汇汇总查询。CTP：`THOST_FTDC_FBEUET_QueryExchSummary = '5'`。
    QueryExchSummary,
    /// 换汇汇率查询。CTP：`THOST_FTDC_FBEUET_QueryExchRate = '6'`。
    QueryExchRate,
    /// 对账文件通知。CTP：`THOST_FTDC_FBEUET_CheckBankAccount = '7'`。
    CheckBankAccount,
    /// 签退。CTP：`THOST_FTDC_FBEUET_SignOut = '8'`。
    SignOut,
    /// 其他。CTP：`THOST_FTDC_FBEUET_Other = 'Z'`。
    Other,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBEUserEventType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::SignIn,
            49 => Self::Exchange,
            50 => Self::ReExchange,
            51 => Self::QueryBankAccount,
            52 => Self::QueryExchangeDetail,
            53 => Self::QueryExchSummary,
            54 => Self::QueryExchRate,
            55 => Self::CheckBankAccount,
            56 => Self::SignOut,
            90 => Self::Other,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SignIn => Some(48),
            Self::Exchange => Some(49),
            Self::ReExchange => Some(50),
            Self::QueryBankAccount => Some(51),
            Self::QueryExchangeDetail => Some(52),
            Self::QueryExchSummary => Some(53),
            Self::QueryExchRate => Some(54),
            Self::CheckBankAccount => Some(55),
            Self::SignOut => Some(56),
            Self::Other => Some(90),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBEUserEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBTEncryModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBTEncryMode {
    /// 不加密。CTP：`THOST_FTDC_EM_NoEncry = '0'`。
    NoEncry,
    /// DES。CTP：`THOST_FTDC_EM_DES = '1'`。
    DES,
    /// 3DES。线值：`'2'`；名称已规范化，原始常量见审核表。
    TripleDes,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBTEncryMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoEncry,
            49 => Self::DES,
            50 => Self::TripleDes,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoEncry => Some(48),
            Self::DES => Some(49),
            Self::TripleDes => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBTEncryMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBTPassWordTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBTPassWordType {
    /// 查询。CTP：`THOST_FTDC_PWT_Query = '0'`。
    Query,
    /// 取款。CTP：`THOST_FTDC_PWT_Fetch = '1'`。
    Fetch,
    /// 转帐。CTP：`THOST_FTDC_PWT_Transfer = '2'`。
    Transfer,
    /// 交易。CTP：`THOST_FTDC_PWT_Trade = '3'`。
    Trade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBTPassWordType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Query,
            49 => Self::Fetch,
            50 => Self::Transfer,
            51 => Self::Trade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Query => Some(48),
            Self::Fetch => Some(49),
            Self::Transfer => Some(50),
            Self::Trade => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBTPassWordType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBTTransferDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBTTransferDirection {
    /// 入金，银行转期货。CTP：`THOST_FTDC_FBTTD_FromBankToFuture = '1'`。
    FromBankToFuture,
    /// 出金，期货转银行。CTP：`THOST_FTDC_FBTTD_FromFutureToBank = '2'`。
    FromFutureToBank,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBTTransferDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::FromBankToFuture,
            50 => Self::FromFutureToBank,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FromBankToFuture => Some(49),
            Self::FromFutureToBank => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBTTransferDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFBTUserEventTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FBTUserEventType {
    /// 签到。CTP：`THOST_FTDC_FBTUET_SignIn = '0'`。
    SignIn,
    /// 银行转期货。CTP：`THOST_FTDC_FBTUET_FromBankToFuture = '1'`。
    FromBankToFuture,
    /// 期货转银行。CTP：`THOST_FTDC_FBTUET_FromFutureToBank = '2'`。
    FromFutureToBank,
    /// 开户。CTP：`THOST_FTDC_FBTUET_OpenAccount = '3'`。
    OpenAccount,
    /// 销户。CTP：`THOST_FTDC_FBTUET_CancelAccount = '4'`。
    CancelAccount,
    /// 变更银行账户。CTP：`THOST_FTDC_FBTUET_ChangeAccount = '5'`。
    ChangeAccount,
    /// 冲正银行转期货。CTP：`THOST_FTDC_FBTUET_RepealFromBankToFuture = '6'`。
    RepealFromBankToFuture,
    /// 冲正期货转银行。CTP：`THOST_FTDC_FBTUET_RepealFromFutureToBank = '7'`。
    RepealFromFutureToBank,
    /// 查询银行账户。CTP：`THOST_FTDC_FBTUET_QueryBankAccount = '8'`。
    QueryBankAccount,
    /// 查询期货账户。CTP：`THOST_FTDC_FBTUET_QueryFutureAccount = '9'`。
    QueryFutureAccount,
    /// 签退。CTP：`THOST_FTDC_FBTUET_SignOut = 'A'`。
    SignOut,
    /// 密钥同步。CTP：`THOST_FTDC_FBTUET_SyncKey = 'B'`。
    SyncKey,
    /// 预约开户。CTP：`THOST_FTDC_FBTUET_ReserveOpenAccount = 'C'`。
    ReserveOpenAccount,
    /// 撤销预约开户。CTP：`THOST_FTDC_FBTUET_CancelReserveOpenAccount = 'D'`。
    CancelReserveOpenAccount,
    /// 预约开户确认。CTP：`THOST_FTDC_FBTUET_ReserveOpenAccountConfirm = 'E'`。
    ReserveOpenAccountConfirm,
    /// 其他。CTP：`THOST_FTDC_FBTUET_Other = 'Z'`。
    Other,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FBTUserEventType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::SignIn,
            49 => Self::FromBankToFuture,
            50 => Self::FromFutureToBank,
            51 => Self::OpenAccount,
            52 => Self::CancelAccount,
            53 => Self::ChangeAccount,
            54 => Self::RepealFromBankToFuture,
            55 => Self::RepealFromFutureToBank,
            56 => Self::QueryBankAccount,
            57 => Self::QueryFutureAccount,
            65 => Self::SignOut,
            66 => Self::SyncKey,
            67 => Self::ReserveOpenAccount,
            68 => Self::CancelReserveOpenAccount,
            69 => Self::ReserveOpenAccountConfirm,
            90 => Self::Other,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SignIn => Some(48),
            Self::FromBankToFuture => Some(49),
            Self::FromFutureToBank => Some(50),
            Self::OpenAccount => Some(51),
            Self::CancelAccount => Some(52),
            Self::ChangeAccount => Some(53),
            Self::RepealFromBankToFuture => Some(54),
            Self::RepealFromFutureToBank => Some(55),
            Self::QueryBankAccount => Some(56),
            Self::QueryFutureAccount => Some(57),
            Self::SignOut => Some(65),
            Self::SyncKey => Some(66),
            Self::ReserveOpenAccount => Some(67),
            Self::CancelReserveOpenAccount => Some(68),
            Self::ReserveOpenAccountConfirm => Some(69),
            Self::Other => Some(90),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FBTUserEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFeeAcceptStyleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeeAcceptStyle {
    /// 按交易收取。CTP：`THOST_FTDC_FAS_ByTrade = '1'`。
    ByTrade,
    /// 按交割收取。线值：`'2'`；名称已规范化，原始常量见审核表。
    ByDelivery,
    /// 不收。CTP：`THOST_FTDC_FAS_None = '3'`。
    None,
    /// 按指定手续费收取。CTP：`THOST_FTDC_FAS_FixFee = '4'`。
    FixFee,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FeeAcceptStyle {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ByTrade,
            50 => Self::ByDelivery,
            51 => Self::None,
            52 => Self::FixFee,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByTrade => Some(49),
            Self::ByDelivery => Some(50),
            Self::None => Some(51),
            Self::FixFee => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FeeAcceptStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFeePayFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeePayFlag {
    /// 由受益方支付费用。CTP：`THOST_FTDC_FPF_BEN = '0'`。
    BEN,
    /// 由发送方支付费用。CTP：`THOST_FTDC_FPF_OUR = '1'`。
    OUR,
    /// 由发送方支付发起的费用，受益方支付接受的费用。CTP：`THOST_FTDC_FPF_SHA = '2'`。
    SHA,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FeePayFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::BEN,
            49 => Self::OUR,
            50 => Self::SHA,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BEN => Some(48),
            Self::OUR => Some(49),
            Self::SHA => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FeePayFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileBusinessCodeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileBusinessCode {
    /// 其他。CTP：`THOST_FTDC_FBC_Others = '0'`。
    Others,
    /// 转账交易明细对账。CTP：`THOST_FTDC_FBC_TransferDetails = '1'`。
    TransferDetails,
    /// 客户账户状态对账。CTP：`THOST_FTDC_FBC_CustAccStatus = '2'`。
    CustAccStatus,
    /// 账户类交易明细对账。CTP：`THOST_FTDC_FBC_AccountTradeDetails = '3'`。
    AccountTradeDetails,
    /// 期货账户信息变更明细对账。CTP：`THOST_FTDC_FBC_FutureAccountChangeInfoDetails = '4'`。
    FutureAccountChangeInfoDetails,
    /// 客户资金台账余额明细对账。CTP：`THOST_FTDC_FBC_CustMoneyDetail = '5'`。
    CustMoneyDetail,
    /// 客户销户结息明细对账。CTP：`THOST_FTDC_FBC_CustCancelAccountInfo = '6'`。
    CustCancelAccountInfo,
    /// 客户资金余额对账结果。CTP：`THOST_FTDC_FBC_CustMoneyResult = '7'`。
    CustMoneyResult,
    /// 其它对账异常结果文件。CTP：`THOST_FTDC_FBC_OthersExceptionResult = '8'`。
    OthersExceptionResult,
    /// 客户结息净额明细。CTP：`THOST_FTDC_FBC_CustInterestNetMoneyDetails = '9'`。
    CustInterestNetMoneyDetails,
    /// 客户资金交收明细。CTP：`THOST_FTDC_FBC_CustMoneySendAndReceiveDetails = 'a'`。
    CustMoneySendAndReceiveDetails,
    /// 法人存管银行资金交收汇总。CTP：`THOST_FTDC_FBC_CorporationMoneyTotal = 'b'`。
    CorporationMoneyTotal,
    /// 主体间资金交收汇总。CTP：`THOST_FTDC_FBC_MainbodyMoneyTotal = 'c'`。
    MainbodyMoneyTotal,
    /// 总分平衡监管数据。CTP：`THOST_FTDC_FBC_MainPartMonitorData = 'd'`。
    MainPartMonitorData,
    /// 存管银行备付金余额。CTP：`THOST_FTDC_FBC_PreparationMoney = 'e'`。
    PreparationMoney,
    /// 协办存管银行资金监管数据。CTP：`THOST_FTDC_FBC_BankMoneyMonitorData = 'f'`。
    BankMoneyMonitorData,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileBusinessCode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Others,
            49 => Self::TransferDetails,
            50 => Self::CustAccStatus,
            51 => Self::AccountTradeDetails,
            52 => Self::FutureAccountChangeInfoDetails,
            53 => Self::CustMoneyDetail,
            54 => Self::CustCancelAccountInfo,
            55 => Self::CustMoneyResult,
            56 => Self::OthersExceptionResult,
            57 => Self::CustInterestNetMoneyDetails,
            97 => Self::CustMoneySendAndReceiveDetails,
            98 => Self::CorporationMoneyTotal,
            99 => Self::MainbodyMoneyTotal,
            100 => Self::MainPartMonitorData,
            101 => Self::PreparationMoney,
            102 => Self::BankMoneyMonitorData,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Others => Some(48),
            Self::TransferDetails => Some(49),
            Self::CustAccStatus => Some(50),
            Self::AccountTradeDetails => Some(51),
            Self::FutureAccountChangeInfoDetails => Some(52),
            Self::CustMoneyDetail => Some(53),
            Self::CustCancelAccountInfo => Some(54),
            Self::CustMoneyResult => Some(55),
            Self::OthersExceptionResult => Some(56),
            Self::CustInterestNetMoneyDetails => Some(57),
            Self::CustMoneySendAndReceiveDetails => Some(97),
            Self::CorporationMoneyTotal => Some(98),
            Self::MainbodyMoneyTotal => Some(99),
            Self::MainPartMonitorData => Some(100),
            Self::PreparationMoney => Some(101),
            Self::BankMoneyMonitorData => Some(102),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileBusinessCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileFormatType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileFormat {
    /// 文本文件(.txt)。CTP：`THOST_FTDC_FFT_Txt = '0'`。
    Txt,
    /// 压缩文件(.zip)。CTP：`THOST_FTDC_FFT_Zip = '1'`。
    Zip,
    /// DBF文件(.dbf)。CTP：`THOST_FTDC_FFT_DBF = '2'`。
    DBF,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileFormat {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Txt,
            49 => Self::Zip,
            50 => Self::DBF,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Txt => Some(48),
            Self::Zip => Some(49),
            Self::DBF => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileGenStyleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileGenStyle {
    /// 下发。CTP：`THOST_FTDC_FGS_FileTransmit = '0'`。
    FileTransmit,
    /// 生成。CTP：`THOST_FTDC_FGS_FileGen = '1'`。
    FileGen,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileGenStyle {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::FileTransmit,
            49 => Self::FileGen,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FileTransmit => Some(48),
            Self::FileGen => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileGenStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileID {
    /// 资金数据。CTP：`THOST_FTDC_FI_SettlementFund = 'F'`。
    SettlementFund,
    /// 成交数据。CTP：`THOST_FTDC_FI_Trade = 'T'`。
    Trade,
    /// 投资者持仓数据。CTP：`THOST_FTDC_FI_InvestorPosition = 'P'`。
    InvestorPosition,
    /// 投资者分项资金数据。CTP：`THOST_FTDC_FI_SubEntryFund = 'O'`。
    SubEntryFund,
    /// 组合持仓数据。CTP：`THOST_FTDC_FI_CZCECombinationPos = 'C'`。
    CZCECombinationPos,
    /// 上报保证金监控中心数据。CTP：`THOST_FTDC_FI_CSRCData = 'R'`。
    CSRCData,
    /// 郑商所平仓了结数据。CTP：`THOST_FTDC_FI_CZCEClose = 'L'`。
    CZCEClose,
    /// 郑商所非平仓了结数据。CTP：`THOST_FTDC_FI_CZCENoClose = 'N'`。
    CZCENoClose,
    /// 持仓明细数据。CTP：`THOST_FTDC_FI_PositionDtl = 'D'`。
    PositionDtl,
    /// 期权执行文件。CTP：`THOST_FTDC_FI_OptionStrike = 'S'`。
    OptionStrike,
    /// 结算价比对文件。CTP：`THOST_FTDC_FI_SettlementPriceComparison = 'M'`。
    SettlementPriceComparison,
    /// 上期所非持仓变动明细。CTP：`THOST_FTDC_FI_NonTradePosChange = 'B'`。
    NonTradePosChange,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            70 => Self::SettlementFund,
            84 => Self::Trade,
            80 => Self::InvestorPosition,
            79 => Self::SubEntryFund,
            67 => Self::CZCECombinationPos,
            82 => Self::CSRCData,
            76 => Self::CZCEClose,
            78 => Self::CZCENoClose,
            68 => Self::PositionDtl,
            83 => Self::OptionStrike,
            77 => Self::SettlementPriceComparison,
            66 => Self::NonTradePosChange,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SettlementFund => Some(70),
            Self::Trade => Some(84),
            Self::InvestorPosition => Some(80),
            Self::SubEntryFund => Some(79),
            Self::CZCECombinationPos => Some(67),
            Self::CSRCData => Some(82),
            Self::CZCEClose => Some(76),
            Self::CZCENoClose => Some(78),
            Self::PositionDtl => Some(68),
            Self::OptionStrike => Some(83),
            Self::SettlementPriceComparison => Some(77),
            Self::NonTradePosChange => Some(66),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileStatus {
    /// 未生成。CTP：`THOST_FTDC_FIS_NoCreate = '0'`。
    NoCreate,
    /// 已生成。CTP：`THOST_FTDC_FIS_Created = '1'`。
    Created,
    /// 生成失败。CTP：`THOST_FTDC_FIS_Failed = '2'`。
    Failed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoCreate,
            49 => Self::Created,
            50 => Self::Failed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoCreate => Some(48),
            Self::Created => Some(49),
            Self::Failed => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    /// 结算。CTP：`THOST_FTDC_FUT_Settlement = '0'`。
    Settlement,
    /// 核对。CTP：`THOST_FTDC_FUT_Check = '1'`。
    Check,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Settlement,
            49 => Self::Check,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Settlement => Some(48),
            Self::Check => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFileUploadStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileUploadStatus {
    /// 上传成功。CTP：`THOST_FTDC_FUS_SucceedUpload = '1'`。
    SucceedUpload,
    /// 上传失败。CTP：`THOST_FTDC_FUS_FailedUpload = '2'`。
    FailedUpload,
    /// 导入成功。CTP：`THOST_FTDC_FUS_SucceedLoad = '3'`。
    SucceedLoad,
    /// 导入部分成功。CTP：`THOST_FTDC_FUS_PartSucceedLoad = '4'`。
    PartSucceedLoad,
    /// 导入失败。CTP：`THOST_FTDC_FUS_FailedLoad = '5'`。
    FailedLoad,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FileUploadStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::SucceedUpload,
            50 => Self::FailedUpload,
            51 => Self::SucceedLoad,
            52 => Self::PartSucceedLoad,
            53 => Self::FailedLoad,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SucceedUpload => Some(49),
            Self::FailedUpload => Some(50),
            Self::SucceedLoad => Some(51),
            Self::PartSucceedLoad => Some(52),
            Self::FailedLoad => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FileUploadStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFindMarginRateAlgoIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FindMarginRateAlgoID {
    /// 基本。CTP：`THOST_FTDC_FMRA_Base = '1'`。
    Base,
    /// 大连商品交易所。CTP：`THOST_FTDC_FMRA_DCE = '2'`。
    DCE,
    /// 郑州商品交易所。CTP：`THOST_FTDC_FMRA_CZCE = '3'`。
    CZCE,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FindMarginRateAlgoID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Base,
            50 => Self::DCE,
            51 => Self::CZCE,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Base => Some(49),
            Self::DCE => Some(50),
            Self::CZCE => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FindMarginRateAlgoID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFlexStatModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexStatMode {
    /// 产品统计。CTP：`THOST_FTDC_FSM_Product = '1'`。
    Product,
    /// 交易所统计。CTP：`THOST_FTDC_FSM_Exchange = '2'`。
    Exchange,
    /// 统计所有。CTP：`THOST_FTDC_FSM_All = '3'`。
    All,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FlexStatMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Product,
            50 => Self::Exchange,
            51 => Self::All,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Product => Some(49),
            Self::Exchange => Some(50),
            Self::All => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FlexStatMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFlowIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowID {
    /// 投资者对应投资者组设置。CTP：`THOST_FTDC_EvM_InvestorGroupFlow = '1'`。
    InvestorGroupFlow,
    /// 投资者手续费率设置。CTP：`THOST_FTDC_EvM_InvestorRate = '2'`。
    InvestorRate,
    /// 投资者手续费率模板关系设置。CTP：`THOST_FTDC_EvM_InvestorCommRateModel = '3'`。
    InvestorCommRateModel,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FlowID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::InvestorGroupFlow,
            50 => Self::InvestorRate,
            51 => Self::InvestorCommRateModel,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InvestorGroupFlow => Some(49),
            Self::InvestorRate => Some(50),
            Self::InvestorCommRateModel => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FlowID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcForQuoteStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForQuoteStatus {
    /// 已经提交。CTP：`THOST_FTDC_FQST_Submitted = 'a'`。
    Submitted,
    /// 已经接受。CTP：`THOST_FTDC_FQST_Accepted = 'b'`。
    Accepted,
    /// 已经被拒绝。CTP：`THOST_FTDC_FQST_Rejected = 'c'`。
    Rejected,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ForQuoteStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            97 => Self::Submitted,
            98 => Self::Accepted,
            99 => Self::Rejected,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Submitted => Some(97),
            Self::Accepted => Some(98),
            Self::Rejected => Some(99),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ForQuoteStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcForceCloseReasonType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForceCloseReason {
    /// 非强平。CTP：`THOST_FTDC_FCC_NotForceClose = '0'`。
    NotForceClose,
    /// 资金不足。CTP：`THOST_FTDC_FCC_LackDeposit = '1'`。
    LackDeposit,
    /// 客户超仓。CTP：`THOST_FTDC_FCC_ClientOverPositionLimit = '2'`。
    ClientOverPositionLimit,
    /// 会员超仓。CTP：`THOST_FTDC_FCC_MemberOverPositionLimit = '3'`。
    MemberOverPositionLimit,
    /// 持仓非整数倍。CTP：`THOST_FTDC_FCC_NotMultiple = '4'`。
    NotMultiple,
    /// 违规。CTP：`THOST_FTDC_FCC_Violation = '5'`。
    Violation,
    /// 其它。CTP：`THOST_FTDC_FCC_Other = '6'`。
    Other,
    /// 自然人临近交割。线值：`'7'`；名称已规范化，原始常量见审核表。
    PersonDelivery,
    /// 本地强平资金不足忽略敞口。线值：`'8'`；名称已规范化，原始常量见审核表。
    NotVerifyCapital,
    /// 本地强平资金不足。CTP：`THOST_FTDC_FCC_LocalLackDeposit = '9'`。
    LocalLackDeposit,
    /// 本地强平违规持仓忽略敞口。CTP：`THOST_FTDC_FCC_LocalViolationNocheck = 'a'`。
    LocalViolationNocheck,
    /// 本地强平违规持仓。CTP：`THOST_FTDC_FCC_LocalViolation = 'b'`。
    LocalViolation,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ForceCloseReason {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NotForceClose,
            49 => Self::LackDeposit,
            50 => Self::ClientOverPositionLimit,
            51 => Self::MemberOverPositionLimit,
            52 => Self::NotMultiple,
            53 => Self::Violation,
            54 => Self::Other,
            55 => Self::PersonDelivery,
            56 => Self::NotVerifyCapital,
            57 => Self::LocalLackDeposit,
            97 => Self::LocalViolationNocheck,
            98 => Self::LocalViolation,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotForceClose => Some(48),
            Self::LackDeposit => Some(49),
            Self::ClientOverPositionLimit => Some(50),
            Self::MemberOverPositionLimit => Some(51),
            Self::NotMultiple => Some(52),
            Self::Violation => Some(53),
            Self::Other => Some(54),
            Self::PersonDelivery => Some(55),
            Self::NotVerifyCapital => Some(56),
            Self::LocalLackDeposit => Some(57),
            Self::LocalViolationNocheck => Some(97),
            Self::LocalViolation => Some(98),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ForceCloseReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcForceCloseTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForceCloseType {
    /// 手工强平。CTP：`THOST_FTDC_FCT_Manual = '0'`。
    Manual,
    /// 单一投资者辅助强平。CTP：`THOST_FTDC_FCT_Single = '1'`。
    Single,
    /// 批量投资者辅助强平。CTP：`THOST_FTDC_FCT_Group = '2'`。
    Group,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ForceCloseType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Manual,
            49 => Self::Single,
            50 => Self::Group,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Manual => Some(48),
            Self::Single => Some(49),
            Self::Group => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ForceCloseType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFreezeStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FreezeStatus {
    /// 活跃。CTP：`THOST_FTDC_FRS_Normal = '1'`。
    Normal,
    /// 休眠。CTP：`THOST_FTDC_FRS_Freeze = '0'`。
    Freeze,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FreezeStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Normal,
            48 => Self::Freeze,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(49),
            Self::Freeze => Some(48),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FreezeStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFunctionCodeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionCode {
    /// 数据异步化。CTP：`THOST_FTDC_FC_DataAsync = '1'`。
    DataAsync,
    /// 强制用户登出。CTP：`THOST_FTDC_FC_ForceUserLogout = '2'`。
    ForceUserLogout,
    /// 变更管理用户口令。CTP：`THOST_FTDC_FC_UserPasswordUpdate = '3'`。
    UserPasswordUpdate,
    /// 变更经纪公司口令。CTP：`THOST_FTDC_FC_BrokerPasswordUpdate = '4'`。
    BrokerPasswordUpdate,
    /// 变更投资者口令。CTP：`THOST_FTDC_FC_InvestorPasswordUpdate = '5'`。
    InvestorPasswordUpdate,
    /// 报单插入。CTP：`THOST_FTDC_FC_OrderInsert = '6'`。
    OrderInsert,
    /// 报单操作。CTP：`THOST_FTDC_FC_OrderAction = '7'`。
    OrderAction,
    /// 同步系统数据。CTP：`THOST_FTDC_FC_SyncSystemData = '8'`。
    SyncSystemData,
    /// 同步经纪公司数据。CTP：`THOST_FTDC_FC_SyncBrokerData = '9'`。
    SyncBrokerData,
    /// 批量同步经纪公司数据。线值：`'A'`；名称已规范化，原始常量见审核表。
    BatchSyncBrokerData,
    /// 超级查询。CTP：`THOST_FTDC_FC_SuperQuery = 'B'`。
    SuperQuery,
    /// 预埋报单插入。CTP：`THOST_FTDC_FC_ParkedOrderInsert = 'C'`。
    ParkedOrderInsert,
    /// 预埋报单操作。CTP：`THOST_FTDC_FC_ParkedOrderAction = 'D'`。
    ParkedOrderAction,
    /// 同步动态令牌。CTP：`THOST_FTDC_FC_SyncOTP = 'E'`。
    SyncOTP,
    /// 删除未知单。CTP：`THOST_FTDC_FC_DeleteOrder = 'F'`。
    DeleteOrder,
    /// 退出紧急状态。CTP：`THOST_FTDC_FC_ExitEmergency = 'G'`。
    ExitEmergency,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FunctionCode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::DataAsync,
            50 => Self::ForceUserLogout,
            51 => Self::UserPasswordUpdate,
            52 => Self::BrokerPasswordUpdate,
            53 => Self::InvestorPasswordUpdate,
            54 => Self::OrderInsert,
            55 => Self::OrderAction,
            56 => Self::SyncSystemData,
            57 => Self::SyncBrokerData,
            65 => Self::BatchSyncBrokerData,
            66 => Self::SuperQuery,
            67 => Self::ParkedOrderInsert,
            68 => Self::ParkedOrderAction,
            69 => Self::SyncOTP,
            70 => Self::DeleteOrder,
            71 => Self::ExitEmergency,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::DataAsync => Some(49),
            Self::ForceUserLogout => Some(50),
            Self::UserPasswordUpdate => Some(51),
            Self::BrokerPasswordUpdate => Some(52),
            Self::InvestorPasswordUpdate => Some(53),
            Self::OrderInsert => Some(54),
            Self::OrderAction => Some(55),
            Self::SyncSystemData => Some(56),
            Self::SyncBrokerData => Some(57),
            Self::BatchSyncBrokerData => Some(65),
            Self::SuperQuery => Some(66),
            Self::ParkedOrderInsert => Some(67),
            Self::ParkedOrderAction => Some(68),
            Self::SyncOTP => Some(69),
            Self::DeleteOrder => Some(70),
            Self::ExitEmergency => Some(71),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FunctionCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundDirection {
    /// 入金。线值：`'1'`；名称已规范化，原始常量见审核表。
    InValue,
    /// 出金。CTP：`THOST_FTDC_FD_Out = '2'`。
    Out,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::InValue,
            50 => Self::Out,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InValue => Some(49),
            Self::Out => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundDirectionEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundDirectionEn {
    /// Deposit。线值：`'1'`；名称已规范化，原始常量见审核表。
    InValue,
    /// Withdrawal。CTP：`THOST_FTDC_FDEN_Out = '2'`。
    Out,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundDirectionEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::InValue,
            50 => Self::Out,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InValue => Some(49),
            Self::Out => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundDirectionEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundEventTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundEventType {
    /// 转账限额。CTP：`THOST_FTDC_FET_Restriction = '0'`。
    Restriction,
    /// 当日转账限额。CTP：`THOST_FTDC_FET_TodayRestriction = '1'`。
    TodayRestriction,
    /// 期商流水。CTP：`THOST_FTDC_FET_Transfer = '2'`。
    Transfer,
    /// 资金冻结。CTP：`THOST_FTDC_FET_Credit = '3'`。
    Credit,
    /// 投资者可提资金比例。CTP：`THOST_FTDC_FET_InvestorWithdrawAlm = '4'`。
    InvestorWithdrawAlm,
    /// 单个银行帐户转账限额。CTP：`THOST_FTDC_FET_BankRestriction = '5'`。
    BankRestriction,
    /// 银期签约账户。线值：`'6'`；名称已规范化，原始常量见审核表。
    AccountRegister,
    /// 交易所出入金。CTP：`THOST_FTDC_FET_ExchangeFundIO = '7'`。
    ExchangeFundIO,
    /// 投资者出入金。CTP：`THOST_FTDC_FET_InvestorFundIO = '8'`。
    InvestorFundIO,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundEventType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Restriction,
            49 => Self::TodayRestriction,
            50 => Self::Transfer,
            51 => Self::Credit,
            52 => Self::InvestorWithdrawAlm,
            53 => Self::BankRestriction,
            54 => Self::AccountRegister,
            55 => Self::ExchangeFundIO,
            56 => Self::InvestorFundIO,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Restriction => Some(48),
            Self::TodayRestriction => Some(49),
            Self::Transfer => Some(50),
            Self::Credit => Some(51),
            Self::InvestorWithdrawAlm => Some(52),
            Self::BankRestriction => Some(53),
            Self::AccountRegister => Some(54),
            Self::ExchangeFundIO => Some(55),
            Self::InvestorFundIO => Some(56),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundIOTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundIOType {
    /// 出入金。CTP：`THOST_FTDC_FIOT_FundIO = '1'`。
    FundIO,
    /// 银期转帐。CTP：`THOST_FTDC_FIOT_Transfer = '2'`。
    Transfer,
    /// 银期换汇。CTP：`THOST_FTDC_FIOT_SwapCurrency = '3'`。
    SwapCurrency,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundIOType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::FundIO,
            50 => Self::Transfer,
            51 => Self::SwapCurrency,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FundIO => Some(49),
            Self::Transfer => Some(50),
            Self::SwapCurrency => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundIOType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundIOTypeEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundIOTypeEn {
    /// Deposit/Withdrawal。CTP：`THOST_FTDC_FIOTEN_FundIO = '1'`。
    FundIO,
    /// Bank-Futures Transfer。CTP：`THOST_FTDC_FIOTEN_Transfer = '2'`。
    Transfer,
    /// Bank-Futures FX Exchange。CTP：`THOST_FTDC_FIOTEN_SwapCurrency = '3'`。
    SwapCurrency,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundIOTypeEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::FundIO,
            50 => Self::Transfer,
            51 => Self::SwapCurrency,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FundIO => Some(49),
            Self::Transfer => Some(50),
            Self::SwapCurrency => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundIOTypeEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundMortDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundMortDirection {
    /// 货币质入。线值：`'1'`；名称已规范化，原始常量见审核表。
    InValue,
    /// 货币质出。CTP：`THOST_FTDC_FMD_Out = '2'`。
    Out,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundMortDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::InValue,
            50 => Self::Out,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InValue => Some(49),
            Self::Out => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundMortDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundMortDirectionEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundMortDirectionEn {
    /// Pledge。线值：`'1'`；名称已规范化，原始常量见审核表。
    InValue,
    /// Redemption。CTP：`THOST_FTDC_FMDEN_Out = '2'`。
    Out,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundMortDirectionEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::InValue,
            50 => Self::Out,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InValue => Some(49),
            Self::Out => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundMortDirectionEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundMortgageTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundMortgageType {
    /// 质押。CTP：`THOST_FTDC_FMT_Mortgage = '1'`。
    Mortgage,
    /// 解质。CTP：`THOST_FTDC_FMT_Redemption = '2'`。
    Redemption,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundMortgageType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Mortgage,
            50 => Self::Redemption,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Mortgage => Some(49),
            Self::Redemption => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundMortgageType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundStatus {
    /// 已录入。CTP：`THOST_FTDC_FS_Record = '1'`。
    Record,
    /// 已复核。CTP：`THOST_FTDC_FS_Check = '2'`。
    Check,
    /// 已冲销。CTP：`THOST_FTDC_FS_Charge = '3'`。
    Charge,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Record,
            50 => Self::Check,
            51 => Self::Charge,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Record => Some(49),
            Self::Check => Some(50),
            Self::Charge => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundType {
    /// 银行存款。线值：`'1'`；名称已规范化，原始常量见审核表。
    Deposit,
    /// 分项资金。CTP：`THOST_FTDC_FT_ItemFund = '2'`。
    ItemFund,
    /// 公司调整。CTP：`THOST_FTDC_FT_Company = '3'`。
    Company,
    /// 资金内转。CTP：`THOST_FTDC_FT_InnerTransfer = '4'`。
    InnerTransfer,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Deposit,
            50 => Self::ItemFund,
            51 => Self::Company,
            52 => Self::InnerTransfer,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Deposit => Some(49),
            Self::ItemFund => Some(50),
            Self::Company => Some(51),
            Self::InnerTransfer => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFundTypeEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FundTypeEn {
    /// Bank Deposit。线值：`'1'`；名称已规范化，原始常量见审核表。
    Deposit,
    /// Payment/Fee。CTP：`THOST_FTDC_FTEN_ItemFund = '2'`。
    ItemFund,
    /// Brokerage Adj。CTP：`THOST_FTDC_FTEN_Company = '3'`。
    Company,
    /// Internal Transfer。CTP：`THOST_FTDC_FTEN_InnerTransfer = '4'`。
    InnerTransfer,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FundTypeEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Deposit,
            50 => Self::ItemFund,
            51 => Self::Company,
            52 => Self::InnerTransfer,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Deposit => Some(49),
            Self::ItemFund => Some(50),
            Self::Company => Some(51),
            Self::InnerTransfer => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FundTypeEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFutureAccTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FutureAccType {
    /// 银行存折。CTP：`THOST_FTDC_FAT_BankBook = '1'`。
    BankBook,
    /// 储蓄卡。CTP：`THOST_FTDC_FAT_SavingCard = '2'`。
    SavingCard,
    /// 信用卡。CTP：`THOST_FTDC_FAT_CreditCard = '3'`。
    CreditCard,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FutureAccType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::BankBook,
            50 => Self::SavingCard,
            51 => Self::CreditCard,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BankBook => Some(49),
            Self::SavingCard => Some(50),
            Self::CreditCard => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FutureAccType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFuturePwdFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FuturePwdFlag {
    /// 不核对。CTP：`THOST_FTDC_FPWD_UnCheck = '0'`。
    UnCheck,
    /// 核对。CTP：`THOST_FTDC_FPWD_Check = '1'`。
    Check,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FuturePwdFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnCheck,
            49 => Self::Check,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnCheck => Some(48),
            Self::Check => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FuturePwdFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcFutureTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FutureType {
    /// 商品期货。CTP：`THOST_FTDC_FUTT_Commodity = '1'`。
    Commodity,
    /// 金融期货。CTP：`THOST_FTDC_FUTT_Financial = '2'`。
    Financial,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl FutureType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Commodity,
            50 => Self::Financial,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Commodity => Some(49),
            Self::Financial => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for FutureType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcGenderType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    /// 未知状态。CTP：`THOST_FTDC_GD_Unknown = '0'`。
    Unknown,
    /// 男。CTP：`THOST_FTDC_GD_Male = '1'`。
    Male,
    /// 女。CTP：`THOST_FTDC_GD_Female = '2'`。
    Female,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl Gender {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Unknown,
            49 => Self::Male,
            50 => Self::Female,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Unknown => Some(48),
            Self::Male => Some(49),
            Self::Female => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcGiveUpDataSourceType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GiveUpDataSource {
    /// 系统生成。线值：`'0'`；名称已规范化，原始常量见审核表。
    GenValue,
    /// 手工添加。CTP：`THOST_FTDC_GUDS_Hand = '1'`。
    Hand,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl GiveUpDataSource {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::GenValue,
            49 => Self::Hand,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::GenValue => Some(48),
            Self::Hand => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for GiveUpDataSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcHandlePositionAlgoIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandlePositionAlgoID {
    /// 基本。CTP：`THOST_FTDC_HPA_Base = '1'`。
    Base,
    /// 大连商品交易所。CTP：`THOST_FTDC_HPA_DCE = '2'`。
    DCE,
    /// 郑州商品交易所。CTP：`THOST_FTDC_HPA_CZCE = '3'`。
    CZCE,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl HandlePositionAlgoID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Base,
            50 => Self::DCE,
            51 => Self::CZCE,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Base => Some(49),
            Self::DCE => Some(50),
            Self::CZCE => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for HandlePositionAlgoID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcHandleTradingAccountAlgoIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandleTradingAccountAlgoID {
    /// 基本。CTP：`THOST_FTDC_HTAA_Base = '1'`。
    Base,
    /// 大连商品交易所。CTP：`THOST_FTDC_HTAA_DCE = '2'`。
    DCE,
    /// 郑州商品交易所。CTP：`THOST_FTDC_HTAA_CZCE = '3'`。
    CZCE,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl HandleTradingAccountAlgoID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Base,
            50 => Self::DCE,
            51 => Self::CZCE,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Base => Some(49),
            Self::DCE => Some(50),
            Self::CZCE => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for HandleTradingAccountAlgoID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcHasBoardType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HasBoard {
    /// 没有。CTP：`THOST_FTDC_HB_No = '0'`。
    No,
    /// 有。CTP：`THOST_FTDC_HB_Yes = '1'`。
    Yes,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl HasBoard {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::No,
            49 => Self::Yes,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::No => Some(48),
            Self::Yes => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for HasBoard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcHasTrusteeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HasTrustee {
    /// 有。CTP：`THOST_FTDC_HT_Yes = '1'`。
    Yes,
    /// 没有。CTP：`THOST_FTDC_HT_No = '0'`。
    No,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl HasTrustee {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Yes,
            48 => Self::No,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Yes => Some(49),
            Self::No => Some(48),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for HasTrustee {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcHedgeFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HedgeFlag {
    /// 投机。CTP：`THOST_FTDC_HF_Speculation = '1'`。
    Speculation,
    /// 套利。CTP：`THOST_FTDC_HF_Arbitrage = '2'`。
    Arbitrage,
    /// 套保。CTP：`THOST_FTDC_HF_Hedge = '3'`。
    Hedge,
    /// 做市商。CTP：`THOST_FTDC_HF_MarketMaker = '5'`。
    MarketMaker,
    /// 第一腿投机第二腿套保。CTP：`THOST_FTDC_HF_SpecHedge = '6'`。
    SpecHedge,
    /// 第一腿套保第二腿投机。CTP：`THOST_FTDC_HF_HedgeSpec = '7'`。
    HedgeSpec,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl HedgeFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Speculation,
            50 => Self::Arbitrage,
            51 => Self::Hedge,
            53 => Self::MarketMaker,
            54 => Self::SpecHedge,
            55 => Self::HedgeSpec,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Speculation => Some(49),
            Self::Arbitrage => Some(50),
            Self::Hedge => Some(51),
            Self::MarketMaker => Some(53),
            Self::SpecHedge => Some(54),
            Self::HedgeSpec => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for HedgeFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcHedgeFlagEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HedgeFlagEn {
    /// Speculation。CTP：`THOST_FTDC_HFEN_Speculation = '1'`。
    Speculation,
    /// Arbitrage。CTP：`THOST_FTDC_HFEN_Arbitrage = '2'`。
    Arbitrage,
    /// Hedge。CTP：`THOST_FTDC_HFEN_Hedge = '3'`。
    Hedge,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl HedgeFlagEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Speculation,
            50 => Self::Arbitrage,
            51 => Self::Hedge,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Speculation => Some(49),
            Self::Arbitrage => Some(50),
            Self::Hedge => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for HedgeFlagEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcIdCardTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdCardType {
    /// 组织机构代码。CTP：`THOST_FTDC_ICT_EID = '0'`。
    EID,
    /// 中国公民身份证。CTP：`THOST_FTDC_ICT_IDCard = '1'`。
    IDCard,
    /// 军官证。CTP：`THOST_FTDC_ICT_OfficerIDCard = '2'`。
    OfficerIDCard,
    /// 警官证。CTP：`THOST_FTDC_ICT_PoliceIDCard = '3'`。
    PoliceIDCard,
    /// 士兵证。CTP：`THOST_FTDC_ICT_SoldierIDCard = '4'`。
    SoldierIDCard,
    /// 户口簿。CTP：`THOST_FTDC_ICT_HouseholdRegister = '5'`。
    HouseholdRegister,
    /// 护照。CTP：`THOST_FTDC_ICT_Passport = '6'`。
    Passport,
    /// 台胞证。CTP：`THOST_FTDC_ICT_TaiwanCompatriotIDCard = '7'`。
    TaiwanCompatriotIDCard,
    /// 回乡证。CTP：`THOST_FTDC_ICT_HomeComingCard = '8'`。
    HomeComingCard,
    /// 营业执照号。CTP：`THOST_FTDC_ICT_LicenseNo = '9'`。
    LicenseNo,
    /// 税务登记号/当地纳税ID。CTP：`THOST_FTDC_ICT_TaxNo = 'A'`。
    TaxNo,
    /// 港澳居民来往内地通行证。CTP：`THOST_FTDC_ICT_HMMainlandTravelPermit = 'B'`。
    HMMainlandTravelPermit,
    /// 台湾居民来往大陆通行证。CTP：`THOST_FTDC_ICT_TwMainlandTravelPermit = 'C'`。
    TwMainlandTravelPermit,
    /// 驾照。CTP：`THOST_FTDC_ICT_DrivingLicense = 'D'`。
    DrivingLicense,
    /// 当地社保ID。CTP：`THOST_FTDC_ICT_SocialID = 'F'`。
    SocialID,
    /// 当地身份证。CTP：`THOST_FTDC_ICT_LocalID = 'G'`。
    LocalID,
    /// 商业登记证。CTP：`THOST_FTDC_ICT_BusinessRegistration = 'H'`。
    BusinessRegistration,
    /// 港澳永久性居民身份证。CTP：`THOST_FTDC_ICT_HKMCIDCard = 'I'`。
    HKMCIDCard,
    /// 人行开户许可证。CTP：`THOST_FTDC_ICT_AccountsPermits = 'J'`。
    AccountsPermits,
    /// 外国人永久居留证。CTP：`THOST_FTDC_ICT_FrgPrmtRdCard = 'K'`。
    FrgPrmtRdCard,
    /// 资管产品备案函。CTP：`THOST_FTDC_ICT_CptMngPrdLetter = 'L'`。
    CptMngPrdLetter,
    /// 港澳台居民居住证。CTP：`THOST_FTDC_ICT_HKMCTwResidencePermit = 'M'`。
    HKMCTwResidencePermit,
    /// 统一社会信用代码。CTP：`THOST_FTDC_ICT_UniformSocialCreditCode = 'N'`。
    UniformSocialCreditCode,
    /// 机构成立证明文件。CTP：`THOST_FTDC_ICT_CorporationCertNo = 'O'`。
    CorporationCertNo,
    /// 其他证件。CTP：`THOST_FTDC_ICT_OtherCard = 'x'`。
    OtherCard,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl IdCardType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::EID,
            49 => Self::IDCard,
            50 => Self::OfficerIDCard,
            51 => Self::PoliceIDCard,
            52 => Self::SoldierIDCard,
            53 => Self::HouseholdRegister,
            54 => Self::Passport,
            55 => Self::TaiwanCompatriotIDCard,
            56 => Self::HomeComingCard,
            57 => Self::LicenseNo,
            65 => Self::TaxNo,
            66 => Self::HMMainlandTravelPermit,
            67 => Self::TwMainlandTravelPermit,
            68 => Self::DrivingLicense,
            70 => Self::SocialID,
            71 => Self::LocalID,
            72 => Self::BusinessRegistration,
            73 => Self::HKMCIDCard,
            74 => Self::AccountsPermits,
            75 => Self::FrgPrmtRdCard,
            76 => Self::CptMngPrdLetter,
            77 => Self::HKMCTwResidencePermit,
            78 => Self::UniformSocialCreditCode,
            79 => Self::CorporationCertNo,
            120 => Self::OtherCard,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::EID => Some(48),
            Self::IDCard => Some(49),
            Self::OfficerIDCard => Some(50),
            Self::PoliceIDCard => Some(51),
            Self::SoldierIDCard => Some(52),
            Self::HouseholdRegister => Some(53),
            Self::Passport => Some(54),
            Self::TaiwanCompatriotIDCard => Some(55),
            Self::HomeComingCard => Some(56),
            Self::LicenseNo => Some(57),
            Self::TaxNo => Some(65),
            Self::HMMainlandTravelPermit => Some(66),
            Self::TwMainlandTravelPermit => Some(67),
            Self::DrivingLicense => Some(68),
            Self::SocialID => Some(70),
            Self::LocalID => Some(71),
            Self::BusinessRegistration => Some(72),
            Self::HKMCIDCard => Some(73),
            Self::AccountsPermits => Some(74),
            Self::FrgPrmtRdCard => Some(75),
            Self::CptMngPrdLetter => Some(76),
            Self::HKMCTwResidencePermit => Some(77),
            Self::UniformSocialCreditCode => Some(78),
            Self::CorporationCertNo => Some(79),
            Self::OtherCard => Some(120),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for IdCardType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcIncludeCloseProfitType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncludeCloseProfit {
    /// 包含平仓盈利。CTP：`THOST_FTDC_ICP_Include = '0'`。
    Include,
    /// 不包含平仓盈利。CTP：`THOST_FTDC_ICP_NotInclude = '2'`。
    NotInclude,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl IncludeCloseProfit {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Include,
            50 => Self::NotInclude,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Include => Some(48),
            Self::NotInclude => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for IncludeCloseProfit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInitSettlementType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InitSettlement {
    /// 结算初始化未开始。CTP：`THOST_FTDC_SIS_UnInitialize = '0'`。
    UnInitialize,
    /// 结算初始化中。CTP：`THOST_FTDC_SIS_Initialize = '1'`。
    Initialize,
    /// 结算初始化完成。CTP：`THOST_FTDC_SIS_Initialized = '2'`。
    Initialized,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InitSettlement {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnInitialize,
            49 => Self::Initialize,
            50 => Self::Initialized,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnInitialize => Some(48),
            Self::Initialize => Some(49),
            Self::Initialized => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InitSettlement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInstLifePhaseType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstLifePhase {
    /// 未上市。CTP：`THOST_FTDC_IP_NotStart = '0'`。
    NotStart,
    /// 上市。CTP：`THOST_FTDC_IP_Started = '1'`。
    Started,
    /// 停牌。CTP：`THOST_FTDC_IP_Pause = '2'`。
    Pause,
    /// 到期。CTP：`THOST_FTDC_IP_Expired = '3'`。
    Expired,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InstLifePhase {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NotStart,
            49 => Self::Started,
            50 => Self::Pause,
            51 => Self::Expired,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotStart => Some(48),
            Self::Started => Some(49),
            Self::Pause => Some(50),
            Self::Expired => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InstLifePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInstMarginCalIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstMarginCalID {
    /// 标准算法收取双边。CTP：`THOST_FTDC_IMID_BothSide = '1'`。
    BothSide,
    /// 单向大边。CTP：`THOST_FTDC_IMID_MMSA = '2'`。
    MMSA,
    /// 新组保SPMM。CTP：`THOST_FTDC_IMID_SPMM = '3'`。
    SPMM,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InstMarginCalID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::BothSide,
            50 => Self::MMSA,
            51 => Self::SPMM,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BothSide => Some(49),
            Self::MMSA => Some(50),
            Self::SPMM => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InstMarginCalID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInstStatusEnterReasonType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstStatusEnterReason {
    /// 自动切换。CTP：`THOST_FTDC_IER_Automatic = '1'`。
    Automatic,
    /// 手动切换。CTP：`THOST_FTDC_IER_Manual = '2'`。
    Manual,
    /// 熔断。CTP：`THOST_FTDC_IER_Fuse = '3'`。
    Fuse,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InstStatusEnterReason {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Automatic,
            50 => Self::Manual,
            51 => Self::Fuse,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Automatic => Some(49),
            Self::Manual => Some(50),
            Self::Fuse => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InstStatusEnterReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInstitutionTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstitutionType {
    /// 银行。CTP：`THOST_FTDC_TS_Bank = '0'`。
    Bank,
    /// 期商。CTP：`THOST_FTDC_TS_Future = '1'`。
    Future,
    /// 券商。CTP：`THOST_FTDC_TS_Store = '2'`。
    Store,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InstitutionType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Bank,
            49 => Self::Future,
            50 => Self::Store,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Bank => Some(48),
            Self::Future => Some(49),
            Self::Store => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InstitutionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInstrumentClassType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentClass {
    /// 一般月份合约。CTP：`THOST_FTDC_EIC_Usual = '1'`。
    Usual,
    /// 临近交割合约。CTP：`THOST_FTDC_EIC_Delivery = '2'`。
    Delivery,
    /// 非组合合约。CTP：`THOST_FTDC_EIC_NonComb = '3'`。
    NonComb,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InstrumentClass {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Usual,
            50 => Self::Delivery,
            51 => Self::NonComb,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Usual => Some(49),
            Self::Delivery => Some(50),
            Self::NonComb => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InstrumentClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentStatusType {
    /// 开盘前。CTP：`THOST_FTDC_IS_BeforeTrading = '0'`。
    BeforeTrading,
    /// 非交易。CTP：`THOST_FTDC_IS_NoTrading = '1'`。
    NoTrading,
    /// 连续交易。线值：`'2'`；名称已规范化，原始常量见审核表。
    Continuous,
    /// 集合竞价报单。CTP：`THOST_FTDC_IS_AuctionOrdering = '3'`。
    AuctionOrdering,
    /// 集合竞价价格平衡。CTP：`THOST_FTDC_IS_AuctionBalance = '4'`。
    AuctionBalance,
    /// 集合竞价撮合。CTP：`THOST_FTDC_IS_AuctionMatch = '5'`。
    AuctionMatch,
    /// 收盘。CTP：`THOST_FTDC_IS_Closed = '6'`。
    Closed,
    /// 交易业务处理。CTP：`THOST_FTDC_IS_TransactionProcessing = '7'`。
    TransactionProcessing,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InstrumentStatusType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::BeforeTrading,
            49 => Self::NoTrading,
            50 => Self::Continuous,
            51 => Self::AuctionOrdering,
            52 => Self::AuctionBalance,
            53 => Self::AuctionMatch,
            54 => Self::Closed,
            55 => Self::TransactionProcessing,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BeforeTrading => Some(48),
            Self::NoTrading => Some(49),
            Self::Continuous => Some(50),
            Self::AuctionOrdering => Some(51),
            Self::AuctionBalance => Some(52),
            Self::AuctionMatch => Some(53),
            Self::Closed => Some(54),
            Self::TransactionProcessing => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InstrumentStatusType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInvestorRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestorRange {
    /// 所有。CTP：`THOST_FTDC_IR_All = '1'`。
    All,
    /// 投资者组。CTP：`THOST_FTDC_IR_Group = '2'`。
    Group,
    /// 单一投资者。CTP：`THOST_FTDC_IR_Single = '3'`。
    Single,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InvestorRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::Group,
            51 => Self::Single,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::Group => Some(50),
            Self::Single => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InvestorRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInvestorRiskStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestorRiskStatus {
    /// 正常。CTP：`THOST_FTDC_IRS_Normal = '1'`。
    Normal,
    /// 警告。CTP：`THOST_FTDC_IRS_Warn = '2'`。
    Warn,
    /// 追保。CTP：`THOST_FTDC_IRS_Call = '3'`。
    Call,
    /// 强平。CTP：`THOST_FTDC_IRS_Force = '4'`。
    Force,
    /// 异常。CTP：`THOST_FTDC_IRS_Exception = '5'`。
    Exception,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InvestorRiskStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Normal,
            50 => Self::Warn,
            51 => Self::Call,
            52 => Self::Force,
            53 => Self::Exception,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(49),
            Self::Warn => Some(50),
            Self::Call => Some(51),
            Self::Force => Some(52),
            Self::Exception => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InvestorRiskStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInvestorSettlementParamIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestorSettlementParamID {
    /// 质押比例。CTP：`THOST_FTDC_ISPI_MortgageRatio = '4'`。
    MortgageRatio,
    /// 保证金算法。CTP：`THOST_FTDC_ISPI_MarginWay = '5'`。
    MarginWay,
    /// 结算单结存是否包含质押。CTP：`THOST_FTDC_ISPI_BillDeposit = '9'`。
    BillDeposit,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InvestorSettlementParamID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            52 => Self::MortgageRatio,
            53 => Self::MarginWay,
            57 => Self::BillDeposit,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::MortgageRatio => Some(52),
            Self::MarginWay => Some(53),
            Self::BillDeposit => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InvestorSettlementParamID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestorTradingRightType {
    /// 只能平仓。CTP：`THOST_FTDC_ITR_CloseOnly = '1'`。
    CloseOnly,
    /// 不能交易。CTP：`THOST_FTDC_ITR_Forbidden = '2'`。
    Forbidden,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InvestorTradingRightType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::CloseOnly,
            50 => Self::Forbidden,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CloseOnly => Some(49),
            Self::Forbidden => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InvestorTradingRightType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcInvestorTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestorType {
    /// 自然人。CTP：`THOST_FTDC_CT_Person = '0'`。
    Person,
    /// 法人。CTP：`THOST_FTDC_CT_Company = '1'`。
    Company,
    /// 投资基金。CTP：`THOST_FTDC_CT_Fund = '2'`。
    Fund,
    /// 特殊法人。CTP：`THOST_FTDC_CT_SpecialOrgan = '3'`。
    SpecialOrgan,
    /// 资管户。CTP：`THOST_FTDC_CT_Asset = '4'`。
    Asset,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl InvestorType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Person,
            49 => Self::Company,
            50 => Self::Fund,
            51 => Self::SpecialOrgan,
            52 => Self::Asset,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Person => Some(48),
            Self::Company => Some(49),
            Self::Fund => Some(50),
            Self::SpecialOrgan => Some(51),
            Self::Asset => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for InvestorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcLanguageTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageType {
    /// 中文。CTP：`THOST_FTDC_LT_Chinese = '1'`。
    Chinese,
    /// 英文。CTP：`THOST_FTDC_LT_English = '2'`。
    English,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl LanguageType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Chinese,
            50 => Self::English,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Chinese => Some(49),
            Self::English => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for LanguageType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcLastFragmentType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LastFragment {
    /// 是最后分片。CTP：`THOST_FTDC_LF_Yes = '0'`。
    Yes,
    /// 不是最后分片。CTP：`THOST_FTDC_LF_No = '1'`。
    No,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl LastFragment {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Yes,
            49 => Self::No,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Yes => Some(48),
            Self::No => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for LastFragment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcLimitUseTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitUseType {
    /// 可重复使用。CTP：`THOST_FTDC_LUT_Repeatable = '1'`。
    Repeatable,
    /// 不可重复使用。CTP：`THOST_FTDC_LUT_Unrepeatable = '2'`。
    Unrepeatable,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl LimitUseType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Repeatable,
            50 => Self::Unrepeatable,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Repeatable => Some(49),
            Self::Unrepeatable => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for LimitUseType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcLinkStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkStatus {
    /// 已经连接。CTP：`THOST_FTDC_LS_Connected = '1'`。
    Connected,
    /// 没有连接。CTP：`THOST_FTDC_LS_Disconnected = '2'`。
    Disconnected,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl LinkStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Connected,
            50 => Self::Disconnected,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Connected => Some(49),
            Self::Disconnected => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for LinkStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcLoginModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoginMode {
    /// 交易。CTP：`THOST_FTDC_LM_Trade = '0'`。
    Trade,
    /// 转账。CTP：`THOST_FTDC_LM_Transfer = '1'`。
    Transfer,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl LoginMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Trade,
            49 => Self::Transfer,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Trade => Some(48),
            Self::Transfer => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for LoginMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcManageStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManageStatus {
    /// 指定存管。CTP：`THOST_FTDC_MSS_Point = '0'`。
    Point,
    /// 预指定。CTP：`THOST_FTDC_MSS_PrePoint = '1'`。
    PrePoint,
    /// 撤销指定。CTP：`THOST_FTDC_MSS_CancelPoint = '2'`。
    CancelPoint,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ManageStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Point,
            49 => Self::PrePoint,
            50 => Self::CancelPoint,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Point => Some(48),
            Self::PrePoint => Some(49),
            Self::CancelPoint => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ManageStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMarginPriceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarginPriceType {
    /// 昨结算价。CTP：`THOST_FTDC_MPT_PreSettlementPrice = '1'`。
    PreSettlementPrice,
    /// 最新价。CTP：`THOST_FTDC_MPT_SettlementPrice = '2'`。
    SettlementPrice,
    /// 成交均价。CTP：`THOST_FTDC_MPT_AveragePrice = '3'`。
    AveragePrice,
    /// 开仓价。CTP：`THOST_FTDC_MPT_OpenPrice = '4'`。
    OpenPrice,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MarginPriceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::PreSettlementPrice,
            50 => Self::SettlementPrice,
            51 => Self::AveragePrice,
            52 => Self::OpenPrice,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::PreSettlementPrice => Some(49),
            Self::SettlementPrice => Some(50),
            Self::AveragePrice => Some(51),
            Self::OpenPrice => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MarginPriceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMarginRateTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarginRateType {
    /// 交易所保证金率。CTP：`THOST_FTDC_MRT_Exchange = '1'`。
    Exchange,
    /// 投资者保证金率。CTP：`THOST_FTDC_MRT_Investor = '2'`。
    Investor,
    /// 投资者交易保证金率。CTP：`THOST_FTDC_MRT_InvestorTrade = '3'`。
    InvestorTrade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MarginRateType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Exchange,
            50 => Self::Investor,
            51 => Self::InvestorTrade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Exchange => Some(49),
            Self::Investor => Some(50),
            Self::InvestorTrade => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MarginRateType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMarginTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarginType {
    /// 交易所保证金率。CTP：`THOST_FTDC_MGT_ExchMarginRate = '0'`。
    ExchMarginRate,
    /// 投资者保证金率。CTP：`THOST_FTDC_MGT_InstrMarginRate = '1'`。
    InstrMarginRate,
    /// 投资者交易保证金率。CTP：`THOST_FTDC_MGT_InstrMarginRateTrade = '2'`。
    InstrMarginRateTrade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MarginType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ExchMarginRate,
            49 => Self::InstrMarginRate,
            50 => Self::InstrMarginRateTrade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ExchMarginRate => Some(48),
            Self::InstrMarginRate => Some(49),
            Self::InstrMarginRateTrade => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MarginType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMatchTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchType {
    /// 基点价值。CTP：`THOST_FTDC_OTC_MT_DV01 = '1'`。
    DV01,
    /// 面值。CTP：`THOST_FTDC_OTC_MT_ParValue = '2'`。
    ParValue,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MatchType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::DV01,
            50 => Self::ParValue,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::DV01 => Some(49),
            Self::ParValue => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MatchType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMaxMarginSideAlgorithmType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaxMarginSideAlgorithm {
    /// 不使用大额单边保证金算法。CTP：`THOST_FTDC_MMSA_NO = '0'`。
    NO,
    /// 使用大额单边保证金算法。CTP：`THOST_FTDC_MMSA_YES = '1'`。
    YES,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MaxMarginSideAlgorithm {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NO,
            49 => Self::YES,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NO => Some(48),
            Self::YES => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MaxMarginSideAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMoneyAccountStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoneyAccountStatus {
    /// 正常。CTP：`THOST_FTDC_MAS_Normal = '0'`。
    Normal,
    /// 销户。CTP：`THOST_FTDC_MAS_Cancel = '1'`。
    Cancel,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MoneyAccountStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::Cancel,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::Cancel => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MoneyAccountStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMonthBillTradeSumType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonthBillTradeSum {
    /// 同日同合约。CTP：`THOST_FTDC_MBTS_ByInstrument = '0'`。
    ByInstrument,
    /// 同日同合约同价格。CTP：`THOST_FTDC_MBTS_ByDayInsPrc = '1'`。
    ByDayInsPrc,
    /// 同合约。CTP：`THOST_FTDC_MBTS_ByDayIns = '2'`。
    ByDayIns,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MonthBillTradeSum {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ByInstrument,
            49 => Self::ByDayInsPrc,
            50 => Self::ByDayIns,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByInstrument => Some(48),
            Self::ByDayInsPrc => Some(49),
            Self::ByDayIns => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MonthBillTradeSum {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMortgageFundUseRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MortgageFundUseRange {
    /// 不能使用。CTP：`THOST_FTDC_MFUR_None = '0'`。
    None,
    /// 用于保证金。CTP：`THOST_FTDC_MFUR_Margin = '1'`。
    Margin,
    /// 用于手续费、盈亏、保证金。CTP：`THOST_FTDC_MFUR_All = '2'`。
    All,
    /// 人民币方案3。CTP：`THOST_FTDC_MFUR_CNY3 = '3'`。
    CNY3,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MortgageFundUseRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::Margin,
            50 => Self::All,
            51 => Self::CNY3,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::Margin => Some(49),
            Self::All => Some(50),
            Self::CNY3 => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MortgageFundUseRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcMortgageTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MortgageType {
    /// 质出。CTP：`THOST_FTDC_MT_Out = '0'`。
    Out,
    /// 质入。线值：`'1'`；名称已规范化，原始常量见审核表。
    InValue,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl MortgageType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Out,
            49 => Self::InValue,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Out => Some(48),
            Self::InValue => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for MortgageType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcNoteTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteType {
    /// 交易结算单。CTP：`THOST_FTDC_NOTETYPE_TradeSettleBill = '1'`。
    TradeSettleBill,
    /// 交易结算月报。CTP：`THOST_FTDC_NOTETYPE_TradeSettleMonth = '2'`。
    TradeSettleMonth,
    /// 追加保证金通知书。CTP：`THOST_FTDC_NOTETYPE_CallMarginNotes = '3'`。
    CallMarginNotes,
    /// 强行平仓通知书。CTP：`THOST_FTDC_NOTETYPE_ForceCloseNotes = '4'`。
    ForceCloseNotes,
    /// 成交通知书。CTP：`THOST_FTDC_NOTETYPE_TradeNotes = '5'`。
    TradeNotes,
    /// 交割通知书。线值：`'6'`；名称已规范化，原始常量见审核表。
    DeliveryNotes,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl NoteType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::TradeSettleBill,
            50 => Self::TradeSettleMonth,
            51 => Self::CallMarginNotes,
            52 => Self::ForceCloseNotes,
            53 => Self::TradeNotes,
            54 => Self::DeliveryNotes,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::TradeSettleBill => Some(49),
            Self::TradeSettleMonth => Some(50),
            Self::CallMarginNotes => Some(51),
            Self::ForceCloseNotes => Some(52),
            Self::TradeNotes => Some(53),
            Self::DeliveryNotes => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for NoteType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcNotifyClassType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotifyClass {
    /// 正常。线值：`'0'`；名称已规范化，原始常量见审核表。
    NoError,
    /// 警示。CTP：`THOST_FTDC_NC_Warn = '1'`。
    Warn,
    /// 追保。CTP：`THOST_FTDC_NC_Call = '2'`。
    Call,
    /// 强平。CTP：`THOST_FTDC_NC_Force = '3'`。
    Force,
    /// 穿仓。线值：`'4'`；名称已规范化，原始常量见审核表。
    NegativeBalance,
    /// 异常。CTP：`THOST_FTDC_NC_Exception = '5'`。
    Exception,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl NotifyClass {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoError,
            49 => Self::Warn,
            50 => Self::Call,
            51 => Self::Force,
            52 => Self::NegativeBalance,
            53 => Self::Exception,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoError => Some(48),
            Self::Warn => Some(49),
            Self::Call => Some(50),
            Self::Force => Some(51),
            Self::NegativeBalance => Some(52),
            Self::Exception => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for NotifyClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOTCTradeTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OTCTradeType {
    /// 大宗交易。CTP：`THOST_FTDC_OTC_TRDT_Block = '0'`。
    Block,
    /// 期转现。CTP：`THOST_FTDC_OTC_TRDT_EFP = '1'`。
    EFP,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OTCTradeType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Block,
            49 => Self::EFP,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Block => Some(48),
            Self::EFP => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OTCTradeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOTPStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OTPStatus {
    /// 未使用。CTP：`THOST_FTDC_OTPS_Unused = '0'`。
    Unused,
    /// 已使用。CTP：`THOST_FTDC_OTPS_Used = '1'`。
    Used,
    /// 注销。CTP：`THOST_FTDC_OTPS_Disuse = '2'`。
    Disuse,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OTPStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Unused,
            49 => Self::Used,
            50 => Self::Disuse,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Unused => Some(48),
            Self::Used => Some(49),
            Self::Disuse => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OTPStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOTPTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OTPType {
    /// 无动态令牌。CTP：`THOST_FTDC_OTP_NONE = '0'`。
    NONE,
    /// 时间令牌。CTP：`THOST_FTDC_OTP_TOTP = '1'`。
    TOTP,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OTPType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NONE,
            49 => Self::TOTP,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NONE => Some(48),
            Self::TOTP => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OTPType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOffsetFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OffsetFlag {
    /// 开仓。CTP：`THOST_FTDC_OF_Open = '0'`。
    Open,
    /// 平仓。CTP：`THOST_FTDC_OF_Close = '1'`。
    Close,
    /// 强平。CTP：`THOST_FTDC_OF_ForceClose = '2'`。
    ForceClose,
    /// 平今。CTP：`THOST_FTDC_OF_CloseToday = '3'`。
    CloseToday,
    /// 平昨。CTP：`THOST_FTDC_OF_CloseYesterday = '4'`。
    CloseYesterday,
    /// 强减。CTP：`THOST_FTDC_OF_ForceOff = '5'`。
    ForceOff,
    /// 本地强平。CTP：`THOST_FTDC_OF_LocalForceClose = '6'`。
    LocalForceClose,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OffsetFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Open,
            49 => Self::Close,
            50 => Self::ForceClose,
            51 => Self::CloseToday,
            52 => Self::CloseYesterday,
            53 => Self::ForceOff,
            54 => Self::LocalForceClose,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Open => Some(48),
            Self::Close => Some(49),
            Self::ForceClose => Some(50),
            Self::CloseToday => Some(51),
            Self::CloseYesterday => Some(52),
            Self::ForceOff => Some(53),
            Self::LocalForceClose => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OffsetFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOffsetFlagEnType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OffsetFlagEn {
    /// Position Opening。CTP：`THOST_FTDC_OFEN_Open = '0'`。
    Open,
    /// Position Close。CTP：`THOST_FTDC_OFEN_Close = '1'`。
    Close,
    /// Forced Liquidation。CTP：`THOST_FTDC_OFEN_ForceClose = '2'`。
    ForceClose,
    /// Close Today。CTP：`THOST_FTDC_OFEN_CloseToday = '3'`。
    CloseToday,
    /// Close Prev.。CTP：`THOST_FTDC_OFEN_CloseYesterday = '4'`。
    CloseYesterday,
    /// Forced Reduction。CTP：`THOST_FTDC_OFEN_ForceOff = '5'`。
    ForceOff,
    /// Local Forced Liquidation。CTP：`THOST_FTDC_OFEN_LocalForceClose = '6'`。
    LocalForceClose,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OffsetFlagEn {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Open,
            49 => Self::Close,
            50 => Self::ForceClose,
            51 => Self::CloseToday,
            52 => Self::CloseYesterday,
            53 => Self::ForceOff,
            54 => Self::LocalForceClose,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Open => Some(48),
            Self::Close => Some(49),
            Self::ForceClose => Some(50),
            Self::CloseToday => Some(51),
            Self::CloseYesterday => Some(52),
            Self::ForceOff => Some(53),
            Self::LocalForceClose => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OffsetFlagEn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOffsetTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OffsetType {
    /// 期权对冲。线值：`'0'`；名称已规范化，原始常量见审核表。
    OptionOffset,
    /// 期货对冲。线值：`'1'`；名称已规范化，原始常量见审核表。
    FutureOffset,
    /// 行权后期货对冲。线值：`'2'`；名称已规范化，原始常量见审核表。
    ExecutionOffset,
    /// 履约后期货对冲。线值：`'3'`；名称已规范化，原始常量见审核表。
    PerformOffset,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OffsetType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::OptionOffset,
            49 => Self::FutureOffset,
            50 => Self::ExecutionOffset,
            51 => Self::PerformOffset,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::OptionOffset => Some(48),
            Self::FutureOffset => Some(49),
            Self::ExecutionOffset => Some(50),
            Self::PerformOffset => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OffsetType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOpenLimitControlLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpenLimitControlLevel {
    /// 不控制。CTP：`THOST_FTDC_PLCL_None = '0'`。
    None,
    /// 产品级别。CTP：`THOST_FTDC_PLCL_Product = '1'`。
    Product,
    /// 合约级别。CTP：`THOST_FTDC_PLCL_Inst = '2'`。
    Inst,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OpenLimitControlLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::Product,
            50 => Self::Inst,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::Product => Some(49),
            Self::Inst => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OpenLimitControlLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOpenOrDestroyType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpenOrDestroy {
    /// 开户。CTP：`THOST_FTDC_OOD_Open = '1'`。
    Open,
    /// 销户。CTP：`THOST_FTDC_OOD_Destroy = '0'`。
    Destroy,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OpenOrDestroy {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Open,
            48 => Self::Destroy,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Open => Some(49),
            Self::Destroy => Some(48),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OpenOrDestroy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOptSelfCloseFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptSelfCloseFlag {
    /// 自对冲期权仓位。CTP：`THOST_FTDC_OSCF_CloseSelfOptionPosition = '1'`。
    CloseSelfOptionPosition,
    /// 保留期权仓位。CTP：`THOST_FTDC_OSCF_ReserveOptionPosition = '2'`。
    ReserveOptionPosition,
    /// 自对冲卖方履约后的期货仓位。CTP：`THOST_FTDC_OSCF_SellCloseSelfFuturePosition = '3'`。
    SellCloseSelfFuturePosition,
    /// 保留卖方履约后的期货仓位。CTP：`THOST_FTDC_OSCF_ReserveFuturePosition = '4'`。
    ReserveFuturePosition,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OptSelfCloseFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::CloseSelfOptionPosition,
            50 => Self::ReserveOptionPosition,
            51 => Self::SellCloseSelfFuturePosition,
            52 => Self::ReserveFuturePosition,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CloseSelfOptionPosition => Some(49),
            Self::ReserveOptionPosition => Some(50),
            Self::SellCloseSelfFuturePosition => Some(51),
            Self::ReserveFuturePosition => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OptSelfCloseFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOptionRoyaltyPriceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionRoyaltyPriceType {
    /// 昨结算价。CTP：`THOST_FTDC_ORPT_PreSettlementPrice = '1'`。
    PreSettlementPrice,
    /// 开仓价。CTP：`THOST_FTDC_ORPT_OpenPrice = '4'`。
    OpenPrice,
    /// 最新价与昨结算价较大值。CTP：`THOST_FTDC_ORPT_MaxPreSettlementPrice = '5'`。
    MaxPreSettlementPrice,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OptionRoyaltyPriceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::PreSettlementPrice,
            52 => Self::OpenPrice,
            53 => Self::MaxPreSettlementPrice,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::PreSettlementPrice => Some(49),
            Self::OpenPrice => Some(52),
            Self::MaxPreSettlementPrice => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OptionRoyaltyPriceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOptionsTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionsType {
    /// 看涨。CTP：`THOST_FTDC_CP_CallOptions = '1'`。
    CallOptions,
    /// 看跌。CTP：`THOST_FTDC_CP_PutOptions = '2'`。
    PutOptions,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OptionsType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::CallOptions,
            50 => Self::PutOptions,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CallOptions => Some(49),
            Self::PutOptions => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OptionsType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderActionStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderActionStatus {
    /// 已经提交。CTP：`THOST_FTDC_OAS_Submitted = 'a'`。
    Submitted,
    /// 已经接受。CTP：`THOST_FTDC_OAS_Accepted = 'b'`。
    Accepted,
    /// 已经被拒绝。CTP：`THOST_FTDC_OAS_Rejected = 'c'`。
    Rejected,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderActionStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            97 => Self::Submitted,
            98 => Self::Accepted,
            99 => Self::Rejected,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Submitted => Some(97),
            Self::Accepted => Some(98),
            Self::Rejected => Some(99),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderActionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderCancelAlgType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderCancelAlg {
    /// 轮询席位撤单。CTP：`THOST_FTDC_OAC_Balance = '1'`。
    Balance,
    /// 优先原报单席位撤单。CTP：`THOST_FTDC_OAC_OrigFirst = '2'`。
    OrigFirst,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderCancelAlg {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Balance,
            50 => Self::OrigFirst,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Balance => Some(49),
            Self::OrigFirst => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderCancelAlg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderFreqControlLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderFreqControlLevel {
    /// 不控制。CTP：`THOST_FTDC_OFCL_None = '0'`。
    None,
    /// 产品级别。CTP：`THOST_FTDC_OFCL_Product = '1'`。
    Product,
    /// 合约级别。CTP：`THOST_FTDC_OFCL_Inst = '2'`。
    Inst,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderFreqControlLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::Product,
            50 => Self::Inst,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::Product => Some(49),
            Self::Inst => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderFreqControlLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderPriceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderPriceType {
    /// 任意价。CTP：`THOST_FTDC_OPT_AnyPrice = '1'`。
    AnyPrice,
    /// 限价。CTP：`THOST_FTDC_OPT_LimitPrice = '2'`。
    LimitPrice,
    /// 最优价。CTP：`THOST_FTDC_OPT_BestPrice = '3'`。
    BestPrice,
    /// 最新价。CTP：`THOST_FTDC_OPT_LastPrice = '4'`。
    LastPrice,
    /// 最新价浮动上浮1个ticks。CTP：`THOST_FTDC_OPT_LastPricePlusOneTicks = '5'`。
    LastPricePlusOneTicks,
    /// 最新价浮动上浮2个ticks。CTP：`THOST_FTDC_OPT_LastPricePlusTwoTicks = '6'`。
    LastPricePlusTwoTicks,
    /// 最新价浮动上浮3个ticks。CTP：`THOST_FTDC_OPT_LastPricePlusThreeTicks = '7'`。
    LastPricePlusThreeTicks,
    /// 卖一价。CTP：`THOST_FTDC_OPT_AskPrice1 = '8'`。
    AskPrice1,
    /// 卖一价浮动上浮1个ticks。CTP：`THOST_FTDC_OPT_AskPrice1PlusOneTicks = '9'`。
    AskPrice1PlusOneTicks,
    /// 卖一价浮动上浮2个ticks。CTP：`THOST_FTDC_OPT_AskPrice1PlusTwoTicks = 'A'`。
    AskPrice1PlusTwoTicks,
    /// 卖一价浮动上浮3个ticks。CTP：`THOST_FTDC_OPT_AskPrice1PlusThreeTicks = 'B'`。
    AskPrice1PlusThreeTicks,
    /// 买一价。CTP：`THOST_FTDC_OPT_BidPrice1 = 'C'`。
    BidPrice1,
    /// 买一价浮动上浮1个ticks。CTP：`THOST_FTDC_OPT_BidPrice1PlusOneTicks = 'D'`。
    BidPrice1PlusOneTicks,
    /// 买一价浮动上浮2个ticks。CTP：`THOST_FTDC_OPT_BidPrice1PlusTwoTicks = 'E'`。
    BidPrice1PlusTwoTicks,
    /// 买一价浮动上浮3个ticks。CTP：`THOST_FTDC_OPT_BidPrice1PlusThreeTicks = 'F'`。
    BidPrice1PlusThreeTicks,
    /// 五档价。CTP：`THOST_FTDC_OPT_FiveLevelPrice = 'G'`。
    FiveLevelPrice,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderPriceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::AnyPrice,
            50 => Self::LimitPrice,
            51 => Self::BestPrice,
            52 => Self::LastPrice,
            53 => Self::LastPricePlusOneTicks,
            54 => Self::LastPricePlusTwoTicks,
            55 => Self::LastPricePlusThreeTicks,
            56 => Self::AskPrice1,
            57 => Self::AskPrice1PlusOneTicks,
            65 => Self::AskPrice1PlusTwoTicks,
            66 => Self::AskPrice1PlusThreeTicks,
            67 => Self::BidPrice1,
            68 => Self::BidPrice1PlusOneTicks,
            69 => Self::BidPrice1PlusTwoTicks,
            70 => Self::BidPrice1PlusThreeTicks,
            71 => Self::FiveLevelPrice,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::AnyPrice => Some(49),
            Self::LimitPrice => Some(50),
            Self::BestPrice => Some(51),
            Self::LastPrice => Some(52),
            Self::LastPricePlusOneTicks => Some(53),
            Self::LastPricePlusTwoTicks => Some(54),
            Self::LastPricePlusThreeTicks => Some(55),
            Self::AskPrice1 => Some(56),
            Self::AskPrice1PlusOneTicks => Some(57),
            Self::AskPrice1PlusTwoTicks => Some(65),
            Self::AskPrice1PlusThreeTicks => Some(66),
            Self::BidPrice1 => Some(67),
            Self::BidPrice1PlusOneTicks => Some(68),
            Self::BidPrice1PlusTwoTicks => Some(69),
            Self::BidPrice1PlusThreeTicks => Some(70),
            Self::FiveLevelPrice => Some(71),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderPriceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderSourceType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderSource {
    /// 来自参与者。CTP：`THOST_FTDC_OSRC_Participant = '0'`。
    Participant,
    /// 来自管理员。CTP：`THOST_FTDC_OSRC_Administrator = '1'`。
    Administrator,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderSource {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Participant,
            49 => Self::Administrator,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Participant => Some(48),
            Self::Administrator => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    /// 全部成交。CTP：`THOST_FTDC_OST_AllTraded = '0'`。
    AllTraded,
    /// 部分成交还在队列中。CTP：`THOST_FTDC_OST_PartTradedQueueing = '1'`。
    PartTradedQueueing,
    /// 部分成交不在队列中。CTP：`THOST_FTDC_OST_PartTradedNotQueueing = '2'`。
    PartTradedNotQueueing,
    /// 未成交还在队列中。CTP：`THOST_FTDC_OST_NoTradeQueueing = '3'`。
    NoTradeQueueing,
    /// 未成交不在队列中。CTP：`THOST_FTDC_OST_NoTradeNotQueueing = '4'`。
    NoTradeNotQueueing,
    /// 撤单。CTP：`THOST_FTDC_OST_Canceled = '5'`。
    Canceled,
    /// 未知。CTP：`THOST_FTDC_OST_Unknown = 'a'`。
    Unknown,
    /// 尚未触发。CTP：`THOST_FTDC_OST_NotTouched = 'b'`。
    NotTouched,
    /// 已触发。CTP：`THOST_FTDC_OST_Touched = 'c'`。
    Touched,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::AllTraded,
            49 => Self::PartTradedQueueing,
            50 => Self::PartTradedNotQueueing,
            51 => Self::NoTradeQueueing,
            52 => Self::NoTradeNotQueueing,
            53 => Self::Canceled,
            97 => Self::Unknown,
            98 => Self::NotTouched,
            99 => Self::Touched,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::AllTraded => Some(48),
            Self::PartTradedQueueing => Some(49),
            Self::PartTradedNotQueueing => Some(50),
            Self::NoTradeQueueing => Some(51),
            Self::NoTradeNotQueueing => Some(52),
            Self::Canceled => Some(53),
            Self::Unknown => Some(97),
            Self::NotTouched => Some(98),
            Self::Touched => Some(99),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderSubmitStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderSubmitStatus {
    /// 已经提交。CTP：`THOST_FTDC_OSS_InsertSubmitted = '0'`。
    InsertSubmitted,
    /// 撤单已经提交。CTP：`THOST_FTDC_OSS_CancelSubmitted = '1'`。
    CancelSubmitted,
    /// 修改已经提交。CTP：`THOST_FTDC_OSS_ModifySubmitted = '2'`。
    ModifySubmitted,
    /// 已经接受。CTP：`THOST_FTDC_OSS_Accepted = '3'`。
    Accepted,
    /// 报单已经被拒绝。CTP：`THOST_FTDC_OSS_InsertRejected = '4'`。
    InsertRejected,
    /// 撤单已经被拒绝。CTP：`THOST_FTDC_OSS_CancelRejected = '5'`。
    CancelRejected,
    /// 改单已经被拒绝。CTP：`THOST_FTDC_OSS_ModifyRejected = '6'`。
    ModifyRejected,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderSubmitStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::InsertSubmitted,
            49 => Self::CancelSubmitted,
            50 => Self::ModifySubmitted,
            51 => Self::Accepted,
            52 => Self::InsertRejected,
            53 => Self::CancelRejected,
            54 => Self::ModifyRejected,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InsertSubmitted => Some(48),
            Self::CancelSubmitted => Some(49),
            Self::ModifySubmitted => Some(50),
            Self::Accepted => Some(51),
            Self::InsertRejected => Some(52),
            Self::CancelRejected => Some(53),
            Self::ModifyRejected => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderSubmitStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrderTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderType {
    /// 正常。CTP：`THOST_FTDC_ORDT_Normal = '0'`。
    Normal,
    /// 报价衍生。CTP：`THOST_FTDC_ORDT_DeriveFromQuote = '1'`。
    DeriveFromQuote,
    /// 组合衍生。CTP：`THOST_FTDC_ORDT_DeriveFromCombination = '2'`。
    DeriveFromCombination,
    /// 组合报单。CTP：`THOST_FTDC_ORDT_Combination = '3'`。
    Combination,
    /// 条件单。CTP：`THOST_FTDC_ORDT_ConditionalOrder = '4'`。
    ConditionalOrder,
    /// 互换单。CTP：`THOST_FTDC_ORDT_Swap = '5'`。
    Swap,
    /// 大宗交易成交衍生。CTP：`THOST_FTDC_ORDT_DeriveFromBlockTrade = '6'`。
    DeriveFromBlockTrade,
    /// 期转现成交衍生。CTP：`THOST_FTDC_ORDT_DeriveFromEFPTrade = '7'`。
    DeriveFromEFPTrade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrderType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::DeriveFromQuote,
            50 => Self::DeriveFromCombination,
            51 => Self::Combination,
            52 => Self::ConditionalOrder,
            53 => Self::Swap,
            54 => Self::DeriveFromBlockTrade,
            55 => Self::DeriveFromEFPTrade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::DeriveFromQuote => Some(49),
            Self::DeriveFromCombination => Some(50),
            Self::Combination => Some(51),
            Self::ConditionalOrder => Some(52),
            Self::Swap => Some(53),
            Self::DeriveFromBlockTrade => Some(54),
            Self::DeriveFromEFPTrade => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrgSystemIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrgSystemID {
    /// 综合交易平台。CTP：`THOST_FTDC_ORGS_Standard = '0'`。
    Standard,
    /// 易盛系统。CTP：`THOST_FTDC_ORGS_ESunny = '1'`。
    ESunny,
    /// 金仕达V6系统。CTP：`THOST_FTDC_ORGS_KingStarV6 = '2'`。
    KingStarV6,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrgSystemID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Standard,
            49 => Self::ESunny,
            50 => Self::KingStarV6,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Standard => Some(48),
            Self::ESunny => Some(49),
            Self::KingStarV6 => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrgSystemID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrganLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganLevel {
    /// 银行总行或期商总部。CTP：`THOST_FTDC_OL_HeadQuarters = '1'`。
    HeadQuarters,
    /// 银行分中心或期货公司营业部。CTP：`THOST_FTDC_OL_Branch = '2'`。
    Branch,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrganLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::HeadQuarters,
            50 => Self::Branch,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::HeadQuarters => Some(49),
            Self::Branch => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrganLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrganStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganStatus {
    /// 启用。CTP：`THOST_FTDC_OS_Ready = '0'`。
    Ready,
    /// 签到。CTP：`THOST_FTDC_OS_CheckIn = '1'`。
    CheckIn,
    /// 签退。CTP：`THOST_FTDC_OS_CheckOut = '2'`。
    CheckOut,
    /// 对帐文件到达。CTP：`THOST_FTDC_OS_CheckFileArrived = '3'`。
    CheckFileArrived,
    /// 对帐。CTP：`THOST_FTDC_OS_CheckDetail = '4'`。
    CheckDetail,
    /// 日终清理。CTP：`THOST_FTDC_OS_DayEndClean = '5'`。
    DayEndClean,
    /// 注销。CTP：`THOST_FTDC_OS_Invalid = '9'`。
    Invalid,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrganStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Ready,
            49 => Self::CheckIn,
            50 => Self::CheckOut,
            51 => Self::CheckFileArrived,
            52 => Self::CheckDetail,
            53 => Self::DayEndClean,
            57 => Self::Invalid,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Ready => Some(48),
            Self::CheckIn => Some(49),
            Self::CheckOut => Some(50),
            Self::CheckFileArrived => Some(51),
            Self::CheckDetail => Some(52),
            Self::DayEndClean => Some(53),
            Self::Invalid => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrganStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcOrganTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganType {
    /// 银行代理。CTP：`THOST_FTDC_OT_Bank = '1'`。
    Bank,
    /// 交易前置。CTP：`THOST_FTDC_OT_Future = '2'`。
    Future,
    /// 银期转帐平台管理。CTP：`THOST_FTDC_OT_PlateForm = '9'`。
    PlateForm,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl OrganType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Bank,
            50 => Self::Future,
            57 => Self::PlateForm,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Bank => Some(49),
            Self::Future => Some(50),
            Self::PlateForm => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for OrganType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcParkedOrderStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParkedOrderStatus {
    /// 未发送。CTP：`THOST_FTDC_PAOS_NotSend = '1'`。
    NotSend,
    /// 已发送。CTP：`THOST_FTDC_PAOS_Send = '2'`。
    Send,
    /// 已删除。CTP：`THOST_FTDC_PAOS_Deleted = '3'`。
    Deleted,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ParkedOrderStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NotSend,
            50 => Self::Send,
            51 => Self::Deleted,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotSend => Some(49),
            Self::Send => Some(50),
            Self::Deleted => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ParkedOrderStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPassWordKeyTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassWordKeyType {
    /// 交换密钥。CTP：`THOST_FTDC_PWKT_ExchangeKey = '0'`。
    ExchangeKey,
    /// 密码密钥。CTP：`THOST_FTDC_PWKT_PassWordKey = '1'`。
    PassWordKey,
    /// MAC密钥。CTP：`THOST_FTDC_PWKT_MACKey = '2'`。
    MACKey,
    /// 报文密钥。CTP：`THOST_FTDC_PWKT_MessageKey = '3'`。
    MessageKey,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PassWordKeyType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ExchangeKey,
            49 => Self::PassWordKey,
            50 => Self::MACKey,
            51 => Self::MessageKey,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ExchangeKey => Some(48),
            Self::PassWordKey => Some(49),
            Self::MACKey => Some(50),
            Self::MessageKey => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PassWordKeyType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPasswordTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PasswordType {
    /// 交易密码。CTP：`THOST_FTDC_PWDT_Trade = '1'`。
    Trade,
    /// 资金密码。CTP：`THOST_FTDC_PWDT_Account = '2'`。
    Account,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PasswordType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Trade,
            50 => Self::Account,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Trade => Some(49),
            Self::Account => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PasswordType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPersonTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PersonType {
    /// 指定下单人。CTP：`THOST_FTDC_PST_Order = '1'`。
    Order,
    /// 开户授权人。CTP：`THOST_FTDC_PST_Open = '2'`。
    Open,
    /// 资金调拨人。CTP：`THOST_FTDC_PST_Fund = '3'`。
    Fund,
    /// 结算单确认人。CTP：`THOST_FTDC_PST_Settlement = '4'`。
    Settlement,
    /// 法人。CTP：`THOST_FTDC_PST_Company = '5'`。
    Company,
    /// 法人代表。CTP：`THOST_FTDC_PST_Corporation = '6'`。
    Corporation,
    /// 投资者联系人。CTP：`THOST_FTDC_PST_LinkMan = '7'`。
    LinkMan,
    /// 分户管理资产负责人。CTP：`THOST_FTDC_PST_Ledger = '8'`。
    Ledger,
    /// 托（保）管人。CTP：`THOST_FTDC_PST_Trustee = '9'`。
    Trustee,
    /// 托（保）管机构法人代表。CTP：`THOST_FTDC_PST_TrusteeCorporation = 'A'`。
    TrusteeCorporation,
    /// 托（保）管机构开户授权人。CTP：`THOST_FTDC_PST_TrusteeOpen = 'B'`。
    TrusteeOpen,
    /// 托（保）管机构联系人。CTP：`THOST_FTDC_PST_TrusteeContact = 'C'`。
    TrusteeContact,
    /// 境外自然人参考证件。CTP：`THOST_FTDC_PST_ForeignerRefer = 'D'`。
    ForeignerRefer,
    /// 法人代表参考证件。CTP：`THOST_FTDC_PST_CorporationRefer = 'E'`。
    CorporationRefer,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PersonType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Order,
            50 => Self::Open,
            51 => Self::Fund,
            52 => Self::Settlement,
            53 => Self::Company,
            54 => Self::Corporation,
            55 => Self::LinkMan,
            56 => Self::Ledger,
            57 => Self::Trustee,
            65 => Self::TrusteeCorporation,
            66 => Self::TrusteeOpen,
            67 => Self::TrusteeContact,
            68 => Self::ForeignerRefer,
            69 => Self::CorporationRefer,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Order => Some(49),
            Self::Open => Some(50),
            Self::Fund => Some(51),
            Self::Settlement => Some(52),
            Self::Company => Some(53),
            Self::Corporation => Some(54),
            Self::LinkMan => Some(55),
            Self::Ledger => Some(56),
            Self::Trustee => Some(57),
            Self::TrusteeCorporation => Some(65),
            Self::TrusteeOpen => Some(66),
            Self::TrusteeContact => Some(67),
            Self::ForeignerRefer => Some(68),
            Self::CorporationRefer => Some(69),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PersonType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPortfTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortfType {
    /// 使用初版交易所算法。CTP：`THOST_FTDC_EET_None = '0'`。
    None,
    /// SPBM算法V1.1.0_附加保证金调整。线值：`'1'`；名称已规范化，原始常量见审核表。
    SpbmAddOnHedge,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PortfType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::SpbmAddOnHedge,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::SpbmAddOnHedge => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PortfType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPortfolioType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Portfolio {
    /// 不使用新型组保算法。CTP：`THOST_FTDC_EPF_None = '0'`。
    None,
    /// SPBM算法。CTP：`THOST_FTDC_EPF_SPBM = '1'`。
    SPBM,
    /// RULE算法。CTP：`THOST_FTDC_EPF_RULE = '2'`。
    RULE,
    /// SPMM算法。CTP：`THOST_FTDC_EPF_SPMM = '3'`。
    SPMM,
    /// RCAMS算法。CTP：`THOST_FTDC_EPF_RCAMS = '4'`。
    RCAMS,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl Portfolio {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::SPBM,
            50 => Self::RULE,
            51 => Self::SPMM,
            52 => Self::RCAMS,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::SPBM => Some(49),
            Self::RULE => Some(50),
            Self::SPMM => Some(51),
            Self::RCAMS => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for Portfolio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPosiDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PosiDirection {
    /// 净。CTP：`THOST_FTDC_PD_Net = '1'`。
    Net,
    /// 多头。CTP：`THOST_FTDC_PD_Long = '2'`。
    Long,
    /// 空头。CTP：`THOST_FTDC_PD_Short = '3'`。
    Short,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PosiDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Net,
            50 => Self::Long,
            51 => Self::Short,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Net => Some(49),
            Self::Long => Some(50),
            Self::Short => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PosiDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPositionDateType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionDate {
    /// 今日持仓。CTP：`THOST_FTDC_PSD_Today = '1'`。
    Today,
    /// 历史持仓。CTP：`THOST_FTDC_PSD_History = '2'`。
    History,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PositionDate {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Today,
            50 => Self::History,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Today => Some(49),
            Self::History => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PositionDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPositionDateTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionDateType {
    /// 使用历史持仓。CTP：`THOST_FTDC_PDT_UseHistory = '1'`。
    UseHistory,
    /// 不使用历史持仓。CTP：`THOST_FTDC_PDT_NoUseHistory = '2'`。
    NoUseHistory,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PositionDateType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::UseHistory,
            50 => Self::NoUseHistory,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UseHistory => Some(49),
            Self::NoUseHistory => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PositionDateType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPositionTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionType {
    /// 净持仓。CTP：`THOST_FTDC_PT_Net = '1'`。
    Net,
    /// 综合持仓。CTP：`THOST_FTDC_PT_Gross = '2'`。
    Gross,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PositionType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Net,
            50 => Self::Gross,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Net => Some(49),
            Self::Gross => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PositionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPriceSourceType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PriceSource {
    /// 前成交价。CTP：`THOST_FTDC_PSRC_LastPrice = '0'`。
    LastPrice,
    /// 买委托价。CTP：`THOST_FTDC_PSRC_Buy = '1'`。
    Buy,
    /// 卖委托价。CTP：`THOST_FTDC_PSRC_Sell = '2'`。
    Sell,
    /// 场外成交价。CTP：`THOST_FTDC_PSRC_OTC = '3'`。
    OTC,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PriceSource {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::LastPrice,
            49 => Self::Buy,
            50 => Self::Sell,
            51 => Self::OTC,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::LastPrice => Some(48),
            Self::Buy => Some(49),
            Self::Sell => Some(50),
            Self::OTC => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PriceSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcProcessStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessStatus {
    /// 未处理。CTP：`THOST_FTDC_PSS_NotProcess = '0'`。
    NotProcess,
    /// 开始处理。CTP：`THOST_FTDC_PSS_StartProcess = '1'`。
    StartProcess,
    /// 处理完成。CTP：`THOST_FTDC_PSS_Finished = '2'`。
    Finished,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProcessStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NotProcess,
            49 => Self::StartProcess,
            50 => Self::Finished,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotProcess => Some(48),
            Self::StartProcess => Some(49),
            Self::Finished => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcProdChangeFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProdChangeFlag {
    /// 持仓量和冻结量均无变化。CTP：`THOST_FTDC_PCF_None = '0'`。
    None,
    /// 持仓量无变化，冻结量有变化。CTP：`THOST_FTDC_PCF_OnlyFrozen = '1'`。
    OnlyFrozen,
    /// 持仓量有变化。CTP：`THOST_FTDC_PCF_PositionChange = '2'`。
    PositionChange,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProdChangeFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::OnlyFrozen,
            50 => Self::PositionChange,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::OnlyFrozen => Some(49),
            Self::PositionChange => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProdChangeFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcProductClassType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductClass {
    /// 期货。CTP：`THOST_FTDC_PC_Futures = '1'`。
    Futures,
    /// 期货期权。CTP：`THOST_FTDC_PC_Options = '2'`。
    Options,
    /// 组合。CTP：`THOST_FTDC_PC_Combination = '3'`。
    Combination,
    /// 即期。CTP：`THOST_FTDC_PC_Spot = '4'`。
    Spot,
    /// 期转现。CTP：`THOST_FTDC_PC_EFP = '5'`。
    EFP,
    /// 现货期权。CTP：`THOST_FTDC_PC_SpotOption = '6'`。
    SpotOption,
    /// TAS合约。CTP：`THOST_FTDC_PC_TAS = '7'`。
    TAS,
    /// 金属指数。CTP：`THOST_FTDC_PC_MI = 'I'`。
    MI,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProductClass {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Futures,
            50 => Self::Options,
            51 => Self::Combination,
            52 => Self::Spot,
            53 => Self::EFP,
            54 => Self::SpotOption,
            55 => Self::TAS,
            73 => Self::MI,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Futures => Some(49),
            Self::Options => Some(50),
            Self::Combination => Some(51),
            Self::Spot => Some(52),
            Self::EFP => Some(53),
            Self::SpotOption => Some(54),
            Self::TAS => Some(55),
            Self::MI => Some(73),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProductClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcProductLifePhaseType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductLifePhase {
    /// 活跃。CTP：`THOST_FTDC_PLP_Active = '1'`。
    Active,
    /// 不活跃。CTP：`THOST_FTDC_PLP_NonActive = '2'`。
    NonActive,
    /// 注销。CTP：`THOST_FTDC_PLP_Canceled = '3'`。
    Canceled,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProductLifePhase {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Active,
            50 => Self::NonActive,
            51 => Self::Canceled,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Active => Some(49),
            Self::NonActive => Some(50),
            Self::Canceled => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProductLifePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcProductStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductStatus {
    /// 可交易。线值：`'1'`；名称已规范化，原始常量见审核表。
    Tradable,
    /// 不可交易。线值：`'2'`；名称已规范化，原始常量见审核表。
    NotTradable,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProductStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Tradable,
            50 => Self::NotTradable,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Tradable => Some(49),
            Self::NotTradable => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProductStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcProductTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductType {
    /// 期货。CTP：`THOST_FTDC_PTE_Futures = '1'`。
    Futures,
    /// 期权。CTP：`THOST_FTDC_PTE_Options = '2'`。
    Options,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProductType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Futures,
            50 => Self::Options,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Futures => Some(49),
            Self::Options => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProductType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPromptTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptType {
    /// 合约上下市。CTP：`THOST_FTDC_CPT_Instrument = '1'`。
    Instrument,
    /// 保证金分段生效。CTP：`THOST_FTDC_CPT_Margin = '2'`。
    Margin,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PromptType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Instrument,
            50 => Self::Margin,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Instrument => Some(49),
            Self::Margin => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PromptType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPropertyInvestorRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyInvestorRange {
    /// 所有。CTP：`THOST_FTDC_PIR_All = '1'`。
    All,
    /// 投资者属性。CTP：`THOST_FTDC_PIR_Property = '2'`。
    Property,
    /// 单一投资者。CTP：`THOST_FTDC_PIR_Single = '3'`。
    Single,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PropertyInvestorRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::Property,
            51 => Self::Single,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::Property => Some(50),
            Self::Single => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PropertyInvestorRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolId {
    /// 期商协议。线值：`'0'`；名称已规范化，原始常量见审核表。
    FutureProtocol,
    /// 工行协议。线值：`'1'`；名称已规范化，原始常量见审核表。
    ICBCProtocol,
    /// 农行协议。线值：`'2'`；名称已规范化，原始常量见审核表。
    ABCProtocol,
    /// 中国银行协议。线值：`'3'`；名称已规范化，原始常量见审核表。
    CBCProtocol,
    /// 建行协议。线值：`'4'`；名称已规范化，原始常量见审核表。
    CCBProtocol,
    /// 交行协议。线值：`'5'`；名称已规范化，原始常量见审核表。
    BOCOMProtocol,
    /// 银期转帐平台协议。线值：`'X'`；名称已规范化，原始常量见审核表。
    FBTPlatformProtocol,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ProtocolId {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::FutureProtocol,
            49 => Self::ICBCProtocol,
            50 => Self::ABCProtocol,
            51 => Self::CBCProtocol,
            52 => Self::CCBProtocol,
            53 => Self::BOCOMProtocol,
            88 => Self::FBTPlatformProtocol,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FutureProtocol => Some(48),
            Self::ICBCProtocol => Some(49),
            Self::ABCProtocol => Some(50),
            Self::CBCProtocol => Some(51),
            Self::CCBProtocol => Some(52),
            Self::BOCOMProtocol => Some(53),
            Self::FBTPlatformProtocol => Some(88),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ProtocolId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPublishStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublishStatus {
    /// 未发布。CTP：`THOST_FTDC_PS_None = '1'`。
    None,
    /// 正在发布。CTP：`THOST_FTDC_PS_Publishing = '2'`。
    Publishing,
    /// 已发布。CTP：`THOST_FTDC_PS_Published = '3'`。
    Published,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PublishStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::None,
            50 => Self::Publishing,
            51 => Self::Published,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(49),
            Self::Publishing => Some(50),
            Self::Published => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PublishStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPwdFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PwdFlag {
    /// 不核对。CTP：`THOST_FTDC_BPWDF_NoCheck = '0'`。
    NoCheck,
    /// 明文核对。CTP：`THOST_FTDC_BPWDF_BlankCheck = '1'`。
    BlankCheck,
    /// 密文核对。CTP：`THOST_FTDC_BPWDF_EncryptCheck = '2'`。
    EncryptCheck,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PwdFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoCheck,
            49 => Self::BlankCheck,
            50 => Self::EncryptCheck,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoCheck => Some(48),
            Self::BlankCheck => Some(49),
            Self::EncryptCheck => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PwdFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcPwdRcdSrcType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PwdRcdSrc {
    /// 来源于Sync初始化数据。CTP：`THOST_FTDC_PRS_Init = '0'`。
    Init,
    /// 来源于实时上场数据。CTP：`THOST_FTDC_PRS_Sync = '1'`。
    Sync,
    /// 来源于用户修改。CTP：`THOST_FTDC_PRS_UserUpd = '2'`。
    UserUpd,
    /// 来源于超户修改，很可能来自主席同步数据。CTP：`THOST_FTDC_PRS_SuperUserUpd = '3'`。
    SuperUserUpd,
    /// 来源于次席同步的修改。CTP：`THOST_FTDC_PRS_SecUpd = '4'`。
    SecUpd,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl PwdRcdSrc {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Init,
            49 => Self::Sync,
            50 => Self::UserUpd,
            51 => Self::SuperUserUpd,
            52 => Self::SecUpd,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Init => Some(48),
            Self::Sync => Some(49),
            Self::UserUpd => Some(50),
            Self::SuperUserUpd => Some(51),
            Self::SecUpd => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for PwdRcdSrc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcQueryInvestorRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryInvestorRange {
    /// 所有。CTP：`THOST_FTDC_QIR_All = '1'`。
    All,
    /// 查询分类。CTP：`THOST_FTDC_QIR_Group = '2'`。
    Group,
    /// 单一投资者。CTP：`THOST_FTDC_QIR_Single = '3'`。
    Single,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl QueryInvestorRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::Group,
            51 => Self::Single,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::Group => Some(50),
            Self::Single => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for QueryInvestorRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcQuestionTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestionType {
    /// 单选。CTP：`THOST_FTDC_QT_Radio = '1'`。
    Radio,
    /// 多选。CTP：`THOST_FTDC_QT_Option = '2'`。
    Option,
    /// 填空。CTP：`THOST_FTDC_QT_Blank = '3'`。
    Blank,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl QuestionType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Radio,
            50 => Self::Option,
            51 => Self::Blank,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Radio => Some(49),
            Self::Option => Some(50),
            Self::Blank => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for QuestionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRCAMSCombinationTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RCAMSCombinationType {
    /// 牛市看涨价差组合。CTP：`THOST_FTDC_ERComb_BUC = '0'`。
    BUC,
    /// 熊市看涨价差组合。CTP：`THOST_FTDC_ERComb_BEC = '1'`。
    BEC,
    /// 熊市看跌价差组合。CTP：`THOST_FTDC_ERComb_BEP = '2'`。
    BEP,
    /// 牛市看跌价差组合。CTP：`THOST_FTDC_ERComb_BUP = '3'`。
    BUP,
    /// 日历价差组合。CTP：`THOST_FTDC_ERComb_CAS = '4'`。
    CAS,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RCAMSCombinationType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::BUC,
            49 => Self::BEC,
            50 => Self::BEP,
            51 => Self::BUP,
            52 => Self::CAS,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BUC => Some(48),
            Self::BEC => Some(49),
            Self::BEP => Some(50),
            Self::BUP => Some(51),
            Self::CAS => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RCAMSCombinationType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRateInvestorRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateInvestorRange {
    /// 公司标准。CTP：`THOST_FTDC_RIR_All = '1'`。
    All,
    /// 模板。CTP：`THOST_FTDC_RIR_Model = '2'`。
    Model,
    /// 单一投资者。CTP：`THOST_FTDC_RIR_Single = '3'`。
    Single,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RateInvestorRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::Model,
            51 => Self::Single,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::Model => Some(50),
            Self::Single => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RateInvestorRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRateTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateType {
    /// 保证金率。CTP：`THOST_FTDC_RATETYPE_MarginRate = '2'`。
    MarginRate,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RateType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            50 => Self::MarginRate,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::MarginRate => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RateType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRatioAttrType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RatioAttr {
    /// 交易费率。CTP：`THOST_FTDC_RA_Trade = '0'`。
    Trade,
    /// 结算费率。CTP：`THOST_FTDC_RA_Settlement = '1'`。
    Settlement,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RatioAttr {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Trade,
            49 => Self::Settlement,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Trade => Some(48),
            Self::Settlement => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RatioAttr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReasonType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reason {
    /// 错单。CTP：`THOST_FTDC_RN_CD = '0'`。
    CD,
    /// 资金在途。CTP：`THOST_FTDC_RN_ZT = '1'`。
    ZT,
    /// 其它。CTP：`THOST_FTDC_RN_QT = '2'`。
    QT,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl Reason {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::CD,
            49 => Self::ZT,
            50 => Self::QT,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CD => Some(48),
            Self::ZT => Some(49),
            Self::QT => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReportStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportStatus {
    /// 未生成报表数据。CTP：`THOST_FTDC_SRS_NoCreate = '0'`。
    NoCreate,
    /// 报表数据生成中。CTP：`THOST_FTDC_SRS_Create = '1'`。
    Create,
    /// 已生成报表数据。CTP：`THOST_FTDC_SRS_Created = '2'`。
    Created,
    /// 生成报表数据失败。CTP：`THOST_FTDC_SRS_CreateFail = '3'`。
    CreateFail,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReportStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoCreate,
            49 => Self::Create,
            50 => Self::Created,
            51 => Self::CreateFail,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoCreate => Some(48),
            Self::Create => Some(49),
            Self::Created => Some(50),
            Self::CreateFail => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReportStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReqFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReqFlag {
    /// 未发送。CTP：`THOST_FTDC_REQF_NoSend = '0'`。
    NoSend,
    /// 发送成功。CTP：`THOST_FTDC_REQF_SendSuccess = '1'`。
    SendSuccess,
    /// 发送失败。CTP：`THOST_FTDC_REQF_SendFailed = '2'`。
    SendFailed,
    /// 等待重发。CTP：`THOST_FTDC_REQF_WaitReSend = '3'`。
    WaitReSend,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReqFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoSend,
            49 => Self::SendSuccess,
            50 => Self::SendFailed,
            51 => Self::WaitReSend,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoSend => Some(48),
            Self::SendSuccess => Some(49),
            Self::SendFailed => Some(50),
            Self::WaitReSend => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReqFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReqRspTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReqRspType {
    /// 请求。CTP：`THOST_FTDC_REQRSP_Request = '0'`。
    Request,
    /// 响应。CTP：`THOST_FTDC_REQRSP_Response = '1'`。
    Response,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReqRspType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Request,
            49 => Self::Response,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Request => Some(48),
            Self::Response => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReqRspType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcResFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResFlag {
    /// 成功。CTP：`THOST_FTDC_RESF_Success = '0'`。
    Success,
    /// 账户余额不足。线值：`'1'`；名称已规范化，原始常量见审核表。
    Insufficient,
    /// 交易结果未知。CTP：`THOST_FTDC_RESF_UnKnown = '8'`。
    UnKnown,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ResFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Success,
            49 => Self::Insufficient,
            56 => Self::UnKnown,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Success => Some(48),
            Self::Insufficient => Some(49),
            Self::UnKnown => Some(56),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ResFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReserveOpenAccStasType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReserveOpenAccStas {
    /// 等待处理中。CTP：`THOST_FTDC_ROAST_Processing = '0'`。
    Processing,
    /// 已撤销。CTP：`THOST_FTDC_ROAST_Cancelled = '1'`。
    Cancelled,
    /// 已开户。CTP：`THOST_FTDC_ROAST_Opened = '2'`。
    Opened,
    /// 无效请求。CTP：`THOST_FTDC_ROAST_Invalid = '3'`。
    Invalid,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReserveOpenAccStas {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Processing,
            49 => Self::Cancelled,
            50 => Self::Opened,
            51 => Self::Invalid,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Processing => Some(48),
            Self::Cancelled => Some(49),
            Self::Opened => Some(50),
            Self::Invalid => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReserveOpenAccStas {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcResponseValueType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseValue {
    /// 检查成功。CTP：`THOST_FTDC_RV_Right = '0'`。
    Right,
    /// 检查失败。CTP：`THOST_FTDC_RV_Refuse = '1'`。
    Refuse,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ResponseValue {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Right,
            49 => Self::Refuse,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Right => Some(48),
            Self::Refuse => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ResponseValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResumeType {
    /// 从本交易日开始重传。线值：`0`；名称已规范化，原始常量见审核表。
    Restart,
    /// 从上次收到的位置续传。线值：`1`；名称已规范化，原始常量见审核表。
    Resume,
    /// 只传送登录后的内容。线值：`2`；名称已规范化，原始常量见审核表。
    Quick,
    /// 不订阅或取消订阅。线值：`3`；名称已规范化，原始常量见审核表。
    None,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ResumeType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Restart,
            1 => Self::Resume,
            2 => Self::Quick,
            3 => Self::None,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Restart => Some(0),
            Self::Resume => Some(1),
            Self::Quick => Some(2),
            Self::None => Some(3),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ResumeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReturnLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnLevel {
    /// 级别1。CTP：`THOST_FTDC_RL_Level1 = '1'`。
    Level1,
    /// 级别2。CTP：`THOST_FTDC_RL_Level2 = '2'`。
    Level2,
    /// 级别3。CTP：`THOST_FTDC_RL_Level3 = '3'`。
    Level3,
    /// 级别4。CTP：`THOST_FTDC_RL_Level4 = '4'`。
    Level4,
    /// 级别5。CTP：`THOST_FTDC_RL_Level5 = '5'`。
    Level5,
    /// 级别6。CTP：`THOST_FTDC_RL_Level6 = '6'`。
    Level6,
    /// 级别7。CTP：`THOST_FTDC_RL_Level7 = '7'`。
    Level7,
    /// 级别8。CTP：`THOST_FTDC_RL_Level8 = '8'`。
    Level8,
    /// 级别9。CTP：`THOST_FTDC_RL_Level9 = '9'`。
    Level9,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReturnLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Level1,
            50 => Self::Level2,
            51 => Self::Level3,
            52 => Self::Level4,
            53 => Self::Level5,
            54 => Self::Level6,
            55 => Self::Level7,
            56 => Self::Level8,
            57 => Self::Level9,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Level1 => Some(49),
            Self::Level2 => Some(50),
            Self::Level3 => Some(51),
            Self::Level4 => Some(52),
            Self::Level5 => Some(53),
            Self::Level6 => Some(54),
            Self::Level7 => Some(55),
            Self::Level8 => Some(56),
            Self::Level9 => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReturnLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReturnPatternType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnPattern {
    /// 按成交手数。CTP：`THOST_FTDC_RP_ByVolume = '1'`。
    ByVolume,
    /// 按留存手续费。CTP：`THOST_FTDC_RP_ByFeeOnHand = '2'`。
    ByFeeOnHand,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReturnPattern {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ByVolume,
            50 => Self::ByFeeOnHand,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByVolume => Some(49),
            Self::ByFeeOnHand => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReturnPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReturnStandardType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnStandard {
    /// 分阶段返还。CTP：`THOST_FTDC_RSD_ByPeriod = '1'`。
    ByPeriod,
    /// 按某一标准。CTP：`THOST_FTDC_RSD_ByStandard = '2'`。
    ByStandard,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReturnStandard {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ByPeriod,
            50 => Self::ByStandard,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByPeriod => Some(49),
            Self::ByStandard => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReturnStandard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcReturnStyleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnStyle {
    /// 按所有品种。CTP：`THOST_FTDC_RS_All = '1'`。
    All,
    /// 按品种。CTP：`THOST_FTDC_RS_ByProduct = '2'`。
    ByProduct,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ReturnStyle {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::All,
            50 => Self::ByProduct,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(49),
            Self::ByProduct => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ReturnStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRightParamTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RightParamType {
    /// 休眠户。CTP：`THOST_FTDC_RPT_Freeze = '1'`。
    Freeze,
    /// 激活休眠户。CTP：`THOST_FTDC_RPT_FreezeActive = '2'`。
    FreezeActive,
    /// 开仓权限限制。CTP：`THOST_FTDC_RPT_OpenLimit = '3'`。
    OpenLimit,
    /// 解除开仓权限限制。CTP：`THOST_FTDC_RPT_RelieveOpenLimit = '4'`。
    RelieveOpenLimit,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RightParamType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Freeze,
            50 => Self::FreezeActive,
            51 => Self::OpenLimit,
            52 => Self::RelieveOpenLimit,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Freeze => Some(49),
            Self::FreezeActive => Some(50),
            Self::OpenLimit => Some(51),
            Self::RelieveOpenLimit => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RightParamType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRiskLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskLevel {
    /// 低风险客户。CTP：`THOST_FTDC_FAS_Low = '1'`。
    Low,
    /// 普通客户。CTP：`THOST_FTDC_FAS_Normal = '2'`。
    Normal,
    /// 关注客户。CTP：`THOST_FTDC_FAS_Focus = '3'`。
    Focus,
    /// 风险客户。CTP：`THOST_FTDC_FAS_Risk = '4'`。
    Risk,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RiskLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Low,
            50 => Self::Normal,
            51 => Self::Focus,
            52 => Self::Risk,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Low => Some(49),
            Self::Normal => Some(50),
            Self::Focus => Some(51),
            Self::Risk => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRiskNotifyMethodType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskNotifyMethod {
    /// 系统通知。CTP：`THOST_FTDC_RNM_System = '0'`。
    System,
    /// 短信通知。CTP：`THOST_FTDC_RNM_SMS = '1'`。
    SMS,
    /// 邮件通知。CTP：`THOST_FTDC_RNM_EMail = '2'`。
    EMail,
    /// 人工通知。CTP：`THOST_FTDC_RNM_Manual = '3'`。
    Manual,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RiskNotifyMethod {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::System,
            49 => Self::SMS,
            50 => Self::EMail,
            51 => Self::Manual,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::System => Some(48),
            Self::SMS => Some(49),
            Self::EMail => Some(50),
            Self::Manual => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RiskNotifyMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRiskNotifyStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskNotifyStatus {
    /// 未生成。CTP：`THOST_FTDC_RNS_NotGen = '0'`。
    NotGen,
    /// 已生成未发送。CTP：`THOST_FTDC_RNS_Generated = '1'`。
    Generated,
    /// 发送失败。CTP：`THOST_FTDC_RNS_SendError = '2'`。
    SendError,
    /// 已发送未接收。CTP：`THOST_FTDC_RNS_SendOk = '3'`。
    SendOk,
    /// 已接收未确认。CTP：`THOST_FTDC_RNS_Received = '4'`。
    Received,
    /// 已确认。CTP：`THOST_FTDC_RNS_Confirmed = '5'`。
    Confirmed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RiskNotifyStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NotGen,
            49 => Self::Generated,
            50 => Self::SendError,
            51 => Self::SendOk,
            52 => Self::Received,
            53 => Self::Confirmed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotGen => Some(48),
            Self::Generated => Some(49),
            Self::SendError => Some(50),
            Self::SendOk => Some(51),
            Self::Received => Some(52),
            Self::Confirmed => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RiskNotifyStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcRiskUserEventType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskUserEvent {
    /// 导出数据。CTP：`THOST_FTDC_RUE_ExportData = '0'`。
    ExportData,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl RiskUserEvent {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ExportData,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ExportData => Some(48),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for RiskUserEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSHFEUploadFileNameType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SHFEUploadFileName {
    /// ^\d{4}_\d{8}_\d{8}_DailyFundChg。CTP：`THOST_FTDC_SUFN_SUFN_O = 'O'`。
    O,
    /// ^\d{4}_\d{8}_\d{8}_Trade。CTP：`THOST_FTDC_SUFN_SUFN_T = 'T'`。
    T,
    /// ^\d{4}_\d{8}_\d{8}_SettlementDetail。CTP：`THOST_FTDC_SUFN_SUFN_P = 'P'`。
    P,
    /// ^\d{4}_\d{8}_\d{8}_Capital。CTP：`THOST_FTDC_SUFN_SUFN_F = 'F'`。
    F,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SHFEUploadFileName {
    pub const fn from_code(code: u8) -> Self {
        match code {
            79 => Self::O,
            84 => Self::T,
            80 => Self::P,
            70 => Self::F,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::O => Some(79),
            Self::T => Some(84),
            Self::P => Some(80),
            Self::F => Some(70),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SHFEUploadFileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSaveStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveStatus {
    /// 归档未完成。CTP：`THOST_FTDC_SSS_UnSaveData = '0'`。
    UnSaveData,
    /// 归档完成。线值：`'1'`；名称已规范化，原始常量见审核表。
    DataSaved,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SaveStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnSaveData,
            49 => Self::DataSaved,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnSaveData => Some(48),
            Self::DataSaved => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SaveStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSecuAccTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecuAccType {
    /// 资金帐号。CTP：`THOST_FTDC_SAT_AccountID = '1'`。
    AccountID,
    /// 资金卡号。CTP：`THOST_FTDC_SAT_CardID = '2'`。
    CardID,
    /// 上海股东帐号。CTP：`THOST_FTDC_SAT_SHStockholderID = '3'`。
    SHStockholderID,
    /// 深圳股东帐号。CTP：`THOST_FTDC_SAT_SZStockholderID = '4'`。
    SZStockholderID,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SecuAccType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::AccountID,
            50 => Self::CardID,
            51 => Self::SHStockholderID,
            52 => Self::SZStockholderID,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::AccountID => Some(49),
            Self::CardID => Some(50),
            Self::SHStockholderID => Some(51),
            Self::SZStockholderID => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SecuAccType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSendMethodType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SendMethod {
    /// 文件发送。CTP：`THOST_FTDC_UOASM_ByAPI = '1'`。
    ByAPI,
    /// 电子发送。CTP：`THOST_FTDC_UOASM_ByFile = '2'`。
    ByFile,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SendMethod {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ByAPI,
            50 => Self::ByFile,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ByAPI => Some(49),
            Self::ByFile => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SendMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSendTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SendType {
    /// 未发送。CTP：`THOST_FTDC_UOAST_NoSend = '0'`。
    NoSend,
    /// 已发送。线值：`'1'`；名称已规范化，原始常量见审核表。
    Sent,
    /// 已生成。CTP：`THOST_FTDC_UOAST_Generated = '2'`。
    Generated,
    /// 报送失败。CTP：`THOST_FTDC_UOAST_SendFail = '3'`。
    SendFail,
    /// 接收成功。CTP：`THOST_FTDC_UOAST_Success = '4'`。
    Success,
    /// 接收失败。CTP：`THOST_FTDC_UOAST_Fail = '5'`。
    Fail,
    /// 取消报送。CTP：`THOST_FTDC_UOAST_Cancel = '6'`。
    Cancel,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SendType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoSend,
            49 => Self::Sent,
            50 => Self::Generated,
            51 => Self::SendFail,
            52 => Self::Success,
            53 => Self::Fail,
            54 => Self::Cancel,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoSend => Some(48),
            Self::Sent => Some(49),
            Self::Generated => Some(50),
            Self::SendFail => Some(51),
            Self::Success => Some(52),
            Self::Fail => Some(53),
            Self::Cancel => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SendType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettArchiveStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettArchiveStatus {
    /// 未归档数据。CTP：`THOST_FTDC_SAS_UnArchived = '0'`。
    UnArchived,
    /// 数据归档中。CTP：`THOST_FTDC_SAS_Archiving = '1'`。
    Archiving,
    /// 已归档数据。CTP：`THOST_FTDC_SAS_Archived = '2'`。
    Archived,
    /// 归档数据失败。CTP：`THOST_FTDC_SAS_ArchiveFail = '3'`。
    ArchiveFail,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettArchiveStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UnArchived,
            49 => Self::Archiving,
            50 => Self::Archived,
            51 => Self::ArchiveFail,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UnArchived => Some(48),
            Self::Archiving => Some(49),
            Self::Archived => Some(50),
            Self::ArchiveFail => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettArchiveStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettleManagerGroupType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettleManagerGroup {
    /// 交易所核对。线值：`'1'`；名称已规范化，原始常量见审核表。
    Exchange,
    /// 内部核对。CTP：`THOST_FTDC_SMG_ASP = '2'`。
    ASP,
    /// 上报数据核对。CTP：`THOST_FTDC_SMG_CSRC = '3'`。
    CSRC,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettleManagerGroup {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Exchange,
            50 => Self::ASP,
            51 => Self::CSRC,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Exchange => Some(49),
            Self::ASP => Some(50),
            Self::CSRC => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettleManagerGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettleManagerLevelType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettleManagerLevel {
    /// 必要。CTP：`THOST_FTDC_SML_Must = '1'`。
    Must,
    /// 警告。CTP：`THOST_FTDC_SML_Alarm = '2'`。
    Alarm,
    /// 提示。CTP：`THOST_FTDC_SML_Prompt = '3'`。
    Prompt,
    /// 不检查。CTP：`THOST_FTDC_SML_Ignore = '4'`。
    Ignore,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettleManagerLevel {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Must,
            50 => Self::Alarm,
            51 => Self::Prompt,
            52 => Self::Ignore,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Must => Some(49),
            Self::Alarm => Some(50),
            Self::Prompt => Some(51),
            Self::Ignore => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettleManagerLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettleManagerTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettleManagerType {
    /// 结算前准备。CTP：`THOST_FTDC_SMT_Before = '1'`。
    Before,
    /// 结算。CTP：`THOST_FTDC_SMT_Settlement = '2'`。
    Settlement,
    /// 结算后核对。CTP：`THOST_FTDC_SMT_After = '3'`。
    After,
    /// 结算后处理。线值：`'4'`；名称已规范化，原始常量见审核表。
    Settled,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettleManagerType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Before,
            50 => Self::Settlement,
            51 => Self::After,
            52 => Self::Settled,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Before => Some(49),
            Self::Settlement => Some(50),
            Self::After => Some(51),
            Self::Settled => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettleManagerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettlementBillTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettlementBillType {
    /// 日报。CTP：`THOST_FTDC_ST_Day = '0'`。
    Day,
    /// 月报。CTP：`THOST_FTDC_ST_Month = '1'`。
    Month,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettlementBillType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Day,
            49 => Self::Month,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Day => Some(48),
            Self::Month => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettlementBillType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettlementStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettlementStatus {
    /// 初始。CTP：`THOST_FTDC_STS_Initialize = '0'`。
    Initialize,
    /// 结算中。线值：`'1'`；名称已规范化，原始常量见审核表。
    Settling,
    /// 已结算。线值：`'2'`；名称已规范化，原始常量见审核表。
    Settled,
    /// 结算完成。CTP：`THOST_FTDC_STS_Finished = '3'`。
    Finished,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettlementStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Initialize,
            49 => Self::Settling,
            50 => Self::Settled,
            51 => Self::Finished,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Initialize => Some(48),
            Self::Settling => Some(49),
            Self::Settled => Some(50),
            Self::Finished => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettlementStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSettlementStyleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettlementStyle {
    /// 逐日盯市。CTP：`THOST_FTDC_SBS_Day = '1'`。
    Day,
    /// 逐笔对冲。CTP：`THOST_FTDC_SBS_Volume = '2'`。
    Volume,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SettlementStyle {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Day,
            50 => Self::Volume,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Day => Some(49),
            Self::Volume => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SettlementStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSexType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sex {
    /// 未知。CTP：`THOST_FTDC_SEX_None = '0'`。
    None,
    /// 男。CTP：`THOST_FTDC_SEX_Man = '1'`。
    Man,
    /// 女。CTP：`THOST_FTDC_SEX_Woman = '2'`。
    Woman,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl Sex {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::None,
            49 => Self::Man,
            50 => Self::Woman,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::None => Some(48),
            Self::Man => Some(49),
            Self::Woman => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSpecPosiTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecPosiType {
    /// 普通持仓明细。CTP：`THOST_FTDC_SPOST_Common = '#'`。
    Common,
    /// TAS合约成交产生的标的合约持仓明细。CTP：`THOST_FTDC_SPOST_Tas = '0'`。
    Tas,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SpecPosiType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            35 => Self::Common,
            48 => Self::Tas,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Common => Some(35),
            Self::Tas => Some(48),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SpecPosiType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSpecProductTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecProductType {
    /// 郑商所套保产品。CTP：`THOST_FTDC_SPT_CzceHedge = '1'`。
    CzceHedge,
    /// 货币质押产品。CTP：`THOST_FTDC_SPT_IneForeignCurrency = '2'`。
    IneForeignCurrency,
    /// 大连短线开平仓产品。CTP：`THOST_FTDC_SPT_DceOpenClose = '3'`。
    DceOpenClose,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SpecProductType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::CzceHedge,
            50 => Self::IneForeignCurrency,
            51 => Self::DceOpenClose,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CzceHedge => Some(49),
            Self::IneForeignCurrency => Some(50),
            Self::DceOpenClose => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SpecProductType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSpecialCreateRuleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialCreateRule {
    /// 没有特殊创建规则。CTP：`THOST_FTDC_SC_NoSpecialRule = '0'`。
    NoSpecialRule,
    /// 不包含春节。CTP：`THOST_FTDC_SC_NoSpringFestival = '1'`。
    NoSpringFestival,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SpecialCreateRule {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoSpecialRule,
            49 => Self::NoSpringFestival,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoSpecialRule => Some(48),
            Self::NoSpringFestival => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SpecialCreateRule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSponsorTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SponsorType {
    /// 期商。CTP：`THOST_FTDC_SPTYPE_Broker = '0'`。
    Broker,
    /// 银行。CTP：`THOST_FTDC_SPTYPE_Bank = '1'`。
    Bank,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SponsorType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Broker,
            49 => Self::Bank,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Broker => Some(48),
            Self::Bank => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SponsorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcStandardStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardStatus {
    /// 已规范。CTP：`THOST_FTDC_STST_Standard = '0'`。
    Standard,
    /// 未规范。CTP：`THOST_FTDC_STST_NonStandard = '1'`。
    NonStandard,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl StandardStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Standard,
            49 => Self::NonStandard,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Standard => Some(48),
            Self::NonStandard => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for StandardStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcStartModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartMode {
    /// 正常。CTP：`THOST_FTDC_SM_Normal = '1'`。
    Normal,
    /// 应急。CTP：`THOST_FTDC_SM_Emerge = '2'`。
    Emerge,
    /// 恢复。CTP：`THOST_FTDC_SM_Restore = '3'`。
    Restore,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl StartMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Normal,
            50 => Self::Emerge,
            51 => Self::Restore,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(49),
            Self::Emerge => Some(50),
            Self::Restore => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for StartMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcStatModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatMode {
    /// ----。CTP：`THOST_FTDC_SM_Non = '0'`。
    Non,
    /// 按合约统计。CTP：`THOST_FTDC_SM_Instrument = '1'`。
    Instrument,
    /// 按产品统计。CTP：`THOST_FTDC_SM_Product = '2'`。
    Product,
    /// 按投资者统计。CTP：`THOST_FTDC_SM_Investor = '3'`。
    Investor,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl StatMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Non,
            49 => Self::Instrument,
            50 => Self::Product,
            51 => Self::Investor,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Non => Some(48),
            Self::Instrument => Some(49),
            Self::Product => Some(50),
            Self::Investor => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for StatMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcStrikeModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrikeMode {
    /// 欧式。CTP：`THOST_FTDC_STM_Continental = '0'`。
    Continental,
    /// 美式。CTP：`THOST_FTDC_STM_American = '1'`。
    American,
    /// 百慕大。CTP：`THOST_FTDC_STM_Bermuda = '2'`。
    Bermuda,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl StrikeMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Continental,
            49 => Self::American,
            50 => Self::Bermuda,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Continental => Some(48),
            Self::American => Some(49),
            Self::Bermuda => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for StrikeMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcStrikeOffsetTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrikeOffsetType {
    /// 实值额。CTP：`THOST_FTDC_STOV_RealValue = '1'`。
    RealValue,
    /// 盈利额。CTP：`THOST_FTDC_STOV_ProfitValue = '2'`。
    ProfitValue,
    /// 实值比例。CTP：`THOST_FTDC_STOV_RealRatio = '3'`。
    RealRatio,
    /// 盈利比例。CTP：`THOST_FTDC_STOV_ProfitRatio = '4'`。
    ProfitRatio,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl StrikeOffsetType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::RealValue,
            50 => Self::ProfitValue,
            51 => Self::RealRatio,
            52 => Self::ProfitRatio,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::RealValue => Some(49),
            Self::ProfitValue => Some(50),
            Self::RealRatio => Some(51),
            Self::ProfitRatio => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for StrikeOffsetType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcStrikeTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrikeType {
    /// 自身对冲。CTP：`THOST_FTDC_STT_Hedge = '0'`。
    Hedge,
    /// 匹配执行。线值：`'1'`；名称已规范化，原始常量见审核表。
    MatchValue,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl StrikeType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Hedge,
            49 => Self::MatchValue,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Hedge => Some(48),
            Self::MatchValue => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for StrikeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSwapSourceTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwapSourceType {
    /// 手工。CTP：`THOST_FTDC_SST_Manual = '0'`。
    Manual,
    /// 自动生成。CTP：`THOST_FTDC_SST_Automatic = '1'`。
    Automatic,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SwapSourceType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Manual,
            49 => Self::Automatic,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Manual => Some(48),
            Self::Automatic => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SwapSourceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSyncDataStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncDataStatus {
    /// 未同步。CTP：`THOST_FTDC_SDS_Initialize = '0'`。
    Initialize,
    /// 同步中。线值：`'1'`；名称已规范化，原始常量见审核表。
    Settling,
    /// 已同步。线值：`'2'`；名称已规范化，原始常量见审核表。
    Settled,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SyncDataStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Initialize,
            49 => Self::Settling,
            50 => Self::Settled,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Initialize => Some(48),
            Self::Settling => Some(49),
            Self::Settled => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SyncDataStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSyncDeltaStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncDeltaStatus {
    /// 交易可读。CTP：`THOST_FTDC_SDS_Readable = '1'`。
    Readable,
    /// 交易在读。CTP：`THOST_FTDC_SDS_Reading = '2'`。
    Reading,
    /// 交易读取完成。线值：`'3'`；名称已规范化，原始常量见审核表。
    ReadCompleted,
    /// 追平失败 交易本地状态结算不存在。CTP：`THOST_FTDC_SDS_OptErr = 'e'`。
    OptErr,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SyncDeltaStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Readable,
            50 => Self::Reading,
            51 => Self::ReadCompleted,
            101 => Self::OptErr,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Readable => Some(49),
            Self::Reading => Some(50),
            Self::ReadCompleted => Some(51),
            Self::OptErr => Some(101),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SyncDeltaStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSyncFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncFlag {
    /// 已同步。CTP：`THOST_FTDC_SYNF_Yes = '0'`。
    Yes,
    /// 未同步。CTP：`THOST_FTDC_SYNF_No = '1'`。
    No,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SyncFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Yes,
            49 => Self::No,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Yes => Some(48),
            Self::No => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SyncFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSyncModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncMode {
    /// 异步。线值：`'0'`；名称已规范化，原始常量见审核表。
    ASyncValue,
    /// 同步。CTP：`THOST_FTDC_SRM_Sync = '1'`。
    Sync,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SyncMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ASyncValue,
            49 => Self::Sync,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ASyncValue => Some(48),
            Self::Sync => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SyncMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSyncTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyncType {
    /// 一次同步。CTP：`THOST_FTDC_SYNT_OneOffSync = '0'`。
    OneOffSync,
    /// 定时同步。CTP：`THOST_FTDC_SYNT_TimerSync = '1'`。
    TimerSync,
    /// 定时完全同步。CTP：`THOST_FTDC_SYNT_TimerFullSync = '2'`。
    TimerFullSync,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SyncType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::OneOffSync,
            49 => Self::TimerSync,
            50 => Self::TimerFullSync,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::OneOffSync => Some(48),
            Self::TimerSync => Some(49),
            Self::TimerFullSync => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SyncType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSysOperModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SysOperMode {
    /// 增加。CTP：`THOST_FTDC_SoM_Add = '1'`。
    Add,
    /// 修改。CTP：`THOST_FTDC_SoM_Update = '2'`。
    Update,
    /// 删除。CTP：`THOST_FTDC_SoM_Delete = '3'`。
    Delete,
    /// 复制。CTP：`THOST_FTDC_SoM_Copy = '4'`。
    Copy,
    /// 激活。CTP：`THOST_FTDC_SoM_AcTive = '5'`。
    AcTive,
    /// 注销。CTP：`THOST_FTDC_SoM_CanCel = '6'`。
    CanCel,
    /// 重置。CTP：`THOST_FTDC_SoM_ReSet = '7'`。
    ReSet,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SysOperMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Add,
            50 => Self::Update,
            51 => Self::Delete,
            52 => Self::Copy,
            53 => Self::AcTive,
            54 => Self::CanCel,
            55 => Self::ReSet,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Add => Some(49),
            Self::Update => Some(50),
            Self::Delete => Some(51),
            Self::Copy => Some(52),
            Self::AcTive => Some(53),
            Self::CanCel => Some(54),
            Self::ReSet => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SysOperMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSysOperTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SysOperType {
    /// 修改操作员密码。CTP：`THOST_FTDC_SoT_UpdatePassword = '0'`。
    UpdatePassword,
    /// 操作员组织架构关系。CTP：`THOST_FTDC_SoT_UserDepartment = '1'`。
    UserDepartment,
    /// 角色管理。CTP：`THOST_FTDC_SoT_RoleManager = '2'`。
    RoleManager,
    /// 角色功能设置。CTP：`THOST_FTDC_SoT_RoleFunction = '3'`。
    RoleFunction,
    /// 基础参数设置。CTP：`THOST_FTDC_SoT_BaseParam = '4'`。
    BaseParam,
    /// 设置操作员。CTP：`THOST_FTDC_SoT_SetUserID = '5'`。
    SetUserID,
    /// 用户角色设置。CTP：`THOST_FTDC_SoT_SetUserRole = '6'`。
    SetUserRole,
    /// 用户IP限制。CTP：`THOST_FTDC_SoT_UserIpRestriction = '7'`。
    UserIpRestriction,
    /// 组织架构管理。CTP：`THOST_FTDC_SoT_DepartmentManager = '8'`。
    DepartmentManager,
    /// 组织架构向查询分类复制。CTP：`THOST_FTDC_SoT_DepartmentCopy = '9'`。
    DepartmentCopy,
    /// 交易编码管理。线值：`'A'`；名称已规范化，原始常量见审核表。
    TradingCode,
    /// 投资者状态维护。CTP：`THOST_FTDC_SoT_InvestorStatus = 'B'`。
    InvestorStatus,
    /// 投资者权限管理。CTP：`THOST_FTDC_SoT_InvestorAuthority = 'C'`。
    InvestorAuthority,
    /// 属性设置。CTP：`THOST_FTDC_SoT_PropertySet = 'D'`。
    PropertySet,
    /// 重置投资者密码。CTP：`THOST_FTDC_SoT_ReSetInvestorPasswd = 'E'`。
    ReSetInvestorPasswd,
    /// 投资者个性信息维护。CTP：`THOST_FTDC_SoT_InvestorPersonalityInfo = 'F'`。
    InvestorPersonalityInfo,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SysOperType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::UpdatePassword,
            49 => Self::UserDepartment,
            50 => Self::RoleManager,
            51 => Self::RoleFunction,
            52 => Self::BaseParam,
            53 => Self::SetUserID,
            54 => Self::SetUserRole,
            55 => Self::UserIpRestriction,
            56 => Self::DepartmentManager,
            57 => Self::DepartmentCopy,
            65 => Self::TradingCode,
            66 => Self::InvestorStatus,
            67 => Self::InvestorAuthority,
            68 => Self::PropertySet,
            69 => Self::ReSetInvestorPasswd,
            70 => Self::InvestorPersonalityInfo,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::UpdatePassword => Some(48),
            Self::UserDepartment => Some(49),
            Self::RoleManager => Some(50),
            Self::RoleFunction => Some(51),
            Self::BaseParam => Some(52),
            Self::SetUserID => Some(53),
            Self::SetUserRole => Some(54),
            Self::UserIpRestriction => Some(55),
            Self::DepartmentManager => Some(56),
            Self::DepartmentCopy => Some(57),
            Self::TradingCode => Some(65),
            Self::InvestorStatus => Some(66),
            Self::InvestorAuthority => Some(67),
            Self::PropertySet => Some(68),
            Self::ReSetInvestorPasswd => Some(69),
            Self::InvestorPersonalityInfo => Some(70),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SysOperType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSysSettlementStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SysSettlementStatus {
    /// 不活跃。CTP：`THOST_FTDC_SS_NonActive = '1'`。
    NonActive,
    /// 启动。CTP：`THOST_FTDC_SS_Startup = '2'`。
    Startup,
    /// 操作。CTP：`THOST_FTDC_SS_Operating = '3'`。
    Operating,
    /// 结算。CTP：`THOST_FTDC_SS_Settlement = '4'`。
    Settlement,
    /// 结算完成。CTP：`THOST_FTDC_SS_SettlementFinished = '5'`。
    SettlementFinished,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SysSettlementStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NonActive,
            50 => Self::Startup,
            51 => Self::Operating,
            52 => Self::Settlement,
            53 => Self::SettlementFinished,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NonActive => Some(49),
            Self::Startup => Some(50),
            Self::Operating => Some(51),
            Self::Settlement => Some(52),
            Self::SettlementFinished => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SysSettlementStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSystemParamIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemParamID {
    /// 投资者代码最小长度。CTP：`THOST_FTDC_SPI_InvestorIDMinLength = '1'`。
    InvestorIDMinLength,
    /// 投资者帐号代码最小长度。CTP：`THOST_FTDC_SPI_AccountIDMinLength = '2'`。
    AccountIDMinLength,
    /// 投资者开户默认登录权限。CTP：`THOST_FTDC_SPI_UserRightLogon = '3'`。
    UserRightLogon,
    /// 投资者交易结算单成交汇总方式。CTP：`THOST_FTDC_SPI_SettlementBillTrade = '4'`。
    SettlementBillTrade,
    /// 统一开户更新交易编码方式。CTP：`THOST_FTDC_SPI_TradingCode = '5'`。
    TradingCode,
    /// 结算是否判断存在未复核的出入金和分项资金。CTP：`THOST_FTDC_SPI_CheckFund = '6'`。
    CheckFund,
    /// 是否启用手续费模板数据权限。CTP：`THOST_FTDC_SPI_CommModelRight = '7'`。
    CommModelRight,
    /// 是否启用保证金率模板数据权限。CTP：`THOST_FTDC_SPI_MarginModelRight = '9'`。
    MarginModelRight,
    /// 是否规范用户才能激活。CTP：`THOST_FTDC_SPI_IsStandardActive = '8'`。
    IsStandardActive,
    /// 上传的交易所结算文件路径。CTP：`THOST_FTDC_SPI_UploadSettlementFile = 'U'`。
    UploadSettlementFile,
    /// 上报保证金监控中心文件路径。CTP：`THOST_FTDC_SPI_DownloadCSRCFile = 'D'`。
    DownloadCSRCFile,
    /// 生成的结算单文件路径。CTP：`THOST_FTDC_SPI_SettlementBillFile = 'S'`。
    SettlementBillFile,
    /// 证监会文件标识。CTP：`THOST_FTDC_SPI_CSRCOthersFile = 'C'`。
    CSRCOthersFile,
    /// 投资者照片路径。CTP：`THOST_FTDC_SPI_InvestorPhoto = 'P'`。
    InvestorPhoto,
    /// 全结经纪公司上传文件路径。CTP：`THOST_FTDC_SPI_CSRCData = 'R'`。
    CSRCData,
    /// 开户密码录入方式。CTP：`THOST_FTDC_SPI_InvestorPwdModel = 'I'`。
    InvestorPwdModel,
    /// 投资者中金所结算文件下载路径。CTP：`THOST_FTDC_SPI_CFFEXInvestorSettleFile = 'F'`。
    CFFEXInvestorSettleFile,
    /// 投资者代码编码方式。CTP：`THOST_FTDC_SPI_InvestorIDType = 'a'`。
    InvestorIDType,
    /// 休眠户最高权益。CTP：`THOST_FTDC_SPI_FreezeMaxReMain = 'r'`。
    FreezeMaxReMain,
    /// 手续费相关操作实时上场开关。CTP：`THOST_FTDC_SPI_IsSync = 'A'`。
    IsSync,
    /// 解除开仓权限限制。CTP：`THOST_FTDC_SPI_RelieveOpenLimit = 'O'`。
    RelieveOpenLimit,
    /// 是否规范用户才能休眠。CTP：`THOST_FTDC_SPI_IsStandardFreeze = 'X'`。
    IsStandardFreeze,
    /// 郑商所是否开放所有品种套保交易。CTP：`THOST_FTDC_SPI_CZCENormalProductHedge = 'B'`。
    CZCENormalProductHedge,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SystemParamID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::InvestorIDMinLength,
            50 => Self::AccountIDMinLength,
            51 => Self::UserRightLogon,
            52 => Self::SettlementBillTrade,
            53 => Self::TradingCode,
            54 => Self::CheckFund,
            55 => Self::CommModelRight,
            57 => Self::MarginModelRight,
            56 => Self::IsStandardActive,
            85 => Self::UploadSettlementFile,
            68 => Self::DownloadCSRCFile,
            83 => Self::SettlementBillFile,
            67 => Self::CSRCOthersFile,
            80 => Self::InvestorPhoto,
            82 => Self::CSRCData,
            73 => Self::InvestorPwdModel,
            70 => Self::CFFEXInvestorSettleFile,
            97 => Self::InvestorIDType,
            114 => Self::FreezeMaxReMain,
            65 => Self::IsSync,
            79 => Self::RelieveOpenLimit,
            88 => Self::IsStandardFreeze,
            66 => Self::CZCENormalProductHedge,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::InvestorIDMinLength => Some(49),
            Self::AccountIDMinLength => Some(50),
            Self::UserRightLogon => Some(51),
            Self::SettlementBillTrade => Some(52),
            Self::TradingCode => Some(53),
            Self::CheckFund => Some(54),
            Self::CommModelRight => Some(55),
            Self::MarginModelRight => Some(57),
            Self::IsStandardActive => Some(56),
            Self::UploadSettlementFile => Some(85),
            Self::DownloadCSRCFile => Some(68),
            Self::SettlementBillFile => Some(83),
            Self::CSRCOthersFile => Some(67),
            Self::InvestorPhoto => Some(80),
            Self::CSRCData => Some(82),
            Self::InvestorPwdModel => Some(73),
            Self::CFFEXInvestorSettleFile => Some(70),
            Self::InvestorIDType => Some(97),
            Self::FreezeMaxReMain => Some(114),
            Self::IsSync => Some(65),
            Self::RelieveOpenLimit => Some(79),
            Self::IsStandardFreeze => Some(88),
            Self::CZCENormalProductHedge => Some(66),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SystemParamID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSystemStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemStatus {
    /// 不活跃。CTP：`THOST_FTDC_ES_NonActive = '1'`。
    NonActive,
    /// 启动。CTP：`THOST_FTDC_ES_Startup = '2'`。
    Startup,
    /// 交易开始初始化。CTP：`THOST_FTDC_ES_Initialize = '3'`。
    Initialize,
    /// 交易完成初始化。CTP：`THOST_FTDC_ES_Initialized = '4'`。
    Initialized,
    /// 收市开始。CTP：`THOST_FTDC_ES_Close = '5'`。
    Close,
    /// 收市完成。CTP：`THOST_FTDC_ES_Closed = '6'`。
    Closed,
    /// 结算。CTP：`THOST_FTDC_ES_Settlement = '7'`。
    Settlement,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SystemStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NonActive,
            50 => Self::Startup,
            51 => Self::Initialize,
            52 => Self::Initialized,
            53 => Self::Close,
            54 => Self::Closed,
            55 => Self::Settlement,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NonActive => Some(49),
            Self::Startup => Some(50),
            Self::Initialize => Some(51),
            Self::Initialized => Some(52),
            Self::Close => Some(53),
            Self::Closed => Some(54),
            Self::Settlement => Some(55),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SystemStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcSystemTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemType {
    /// 银期转帐。CTP：`THOST_FTDC_SYT_FutureBankTransfer = '0'`。
    FutureBankTransfer,
    /// 银证转帐。CTP：`THOST_FTDC_SYT_StockBankTransfer = '1'`。
    StockBankTransfer,
    /// 第三方存管。CTP：`THOST_FTDC_SYT_TheThirdPartStore = '2'`。
    TheThirdPartStore,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl SystemType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::FutureBankTransfer,
            49 => Self::StockBankTransfer,
            50 => Self::TheThirdPartStore,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::FutureBankTransfer => Some(48),
            Self::StockBankTransfer => Some(49),
            Self::TheThirdPartStore => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for SystemType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TGSessionQryStatusType {
    /// 查询状态空闲。CTP：`THOST_FTDC_TGQS_QryIdle = '1'`。
    QryIdle,
    /// 查询状态频繁。CTP：`THOST_FTDC_TGQS_QryBusy = '2'`。
    QryBusy,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TGSessionQryStatusType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::QryIdle,
            50 => Self::QryBusy,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::QryIdle => Some(49),
            Self::QryBusy => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TGSessionQryStatusType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTemplateTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateType {
    /// 全量。CTP：`THOST_FTDC_TPT_Full = '1'`。
    Full,
    /// 增量。CTP：`THOST_FTDC_TPT_Increment = '2'`。
    Increment,
    /// 备份。CTP：`THOST_FTDC_TPT_BackUp = '3'`。
    BackUp,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TemplateType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Full,
            50 => Self::Increment,
            51 => Self::BackUp,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Full => Some(49),
            Self::Increment => Some(50),
            Self::BackUp => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TemplateType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTimeConditionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeCondition {
    /// 立即完成，否则撤销。CTP：`THOST_FTDC_TC_IOC = '1'`。
    IOC,
    /// 本节有效。CTP：`THOST_FTDC_TC_GFS = '2'`。
    GFS,
    /// 当日有效。CTP：`THOST_FTDC_TC_GFD = '3'`。
    GFD,
    /// 指定日期前有效。CTP：`THOST_FTDC_TC_GTD = '4'`。
    GTD,
    /// 撤销前有效。CTP：`THOST_FTDC_TC_GTC = '5'`。
    GTC,
    /// 集合竞价有效。CTP：`THOST_FTDC_TC_GFA = '6'`。
    GFA,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TimeCondition {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::IOC,
            50 => Self::GFS,
            51 => Self::GFD,
            52 => Self::GTD,
            53 => Self::GTC,
            54 => Self::GFA,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::IOC => Some(49),
            Self::GFS => Some(50),
            Self::GFD => Some(51),
            Self::GTD => Some(52),
            Self::GTC => Some(53),
            Self::GFA => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TimeCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTimeRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeRange {
    /// 一般月份。CTP：`THOST_FTDC_ETR_USUAL = '1'`。
    USUAL,
    /// 交割月前一个月上半月。CTP：`THOST_FTDC_ETR_FNSP = '2'`。
    FNSP,
    /// 交割月前一个月下半月。CTP：`THOST_FTDC_ETR_BNSP = '3'`。
    BNSP,
    /// 交割月份。CTP：`THOST_FTDC_ETR_SPOT = '4'`。
    SPOT,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TimeRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::USUAL,
            50 => Self::FNSP,
            51 => Self::BNSP,
            52 => Self::SPOT,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::USUAL => Some(49),
            Self::FNSP => Some(50),
            Self::BNSP => Some(51),
            Self::SPOT => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TimeRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradeParamIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeParamID {
    /// 系统加密算法。CTP：`THOST_FTDC_TPID_EncryptionStandard = 'E'`。
    EncryptionStandard,
    /// 系统风险算法。CTP：`THOST_FTDC_TPID_RiskMode = 'R'`。
    RiskMode,
    /// 系统风险算法是否全局 0-否 1-是。CTP：`THOST_FTDC_TPID_RiskModeGlobal = 'G'`。
    RiskModeGlobal,
    /// 密码加密算法。线值：`'P'`；名称已规范化，原始常量见审核表。
    ModeEncode,
    /// 价格小数位数参数。线值：`'T'`；名称已规范化，原始常量见审核表。
    TickMode,
    /// 用户最大会话数。CTP：`THOST_FTDC_TPID_SingleUserSessionMaxNum = 'S'`。
    SingleUserSessionMaxNum,
    /// 最大连续登录失败数。CTP：`THOST_FTDC_TPID_LoginFailMaxNum = 'L'`。
    LoginFailMaxNum,
    /// 是否强制认证。CTP：`THOST_FTDC_TPID_IsAuthForce = 'A'`。
    IsAuthForce,
    /// 是否冻结证券持仓。CTP：`THOST_FTDC_TPID_IsPosiFreeze = 'F'`。
    IsPosiFreeze,
    /// 是否限仓。CTP：`THOST_FTDC_TPID_IsPosiLimit = 'M'`。
    IsPosiLimit,
    /// 郑商所询价时间间隔。CTP：`THOST_FTDC_TPID_ForQuoteTimeInterval = 'Q'`。
    ForQuoteTimeInterval,
    /// 是否期货限仓。CTP：`THOST_FTDC_TPID_IsFuturePosiLimit = 'B'`。
    IsFuturePosiLimit,
    /// 是否期货下单频率限制。CTP：`THOST_FTDC_TPID_IsFutureOrderFreq = 'C'`。
    IsFutureOrderFreq,
    /// 行权冻结是否计算盈利。CTP：`THOST_FTDC_TPID_IsExecOrderProfit = 'H'`。
    IsExecOrderProfit,
    /// 银期开户是否验证开户银行卡号是否是预留银行账户。CTP：`THOST_FTDC_TPID_IsCheckBankAcc = 'I'`。
    IsCheckBankAcc,
    /// 弱密码最后修改日期。CTP：`THOST_FTDC_TPID_PasswordDeadLine = 'J'`。
    PasswordDeadLine,
    /// 强密码校验。CTP：`THOST_FTDC_TPID_IsStrongPassword = 'K'`。
    IsStrongPassword,
    /// 自有资金质押比。线值：`'a'`；名称已规范化，原始常量见审核表。
    BalanceMortgage,
    /// 最小密码长度。CTP：`THOST_FTDC_TPID_MinPwdLen = 'O'`。
    MinPwdLen,
    /// IP当日最大登陆失败次数。CTP：`THOST_FTDC_TPID_LoginFailMaxNumForIP = 'U'`。
    LoginFailMaxNumForIP,
    /// 密码有效期。CTP：`THOST_FTDC_TPID_PasswordPeriod = 'V'`。
    PasswordPeriod,
    /// 历史密码重复限制次数。CTP：`THOST_FTDC_TPID_PwdHistoryCmp = 'X'`。
    PwdHistoryCmp,
    /// 转账是否验证预留银行账户。线值：`'i'`；名称已规范化，原始常量见审核表。
    TransferCheckProperty,
    /// 非交易时间异常报单校验参数。CTP：`THOST_FTDC_TPID_TradeChkPhase = 'j'`。
    TradeChkPhase,
    /// 其他异常报单校验参数（价格和手数）。CTP：`THOST_FTDC_TPID_TradeChkPriceVol = 'k'`。
    TradeChkPriceVol,
    /// 卖出垂直价差组合新算法。CTP：`THOST_FTDC_TPID_NewBESMarginAlgo = 'l'`。
    NewBESMarginAlgo,
    /// 大商所期货组合平仓冻结算法。CTP：`THOST_FTDC_TPID_IsDceFutCloseFrozen = 'm'`。
    IsDceFutCloseFrozen,
    /// 是否启用短信验证。CTP：`THOST_FTDC_TPID_UseSMSVerify = 'n'`。
    UseSMSVerify,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradeParamID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            69 => Self::EncryptionStandard,
            82 => Self::RiskMode,
            71 => Self::RiskModeGlobal,
            80 => Self::ModeEncode,
            84 => Self::TickMode,
            83 => Self::SingleUserSessionMaxNum,
            76 => Self::LoginFailMaxNum,
            65 => Self::IsAuthForce,
            70 => Self::IsPosiFreeze,
            77 => Self::IsPosiLimit,
            81 => Self::ForQuoteTimeInterval,
            66 => Self::IsFuturePosiLimit,
            67 => Self::IsFutureOrderFreq,
            72 => Self::IsExecOrderProfit,
            73 => Self::IsCheckBankAcc,
            74 => Self::PasswordDeadLine,
            75 => Self::IsStrongPassword,
            97 => Self::BalanceMortgage,
            79 => Self::MinPwdLen,
            85 => Self::LoginFailMaxNumForIP,
            86 => Self::PasswordPeriod,
            88 => Self::PwdHistoryCmp,
            105 => Self::TransferCheckProperty,
            106 => Self::TradeChkPhase,
            107 => Self::TradeChkPriceVol,
            108 => Self::NewBESMarginAlgo,
            109 => Self::IsDceFutCloseFrozen,
            110 => Self::UseSMSVerify,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::EncryptionStandard => Some(69),
            Self::RiskMode => Some(82),
            Self::RiskModeGlobal => Some(71),
            Self::ModeEncode => Some(80),
            Self::TickMode => Some(84),
            Self::SingleUserSessionMaxNum => Some(83),
            Self::LoginFailMaxNum => Some(76),
            Self::IsAuthForce => Some(65),
            Self::IsPosiFreeze => Some(70),
            Self::IsPosiLimit => Some(77),
            Self::ForQuoteTimeInterval => Some(81),
            Self::IsFuturePosiLimit => Some(66),
            Self::IsFutureOrderFreq => Some(67),
            Self::IsExecOrderProfit => Some(72),
            Self::IsCheckBankAcc => Some(73),
            Self::PasswordDeadLine => Some(74),
            Self::IsStrongPassword => Some(75),
            Self::BalanceMortgage => Some(97),
            Self::MinPwdLen => Some(79),
            Self::LoginFailMaxNumForIP => Some(85),
            Self::PasswordPeriod => Some(86),
            Self::PwdHistoryCmp => Some(88),
            Self::TransferCheckProperty => Some(105),
            Self::TradeChkPhase => Some(106),
            Self::TradeChkPriceVol => Some(107),
            Self::NewBESMarginAlgo => Some(108),
            Self::IsDceFutCloseFrozen => Some(109),
            Self::UseSMSVerify => Some(110),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradeParamID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradeSourceType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeSource {
    /// 来自交易所普通回报。CTP：`THOST_FTDC_TSRC_NORMAL = '0'`。
    NORMAL,
    /// 来自查询。CTP：`THOST_FTDC_TSRC_QUERY = '1'`。
    QUERY,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradeSource {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NORMAL,
            49 => Self::QUERY,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NORMAL => Some(48),
            Self::QUERY => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradeSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradeSumStatModeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeSumStatMode {
    /// 按合约统计。CTP：`THOST_FTDC_TSSM_Instrument = '1'`。
    Instrument,
    /// 按产品统计。CTP：`THOST_FTDC_TSSM_Product = '2'`。
    Product,
    /// 按交易所统计。CTP：`THOST_FTDC_TSSM_Exchange = '3'`。
    Exchange,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradeSumStatMode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Instrument,
            50 => Self::Product,
            51 => Self::Exchange,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Instrument => Some(49),
            Self::Product => Some(50),
            Self::Exchange => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradeSumStatMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradeTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeType {
    /// 组合持仓拆分为单一持仓,初始化不应包含该类型的持仓。CTP：`THOST_FTDC_TRDT_SplitCombination = '#'`。
    SplitCombination,
    /// 普通成交。CTP：`THOST_FTDC_TRDT_Common = '0'`。
    Common,
    /// 期权执行。CTP：`THOST_FTDC_TRDT_OptionsExecution = '1'`。
    OptionsExecution,
    /// OTC成交。CTP：`THOST_FTDC_TRDT_OTC = '2'`。
    OTC,
    /// 期转现衍生成交。CTP：`THOST_FTDC_TRDT_EFPDerived = '3'`。
    EFPDerived,
    /// 组合衍生成交。CTP：`THOST_FTDC_TRDT_CombinationDerived = '4'`。
    CombinationDerived,
    /// 大宗交易成交。CTP：`THOST_FTDC_TRDT_BlockTrade = '5'`。
    BlockTrade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradeType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            35 => Self::SplitCombination,
            48 => Self::Common,
            49 => Self::OptionsExecution,
            50 => Self::OTC,
            51 => Self::EFPDerived,
            52 => Self::CombinationDerived,
            53 => Self::BlockTrade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::SplitCombination => Some(35),
            Self::Common => Some(48),
            Self::OptionsExecution => Some(49),
            Self::OTC => Some(50),
            Self::EFPDerived => Some(51),
            Self::CombinationDerived => Some(52),
            Self::BlockTrade => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTraderConnectStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraderConnectStatus {
    /// 没有任何连接。CTP：`THOST_FTDC_TCS_NotConnected = '1'`。
    NotConnected,
    /// 已经连接。CTP：`THOST_FTDC_TCS_Connected = '2'`。
    Connected,
    /// 已经发出合约查询请求。CTP：`THOST_FTDC_TCS_QryInstrumentSent = '3'`。
    QryInstrumentSent,
    /// 订阅私有流。CTP：`THOST_FTDC_TCS_SubPrivateFlow = '4'`。
    SubPrivateFlow,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TraderConnectStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::NotConnected,
            50 => Self::Connected,
            51 => Self::QryInstrumentSent,
            52 => Self::SubPrivateFlow,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NotConnected => Some(49),
            Self::Connected => Some(50),
            Self::QryInstrumentSent => Some(51),
            Self::SubPrivateFlow => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TraderConnectStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradingRightType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradingRight {
    /// 可以交易。CTP：`THOST_FTDC_TR_Allow = '0'`。
    Allow,
    /// 只能平仓。CTP：`THOST_FTDC_TR_CloseOnly = '1'`。
    CloseOnly,
    /// 不能交易。CTP：`THOST_FTDC_TR_Forbidden = '2'`。
    Forbidden,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradingRight {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Allow,
            49 => Self::CloseOnly,
            50 => Self::Forbidden,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Allow => Some(48),
            Self::CloseOnly => Some(49),
            Self::Forbidden => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradingRight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradingRoleType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradingRole {
    /// 代理。CTP：`THOST_FTDC_ER_Broker = '1'`。
    Broker,
    /// 自营。CTP：`THOST_FTDC_ER_Host = '2'`。
    Host,
    /// 做市商。CTP：`THOST_FTDC_ER_Maker = '3'`。
    Maker,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradingRole {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Broker,
            50 => Self::Host,
            51 => Self::Maker,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Broker => Some(49),
            Self::Host => Some(50),
            Self::Maker => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradingRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTradingTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradingType {
    /// 所有状态。CTP：`THOST_FTDC_TD_ALL = '0'`。
    ALL,
    /// 交易。CTP：`THOST_FTDC_TD_TRADE = '1'`。
    TRADE,
    /// 非交易。线值：`'2'`；名称已规范化，原始常量见审核表。
    NoTrade,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TradingType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::ALL,
            49 => Self::TRADE,
            50 => Self::NoTrade,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ALL => Some(48),
            Self::TRADE => Some(49),
            Self::NoTrade => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TradingType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTransferDirectionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransferDirection {
    /// 移出。CTP：`THOST_FTDC_TD_Out = '0'`。
    Out,
    /// 移入。线值：`'1'`；名称已规范化，原始常量见审核表。
    InValue,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TransferDirection {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Out,
            49 => Self::InValue,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Out => Some(48),
            Self::InValue => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TransferDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTransferStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransferStatus {
    /// 正常。CTP：`THOST_FTDC_TRFS_Normal = '0'`。
    Normal,
    /// 被冲正。CTP：`THOST_FTDC_TRFS_Repealed = '1'`。
    Repealed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TransferStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Normal,
            49 => Self::Repealed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Normal => Some(48),
            Self::Repealed => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TransferStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTransferTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransferType {
    /// 银行转期货。CTP：`THOST_FTDC_TT_BankToFuture = '0'`。
    BankToFuture,
    /// 期货转银行。CTP：`THOST_FTDC_TT_FutureToBank = '1'`。
    FutureToBank,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TransferType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::BankToFuture,
            49 => Self::FutureToBank,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BankToFuture => Some(48),
            Self::FutureToBank => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TransferType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTransferValidFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransferValidFlag {
    /// 无效或失败。CTP：`THOST_FTDC_TVF_Invalid = '0'`。
    Invalid,
    /// 有效。CTP：`THOST_FTDC_TVF_Valid = '1'`。
    Valid,
    /// 冲正。CTP：`THOST_FTDC_TVF_Reverse = '2'`。
    Reverse,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TransferValidFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Invalid,
            49 => Self::Valid,
            50 => Self::Reverse,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Invalid => Some(48),
            Self::Valid => Some(49),
            Self::Reverse => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TransferValidFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcTxnEndFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxnEndFlag {
    /// 正常处理中。CTP：`THOST_FTDC_TEF_NormalProcessing = '0'`。
    NormalProcessing,
    /// 成功结束。CTP：`THOST_FTDC_TEF_Success = '1'`。
    Success,
    /// 失败结束。CTP：`THOST_FTDC_TEF_Failed = '2'`。
    Failed,
    /// 异常中。CTP：`THOST_FTDC_TEF_Abnormal = '3'`。
    Abnormal,
    /// 已人工异常处理。CTP：`THOST_FTDC_TEF_ManualProcessedForException = '4'`。
    ManualProcessedForException,
    /// 通讯异常 ，请人工处理。线值：`'5'`；名称已规范化，原始常量见审核表。
    CommunicationFailedNeedManualProcess,
    /// 系统出错，请人工处理。CTP：`THOST_FTDC_TEF_SysErrorNeedManualProcess = '6'`。
    SysErrorNeedManualProcess,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl TxnEndFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NormalProcessing,
            49 => Self::Success,
            50 => Self::Failed,
            51 => Self::Abnormal,
            52 => Self::ManualProcessedForException,
            53 => Self::CommunicationFailedNeedManualProcess,
            54 => Self::SysErrorNeedManualProcess,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NormalProcessing => Some(48),
            Self::Success => Some(49),
            Self::Failed => Some(50),
            Self::Abnormal => Some(51),
            Self::ManualProcessedForException => Some(52),
            Self::CommunicationFailedNeedManualProcess => Some(53),
            Self::SysErrorNeedManualProcess => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for TxnEndFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型名称已按 Rust 语义规范调整，原始名称见审核表。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UOAAssetManagerType {
    /// 期货类。CTP：`THOST_FTDC_UOAAT_Futures = '1'`。
    Futures,
    /// 综合类。CTP：`THOST_FTDC_UOAAT_SpecialOrgan = '2'`。
    SpecialOrgan,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UOAAssetManagerType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Futures,
            50 => Self::SpecialOrgan,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Futures => Some(49),
            Self::SpecialOrgan => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UOAAssetManagerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUOAAutoSendType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UOAAutoSend {
    /// 自动发送并接收。CTP：`THOST_FTDC_UOAA_ASR = '1'`。
    ASR,
    /// 自动发送，不自动接收。CTP：`THOST_FTDC_UOAA_ASNR = '2'`。
    ASNR,
    /// 不自动发送，自动接收。CTP：`THOST_FTDC_UOAA_NSAR = '3'`。
    NSAR,
    /// 不自动发送，也不自动接收。CTP：`THOST_FTDC_UOAA_NSR = '4'`。
    NSR,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UOAAutoSend {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::ASR,
            50 => Self::ASNR,
            51 => Self::NSAR,
            52 => Self::NSR,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::ASR => Some(49),
            Self::ASNR => Some(50),
            Self::NSAR => Some(51),
            Self::NSR => Some(52),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UOAAutoSend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUpdateFlagType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateFlag {
    /// 未更新。CTP：`THOST_FTDC_UF_NoUpdate = '0'`。
    NoUpdate,
    /// 更新全部信息成功。CTP：`THOST_FTDC_UF_Success = '1'`。
    Success,
    /// 更新全部信息失败。CTP：`THOST_FTDC_UF_Fail = '2'`。
    Fail,
    /// 更新交易编码成功。CTP：`THOST_FTDC_UF_TCSuccess = '3'`。
    TCSuccess,
    /// 更新交易编码失败。CTP：`THOST_FTDC_UF_TCFail = '4'`。
    TCFail,
    /// 已丢弃。CTP：`THOST_FTDC_UF_Cancel = '5'`。
    Cancel,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UpdateFlag {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoUpdate,
            49 => Self::Success,
            50 => Self::Fail,
            51 => Self::TCSuccess,
            52 => Self::TCFail,
            53 => Self::Cancel,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoUpdate => Some(48),
            Self::Success => Some(49),
            Self::Fail => Some(50),
            Self::TCSuccess => Some(51),
            Self::TCFail => Some(52),
            Self::Cancel => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UpdateFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUsedStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsedStatus {
    /// 未生效。CTP：`THOST_FTDC_CHU_Unused = '0'`。
    Unused,
    /// 已生效。CTP：`THOST_FTDC_CHU_Used = '1'`。
    Used,
    /// 生效失败。CTP：`THOST_FTDC_CHU_Fail = '2'`。
    Fail,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UsedStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Unused,
            49 => Self::Used,
            50 => Self::Fail,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Unused => Some(48),
            Self::Used => Some(49),
            Self::Fail => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UsedStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUserEventTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserEventType {
    /// 登录。CTP：`THOST_FTDC_UET_Login = '1'`。
    Login,
    /// 登出。CTP：`THOST_FTDC_UET_Logout = '2'`。
    Logout,
    /// CTP校验通过。CTP：`THOST_FTDC_UET_Trading = '3'`。
    Trading,
    /// CTP校验失败。CTP：`THOST_FTDC_UET_TradingError = '4'`。
    TradingError,
    /// 修改密码。CTP：`THOST_FTDC_UET_UpdatePassword = '5'`。
    UpdatePassword,
    /// 客户端认证。CTP：`THOST_FTDC_UET_Authenticate = '6'`。
    Authenticate,
    /// 终端信息上报。CTP：`THOST_FTDC_UET_SubmitSysInfo = '7'`。
    SubmitSysInfo,
    /// 转账。CTP：`THOST_FTDC_UET_Transfer = '8'`。
    Transfer,
    /// 其他。CTP：`THOST_FTDC_UET_Other = '9'`。
    Other,
    /// 修改资金密码。CTP：`THOST_FTDC_UET_UpdateTradingAccountPassword = 'a'`。
    UpdateTradingAccountPassword,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UserEventType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Login,
            50 => Self::Logout,
            51 => Self::Trading,
            52 => Self::TradingError,
            53 => Self::UpdatePassword,
            54 => Self::Authenticate,
            55 => Self::SubmitSysInfo,
            56 => Self::Transfer,
            57 => Self::Other,
            97 => Self::UpdateTradingAccountPassword,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Login => Some(49),
            Self::Logout => Some(50),
            Self::Trading => Some(51),
            Self::TradingError => Some(52),
            Self::UpdatePassword => Some(53),
            Self::Authenticate => Some(54),
            Self::SubmitSysInfo => Some(55),
            Self::Transfer => Some(56),
            Self::Other => Some(57),
            Self::UpdateTradingAccountPassword => Some(97),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UserEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUserRangeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserRange {
    /// 所有。CTP：`THOST_FTDC_UR_All = '0'`。
    All,
    /// 单一操作员。CTP：`THOST_FTDC_UR_Single = '1'`。
    Single,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UserRange {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::All,
            49 => Self::Single,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::All => Some(48),
            Self::Single => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UserRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUserRightTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserRightType {
    /// 登录。CTP：`THOST_FTDC_URT_Logon = '1'`。
    Logon,
    /// 银期转帐。CTP：`THOST_FTDC_URT_Transfer = '2'`。
    Transfer,
    /// 邮寄结算单。CTP：`THOST_FTDC_URT_EMail = '3'`。
    EMail,
    /// 传真结算单。CTP：`THOST_FTDC_URT_Fax = '4'`。
    Fax,
    /// 条件单。CTP：`THOST_FTDC_URT_ConditionOrder = '5'`。
    ConditionOrder,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UserRightType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Logon,
            50 => Self::Transfer,
            51 => Self::EMail,
            52 => Self::Fax,
            53 => Self::ConditionOrder,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Logon => Some(49),
            Self::Transfer => Some(50),
            Self::EMail => Some(51),
            Self::Fax => Some(52),
            Self::ConditionOrder => Some(53),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UserRightType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcUserTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserType {
    /// 投资者。CTP：`THOST_FTDC_UT_Investor = '0'`。
    Investor,
    /// 操作员。CTP：`THOST_FTDC_UT_Operator = '1'`。
    Operator,
    /// 管理员。CTP：`THOST_FTDC_UT_SuperUser = '2'`。
    SuperUser,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl UserType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Investor,
            49 => Self::Operator,
            50 => Self::SuperUser,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Investor => Some(48),
            Self::Operator => Some(49),
            Self::SuperUser => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for UserType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcValueMethodType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueMethod {
    /// 按绝对值。CTP：`THOST_FTDC_VM_Absolute = '0'`。
    Absolute,
    /// 按比率。CTP：`THOST_FTDC_VM_Ratio = '1'`。
    Ratio,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl ValueMethod {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Absolute,
            49 => Self::Ratio,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Absolute => Some(48),
            Self::Ratio => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for ValueMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcVirBankAccTypeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirBankAccType {
    /// 存折。CTP：`THOST_FTDC_VBAT_BankBook = '1'`。
    BankBook,
    /// 储蓄卡。CTP：`THOST_FTDC_VBAT_BankCard = '2'`。
    BankCard,
    /// 信用卡。CTP：`THOST_FTDC_VBAT_CreditCard = '3'`。
    CreditCard,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl VirBankAccType {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::BankBook,
            50 => Self::BankCard,
            51 => Self::CreditCard,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::BankBook => Some(49),
            Self::BankCard => Some(50),
            Self::CreditCard => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for VirBankAccType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcVirDealStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirDealStatus {
    /// 正在处理。CTP：`THOST_FTDC_VDS_Dealing = '1'`。
    Dealing,
    /// 处理成功。线值：`'2'`；名称已规范化，原始常量见审核表。
    DealSucceed,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl VirDealStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Dealing,
            50 => Self::DealSucceed,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Dealing => Some(49),
            Self::DealSucceed => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for VirDealStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcVirTradeStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirTradeStatus {
    /// 正常处理中。CTP：`THOST_FTDC_VTS_NaturalDeal = '0'`。
    NaturalDeal,
    /// 成功结束。CTP：`THOST_FTDC_VTS_SucceedEnd = '1'`。
    SucceedEnd,
    /// 失败结束。CTP：`THOST_FTDC_VTS_FailedEND = '2'`。
    FailedEND,
    /// 异常中。CTP：`THOST_FTDC_VTS_Exception = '3'`。
    Exception,
    /// 已人工异常处理。CTP：`THOST_FTDC_VTS_ManualDeal = '4'`。
    ManualDeal,
    /// 通讯异常 ，请人工处理。CTP：`THOST_FTDC_VTS_MesException = '5'`。
    MesException,
    /// 系统出错，请人工处理。CTP：`THOST_FTDC_VTS_SysException = '6'`。
    SysException,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl VirTradeStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NaturalDeal,
            49 => Self::SucceedEnd,
            50 => Self::FailedEND,
            51 => Self::Exception,
            52 => Self::ManualDeal,
            53 => Self::MesException,
            54 => Self::SysException,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NaturalDeal => Some(48),
            Self::SucceedEnd => Some(49),
            Self::FailedEND => Some(50),
            Self::Exception => Some(51),
            Self::ManualDeal => Some(52),
            Self::MesException => Some(53),
            Self::SysException => Some(54),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for VirTradeStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcVirementAvailAbilityType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirementAvailAbility {
    /// 未确认。CTP：`THOST_FTDC_VAA_NoAvailAbility = '0'`。
    NoAvailAbility,
    /// 有效。CTP：`THOST_FTDC_VAA_AvailAbility = '1'`。
    AvailAbility,
    /// 冲正。CTP：`THOST_FTDC_VAA_Repeal = '2'`。
    Repeal,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl VirementAvailAbility {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::NoAvailAbility,
            49 => Self::AvailAbility,
            50 => Self::Repeal,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::NoAvailAbility => Some(48),
            Self::AvailAbility => Some(49),
            Self::Repeal => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for VirementAvailAbility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcVirementStatusType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirementStatus {
    /// 正常。CTP：`THOST_FTDC_VMS_Natural = '0'`。
    Natural,
    /// 销户。CTP：`THOST_FTDC_VMS_Canceled = '9'`。
    Canceled,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl VirementStatus {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Natural,
            57 => Self::Canceled,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Natural => Some(48),
            Self::Canceled => Some(57),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for VirementStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcVolumeConditionType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolumeCondition {
    /// 任何数量。CTP：`THOST_FTDC_VC_AV = '1'`。
    AV,
    /// 最小数量。CTP：`THOST_FTDC_VC_MV = '2'`。
    MV,
    /// 全部数量。CTP：`THOST_FTDC_VC_CV = '3'`。
    CV,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl VolumeCondition {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::AV,
            50 => Self::MV,
            51 => Self::CV,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::AV => Some(49),
            Self::MV => Some(50),
            Self::CV => Some(51),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for VolumeCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcWeakPasswordSourceType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeakPasswordSource {
    /// 弱密码库。CTP：`THOST_FTDC_WPSR_Lib = '1'`。
    Lib,
    /// 手工录入。CTP：`THOST_FTDC_WPSR_Manual = '2'`。
    Manual,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl WeakPasswordSource {
    pub const fn from_code(code: u8) -> Self {
        match code {
            49 => Self::Lib,
            50 => Self::Manual,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Lib => Some(49),
            Self::Manual => Some(50),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for WeakPasswordSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcWithDrawParamIDType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WithDrawParamID {
    /// 权利金收支是否可提 1 代表可提 0 不可提。CTP：`THOST_FTDC_WDPID_CashIn = 'C'`。
    CashIn,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl WithDrawParamID {
    pub const fn from_code(code: u8) -> Self {
        match code {
            67 => Self::CashIn,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::CashIn => Some(67),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for WithDrawParamID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}

/// CTP 类型：`TThostFtdcYesNoIndicatorType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YesNoIndicator {
    /// 是。CTP：`THOST_FTDC_YNI_Yes = '0'`。
    Yes,
    /// 否。CTP：`THOST_FTDC_YNI_No = '1'`。
    No,
    /// 未识别的响应值，保留原始字节；请求发送时拒绝。
    UnknownValue(u8),
}

impl YesNoIndicator {
    pub const fn from_code(code: u8) -> Self {
        match code {
            48 => Self::Yes,
            49 => Self::No,
            value => Self::UnknownValue(value),
        }
    }

    pub const fn code(self) -> Option<u8> {
        match self {
            Self::Yes => Some(48),
            Self::No => Some(49),
            Self::UnknownValue(_) => None,
        }
    }
}

impl fmt::Display for YesNoIndicator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(value) => write!(formatter, "UnknownValue({value})"),
            _ => write!(formatter, "{:?}", self),
        }
    }
}
