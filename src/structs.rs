// 由 ctp-generator 根据审核表生成，用于类型与原生桥接。
// 请勿手工编辑；正式映射以当前审核表为准。

use crate::enums::*;
use crate::types::*;

/// 银行账户属性
#[derive(Debug, Clone)]
pub struct AccountProperty {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 银行统一标识类型。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行账户。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行账户的开户人名称。CTP：`OpenName`，`TThostFtdcInvestorFullNameType`。
    pub open_name: String,
    /// 银行账户的开户行。CTP：`OpenBank`，`TThostFtdcOpenBankType`。
    pub open_bank: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
    /// 账户来源。CTP：`AccountSourceType`，`TThostFtdcAccountSourceTypeType`。
    pub account_source_type: AccountSourceType,
    /// 开户日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 注销日期。CTP：`CancelDate`，`TThostFtdcDateType`。
    pub cancel_date: DateText,
    /// 录入员代码。CTP：`OperatorID`，`TThostFtdcOperatorIDType`。
    pub operator_id: String,
    /// 录入日期。CTP：`OperateDate`，`TThostFtdcDateType`。
    pub operate_date: DateText,
    /// 录入时间。CTP：`OperateTime`，`TThostFtdcTimeType`。
    pub operate_time: TimeText,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 客户开销户信息表
#[derive(Debug, Clone)]
pub struct AccountRegister {
    /// 交易日期。CTP：`TradeDay`，`TThostFtdcTradeDateType`。
    pub trade_day: String,
    /// 银行编码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构编码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 期货公司编码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期货公司分支机构编码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 开销户类别。CTP：`OpenOrDestroy`，`TThostFtdcOpenOrDestroyType`。
    pub open_or_destroy: OpenOrDestroy,
    /// 签约日期。CTP：`RegDate`，`TThostFtdcTradeDateType`。
    pub reg_date: String,
    /// 解约日期。CTP：`OutDate`，`TThostFtdcTradeDateType`。
    pub out_date: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 服务地址和AppID的关系
#[derive(Debug, Clone)]
pub struct AddrAppIDRelation {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 服务地址。CTP：`Address`，`TThostFtdcIpAddrType`。
    pub address: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
}

/// App客户端认证码
#[derive(Debug, Clone)]
pub struct AppAuthenticationCode {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// 认证码。CTP：`AuthCode`，`TThostFtdcAuthCodeType`。
    pub auth_code: String,
    /// 旧认证码。CTP：`PreAuthCode`，`TThostFtdcAuthCodeType`。
    pub pre_auth_code: String,
    /// App类型。CTP：`AppType`，`TThostFtdcAppTypeType`。
    pub app_type: AppType,
}

/// App客户端权限分配
#[derive(Debug, Clone)]
pub struct AppIDAuthAssign {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
}

/// 禁止认证IP
#[derive(Debug, Clone)]
pub struct AuthForbiddenIP {
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 用户IP绑定信息
#[derive(Debug, Clone)]
pub struct AuthIP {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// 用户代码。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 终端用户绑定信息
#[derive(Debug, Clone)]
pub struct AuthUserID {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 校验类型。CTP：`AuthType`，`TThostFtdcAuthTypeType`。
    pub auth_type: AuthType,
}

/// 客户端认证信息
#[derive(Debug, Clone)]
pub struct AuthenticationInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 认证信息。CTP：`AuthInfo`，`TThostFtdcAuthInfoType`。
    pub auth_info: String,
    /// 是否为认证结果。CTP：`IsResult`，`TThostFtdcBoolType`。
    pub is_result: bool,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// App类型。CTP：`AppType`，`TThostFtdcAppTypeType`。
    pub app_type: AppType,
    /// 终端IP地址。CTP：`ClientIPAddress`，`TThostFtdcIPAddressType`。
    pub client_ip_address: String,
}

/// 批量报单操作
#[derive(Debug, Clone)]
pub struct BatchOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单操作引用。CTP：`OrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub order_action_ref: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 经纪公司
#[derive(Debug, Clone)]
pub struct Broker {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 经纪公司简称。CTP：`BrokerAbbr`，`TThostFtdcBrokerAbbrType`。
    pub broker_abbr: String,
    /// 经纪公司名称。CTP：`BrokerName`，`TThostFtdcBrokerNameType`。
    pub broker_name: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
}

/// 经纪公司资金
#[derive(Debug, Clone)]
pub struct BrokerDeposit {
    /// 交易日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 上次结算准备金。CTP：`PreBalance`，`TThostFtdcMoneyType`。
    pub pre_balance: f64,
    /// 当前保证金总额。CTP：`CurrMargin`，`TThostFtdcMoneyType`。
    pub current_margin: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 期货结算准备金。CTP：`Balance`，`TThostFtdcMoneyType`。
    pub balance: f64,
    /// 入金金额。CTP：`Deposit`，`TThostFtdcMoneyType`。
    pub deposit: f64,
    /// 出金金额。CTP：`Withdraw`，`TThostFtdcMoneyType`。
    pub withdraw: f64,
    /// 可提资金。CTP：`Available`，`TThostFtdcMoneyType`。
    pub available: f64,
    /// 基本准备金。CTP：`Reserve`，`TThostFtdcMoneyType`。
    pub reserve: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
}

/// 经纪公司同步
#[derive(Debug, Clone)]
pub struct BrokerSync {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 经纪公司交易算法
#[derive(Debug, Clone)]
pub struct BrokerTradingAlgos {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 持仓处理算法编号。CTP：`HandlePositionAlgoID`，`TThostFtdcHandlePositionAlgoIDType`。
    pub handle_position_algo_id: HandlePositionAlgoID,
    /// 寻找保证金率算法编号。CTP：`FindMarginRateAlgoID`，`TThostFtdcFindMarginRateAlgoIDType`。
    pub find_margin_rate_algo_id: FindMarginRateAlgoID,
    /// 资金处理算法编号。CTP：`HandleTradingAccountAlgoID`，`TThostFtdcHandleTradingAccountAlgoIDType`。
    pub handle_trading_account_algo_id: HandleTradingAccountAlgoID,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 经纪公司交易参数
#[derive(Debug, Clone)]
pub struct BrokerTradingParams {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 保证金价格类型。CTP：`MarginPriceType`，`TThostFtdcMarginPriceTypeType`。
    pub margin_price_type: MarginPriceType,
    /// 盈亏算法。CTP：`Algorithm`，`TThostFtdcAlgorithmType`。
    pub algorithm: Algorithm,
    /// 可用是否包含平仓盈利。CTP：`AvailIncludeCloseProfit`，`TThostFtdcIncludeCloseProfitType`。
    pub avail_include_close_profit: IncludeCloseProfit,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 期权权利金价格类型。CTP：`OptionRoyaltyPriceType`，`TThostFtdcOptionRoyaltyPriceTypeType`。
    pub option_royalty_price_type: OptionRoyaltyPriceType,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
}

/// 经纪公司用户
#[derive(Debug, Clone)]
pub struct BrokerUser {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户名称。CTP：`UserName`，`TThostFtdcUserNameType`。
    pub user_name: String,
    /// 用户类型。CTP：`UserType`，`TThostFtdcUserTypeType`。
    pub user_type: UserType,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
    /// 是否使用令牌。CTP：`IsUsingOTP`，`TThostFtdcBoolType`。
    pub is_using_otp: bool,
    /// 是否强制终端认证。CTP：`IsAuthForce`，`TThostFtdcBoolType`。
    pub is_auth_force: bool,
}

/// 查询经纪公司用户事件
#[derive(Debug, Clone)]
pub struct BrokerUserEvent {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户事件类型。CTP：`UserEventType`，`TThostFtdcUserEventTypeType`。
    pub user_event_type: UserEventType,
    /// 用户事件序号。CTP：`EventSequenceNo`，`TThostFtdcSequenceNoType`。
    pub event_sequence_no: i32,
    /// 事件发生日期。CTP：`EventDate`，`TThostFtdcDateType`。
    pub event_date: DateText,
    /// 事件发生时间。CTP：`EventTime`，`TThostFtdcTimeType`。
    pub event_time: TimeText,
    /// 用户事件信息。CTP：`UserEventInfo`，`TThostFtdcUserEventInfoType`。
    pub user_event_info: String,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
}

/// 经纪公司用户功能权限
#[derive(Debug, Clone)]
pub struct BrokerUserFunction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 经纪公司功能代码。CTP：`BrokerFunctionCode`，`TThostFtdcBrokerFunctionCodeType`。
    pub broker_function_code: BrokerFunctionCode,
}

/// 用户动态令牌参数
#[derive(Debug, Clone)]
pub struct BrokerUserOTPParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 动态令牌提供商。CTP：`OTPVendorsID`，`TThostFtdcOTPVendorsIDType`。
    pub otp_vendors_id: String,
    /// 动态令牌序列号。CTP：`SerialNumber`，`TThostFtdcSerialNumberType`。
    pub serial_number: String,
    /// 令牌密钥。CTP：`AuthKey`，`TThostFtdcAuthKeyType`。
    pub auth_key: String,
    /// 漂移值。CTP：`LastDrift`，`TThostFtdcLastDriftType`。
    pub last_drift: i32,
    /// 成功值。CTP：`LastSuccess`，`TThostFtdcLastSuccessType`。
    pub last_success: i32,
    /// 动态令牌类型。CTP：`OTPType`，`TThostFtdcOTPTypeType`。
    pub otp_type: OTPType,
}

/// 经纪公司用户口令
#[derive(Debug, Clone)]
pub struct BrokerUserPassword {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 上次修改时间。CTP：`LastUpdateTime`，`TThostFtdcDateTimeType`。
    pub last_update_time: String,
    /// 上次登陆时间。CTP：`LastLoginTime`，`TThostFtdcDateTimeType`。
    pub last_login_time: String,
    /// 密码过期时间。CTP：`ExpireDate`，`TThostFtdcDateType`。
    pub expire_date: DateText,
    /// 弱密码过期时间。CTP：`WeakExpireDate`，`TThostFtdcDateType`。
    pub weak_expire_date: DateText,
}

/// 经济公司是否有在本标示的交易权限
#[derive(Debug, Clone)]
pub struct BrokerUserRightAssign {
    /// 应用单元代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 能否交易。CTP：`Tradeable`，`TThostFtdcBoolType`。
    pub tradeable: bool,
}

/// 经纪公司可提资金算法表
#[derive(Debug, Clone)]
pub struct BrokerWithdrawAlgorithm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 可提资金算法。CTP：`WithdrawAlgorithm`，`TThostFtdcAlgorithmType`。
    pub withdraw_algorithm: Algorithm,
    /// 资金使用率。CTP：`UsingRatio`，`TThostFtdcRatioType`。
    pub using_ratio: f64,
    /// 可提是否包含平仓盈利。CTP：`IncludeCloseProfit`，`TThostFtdcIncludeCloseProfitType`。
    pub include_close_profit: IncludeCloseProfit,
    /// 本日无仓且无成交客户是否受可提比例限制。CTP：`AllWithoutTrade`，`TThostFtdcAllWithoutTradeType`。
    pub all_without_trade: AllWithoutTrade,
    /// 可用是否包含平仓盈利。CTP：`AvailIncludeCloseProfit`，`TThostFtdcIncludeCloseProfitType`。
    pub avail_include_close_profit: IncludeCloseProfit,
    /// 是否启用用户事件。CTP：`IsBrokerUserEvent`，`TThostFtdcBoolType`。
    pub is_broker_user_event: bool,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 货币质押比率。CTP：`FundMortgageRatio`，`TThostFtdcRatioType`。
    pub fund_mortgage_ratio: f64,
    /// 权益算法。CTP：`BalanceAlgorithm`，`TThostFtdcBalanceAlgorithmType`。
    pub balance_algorithm: BalanceAlgorithm,
}

/// 交易所公告
#[derive(Debug, Clone)]
pub struct Bulletin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 公告编号。CTP：`BulletinID`，`TThostFtdcBulletinIDType`。
    pub bulletin_id: i32,
    /// 序列号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 公告类型。CTP：`NewsType`，`TThostFtdcNewsTypeType`。
    pub news_type: String,
    /// 紧急程度。CTP：`NewsUrgency`，`TThostFtdcNewsUrgencyType`。
    pub news_urgency: NewsUrgency,
    /// 发送时间。CTP：`SendTime`，`TThostFtdcTimeType`。
    pub send_time: TimeText,
    /// 消息摘要。CTP：`Abstract`，`TThostFtdcAbstractType`。
    pub summary: String,
    /// 消息来源。CTP：`ComeFrom`，`TThostFtdcComeFromType`。
    pub source: String,
    /// 消息正文。CTP：`Content`，`TThostFtdcContentType`。
    pub content: String,
    /// WEB地址。CTP：`URLLink`，`TThostFtdcURLLinkType`。
    pub url_link: String,
    /// 市场代码。CTP：`MarketID`，`TThostFtdcMarketIDType`。
    pub market_id: String,
}

/// 保证金监管系统经纪公司密钥
#[derive(Debug, Clone)]
pub struct CFMMCBrokerKey {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 经纪公司统一编码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 密钥生成日期。CTP：`CreateDate`，`TThostFtdcDateType`。
    pub create_date: DateText,
    /// 密钥生成时间。CTP：`CreateTime`，`TThostFtdcTimeType`。
    pub create_time: TimeText,
    /// 密钥编号。CTP：`KeyID`，`TThostFtdcSequenceNoType`。
    pub key_id: i32,
    /// 动态密钥。CTP：`CurrentKey`，`TThostFtdcCFMMCKeyType`。
    pub current_key: String,
    /// 动态密钥类型。CTP：`KeyKind`，`TThostFtdcCFMMCKeyKindType`。
    pub key_kind: CFMMCKeyKind,
}

/// 保证金监管系统经纪公司资金账户密钥
#[derive(Debug, Clone)]
pub struct CFMMCTradingAccountKey {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 经纪公司统一编码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 密钥编号。CTP：`KeyID`，`TThostFtdcSequenceNoType`。
    pub key_id: i32,
    /// 动态密钥。CTP：`CurrentKey`，`TThostFtdcCFMMCKeyType`。
    pub current_key: String,
}

/// 监控中心用户令牌
#[derive(Debug, Clone)]
pub struct CFMMCTradingAccountToken {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 经纪公司统一编码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 密钥编号。CTP：`KeyID`，`TThostFtdcSequenceNoType`。
    pub key_id: i32,
    /// 动态令牌。CTP：`Token`，`TThostFtdcCFMMCTokenType`。
    pub token: String,
}

/// 银期销户信息
#[derive(Debug, Clone)]
pub struct CancelAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 汇钞标志。CTP：`CashExchangeCode`，`TThostFtdcCashExchangeCodeType`。
    pub cash_exchange_code: CashExchangeCode,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 撤销对冲设置
#[derive(Debug, Clone)]
pub struct CancelOffsetSetting {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 标的期货合约代码。CTP：`UnderlyingInstrID`，`TThostFtdcInstrumentIDType`。
    pub underlying_instrument_id: InstrumentId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 对冲类型。CTP：`OffsetType`，`TThostFtdcOffsetTypeType`。
    pub offset_type: OffsetType,
    /// 申请对冲的合约数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 是否对冲。CTP：`IsOffset`，`TThostFtdcBoolType`。
    pub is_offset: bool,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 交易所合约代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 交易所期权系列号。CTP：`ExchangeSerialNo`，`TThostFtdcExchangeInstIDType`。
    pub exchange_serial_no: ExchangeInstrumentId,
    /// 交易所产品代码。CTP：`ExchangeProductID`，`TThostFtdcProductIDType`。
    pub exchange_product_id: ProductId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
}

/// 银期变更银行账号信息
#[derive(Debug, Clone)]
pub struct ChangeAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 新银行帐号。CTP：`NewBankAccount`，`TThostFtdcBankAccountType`。
    pub new_bank_account: String,
    /// 新银行密码。CTP：`NewBankPassWord`，`TThostFtdcPasswordType`。
    pub new_bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 申请组合
#[derive(Debug, Clone)]
pub struct CombAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组合引用。CTP：`CombActionRef`，`TThostFtdcOrderRefType`。
    pub comb_action_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 组合指令方向。CTP：`CombDirection`，`TThostFtdcCombDirectionType`。
    pub comb_direction: CombDirection,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 本地申请组合编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 组合状态。CTP：`ActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub action_status: OrderActionStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 组合编号。CTP：`ComTradeID`，`TThostFtdcTradeIDType`。
    pub com_trade_id: TradeId,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 组合合约安全系数
#[derive(Debug, Clone)]
pub struct CombInstrumentGuard {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 担保比例。CTP：`GuarantRatio`，`TThostFtdcRatioType`。
    pub guaranty_ratio: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 组合腿信息
#[derive(Debug, Clone)]
pub struct CombLeg {
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 单腿合约代码。CTP：`LegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub leg_instrument_id: InstrumentId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 单腿乘数。CTP：`LegMultiple`，`TThostFtdcLegMultipleType`。
    pub leg_multiple: i32,
    /// 派生层数。CTP：`ImplyLevel`，`TThostFtdcImplyLevelType`。
    pub imply_level: i32,
}

/// 组合优惠比例
#[derive(Debug, Clone)]
pub struct CombPromotionParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 期权组合保证金比例。CTP：`Xparameter`，`TThostFtdcDiscountRatioType`。
    pub xparameter: f64,
}

/// 组合交易合约的单腿
#[derive(Debug, Clone)]
pub struct CombinationLeg {
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 单腿乘数。CTP：`LegMultiple`，`TThostFtdcLegMultipleType`。
    pub leg_multiple: i32,
    /// 派生层数。CTP：`ImplyLevel`，`TThostFtdcImplyLevelType`。
    pub imply_level: i32,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 单腿合约代码。CTP：`LegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub leg_instrument_id: InstrumentId,
}

/// 通讯阶段
#[derive(Debug, Clone)]
pub struct CommPhase {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 通讯时段编号。CTP：`CommPhaseNo`，`TThostFtdcCommPhaseNoType`。
    pub commission_phase_no: i16,
    /// 系统编号。CTP：`SystemID`，`TThostFtdcSystemIDType`。
    pub system_id: String,
}

/// 投资者手续费率模板
#[derive(Debug, Clone)]
pub struct CommRateModel {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 手续费率模板代码。CTP：`CommModelID`，`TThostFtdcInvestorIDType`。
    pub commission_model_id: InvestorId,
    /// 模板名称。CTP：`CommModelName`，`TThostFtdcCommModelNameType`。
    pub commission_model_name: String,
}

/// 查询签约银行响应
#[derive(Debug, Clone)]
pub struct ContractBank {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分中心代码。CTP：`BankBrchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 银行名称。CTP：`BankName`，`TThostFtdcBankNameType`。
    pub bank_name: String,
    /// 上报csrc的银行代码。CTP：`csrcBankID`，`TThostFtdcBankIDType`。
    pub csrc_bank_id: String,
}

/// 当前交易中心
#[derive(Debug, Clone)]
pub struct CurrDRIdentity {
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
}

/// 当前银期所属交易中心
#[derive(Debug, Clone)]
pub struct CurrTransferIdentity {
    /// 交易中心代码。CTP：`IdentityID`，`TThostFtdcDRIdentityIDType`。
    pub identity_id: i32,
}

/// 当前时间
#[derive(Debug, Clone)]
pub struct CurrentTime {
    /// 当前交易日。CTP：`CurrDate`，`TThostFtdcDateType`。
    pub current_date: DateText,
    /// 当前时间。CTP：`CurrTime`，`TThostFtdcTimeType`。
    pub current_time: TimeText,
    /// 当前时间（毫秒）。CTP：`CurrMillisec`，`TThostFtdcMillisecType`。
    pub current_millisecond: i32,
    /// 自然日期。CTP：`ActionDay`，`TThostFtdcDateType`。
    pub action_day: DateText,
}

/// 灾备交易转换报文
#[derive(Debug, Clone)]
pub struct DRTransfer {
    /// 原交易中心代码。CTP：`OrigDRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub orig_dr_identity_id: i32,
    /// 目标交易中心代码。CTP：`DestDRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dest_dr_identity_id: i32,
    /// 原应用单元代码。CTP：`OrigBrokerID`，`TThostFtdcBrokerIDType`。
    pub orig_broker_id: BrokerId,
    /// 目标易用单元代码。CTP：`DestBrokerID`，`TThostFtdcBrokerIDType`。
    pub dest_broker_id: BrokerId,
}

/// 操作员组织架构关系
#[derive(Debug, Clone)]
pub struct DepartmentUser {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcDepartmentRangeType`。
    pub investor_range: DepartmentRange,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 验证期货资金密码和客户信息
#[derive(Debug, Clone)]
pub struct DepositResultInform {
    /// 出入金流水号，该流水号为银期报盘返回的流水号。CTP：`DepositSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub deposit_seq_no: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 入金金额。CTP：`Deposit`，`TThostFtdcMoneyType`。
    pub deposit: f64,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 返回代码。CTP：`ReturnCode`，`TThostFtdcReturnCodeType`。
    pub return_code: String,
    /// 返回码描述。CTP：`DescrInfoForReturnCode`，`TThostFtdcDescrInfoForReturnCodeType`。
    pub descr_info_for_return_code: String,
}

/// 深度行情
#[derive(Debug, Clone)]
pub struct DepthMarketData {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 最新价。CTP：`LastPrice`，`TThostFtdcPriceType`。
    pub last_price: f64,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 昨收盘。CTP：`PreClosePrice`，`TThostFtdcPriceType`。
    pub pre_close_price: f64,
    /// 昨持仓量。CTP：`PreOpenInterest`，`TThostFtdcLargeVolumeType`。
    pub pre_open_interest: f64,
    /// 今开盘。CTP：`OpenPrice`，`TThostFtdcPriceType`。
    pub open_price: f64,
    /// 最高价。CTP：`HighestPrice`，`TThostFtdcPriceType`。
    pub highest_price: f64,
    /// 最低价。CTP：`LowestPrice`，`TThostFtdcPriceType`。
    pub lowest_price: f64,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 成交金额。CTP：`Turnover`，`TThostFtdcMoneyType`。
    pub turnover: f64,
    /// 持仓量。CTP：`OpenInterest`，`TThostFtdcLargeVolumeType`。
    pub open_interest: f64,
    /// 今收盘。CTP：`ClosePrice`，`TThostFtdcPriceType`。
    pub close_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 涨停板价。CTP：`UpperLimitPrice`，`TThostFtdcPriceType`。
    pub upper_limit_price: f64,
    /// 跌停板价。CTP：`LowerLimitPrice`，`TThostFtdcPriceType`。
    pub lower_limit_price: f64,
    /// 昨虚实度。CTP：`PreDelta`，`TThostFtdcRatioType`。
    pub pre_delta: f64,
    /// 今虚实度。CTP：`CurrDelta`，`TThostFtdcRatioType`。
    pub current_delta: f64,
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 最后修改毫秒。CTP：`UpdateMillisec`，`TThostFtdcMillisecType`。
    pub update_millis: i32,
    /// 申买价一。CTP：`BidPrice1`，`TThostFtdcPriceType`。
    pub bid_price1: f64,
    /// 申买量一。CTP：`BidVolume1`，`TThostFtdcVolumeType`。
    pub bid_volume1: i32,
    /// 申卖价一。CTP：`AskPrice1`，`TThostFtdcPriceType`。
    pub ask_price1: f64,
    /// 申卖量一。CTP：`AskVolume1`，`TThostFtdcVolumeType`。
    pub ask_volume1: i32,
    /// 申买价二。CTP：`BidPrice2`，`TThostFtdcPriceType`。
    pub bid_price2: f64,
    /// 申买量二。CTP：`BidVolume2`，`TThostFtdcVolumeType`。
    pub bid_volume2: i32,
    /// 申卖价二。CTP：`AskPrice2`，`TThostFtdcPriceType`。
    pub ask_price2: f64,
    /// 申卖量二。CTP：`AskVolume2`，`TThostFtdcVolumeType`。
    pub ask_volume2: i32,
    /// 申买价三。CTP：`BidPrice3`，`TThostFtdcPriceType`。
    pub bid_price3: f64,
    /// 申买量三。CTP：`BidVolume3`，`TThostFtdcVolumeType`。
    pub bid_volume3: i32,
    /// 申卖价三。CTP：`AskPrice3`，`TThostFtdcPriceType`。
    pub ask_price3: f64,
    /// 申卖量三。CTP：`AskVolume3`，`TThostFtdcVolumeType`。
    pub ask_volume3: i32,
    /// 申买价四。CTP：`BidPrice4`，`TThostFtdcPriceType`。
    pub bid_price4: f64,
    /// 申买量四。CTP：`BidVolume4`，`TThostFtdcVolumeType`。
    pub bid_volume4: i32,
    /// 申卖价四。CTP：`AskPrice4`，`TThostFtdcPriceType`。
    pub ask_price4: f64,
    /// 申卖量四。CTP：`AskVolume4`，`TThostFtdcVolumeType`。
    pub ask_volume4: i32,
    /// 申买价五。CTP：`BidPrice5`，`TThostFtdcPriceType`。
    pub bid_price5: f64,
    /// 申买量五。CTP：`BidVolume5`，`TThostFtdcVolumeType`。
    pub bid_volume5: i32,
    /// 申卖价五。CTP：`AskPrice5`，`TThostFtdcPriceType`。
    pub ask_price5: f64,
    /// 申卖量五。CTP：`AskVolume5`，`TThostFtdcVolumeType`。
    pub ask_volume5: i32,
    /// 当日均价。CTP：`AveragePrice`，`TThostFtdcPriceType`。
    pub average_price: f64,
    /// 业务日期。CTP：`ActionDay`，`TThostFtdcDateType`。
    pub action_day: DateText,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 上带价。CTP：`BandingUpperPrice`，`TThostFtdcPriceType`。
    pub banding_upper_price: f64,
    /// 下带价。CTP：`BandingLowerPrice`，`TThostFtdcPriceType`。
    pub banding_lower_price: f64,
}

/// 会员资金折扣
#[derive(Debug, Clone)]
pub struct Discount {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 资金折扣比例。CTP：`Discount`，`TThostFtdcRatioType`。
    pub discount: f64,
}

/// 信息分发
#[derive(Debug, Clone)]
pub struct Dissemination {
    /// 序列系列号。CTP：`SequenceSeries`，`TThostFtdcSequenceSeriesType`。
    pub sequence_series: i16,
    /// 序列号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
}

/// 仓单折抵信息
#[derive(Debug, Clone)]
pub struct EWarrantOffset {
    /// 交易日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 错误执行宣告
#[derive(Debug, Clone)]
pub struct ErrExecOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行宣告引用。CTP：`ExecOrderRef`，`TThostFtdcOrderRefType`。
    pub exec_order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 执行类型。CTP：`ActionType`，`TThostFtdcActionTypeType`。
    pub action_type: ActionType,
    /// 保留头寸申请的持仓方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 期权行权后是否保留期货头寸的标记,该字段已废弃。CTP：`ReservePositionFlag`，`TThostFtdcExecOrderPositionFlagType`。
    pub reserve_position_flag: ExecOrderPositionFlag,
    /// 期权行权后生成的头寸是否自动平仓。CTP：`CloseFlag`，`TThostFtdcExecOrderCloseFlagType`。
    pub close_flag: ExecOrderCloseFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 错误执行宣告操作
#[derive(Debug, Clone)]
pub struct ErrExecOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行宣告操作引用。CTP：`ExecOrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub exec_order_action_ref: i32,
    /// 执行宣告引用。CTP：`ExecOrderRef`，`TThostFtdcOrderRefType`。
    pub exec_order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行宣告操作编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 错误报单
#[derive(Debug, Clone)]
pub struct ErrOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 报单价格条件。CTP：`OrderPriceType`，`TThostFtdcOrderPriceTypeType`。
    pub order_price_type: OrderPriceType,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合开平标志。CTP：`CombOffsetFlag`，`TThostFtdcCombOffsetFlagType`。
    pub comb_offset_flag: CombOffsetFlag,
    /// 组合投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量。CTP：`VolumeTotalOriginal`，`TThostFtdcVolumeType`。
    pub volume_total_original: i32,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// GTD日期。CTP：`GTDDate`，`TThostFtdcDateType`。
    pub gtd_date: DateText,
    /// 成交量类型。CTP：`VolumeCondition`，`TThostFtdcVolumeConditionType`。
    pub volume_condition: VolumeCondition,
    /// 最小成交量。CTP：`MinVolume`，`TThostFtdcVolumeType`。
    pub min_volume: i32,
    /// 触发条件。CTP：`ContingentCondition`，`TThostFtdcContingentConditionType`。
    pub contingent_condition: ContingentCondition,
    /// 止损价。CTP：`StopPrice`，`TThostFtdcPriceType`。
    pub stop_price: f64,
    /// 强平原因。CTP：`ForceCloseReason`，`TThostFtdcForceCloseReasonType`。
    pub force_close_reason: ForceCloseReason,
    /// 自动挂起标志。CTP：`IsAutoSuspend`，`TThostFtdcBoolType`。
    pub is_auto_suspend: bool,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 用户强平标志。CTP：`UserForceClose`，`TThostFtdcBoolType`。
    pub user_force_close: bool,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 互换单标志。CTP：`IsSwapOrder`，`TThostFtdcBoolType`。
    pub is_swap_order: bool,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 错误报单操作
#[derive(Debug, Clone)]
pub struct ErrOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单操作引用。CTP：`OrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub order_action_ref: i32,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量变化。CTP：`VolumeChange`，`TThostFtdcVolumeType`。
    pub volume_change: i32,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 查询错误报单操作
#[derive(Debug, Clone)]
pub struct ErrorConditionalOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 报单价格条件。CTP：`OrderPriceType`，`TThostFtdcOrderPriceTypeType`。
    pub order_price_type: OrderPriceType,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合开平标志。CTP：`CombOffsetFlag`，`TThostFtdcCombOffsetFlagType`。
    pub comb_offset_flag: CombOffsetFlag,
    /// 组合投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量。CTP：`VolumeTotalOriginal`，`TThostFtdcVolumeType`。
    pub volume_total_original: i32,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// GTD日期。CTP：`GTDDate`，`TThostFtdcDateType`。
    pub gtd_date: DateText,
    /// 成交量类型。CTP：`VolumeCondition`，`TThostFtdcVolumeConditionType`。
    pub volume_condition: VolumeCondition,
    /// 最小成交量。CTP：`MinVolume`，`TThostFtdcVolumeType`。
    pub min_volume: i32,
    /// 触发条件。CTP：`ContingentCondition`，`TThostFtdcContingentConditionType`。
    pub contingent_condition: ContingentCondition,
    /// 止损价。CTP：`StopPrice`，`TThostFtdcPriceType`。
    pub stop_price: f64,
    /// 强平原因。CTP：`ForceCloseReason`，`TThostFtdcForceCloseReasonType`。
    pub force_close_reason: ForceCloseReason,
    /// 自动挂起标志。CTP：`IsAutoSuspend`，`TThostFtdcBoolType`。
    pub is_auto_suspend: bool,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 报单来源。CTP：`OrderSource`，`TThostFtdcOrderSourceType`。
    pub order_source: OrderSource,
    /// 报单状态。CTP：`OrderStatus`，`TThostFtdcOrderStatusType`。
    pub order_status: OrderStatus,
    /// 报单类型。CTP：`OrderType`，`TThostFtdcOrderTypeType`。
    pub order_type: OrderType,
    /// 今成交数量。CTP：`VolumeTraded`，`TThostFtdcVolumeType`。
    pub volume_traded: i32,
    /// 剩余数量。CTP：`VolumeTotal`，`TThostFtdcVolumeType`。
    pub volume_total: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 委托时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 激活时间。CTP：`ActiveTime`，`TThostFtdcTimeType`。
    pub active_time: TimeText,
    /// 挂起时间。CTP：`SuspendTime`，`TThostFtdcTimeType`。
    pub suspend_time: TimeText,
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 最后修改交易所交易员代码。CTP：`ActiveTraderID`，`TThostFtdcTraderIDType`。
    pub active_trader_id: String,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 用户强平标志。CTP：`UserForceClose`，`TThostFtdcBoolType`。
    pub user_force_close: bool,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerOrderSeq`，`TThostFtdcSequenceNoType`。
    pub broker_order_seq: i32,
    /// 相关报单。CTP：`RelativeOrderSysID`，`TThostFtdcOrderSysIDType`。
    pub relative_order_system_id: OrderSysId,
    /// 郑商所成交数量。CTP：`ZCETotalTradedVolume`，`TThostFtdcVolumeType`。
    pub zce_total_traded_volume: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 互换单标志。CTP：`IsSwapOrder`，`TThostFtdcBoolType`。
    pub is_swap_order: bool,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所报撤频率限制
#[derive(Debug, Clone)]
pub struct ExchOrderFreqParam {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 频率限制。CTP：`FreqLimit`，`TThostFtdcVolumeType`。
    pub frequency_limit: i32,
}

/// 交易所
#[derive(Debug, Clone)]
pub struct Exchange {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所名称。CTP：`ExchangeName`，`TThostFtdcExchangeNameType`。
    pub exchange_name: String,
    /// 交易所属性。CTP：`ExchangeProperty`，`TThostFtdcExchangePropertyType`。
    pub exchange_property: ExchangeProperty,
}

/// 交易所批量报单操作
#[derive(Debug, Clone)]
pub struct ExchangeBatchOrderAction {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所申请组合信息
#[derive(Debug, Clone)]
pub struct ExchangeCombAction {
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 组合指令方向。CTP：`CombDirection`，`TThostFtdcCombDirectionType`。
    pub comb_direction: CombDirection,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 本地申请组合编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 组合状态。CTP：`ActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub action_status: OrderActionStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 组合编号。CTP：`ComTradeID`，`TThostFtdcTradeIDType`。
    pub com_trade_id: TradeId,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所执行宣告信息
#[derive(Debug, Clone)]
pub struct ExchangeExecOrder {
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 执行类型。CTP：`ActionType`，`TThostFtdcActionTypeType`。
    pub action_type: ActionType,
    /// 保留头寸申请的持仓方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 期权行权后是否保留期货头寸的标记,该字段已废弃。CTP：`ReservePositionFlag`，`TThostFtdcExecOrderPositionFlagType`。
    pub reserve_position_flag: ExecOrderPositionFlag,
    /// 期权行权后生成的头寸是否自动平仓。CTP：`CloseFlag`，`TThostFtdcExecOrderCloseFlagType`。
    pub close_flag: ExecOrderCloseFlag,
    /// 本地执行宣告编号。CTP：`ExecOrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub exec_order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 执行宣告提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 执行宣告编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 执行结果。CTP：`ExecResult`，`TThostFtdcExecResultType`。
    pub exec_result: ExecResult,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所执行宣告操作
#[derive(Debug, Clone)]
pub struct ExchangeExecOrderAction {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行宣告操作编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地执行宣告编号。CTP：`ExecOrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub exec_order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 执行类型。CTP：`ActionType`，`TThostFtdcActionTypeType`。
    pub action_type: ActionType,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 交易所询价信息
#[derive(Debug, Clone)]
pub struct ExchangeForQuote {
    /// 本地询价编号。CTP：`ForQuoteLocalID`，`TThostFtdcOrderLocalIDType`。
    pub for_quote_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 询价状态。CTP：`ForQuoteStatus`，`TThostFtdcForQuoteStatusType`。
    pub for_quote_status: ForQuoteStatus,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所保证金率
#[derive(Debug, Clone)]
pub struct ExchangeMarginRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 交易所保证金率调整
#[derive(Debug, Clone)]
pub struct ExchangeMarginRateAdjust {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 跟随交易所投资者多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 跟随交易所投资者多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 跟随交易所投资者空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 跟随交易所投资者空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 交易所多头保证金率。CTP：`ExchLongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub exchange_long_margin_ratio_by_money: f64,
    /// 交易所多头保证金费。CTP：`ExchLongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub exchange_long_margin_ratio_by_volume: f64,
    /// 交易所空头保证金率。CTP：`ExchShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub exchange_short_margin_ratio_by_money: f64,
    /// 交易所空头保证金费。CTP：`ExchShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub exchange_short_margin_ratio_by_volume: f64,
    /// 不跟随交易所投资者多头保证金率。CTP：`NoLongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub no_long_margin_ratio_by_money: f64,
    /// 不跟随交易所投资者多头保证金费。CTP：`NoLongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub no_long_margin_ratio_by_volume: f64,
    /// 不跟随交易所投资者空头保证金率。CTP：`NoShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub no_short_margin_ratio_by_money: f64,
    /// 不跟随交易所投资者空头保证金费。CTP：`NoShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub no_short_margin_ratio_by_volume: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 交易所期权自对冲信息
#[derive(Debug, Clone)]
pub struct ExchangeOptionSelfClose {
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 期权行权的头寸是否自对冲。CTP：`OptSelfCloseFlag`，`TThostFtdcOptSelfCloseFlagType`。
    pub opt_self_close_flag: OptSelfCloseFlag,
    /// 本地期权自对冲编号。CTP：`OptionSelfCloseLocalID`，`TThostFtdcOrderLocalIDType`。
    pub option_self_close_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期权自对冲提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 期权自对冲编号。CTP：`OptionSelfCloseSysID`，`TThostFtdcOrderSysIDType`。
    pub option_self_close_system_id: OrderSysId,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 自对冲结果。CTP：`ExecResult`，`TThostFtdcExecResultType`。
    pub exec_result: ExecResult,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所期权自对冲操作
#[derive(Debug, Clone)]
pub struct ExchangeOptionSelfCloseAction {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 期权自对冲操作编号。CTP：`OptionSelfCloseSysID`，`TThostFtdcOrderSysIDType`。
    pub option_self_close_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地期权自对冲编号。CTP：`OptionSelfCloseLocalID`，`TThostFtdcOrderLocalIDType`。
    pub option_self_close_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 期权行权的头寸是否自对冲。CTP：`OptSelfCloseFlag`，`TThostFtdcOptSelfCloseFlagType`。
    pub opt_self_close_flag: OptSelfCloseFlag,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 交易所报单
#[derive(Debug, Clone)]
pub struct ExchangeOrder {
    /// 报单价格条件。CTP：`OrderPriceType`，`TThostFtdcOrderPriceTypeType`。
    pub order_price_type: OrderPriceType,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合开平标志。CTP：`CombOffsetFlag`，`TThostFtdcCombOffsetFlagType`。
    pub comb_offset_flag: CombOffsetFlag,
    /// 组合投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量。CTP：`VolumeTotalOriginal`，`TThostFtdcVolumeType`。
    pub volume_total_original: i32,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// GTD日期。CTP：`GTDDate`，`TThostFtdcDateType`。
    pub gtd_date: DateText,
    /// 成交量类型。CTP：`VolumeCondition`，`TThostFtdcVolumeConditionType`。
    pub volume_condition: VolumeCondition,
    /// 最小成交量。CTP：`MinVolume`，`TThostFtdcVolumeType`。
    pub min_volume: i32,
    /// 触发条件。CTP：`ContingentCondition`，`TThostFtdcContingentConditionType`。
    pub contingent_condition: ContingentCondition,
    /// 止损价。CTP：`StopPrice`，`TThostFtdcPriceType`。
    pub stop_price: f64,
    /// 强平原因。CTP：`ForceCloseReason`，`TThostFtdcForceCloseReasonType`。
    pub force_close_reason: ForceCloseReason,
    /// 自动挂起标志。CTP：`IsAutoSuspend`，`TThostFtdcBoolType`。
    pub is_auto_suspend: bool,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 报单来源。CTP：`OrderSource`，`TThostFtdcOrderSourceType`。
    pub order_source: OrderSource,
    /// 报单状态。CTP：`OrderStatus`，`TThostFtdcOrderStatusType`。
    pub order_status: OrderStatus,
    /// 报单类型。CTP：`OrderType`，`TThostFtdcOrderTypeType`。
    pub order_type: OrderType,
    /// 今成交数量。CTP：`VolumeTraded`，`TThostFtdcVolumeType`。
    pub volume_traded: i32,
    /// 剩余数量。CTP：`VolumeTotal`，`TThostFtdcVolumeType`。
    pub volume_total: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 委托时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 激活时间。CTP：`ActiveTime`，`TThostFtdcTimeType`。
    pub active_time: TimeText,
    /// 挂起时间。CTP：`SuspendTime`，`TThostFtdcTimeType`。
    pub suspend_time: TimeText,
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 最后修改交易所交易员代码。CTP：`ActiveTraderID`，`TThostFtdcTraderIDType`。
    pub active_trader_id: String,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所报单操作
#[derive(Debug, Clone)]
pub struct ExchangeOrderAction {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量变化。CTP：`VolumeChange`，`TThostFtdcVolumeType`。
    pub volume_change: i32,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 交易所报单操作失败
#[derive(Debug, Clone)]
pub struct ExchangeOrderActionError {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 交易所报单插入失败
#[derive(Debug, Clone)]
pub struct ExchangeOrderInsertError {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 交易所报价信息
#[derive(Debug, Clone)]
pub struct ExchangeQuote {
    /// 卖价格。CTP：`AskPrice`，`TThostFtdcPriceType`。
    pub ask_price: f64,
    /// 买价格。CTP：`BidPrice`，`TThostFtdcPriceType`。
    pub bid_price: f64,
    /// 卖数量。CTP：`AskVolume`，`TThostFtdcVolumeType`。
    pub ask_volume: i32,
    /// 买数量。CTP：`BidVolume`，`TThostFtdcVolumeType`。
    pub bid_volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 卖开平标志。CTP：`AskOffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub ask_offset_flag: OffsetFlag,
    /// 买开平标志。CTP：`BidOffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub bid_offset_flag: OffsetFlag,
    /// 卖投机套保标志。CTP：`AskHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub ask_hedge_flag: HedgeFlag,
    /// 买投机套保标志。CTP：`BidHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub bid_hedge_flag: HedgeFlag,
    /// 本地报价编号。CTP：`QuoteLocalID`，`TThostFtdcOrderLocalIDType`。
    pub quote_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报价提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 报价提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 报价编号。CTP：`QuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub quote_system_id: OrderSysId,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 报价状态。CTP：`QuoteStatus`，`TThostFtdcOrderStatusType`。
    pub quote_status: OrderStatus,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 卖方报单编号。CTP：`AskOrderSysID`，`TThostFtdcOrderSysIDType`。
    pub ask_order_system_id: OrderSysId,
    /// 买方报单编号。CTP：`BidOrderSysID`，`TThostFtdcOrderSysIDType`。
    pub bid_order_system_id: OrderSysId,
    /// 应价编号。CTP：`ForQuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub for_quote_system_id: OrderSysId,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
}

/// 交易所报价操作
#[derive(Debug, Clone)]
pub struct ExchangeQuoteAction {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报价操作编号。CTP：`QuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub quote_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报价编号。CTP：`QuoteLocalID`，`TThostFtdcOrderLocalIDType`。
    pub quote_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 汇率
#[derive(Debug, Clone)]
pub struct ExchangeRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 源币种。CTP：`FromCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub from_currency_id: String,
    /// 源币种单位数量。CTP：`FromCurrencyUnit`，`TThostFtdcCurrencyUnitType`。
    pub from_currency_unit: f64,
    /// 目标币种。CTP：`ToCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub to_currency_id: String,
    /// 汇率。CTP：`ExchangeRate`，`TThostFtdcExchangeRateType`。
    pub exchange_rate: f64,
}

/// 交易所状态
#[derive(Debug, Clone)]
pub struct ExchangeSequence {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 合约交易状态。CTP：`MarketStatus`，`TThostFtdcInstrumentStatusType`。
    pub market_status: InstrumentStatusType,
}

/// 交易所成交
#[derive(Debug, Clone)]
pub struct ExchangeTrade {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 成交编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易角色。CTP：`TradingRole`，`TThostFtdcTradingRoleType`。
    pub trading_role: TradingRole,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 价格。CTP：`Price`，`TThostFtdcPriceType`。
    pub price: f64,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 成交时期。CTP：`TradeDate`，`TThostFtdcDateType`。
    pub trade_date: DateText,
    /// 成交时间。CTP：`TradeTime`，`TThostFtdcTimeType`。
    pub trade_time: TimeText,
    /// 成交类型。CTP：`TradeType`，`TThostFtdcTradeTypeType`。
    pub trade_type: TradeType,
    /// 成交价来源。CTP：`PriceSource`，`TThostFtdcPriceSourceType`。
    pub price_source: PriceSource,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 成交来源。CTP：`TradeSource`，`TThostFtdcTradeSourceType`。
    pub trade_source: TradeSource,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 执行宣告
#[derive(Debug, Clone)]
pub struct ExecOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行宣告引用。CTP：`ExecOrderRef`，`TThostFtdcOrderRefType`。
    pub exec_order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 执行类型。CTP：`ActionType`，`TThostFtdcActionTypeType`。
    pub action_type: ActionType,
    /// 保留头寸申请的持仓方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 期权行权后是否保留期货头寸的标记,该字段已废弃。CTP：`ReservePositionFlag`，`TThostFtdcExecOrderPositionFlagType`。
    pub reserve_position_flag: ExecOrderPositionFlag,
    /// 期权行权后生成的头寸是否自动平仓。CTP：`CloseFlag`，`TThostFtdcExecOrderCloseFlagType`。
    pub close_flag: ExecOrderCloseFlag,
    /// 本地执行宣告编号。CTP：`ExecOrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub exec_order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 执行宣告提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 执行宣告编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 执行结果。CTP：`ExecResult`，`TThostFtdcExecResultType`。
    pub exec_result: ExecResult,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerExecOrderSeq`，`TThostFtdcSequenceNoType`。
    pub broker_exec_order_seq: i32,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 执行宣告操作
#[derive(Debug, Clone)]
pub struct ExecOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行宣告操作引用。CTP：`ExecOrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub exec_order_action_ref: i32,
    /// 执行宣告引用。CTP：`ExecOrderRef`，`TThostFtdcOrderRefType`。
    pub exec_order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行宣告操作编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地执行宣告编号。CTP：`ExecOrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub exec_order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 执行类型。CTP：`ActionType`，`TThostFtdcActionTypeType`。
    pub action_type: ActionType,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 退出紧急状态参数
#[derive(Debug, Clone)]
pub struct ExitEmergency {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// Fens用户信息
#[derive(Debug, Clone)]
pub struct FensUserInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 登录模式。CTP：`LoginMode`，`TThostFtdcLoginModeType`。
    pub login_mode: LoginMode,
}

/// 询价
#[derive(Debug, Clone)]
pub struct ForQuote {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 询价引用。CTP：`ForQuoteRef`，`TThostFtdcOrderRefType`。
    pub for_quote_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 本地询价编号。CTP：`ForQuoteLocalID`，`TThostFtdcOrderLocalIDType`。
    pub for_quote_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 询价状态。CTP：`ForQuoteStatus`，`TThostFtdcForQuoteStatusType`。
    pub for_quote_status: ForQuoteStatus,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司询价编号。CTP：`BrokerForQutoSeq`，`TThostFtdcSequenceNoType`。
    pub broker_for_quote_seq: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 询价价差参数
#[derive(Debug, Clone)]
pub struct ForQuoteParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 最新价。CTP：`LastPrice`，`TThostFtdcPriceType`。
    pub last_price: f64,
    /// 价差。CTP：`PriceInterval`，`TThostFtdcPriceType`。
    pub price_interval: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 发给做市商的询价请求
#[derive(Debug, Clone)]
pub struct ForQuoteRsp {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 询价编号。CTP：`ForQuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub for_quote_system_id: OrderSysId,
    /// 询价时间。CTP：`ForQuoteTime`，`TThostFtdcTimeType`。
    pub for_quote_time: TimeText,
    /// 业务日期。CTP：`ActionDay`，`TThostFtdcDateType`。
    pub action_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 强制交易员退出
#[derive(Debug, Clone)]
pub struct ForceUserLogout {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 前置信息
#[derive(Debug, Clone)]
pub struct FrontInfo {
    /// 前置地址。CTP：`FrontAddr`，`TThostFtdcAddressType`。
    pub front_address: String,
    /// 查询流控。CTP：`QryFreq`，`TThostFtdcQueryFreqType`。
    pub qry_frequency: i32,
    /// FTD流控。CTP：`FTDPkgFreq`，`TThostFtdcQueryFreqType`。
    pub ftd_pkg_frequency: i32,
}

/// 前置状态
#[derive(Debug, Clone)]
pub struct FrontStatus {
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 上次报告日期。CTP：`LastReportDate`，`TThostFtdcDateType`。
    pub last_report_date: DateText,
    /// 上次报告时间。CTP：`LastReportTime`，`TThostFtdcTimeType`。
    pub last_report_time: TimeText,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
}

/// 期货持仓限制参数
#[derive(Debug, Clone)]
pub struct FutureLimitPosiParam {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 当日投机开仓数量限制。CTP：`SpecOpenVolume`，`TThostFtdcVolumeType`。
    pub spec_open_volume: i32,
    /// 当日套利开仓数量限制。CTP：`ArbiOpenVolume`，`TThostFtdcVolumeType`。
    pub arbi_open_volume: i32,
    /// 当日投机+套利开仓数量限制。CTP：`OpenVolume`，`TThostFtdcVolumeType`。
    pub open_volume: i32,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// 期商签到签退
#[derive(Debug, Clone)]
pub struct FutureSignIO {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
}

/// 套保申请回报
#[derive(Debug, Clone)]
pub struct HedgeCfm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerOrderSeq`，`TThostFtdcSequenceNoType`。
    pub broker_order_seq: i32,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 申请状态。CTP：`ApplyStatus`，`TThostFtdcApplyStatusType`。
    pub apply_status: ApplyStatus,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 成功处理数量。CTP：`DealVolume`，`TThostFtdcVolumeType`。
    pub deal_volume: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 委托时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 日期。CTP：`ReqDate`，`TThostFtdcDateType`。
    pub req_date: DateText,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 套保申请撤销回报
#[derive(Debug, Clone)]
pub struct HedgeCfmAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合同编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// IP列表
#[derive(Debug, Clone)]
pub struct IPList {
    /// 是否白名单。CTP：`IsWhite`，`TThostFtdcBoolType`。
    pub is_white: bool,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 股指现货指数
#[derive(Debug, Clone)]
pub struct IndexPrice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 指数现货收盘价。CTP：`ClosePrice`，`TThostFtdcPriceType`。
    pub close_price: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 输入批量报单操作
#[derive(Debug, Clone)]
pub struct InputBatchOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单操作引用。CTP：`OrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub order_action_ref: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入的申请组合
#[derive(Debug, Clone)]
pub struct InputCombAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组合引用。CTP：`CombActionRef`，`TThostFtdcOrderRefType`。
    pub comb_action_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 组合指令方向。CTP：`CombDirection`，`TThostFtdcCombDirectionType`。
    pub comb_direction: CombDirection,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入的执行宣告
#[derive(Debug, Clone)]
pub struct InputExecOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行宣告引用。CTP：`ExecOrderRef`，`TThostFtdcOrderRefType`。
    pub exec_order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 执行类型。CTP：`ActionType`，`TThostFtdcActionTypeType`。
    pub action_type: ActionType,
    /// 保留头寸申请的持仓方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 期权行权后是否保留期货头寸的标记,该字段已废弃。CTP：`ReservePositionFlag`，`TThostFtdcExecOrderPositionFlagType`。
    pub reserve_position_flag: ExecOrderPositionFlag,
    /// 期权行权后生成的头寸是否自动平仓。CTP：`CloseFlag`，`TThostFtdcExecOrderCloseFlagType`。
    pub close_flag: ExecOrderCloseFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入执行宣告操作
#[derive(Debug, Clone)]
pub struct InputExecOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行宣告操作引用。CTP：`ExecOrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub exec_order_action_ref: i32,
    /// 执行宣告引用。CTP：`ExecOrderRef`，`TThostFtdcOrderRefType`。
    pub exec_order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行宣告操作编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入的询价
#[derive(Debug, Clone)]
pub struct InputForQuote {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 询价引用。CTP：`ForQuoteRef`，`TThostFtdcOrderRefType`。
    pub for_quote_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 套保确认输入基本信息
#[derive(Debug, Clone)]
pub struct InputHedgeCfm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 套保申请撤销
#[derive(Debug, Clone)]
pub struct InputHedgeCfmAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合同编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 输入的对冲设置
#[derive(Debug, Clone)]
pub struct InputOffsetSetting {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 标的期货合约代码。CTP：`UnderlyingInstrID`，`TThostFtdcInstrumentIDType`。
    pub underlying_instrument_id: InstrumentId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 对冲类型。CTP：`OffsetType`，`TThostFtdcOffsetTypeType`。
    pub offset_type: OffsetType,
    /// 申请对冲的合约数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 是否对冲。CTP：`IsOffset`，`TThostFtdcBoolType`。
    pub is_offset: bool,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 输入的期权自对冲
#[derive(Debug, Clone)]
pub struct InputOptionSelfClose {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 期权自对冲引用。CTP：`OptionSelfCloseRef`，`TThostFtdcOrderRefType`。
    pub option_self_close_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 期权行权的头寸是否自对冲。CTP：`OptSelfCloseFlag`，`TThostFtdcOptSelfCloseFlagType`。
    pub opt_self_close_flag: OptSelfCloseFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入期权自对冲操作
#[derive(Debug, Clone)]
pub struct InputOptionSelfCloseAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 期权自对冲操作引用。CTP：`OptionSelfCloseActionRef`，`TThostFtdcOrderActionRefType`。
    pub option_self_close_action_ref: i32,
    /// 期权自对冲引用。CTP：`OptionSelfCloseRef`，`TThostFtdcOrderRefType`。
    pub option_self_close_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 期权自对冲操作编号。CTP：`OptionSelfCloseSysID`，`TThostFtdcOrderSysIDType`。
    pub option_self_close_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入报单
#[derive(Debug, Clone)]
pub struct InputOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 报单价格条件。CTP：`OrderPriceType`，`TThostFtdcOrderPriceTypeType`。
    pub order_price_type: OrderPriceType,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合开平标志。CTP：`CombOffsetFlag`，`TThostFtdcCombOffsetFlagType`。
    pub comb_offset_flag: CombOffsetFlag,
    /// 组合投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量。CTP：`VolumeTotalOriginal`，`TThostFtdcVolumeType`。
    pub volume_total_original: i32,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// GTD日期。CTP：`GTDDate`，`TThostFtdcDateType`。
    pub gtd_date: DateText,
    /// 成交量类型。CTP：`VolumeCondition`，`TThostFtdcVolumeConditionType`。
    pub volume_condition: VolumeCondition,
    /// 最小成交量。CTP：`MinVolume`，`TThostFtdcVolumeType`。
    pub min_volume: i32,
    /// 触发条件。CTP：`ContingentCondition`，`TThostFtdcContingentConditionType`。
    pub contingent_condition: ContingentCondition,
    /// 止损价。CTP：`StopPrice`，`TThostFtdcPriceType`。
    pub stop_price: f64,
    /// 强平原因。CTP：`ForceCloseReason`，`TThostFtdcForceCloseReasonType`。
    pub force_close_reason: ForceCloseReason,
    /// 自动挂起标志。CTP：`IsAutoSuspend`，`TThostFtdcBoolType`。
    pub is_auto_suspend: bool,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 用户强平标志。CTP：`UserForceClose`，`TThostFtdcBoolType`。
    pub user_force_close: bool,
    /// 互换单标志。CTP：`IsSwapOrder`，`TThostFtdcBoolType`。
    pub is_swap_order: bool,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 输入报单操作
#[derive(Debug, Clone)]
pub struct InputOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单操作引用。CTP：`OrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub order_action_ref: i32,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量变化。CTP：`VolumeChange`，`TThostFtdcVolumeType`。
    pub volume_change: i32,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 输入的报价
#[derive(Debug, Clone)]
pub struct InputQuote {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报价引用。CTP：`QuoteRef`，`TThostFtdcOrderRefType`。
    pub quote_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 卖价格。CTP：`AskPrice`，`TThostFtdcPriceType`。
    pub ask_price: f64,
    /// 买价格。CTP：`BidPrice`，`TThostFtdcPriceType`。
    pub bid_price: f64,
    /// 卖数量。CTP：`AskVolume`，`TThostFtdcVolumeType`。
    pub ask_volume: i32,
    /// 买数量。CTP：`BidVolume`，`TThostFtdcVolumeType`。
    pub bid_volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 卖开平标志。CTP：`AskOffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub ask_offset_flag: OffsetFlag,
    /// 买开平标志。CTP：`BidOffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub bid_offset_flag: OffsetFlag,
    /// 卖投机套保标志。CTP：`AskHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub ask_hedge_flag: HedgeFlag,
    /// 买投机套保标志。CTP：`BidHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub bid_hedge_flag: HedgeFlag,
    /// 衍生卖报单引用。CTP：`AskOrderRef`，`TThostFtdcOrderRefType`。
    pub ask_order_ref: OrderRef,
    /// 衍生买报单引用。CTP：`BidOrderRef`，`TThostFtdcOrderRefType`。
    pub bid_order_ref: OrderRef,
    /// 应价编号。CTP：`ForQuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub for_quote_system_id: OrderSysId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 被顶单编号。CTP：`ReplaceSysID`，`TThostFtdcOrderSysIDType`。
    pub replace_system_id: OrderSysId,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 输入报价操作
#[derive(Debug, Clone)]
pub struct InputQuoteAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报价操作引用。CTP：`QuoteActionRef`，`TThostFtdcOrderActionRefType`。
    pub quote_action_ref: i32,
    /// 报价引用。CTP：`QuoteRef`，`TThostFtdcOrderRefType`。
    pub quote_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报价操作编号。CTP：`QuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub quote_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 套利确认输入基本信息
#[derive(Debug, Clone)]
pub struct InputSpdApply {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`FirstLegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub first_leg_instrument_id: InstrumentId,
    /// 合约代码。CTP：`SecondLegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub second_leg_instrument_id: InstrumentId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合定单类型。CTP：`CmbType`，`TThostFtdcCmbTypeType`。
    pub cmb_type: CmbType,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 套利申请撤销
#[derive(Debug, Clone)]
pub struct InputSpdApplyAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合同编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 合约交易限制
#[derive(Debug, Clone)]
pub struct InstrTradeLimit {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 最小开仓手数。CTP：`MinOpenVolume`，`TThostFtdcVolumeType`。
    pub min_open_volume: i32,
}

/// 合约
#[derive(Debug, Clone)]
pub struct Instrument {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约名称。CTP：`InstrumentName`，`TThostFtdcInstrumentNameType`。
    pub instrument_name: String,
    /// 产品类型。CTP：`ProductClass`，`TThostFtdcProductClassType`。
    pub product_class: ProductClass,
    /// 交割年份。CTP：`DeliveryYear`，`TThostFtdcYearType`。
    pub delivery_year: i32,
    /// 交割月。CTP：`DeliveryMonth`，`TThostFtdcMonthType`。
    pub delivery_month: i32,
    /// 市价单最大下单量。CTP：`MaxMarketOrderVolume`，`TThostFtdcVolumeType`。
    pub max_market_order_volume: i32,
    /// 市价单最小下单量。CTP：`MinMarketOrderVolume`，`TThostFtdcVolumeType`。
    pub min_market_order_volume: i32,
    /// 限价单最大下单量。CTP：`MaxLimitOrderVolume`，`TThostFtdcVolumeType`。
    pub max_limit_order_volume: i32,
    /// 限价单最小下单量。CTP：`MinLimitOrderVolume`，`TThostFtdcVolumeType`。
    pub min_limit_order_volume: i32,
    /// 合约数量乘数。CTP：`VolumeMultiple`，`TThostFtdcVolumeMultipleType`。
    pub volume_multiple: i32,
    /// 最小变动价位。CTP：`PriceTick`，`TThostFtdcPriceType`。
    pub price_tick: f64,
    /// 创建日。CTP：`CreateDate`，`TThostFtdcDateType`。
    pub create_date: DateText,
    /// 上市日。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 到期日。CTP：`ExpireDate`，`TThostFtdcDateType`。
    pub expire_date: DateText,
    /// 开始交割日。CTP：`StartDelivDate`，`TThostFtdcDateType`。
    pub start_delivery_date: DateText,
    /// 结束交割日。CTP：`EndDelivDate`，`TThostFtdcDateType`。
    pub end_delivery_date: DateText,
    /// 合约生命周期状态。CTP：`InstLifePhase`，`TThostFtdcInstLifePhaseType`。
    pub instrument_life_phase: InstLifePhase,
    /// 当前是否交易。CTP：`IsTrading`，`TThostFtdcBoolType`。
    pub is_trading: bool,
    /// 持仓类型。CTP：`PositionType`，`TThostFtdcPositionTypeType`。
    pub position_type: PositionType,
    /// 持仓日期类型。CTP：`PositionDateType`，`TThostFtdcPositionDateTypeType`。
    pub position_date_type: PositionDateType,
    /// 多头保证金率。CTP：`LongMarginRatio`，`TThostFtdcRatioType`。
    pub long_margin_ratio: f64,
    /// 空头保证金率。CTP：`ShortMarginRatio`，`TThostFtdcRatioType`。
    pub short_margin_ratio: f64,
    /// 是否使用大额单边保证金算法。CTP：`MaxMarginSideAlgorithm`，`TThostFtdcMaxMarginSideAlgorithmType`。
    pub max_margin_side_algorithm: MaxMarginSideAlgorithm,
    /// 执行价。CTP：`StrikePrice`，`TThostFtdcPriceType`。
    pub strike_price: f64,
    /// 期权类型。CTP：`OptionsType`，`TThostFtdcOptionsTypeType`。
    pub options_type: OptionsType,
    /// 合约基础商品乘数。CTP：`UnderlyingMultiple`，`TThostFtdcUnderlyingMultipleType`。
    pub underlying_multiple: f64,
    /// 组合类型。CTP：`CombinationType`，`TThostFtdcCombinationTypeType`。
    pub combination_type: CombinationType,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 基础商品代码。CTP：`UnderlyingInstrID`，`TThostFtdcInstrumentIDType`。
    pub underlying_instrument_id: InstrumentId,
}

/// 合约手续费率
#[derive(Debug, Clone)]
pub struct InstrumentCommissionRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 业务类型。CTP：`BizType`，`TThostFtdcBizTypeType`。
    pub biz_type: BizType,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 合约保证金率
#[derive(Debug, Clone)]
pub struct InstrumentMarginRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 是否相对交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 合约保证金率调整
#[derive(Debug, Clone)]
pub struct InstrumentMarginRateAdjust {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 是否相对交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 合约保证金率调整
#[derive(Debug, Clone)]
pub struct InstrumentMarginRateUL {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 当前报单手续费的详细内容
#[derive(Debug, Clone)]
pub struct InstrumentOrderCommRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 报单手续费。CTP：`OrderCommByVolume`，`TThostFtdcRatioType`。
    pub order_commission_by_volume: f64,
    /// 撤单手续费。CTP：`OrderActionCommByVolume`，`TThostFtdcRatioType`。
    pub order_action_commission_by_volume: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 报单手续费。CTP：`OrderCommByTrade`，`TThostFtdcRatioType`。
    pub order_commission_by_trade: f64,
    /// 撤单手续费。CTP：`OrderActionCommByTrade`，`TThostFtdcRatioType`。
    pub order_action_commission_by_trade: f64,
}

/// 合约状态
#[derive(Debug, Clone)]
pub struct InstrumentStatus {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 结算组代码。CTP：`SettlementGroupID`，`TThostFtdcSettlementGroupIDType`。
    pub settlement_group_id: String,
    /// 合约交易状态。CTP：`InstrumentStatus`，`TThostFtdcInstrumentStatusType`。
    pub instrument_status: InstrumentStatusType,
    /// 交易阶段编号。CTP：`TradingSegmentSN`，`TThostFtdcTradingSegmentSNType`。
    pub trading_segment_sn: i32,
    /// 进入本状态时间。CTP：`EnterTime`，`TThostFtdcTimeType`。
    pub enter_time: TimeText,
    /// 进入本状态原因。CTP：`EnterReason`，`TThostFtdcInstStatusEnterReasonType`。
    pub enter_reason: InstStatusEnterReason,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 投资者合约交易权限
#[derive(Debug, Clone)]
pub struct InstrumentTradingRight {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易权限。CTP：`TradingRight`，`TThostFtdcTradingRightType`。
    pub trading_right: TradingRight,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 投资单元
#[derive(Debug, Clone)]
pub struct InvestUnit {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 投资者单元名称。CTP：`InvestorUnitName`，`TThostFtdcPartyNameType`。
    pub investor_unit_name: String,
    /// 投资者分组代码。CTP：`InvestorGroupID`，`TThostFtdcInvestorIDType`。
    pub investor_group_id: InvestorId,
    /// 手续费率模板代码。CTP：`CommModelID`，`TThostFtdcInvestorIDType`。
    pub commission_model_id: InvestorId,
    /// 保证金率模板代码。CTP：`MarginModelID`，`TThostFtdcInvestorIDType`。
    pub margin_model_id: InvestorId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 投资者
#[derive(Debug, Clone)]
pub struct Investor {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者分组代码。CTP：`InvestorGroupID`，`TThostFtdcInvestorIDType`。
    pub investor_group_id: InvestorId,
    /// 投资者名称。CTP：`InvestorName`，`TThostFtdcPartyNameType`。
    pub investor_name: String,
    /// 证件类型。CTP：`IdentifiedCardType`，`TThostFtdcIdCardTypeType`。
    pub identified_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
    /// 联系电话。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 通讯地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 开户日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 手机。CTP：`Mobile`，`TThostFtdcMobileType`。
    pub mobile: String,
    /// 手续费率模板代码。CTP：`CommModelID`，`TThostFtdcInvestorIDType`。
    pub commission_model_id: InvestorId,
    /// 保证金率模板代码。CTP：`MarginModelID`，`TThostFtdcInvestorIDType`。
    pub margin_model_id: InvestorId,
    /// 是否频率控制。CTP：`IsOrderFreq`，`TThostFtdcEnumBoolType`。
    pub is_order_frequency: EnumBool,
    /// 是否开仓限制。CTP：`IsOpenVolLimit`，`TThostFtdcEnumBoolType`。
    pub is_open_vol_limit: EnumBool,
}

/// 投资者账户
#[derive(Debug, Clone)]
pub struct InvestorAccount {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 投资者商品群SPMM记录
#[derive(Debug, Clone)]
pub struct InvestorCommodityGroupSPMMMargin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
    /// 优惠仓位应收保证金。CTP：`MarginBeforeDiscount`，`TThostFtdcMoneyType`。
    pub margin_before_discount: f64,
    /// 不优惠仓位应收保证金。CTP：`MarginNoDiscount`，`TThostFtdcMoneyType`。
    pub margin_no_discount: f64,
    /// 多头风险。CTP：`LongRisk`，`TThostFtdcMoneyType`。
    pub long_risk: f64,
    /// 空头风险。CTP：`ShortRisk`，`TThostFtdcMoneyType`。
    pub short_risk: f64,
    /// 商品群平仓冻结保证金。CTP：`CloseFrozenMargin`，`TThostFtdcMoneyType`。
    pub close_frozen_margin: f64,
    /// SPMM跨品种优惠系数。CTP：`InterCommodityRate`，`TThostFtdcSPMMDiscountRatioType`。
    pub inter_commodity_rate: f64,
    /// 商品群最小保证金比例。CTP：`MiniMarginRatio`，`TThostFtdcSPMMDiscountRatioType`。
    pub mini_margin_ratio: f64,
    /// 投资者保证金和交易所保证金的比例。CTP：`AdjustRatio`，`TThostFtdcRatioType`。
    pub adjust_ratio: f64,
    /// SPMM品种内优惠汇总。CTP：`IntraCommodityDiscount`，`TThostFtdcMoneyType`。
    pub intra_commodity_discount: f64,
    /// SPMM跨品种优惠。CTP：`InterCommodityDiscount`，`TThostFtdcMoneyType`。
    pub inter_commodity_discount: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者保证金。CTP：`InvestorMargin`，`TThostFtdcMoneyType`。
    pub investor_margin: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 行权冻结资金。CTP：`StrikeFrozenMargin`，`TThostFtdcMoneyType`。
    pub strike_frozen_margin: f64,
}

/// 投资者商品组SPMM记录
#[derive(Debug, Clone)]
pub struct InvestorCommoditySPMMMargin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品组代码。CTP：`CommodityID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_id: String,
    /// 优惠仓位应收保证金。CTP：`MarginBeforeDiscount`，`TThostFtdcMoneyType`。
    pub margin_before_discount: f64,
    /// 不优惠仓位应收保证金。CTP：`MarginNoDiscount`，`TThostFtdcMoneyType`。
    pub margin_no_discount: f64,
    /// 多头实仓风险。CTP：`LongPosRisk`，`TThostFtdcMoneyType`。
    pub long_pos_risk: f64,
    /// 多头开仓冻结风险。CTP：`LongOpenFrozenRisk`，`TThostFtdcMoneyType`。
    pub long_open_frozen_risk: f64,
    /// 多头被平冻结风险。CTP：`LongCloseFrozenRisk`，`TThostFtdcMoneyType`。
    pub long_close_frozen_risk: f64,
    /// 空头实仓风险。CTP：`ShortPosRisk`，`TThostFtdcMoneyType`。
    pub short_pos_risk: f64,
    /// 空头开仓冻结风险。CTP：`ShortOpenFrozenRisk`，`TThostFtdcMoneyType`。
    pub short_open_frozen_risk: f64,
    /// 空头被平冻结风险。CTP：`ShortCloseFrozenRisk`，`TThostFtdcMoneyType`。
    pub short_close_frozen_risk: f64,
    /// SPMM品种内跨期优惠系数。CTP：`IntraCommodityRate`，`TThostFtdcSPMMDiscountRatioType`。
    pub intra_commodity_rate: f64,
    /// SPMM期权优惠系数。CTP：`OptionDiscountRate`，`TThostFtdcSPMMDiscountRatioType`。
    pub option_discount_rate: f64,
    /// 实仓对冲优惠金额。CTP：`PosDiscount`，`TThostFtdcMoneyType`。
    pub pos_discount: f64,
    /// 开仓报单对冲优惠金额。CTP：`OpenFrozenDiscount`，`TThostFtdcMoneyType`。
    pub open_frozen_discount: f64,
    /// 品种风险净头。CTP：`NetRisk`，`TThostFtdcMoneyType`。
    pub net_risk: f64,
    /// 平仓冻结保证金。CTP：`CloseFrozenMargin`，`TThostFtdcMoneyType`。
    pub close_frozen_margin: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 行权冻结资金。CTP：`StrikeFrozenMargin`，`TThostFtdcMoneyType`。
    pub strike_frozen_margin: f64,
}

/// 组织架构投资者对应关系
#[derive(Debug, Clone)]
pub struct InvestorDepartmentFlat {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组织架构代码。CTP：`DepartmentID`，`TThostFtdcInvestorIDType`。
    pub department_id: InvestorId,
}

/// 投资者组
#[derive(Debug, Clone)]
pub struct InvestorGroup {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者分组代码。CTP：`InvestorGroupID`，`TThostFtdcInvestorIDType`。
    pub investor_group_id: InvestorId,
    /// 投资者分组名称。CTP：`InvestorGroupName`，`TThostFtdcInvestorGroupNameType`。
    pub investor_group_name: String,
}

/// 投资者申报费阶梯收取设置
#[derive(Debug, Clone)]
pub struct InvestorInfoCntSetting {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 是否收取申报费。CTP：`IsCalInfoComm`，`TThostFtdcBoolType`。
    pub is_cal_info_commission: bool,
    /// 是否限制信息量。CTP：`IsLimitInfoMax`，`TThostFtdcBoolType`。
    pub is_limit_info_max: bool,
    /// 信息量限制笔数。CTP：`InfoMaxLimit`，`TThostFtdcVolumeType`。
    pub info_max_limit: i32,
}

/// 投资者申报费阶梯收取记录
#[derive(Debug, Clone)]
pub struct InvestorInfoCommRec {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 报单总笔数。CTP：`OrderCount`，`TThostFtdcVolumeType`。
    pub order_count: i32,
    /// 撤单总笔数。CTP：`OrderActionCount`，`TThostFtdcVolumeType`。
    pub order_action_count: i32,
    /// 询价总次数。CTP：`ForQuoteCnt`，`TThostFtdcVolumeType`。
    pub for_quote_count: i32,
    /// 申报费。CTP：`InfoComm`，`TThostFtdcMoneyType`。
    pub info_commission: f64,
    /// 是否期权系列。CTP：`IsOptSeries`，`TThostFtdcBoolType`。
    pub is_opt_series: bool,
    /// 品种代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 信息量总量。CTP：`InfoCnt`，`TThostFtdcVolumeType`。
    pub info_count: i32,
}

/// 新组保保证金系数投资者模板对应关系
#[derive(Debug, Clone)]
pub struct InvestorPortfMarginModel {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 保证金系数模板。CTP：`MarginModelID`，`TThostFtdcInvestorIDType`。
    pub margin_model_id: InvestorId,
}

/// 投资者新型组合保证金系数
#[derive(Debug, Clone)]
pub struct InvestorPortfMarginRatio {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员对投资者收取的保证金和交易所对投资者收取的保证金的比例。CTP：`MarginRatio`，`TThostFtdcRatioType`。
    pub margin_ratio: f64,
    /// 产品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
}

/// 投资者新组保设置
#[derive(Debug, Clone)]
pub struct InvestorPortfSetting {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者编号。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 是否开启新组保。CTP：`UsePortf`，`TThostFtdcBoolType`。
    pub use_portfolio: bool,
}

/// 投资者持仓
#[derive(Debug, Clone)]
pub struct InvestorPosition {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 持仓多空方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 持仓日期。CTP：`PositionDate`，`TThostFtdcPositionDateType`。
    pub position_date: PositionDate,
    /// 上日持仓。CTP：`YdPosition`，`TThostFtdcVolumeType`。
    pub yd_position: i32,
    /// 今日持仓。CTP：`Position`，`TThostFtdcVolumeType`。
    pub position: i32,
    /// 多头冻结。CTP：`LongFrozen`，`TThostFtdcVolumeType`。
    pub long_frozen: i32,
    /// 空头冻结。CTP：`ShortFrozen`，`TThostFtdcVolumeType`。
    pub short_frozen: i32,
    /// 开仓冻结金额。CTP：`LongFrozenAmount`，`TThostFtdcMoneyType`。
    pub long_frozen_amount: f64,
    /// 开仓冻结金额。CTP：`ShortFrozenAmount`，`TThostFtdcMoneyType`。
    pub short_frozen_amount: f64,
    /// 开仓量。CTP：`OpenVolume`，`TThostFtdcVolumeType`。
    pub open_volume: i32,
    /// 平仓量。CTP：`CloseVolume`，`TThostFtdcVolumeType`。
    pub close_volume: i32,
    /// 开仓金额。CTP：`OpenAmount`，`TThostFtdcMoneyType`。
    pub open_amount: f64,
    /// 平仓金额。CTP：`CloseAmount`，`TThostFtdcMoneyType`。
    pub close_amount: f64,
    /// 持仓成本。CTP：`PositionCost`，`TThostFtdcMoneyType`。
    pub position_cost: f64,
    /// 上次占用的保证金。CTP：`PreMargin`，`TThostFtdcMoneyType`。
    pub pre_margin: f64,
    /// 占用的保证金。CTP：`UseMargin`，`TThostFtdcMoneyType`。
    pub use_margin: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 开仓成本。CTP：`OpenCost`，`TThostFtdcMoneyType`。
    pub open_cost: f64,
    /// 交易所保证金。CTP：`ExchangeMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 组合成交形成的持仓。CTP：`CombPosition`，`TThostFtdcVolumeType`。
    pub comb_position: i32,
    /// 组合多头冻结。CTP：`CombLongFrozen`，`TThostFtdcVolumeType`。
    pub comb_long_frozen: i32,
    /// 组合空头冻结。CTP：`CombShortFrozen`，`TThostFtdcVolumeType`。
    pub comb_short_frozen: i32,
    /// 逐日盯市平仓盈亏。CTP：`CloseProfitByDate`，`TThostFtdcMoneyType`。
    pub close_profit_by_date: f64,
    /// 逐笔对冲平仓盈亏。CTP：`CloseProfitByTrade`，`TThostFtdcMoneyType`。
    pub close_profit_by_trade: f64,
    /// 今日持仓。CTP：`TodayPosition`，`TThostFtdcVolumeType`。
    pub today_position: i32,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 执行冻结。CTP：`StrikeFrozen`，`TThostFtdcVolumeType`。
    pub strike_frozen: i32,
    /// 执行冻结金额。CTP：`StrikeFrozenAmount`，`TThostFtdcMoneyType`。
    pub strike_frozen_amount: f64,
    /// 放弃执行冻结。CTP：`AbandonFrozen`，`TThostFtdcVolumeType`。
    pub abandon_frozen: i32,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行冻结的昨仓。CTP：`YdStrikeFrozen`，`TThostFtdcVolumeType`。
    pub yd_strike_frozen: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 持仓成本差值。CTP：`PositionCostOffset`，`TThostFtdcMoneyType`。
    pub position_cost_offset: f64,
    /// tas持仓手数。CTP：`TasPosition`，`TThostFtdcVolumeType`。
    pub tas_position: i32,
    /// tas持仓成本。CTP：`TasPositionCost`，`TThostFtdcMoneyType`。
    pub tas_position_cost: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 期权市值。CTP：`OptionValue`，`TThostFtdcMoneyType`。
    pub option_value: f64,
}

/// 投资者组合持仓明细
#[derive(Debug, Clone)]
pub struct InvestorPositionCombineDetail {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 开仓日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组合编号。CTP：`ComTradeID`，`TThostFtdcTradeIDType`。
    pub com_trade_id: TradeId,
    /// 撮合编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 买卖。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 持仓量。CTP：`TotalAmt`，`TThostFtdcVolumeType`。
    pub total_amount: i32,
    /// 投资者保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 单腿乘数。CTP：`LegMultiple`，`TThostFtdcLegMultipleType`。
    pub leg_multiple: i32,
    /// 成交组号。CTP：`TradeGroupID`，`TThostFtdcTradeGroupIDType`。
    pub trade_group_id: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 组合持仓合约编码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
}

/// 投资者持仓明细
#[derive(Debug, Clone)]
pub struct InvestorPositionDetail {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 买卖。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 开仓日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 成交编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 开仓价。CTP：`OpenPrice`，`TThostFtdcPriceType`。
    pub open_price: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 成交类型。CTP：`TradeType`，`TThostFtdcTradeTypeType`。
    pub trade_type: TradeType,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 逐日盯市平仓盈亏。CTP：`CloseProfitByDate`，`TThostFtdcMoneyType`。
    pub close_profit_by_date: f64,
    /// 逐笔对冲平仓盈亏。CTP：`CloseProfitByTrade`，`TThostFtdcMoneyType`。
    pub close_profit_by_trade: f64,
    /// 逐日盯市持仓盈亏。CTP：`PositionProfitByDate`，`TThostFtdcMoneyType`。
    pub position_profit_by_date: f64,
    /// 逐笔对冲持仓盈亏。CTP：`PositionProfitByTrade`，`TThostFtdcMoneyType`。
    pub position_profit_by_trade: f64,
    /// 投资者保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 昨结算价。CTP：`LastSettlementPrice`，`TThostFtdcPriceType`。
    pub last_settlement_price: f64,
    /// 结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 平仓量。CTP：`CloseVolume`，`TThostFtdcVolumeType`。
    pub close_volume: i32,
    /// 平仓金额。CTP：`CloseAmount`，`TThostFtdcMoneyType`。
    pub close_amount: f64,
    /// 先开先平剩余数量。CTP：`TimeFirstVolume`，`TThostFtdcVolumeType`。
    pub time_first_volume: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 特殊持仓标志。CTP：`SpecPosiType`，`TThostFtdcSpecPosiTypeType`。
    pub spec_position_type: SpecPosiType,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
}

/// 投资者品种RCAMS保证金
#[derive(Debug, Clone)]
pub struct InvestorProdRCAMSMargin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 投套标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
    /// 品种组合前风险。CTP：`RiskBeforeDiscount`，`TThostFtdcMoneyType`。
    pub risk_before_discount: f64,
    /// 同合约对冲风险。CTP：`IntraInstrRisk`，`TThostFtdcMoneyType`。
    pub intra_instrument_risk: f64,
    /// 品种买持仓风险。CTP：`BPosRisk`，`TThostFtdcMoneyType`。
    pub b_pos_risk: f64,
    /// 品种卖持仓风险。CTP：`SPosRisk`，`TThostFtdcMoneyType`。
    pub spos_risk: f64,
    /// 品种内对冲风险。CTP：`IntraProdRisk`，`TThostFtdcMoneyType`。
    pub intra_product_risk: f64,
    /// 品种净持仓风险。CTP：`NetRisk`，`TThostFtdcMoneyType`。
    pub net_risk: f64,
    /// 品种间对冲风险。CTP：`InterProdRisk`，`TThostFtdcMoneyType`。
    pub inter_product_risk: f64,
    /// 空头期权风险调整。CTP：`ShortOptRiskAdj`，`TThostFtdcMoneyType`。
    pub short_opt_risk_adj: f64,
    /// 空头期权权利金。CTP：`OptionRoyalty`，`TThostFtdcMoneyType`。
    pub option_royalty: f64,
    /// 大边组合平仓冻结保证金。CTP：`MMSACloseFrozenMargin`，`TThostFtdcMoneyType`。
    pub mmsa_close_frozen_margin: f64,
    /// 策略组合平仓/行权冻结保证金。CTP：`CloseCombFrozenMargin`，`TThostFtdcMoneyType`。
    pub close_comb_frozen_margin: f64,
    /// 平仓/行权冻结保证金。CTP：`CloseFrozenMargin`，`TThostFtdcMoneyType`。
    pub close_frozen_margin: f64,
    /// 大边组合开仓冻结保证金。CTP：`MMSAOpenFrozenMargin`，`TThostFtdcMoneyType`。
    pub mmsa_open_frozen_margin: f64,
    /// 交割月期货开仓冻结保证金。CTP：`DeliveryOpenFrozenMargin`，`TThostFtdcMoneyType`。
    pub delivery_open_frozen_margin: f64,
    /// 开仓冻结保证金。CTP：`OpenFrozenMargin`，`TThostFtdcMoneyType`。
    pub open_frozen_margin: f64,
    /// 投资者冻结保证金。CTP：`UseFrozenMargin`，`TThostFtdcMoneyType`。
    pub use_frozen_margin: f64,
    /// 大边组合交易所持仓保证金。CTP：`MMSAExchMargin`，`TThostFtdcMoneyType`。
    pub mmsa_exchange_margin: f64,
    /// 交割月期货交易所持仓保证金。CTP：`DeliveryExchMargin`，`TThostFtdcMoneyType`。
    pub delivery_exchange_margin: f64,
    /// 策略组合交易所保证金。CTP：`CombExchMargin`，`TThostFtdcMoneyType`。
    pub comb_exchange_margin: f64,
    /// 交易所持仓保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者持仓保证金。CTP：`UseMargin`，`TThostFtdcMoneyType`。
    pub use_margin: f64,
}

/// 投资者产品RULE保证金
#[derive(Debug, Clone)]
pub struct InvestorProdRULEMargin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 合约类型。CTP：`InstrumentClass`，`TThostFtdcInstrumentClassType`。
    pub instrument_class: InstrumentClass,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
    /// 买标准持仓。CTP：`BStdPosition`，`TThostFtdcStdPositionType`。
    pub b_std_position: f64,
    /// 卖标准持仓。CTP：`SStdPosition`，`TThostFtdcStdPositionType`。
    pub s_std_position: f64,
    /// 买标准开仓冻结。CTP：`BStdOpenFrozen`，`TThostFtdcStdPositionType`。
    pub b_std_open_frozen: f64,
    /// 卖标准开仓冻结。CTP：`SStdOpenFrozen`，`TThostFtdcStdPositionType`。
    pub s_std_open_frozen: f64,
    /// 买标准平仓冻结。CTP：`BStdCloseFrozen`，`TThostFtdcStdPositionType`。
    pub b_std_close_frozen: f64,
    /// 卖标准平仓冻结。CTP：`SStdCloseFrozen`，`TThostFtdcStdPositionType`。
    pub s_std_close_frozen: f64,
    /// 品种内对冲标准持仓。CTP：`IntraProdStdPosition`，`TThostFtdcStdPositionType`。
    pub intra_product_std_position: f64,
    /// 品种内单腿标准持仓。CTP：`NetStdPosition`，`TThostFtdcStdPositionType`。
    pub net_std_position: f64,
    /// 品种间对冲标准持仓。CTP：`InterProdStdPosition`，`TThostFtdcStdPositionType`。
    pub inter_product_std_position: f64,
    /// 单腿标准持仓。CTP：`SingleStdPosition`，`TThostFtdcStdPositionType`。
    pub single_std_position: f64,
    /// 品种内对锁保证金。CTP：`IntraProdMargin`，`TThostFtdcMoneyType`。
    pub intra_product_margin: f64,
    /// 品种间对锁保证金。CTP：`InterProdMargin`，`TThostFtdcMoneyType`。
    pub inter_product_margin: f64,
    /// 跨品种单腿保证金。CTP：`SingleMargin`，`TThostFtdcMoneyType`。
    pub single_margin: f64,
    /// 非组合合约保证金。CTP：`NonCombMargin`，`TThostFtdcMoneyType`。
    pub non_comb_margin: f64,
    /// 附加保证金。CTP：`AddOnMargin`，`TThostFtdcMoneyType`。
    pub add_on_margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 附加冻结保证金。CTP：`AddOnFrozenMargin`，`TThostFtdcMoneyType`。
    pub add_on_frozen_margin: f64,
    /// 开仓冻结保证金。CTP：`OpenFrozenMargin`，`TThostFtdcMoneyType`。
    pub open_frozen_margin: f64,
    /// 平仓冻结保证金。CTP：`CloseFrozenMargin`，`TThostFtdcMoneyType`。
    pub close_frozen_margin: f64,
    /// 品种保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 冻结保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
}

/// 投资者产品SPBM明细
#[derive(Debug, Clone)]
pub struct InvestorProdSPBMDetail {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 合约内对锁保证金。CTP：`IntraInstrMargin`，`TThostFtdcMoneyType`。
    pub intra_instrument_margin: f64,
    /// 买归集保证金。CTP：`BCollectingMargin`，`TThostFtdcMoneyType`。
    pub b_collecting_margin: f64,
    /// 卖归集保证金。CTP：`SCollectingMargin`，`TThostFtdcMoneyType`。
    pub s_collecting_margin: f64,
    /// 品种内合约间对锁保证金。CTP：`IntraProdMargin`，`TThostFtdcMoneyType`。
    pub intra_product_margin: f64,
    /// 净保证金。CTP：`NetMargin`，`TThostFtdcMoneyType`。
    pub net_margin: f64,
    /// 产品间对锁保证金。CTP：`InterProdMargin`，`TThostFtdcMoneyType`。
    pub inter_product_margin: f64,
    /// 裸保证金。CTP：`SingleMargin`，`TThostFtdcMoneyType`。
    pub single_margin: f64,
    /// 附加保证金。CTP：`AddOnMargin`，`TThostFtdcMoneyType`。
    pub add_on_margin: f64,
    /// 交割月保证金。CTP：`DeliveryMargin`，`TThostFtdcMoneyType`。
    pub delivery_margin: f64,
    /// 看涨期权最低风险。CTP：`CallOptionMinRisk`，`TThostFtdcMoneyType`。
    pub call_option_min_risk: f64,
    /// 看跌期权最低风险。CTP：`PutOptionMinRisk`，`TThostFtdcMoneyType`。
    pub put_option_min_risk: f64,
    /// 卖方期权最低风险。CTP：`OptionMinRisk`，`TThostFtdcMoneyType`。
    pub option_min_risk: f64,
    /// 买方期权冲抵价值。CTP：`OptionValueOffset`，`TThostFtdcMoneyType`。
    pub option_value_offset: f64,
    /// 卖方期权权利金。CTP：`OptionRoyalty`，`TThostFtdcMoneyType`。
    pub option_royalty: f64,
    /// 价值冲抵。CTP：`RealOptionValueOffset`，`TThostFtdcMoneyType`。
    pub real_option_value_offset: f64,
    /// 保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
}

/// 投资者品种/跨品种保证金
#[derive(Debug, Clone)]
pub struct InvestorProductGroupMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 多头冻结的保证金。CTP：`LongFrozenMargin`，`TThostFtdcMoneyType`。
    pub long_frozen_margin: f64,
    /// 空头冻结的保证金。CTP：`ShortFrozenMargin`，`TThostFtdcMoneyType`。
    pub short_frozen_margin: f64,
    /// 占用的保证金。CTP：`UseMargin`，`TThostFtdcMoneyType`。
    pub use_margin: f64,
    /// 多头保证金。CTP：`LongUseMargin`，`TThostFtdcMoneyType`。
    pub long_use_margin: f64,
    /// 空头保证金。CTP：`ShortUseMargin`，`TThostFtdcMoneyType`。
    pub short_use_margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 交易所多头保证金。CTP：`LongExchMargin`，`TThostFtdcMoneyType`。
    pub long_exchange_margin: f64,
    /// 交易所空头保证金。CTP：`ShortExchMargin`，`TThostFtdcMoneyType`。
    pub short_exchange_margin: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 折抵总金额。CTP：`OffsetAmount`，`TThostFtdcMoneyType`。
    pub offset_amount: f64,
    /// 多头折抵总金额。CTP：`LongOffsetAmount`，`TThostFtdcMoneyType`。
    pub long_offset_amount: f64,
    /// 空头折抵总金额。CTP：`ShortOffsetAmount`，`TThostFtdcMoneyType`。
    pub short_offset_amount: f64,
    /// 交易所折抵总金额。CTP：`ExchOffsetAmount`，`TThostFtdcMoneyType`。
    pub exchange_offset_amount: f64,
    /// 交易所多头折抵总金额。CTP：`LongExchOffsetAmount`，`TThostFtdcMoneyType`。
    pub long_exchange_offset_amount: f64,
    /// 交易所空头折抵总金额。CTP：`ShortExchOffsetAmount`，`TThostFtdcMoneyType`。
    pub short_exchange_offset_amount: f64,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 品种/跨品种标示。CTP：`ProductGroupID`，`TThostFtdcInstrumentIDType`。
    pub product_group_id: InstrumentId,
}

/// 投资者预留信息
#[derive(Debug, Clone)]
pub struct InvestorReserveInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 预留信息。CTP：`ReserveInfo`，`TThostFtdcReserveInfoType`。
    pub reserve_info: String,
}

/// 投资者交易权限设置
#[derive(Debug, Clone)]
pub struct InvestorTradingRight {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易权限。CTP：`InvstTradingRight`，`TThostFtdcInvstTradingRightType`。
    pub invest_trading_right: InvestorTradingRightType,
}

/// 经纪公司可提资金算法表
#[derive(Debug, Clone)]
pub struct InvestorWithdrawAlgorithm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 可提资金比例。CTP：`UsingRatio`，`TThostFtdcRatioType`。
    pub using_ratio: f64,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 货币质押比率。CTP：`FundMortgageRatio`，`TThostFtdcRatioType`。
    pub fund_mortgage_ratio: f64,
}

/// 服务地址参数
#[derive(Debug, Clone)]
pub struct IpAddrParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 服务地址。CTP：`Address`，`TThostFtdcIpAddrType`。
    pub address: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 交易中心名称。CTP：`DRIdentityName`，`TThostFtdcDRIdentityNameType`。
    pub dr_identity_name: String,
    /// 交易地址OR行情地址。CTP：`AddrSrvMode`，`TThostFtdcAddrSrvModeType`。
    pub address_srv_mode: AddrSrvMode,
    /// 地址版本。CTP：`AddrVer`，`TThostFtdcAddrVerType`。
    pub address_ver: AddrVer,
    /// 服务地址编号。CTP：`AddrNo`，`TThostFtdcCommonIntType`。
    pub address_no: i32,
    /// 服务地址名称。CTP：`AddrName`，`TThostFtdcAddrNameType`。
    pub address_name: String,
    /// 是否是国密地址。CTP：`IsSM`，`TThostFtdcBoolType`。
    pub is_sm: bool,
    /// 是否是内网地址。CTP：`IsLocalAddr`，`TThostFtdcBoolType`。
    pub is_local_address: bool,
    /// 地址补充信息。CTP：`Remark`，`TThostFtdcAddrRemarkType`。
    pub remark: String,
    /// 站点。CTP：`Site`，`TThostFtdcSiteType`。
    pub site: String,
    /// 网络运营商。CTP：`NetOperator`，`TThostFtdcNetOperatorType`。
    pub net_operator: String,
    /// 系统名称。CTP：`SysName`，`TThostFtdcAddrNameType`。
    pub system_name: String,
}

/// 联系人
#[derive(Debug, Clone)]
pub struct LinkMan {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 联系人类型。CTP：`PersonType`，`TThostFtdcPersonTypeType`。
    pub person_type: PersonType,
    /// 证件类型。CTP：`IdentifiedCardType`，`TThostFtdcIdCardTypeType`。
    pub identified_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 名称。CTP：`PersonName`，`TThostFtdcPartyNameType`。
    pub person_name: String,
    /// 联系电话。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 通讯地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮政编码。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 优先级。CTP：`Priority`，`TThostFtdcPriorityType`。
    pub priority: i32,
    /// 开户邮政编码。CTP：`UOAZipCode`，`TThostFtdcUOAZipCodeType`。
    pub uoa_zip_code: String,
    /// 全称。CTP：`PersonFullName`，`TThostFtdcInvestorFullNameType`。
    pub person_full_name: String,
}

/// 装载结算信息
#[derive(Debug, Clone)]
pub struct LoadSettlementInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 内网地址配置
#[derive(Debug, Clone)]
pub struct LocalAddrConfig {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 对端地址。CTP：`PeerAddr`，`TThostFtdcIpAddrType`。
    pub peer_address: String,
    /// 子网掩码。CTP：`NetMask`，`TThostFtdcIpAddrType`。
    pub net_mask: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 内网服务地址。CTP：`LocalAddress`，`TThostFtdcIpAddrType`。
    pub local_address: String,
}

/// 禁止登录IP
#[derive(Debug, Clone)]
pub struct LoginForbiddenIP {
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 禁止登录用户
#[derive(Debug, Clone)]
pub struct LoginForbiddenUser {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 登录信息
#[derive(Debug, Clone)]
pub struct LoginInfo {
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 登录日期。CTP：`LoginDate`，`TThostFtdcDateType`。
    pub login_date: DateText,
    /// 登录时间。CTP：`LoginTime`，`TThostFtdcTimeType`。
    pub login_time: TimeText,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// 系统名称。CTP：`SystemName`，`TThostFtdcSystemNameType`。
    pub system_name: String,
    /// 密码,已弃用。CTP：`PasswordDeprecated`，`TThostFtdcPasswordType`。
    pub password_deprecated: String,
    /// 最大报单引用。CTP：`MaxOrderRef`，`TThostFtdcOrderRefType`。
    pub max_order_ref: OrderRef,
    /// 上期所时间。CTP：`SHFETime`，`TThostFtdcTimeType`。
    pub shfe_time: TimeText,
    /// 大商所时间。CTP：`DCETime`，`TThostFtdcTimeType`。
    pub dce_time: TimeText,
    /// 郑商所时间。CTP：`CZCETime`，`TThostFtdcTimeType`。
    pub czce_time: TimeText,
    /// 中金所时间。CTP：`FFEXTime`，`TThostFtdcTimeType`。
    pub ffex_time: TimeText,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 动态密码。CTP：`OneTimePassword`，`TThostFtdcPasswordType`。
    pub one_time_password: String,
    /// 能源中心时间。CTP：`INETime`，`TThostFtdcTimeType`。
    pub ine_time: TimeText,
    /// 查询时是否需要流控。CTP：`IsQryControl`，`TThostFtdcBoolType`。
    pub is_qry_control: bool,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 登录信息
#[derive(Debug, Clone)]
pub struct LogoutAll {
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 系统名称。CTP：`SystemName`，`TThostFtdcSystemNameType`。
    pub system_name: String,
}

/// 交易所行情报盘机
#[derive(Debug, Clone)]
pub struct MDTraderOffer {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所交易员连接状态。CTP：`TraderConnectStatus`，`TThostFtdcTraderConnectStatusType`。
    pub trader_connect_status: TraderConnectStatus,
    /// 发出连接请求的日期。CTP：`ConnectRequestDate`，`TThostFtdcDateType`。
    pub connect_request_date: DateText,
    /// 发出连接请求的时间。CTP：`ConnectRequestTime`，`TThostFtdcTimeType`。
    pub connect_request_time: TimeText,
    /// 上次报告日期。CTP：`LastReportDate`，`TThostFtdcDateType`。
    pub last_report_date: DateText,
    /// 上次报告时间。CTP：`LastReportTime`，`TThostFtdcTimeType`。
    pub last_report_time: TimeText,
    /// 完成连接日期。CTP：`ConnectDate`，`TThostFtdcDateType`。
    pub connect_date: DateText,
    /// 完成连接时间。CTP：`ConnectTime`，`TThostFtdcTimeType`。
    pub connect_time: TimeText,
    /// 启动日期。CTP：`StartDate`，`TThostFtdcDateType`。
    pub start_date: DateText,
    /// 启动时间。CTP：`StartTime`，`TThostFtdcTimeType`。
    pub start_time: TimeText,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 本席位最大成交编号。CTP：`MaxTradeID`，`TThostFtdcTradeIDType`。
    pub max_trade_id: TradeId,
    /// 本席位最大报单备拷。CTP：`MaxOrderMessageReference`，`TThostFtdcReturnCodeType`。
    pub max_order_message_reference: String,
    /// 撤单时选择席位算法。CTP：`OrderCancelAlg`，`TThostFtdcOrderCancelAlgType`。
    pub order_cancel_alg: OrderCancelAlg,
}

/// 做市商合约手续费率
#[derive(Debug, Clone)]
pub struct MMInstrumentCommissionRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 当前做市商期权合约手续费的详细内容
#[derive(Debug, Clone)]
pub struct MMOptionInstrCommRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 执行手续费率。CTP：`StrikeRatioByMoney`，`TThostFtdcRatioType`。
    pub strike_ratio_by_money: f64,
    /// 执行手续费。CTP：`StrikeRatioByVolume`，`TThostFtdcRatioType`。
    pub strike_ratio_by_volume: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 手工同步用户动态令牌
#[derive(Debug, Clone)]
pub struct ManualSyncBrokerUserOTP {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 动态令牌类型。CTP：`OTPType`，`TThostFtdcOTPTypeType`。
    pub otp_type: OTPType,
    /// 第一个动态密码。CTP：`FirstOTP`，`TThostFtdcPasswordType`。
    pub first_otp: String,
    /// 第二个动态密码。CTP：`SecondOTP`，`TThostFtdcPasswordType`。
    pub second_otp: String,
}

/// 投资者保证金率模板
#[derive(Debug, Clone)]
pub struct MarginModel {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 保证金率模板代码。CTP：`MarginModelID`，`TThostFtdcInvestorIDType`。
    pub margin_model_id: InvestorId,
    /// 模板名称。CTP：`MarginModelName`，`TThostFtdcCommModelNameType`。
    pub margin_model_name: String,
}

/// 市场行情
#[derive(Debug, Clone)]
pub struct MarketData {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 最新价。CTP：`LastPrice`，`TThostFtdcPriceType`。
    pub last_price: f64,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 昨收盘。CTP：`PreClosePrice`，`TThostFtdcPriceType`。
    pub pre_close_price: f64,
    /// 昨持仓量。CTP：`PreOpenInterest`，`TThostFtdcLargeVolumeType`。
    pub pre_open_interest: f64,
    /// 今开盘。CTP：`OpenPrice`，`TThostFtdcPriceType`。
    pub open_price: f64,
    /// 最高价。CTP：`HighestPrice`，`TThostFtdcPriceType`。
    pub highest_price: f64,
    /// 最低价。CTP：`LowestPrice`，`TThostFtdcPriceType`。
    pub lowest_price: f64,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 成交金额。CTP：`Turnover`，`TThostFtdcMoneyType`。
    pub turnover: f64,
    /// 持仓量。CTP：`OpenInterest`，`TThostFtdcLargeVolumeType`。
    pub open_interest: f64,
    /// 今收盘。CTP：`ClosePrice`，`TThostFtdcPriceType`。
    pub close_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 涨停板价。CTP：`UpperLimitPrice`，`TThostFtdcPriceType`。
    pub upper_limit_price: f64,
    /// 跌停板价。CTP：`LowerLimitPrice`，`TThostFtdcPriceType`。
    pub lower_limit_price: f64,
    /// 昨虚实度。CTP：`PreDelta`，`TThostFtdcRatioType`。
    pub pre_delta: f64,
    /// 今虚实度。CTP：`CurrDelta`，`TThostFtdcRatioType`。
    pub current_delta: f64,
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 最后修改毫秒。CTP：`UpdateMillisec`，`TThostFtdcMillisecType`。
    pub update_millis: i32,
    /// 业务日期。CTP：`ActionDay`，`TThostFtdcDateType`。
    pub action_day: DateText,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 行情申卖二、三属性
#[derive(Debug, Clone)]
pub struct MarketDataAsk23 {
    /// 申卖价二。CTP：`AskPrice2`，`TThostFtdcPriceType`。
    pub ask_price2: f64,
    /// 申卖量二。CTP：`AskVolume2`，`TThostFtdcVolumeType`。
    pub ask_volume2: i32,
    /// 申卖价三。CTP：`AskPrice3`，`TThostFtdcPriceType`。
    pub ask_price3: f64,
    /// 申卖量三。CTP：`AskVolume3`，`TThostFtdcVolumeType`。
    pub ask_volume3: i32,
}

/// 行情申卖四、五属性
#[derive(Debug, Clone)]
pub struct MarketDataAsk45 {
    /// 申卖价四。CTP：`AskPrice4`，`TThostFtdcPriceType`。
    pub ask_price4: f64,
    /// 申卖量四。CTP：`AskVolume4`，`TThostFtdcVolumeType`。
    pub ask_volume4: i32,
    /// 申卖价五。CTP：`AskPrice5`，`TThostFtdcPriceType`。
    pub ask_price5: f64,
    /// 申卖量五。CTP：`AskVolume5`，`TThostFtdcVolumeType`。
    pub ask_volume5: i32,
}

/// 成交均价
#[derive(Debug, Clone)]
pub struct MarketDataAveragePrice {
    /// 当日均价。CTP：`AveragePrice`，`TThostFtdcPriceType`。
    pub average_price: f64,
}

/// 行情上下带价
#[derive(Debug, Clone)]
pub struct MarketDataBandingPrice {
    /// 上带价。CTP：`BandingUpperPrice`，`TThostFtdcPriceType`。
    pub banding_upper_price: f64,
    /// 下带价。CTP：`BandingLowerPrice`，`TThostFtdcPriceType`。
    pub banding_lower_price: f64,
}

/// 行情基础属性
#[derive(Debug, Clone)]
pub struct MarketDataBase {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 昨收盘。CTP：`PreClosePrice`，`TThostFtdcPriceType`。
    pub pre_close_price: f64,
    /// 昨持仓量。CTP：`PreOpenInterest`，`TThostFtdcLargeVolumeType`。
    pub pre_open_interest: f64,
    /// 昨虚实度。CTP：`PreDelta`，`TThostFtdcRatioType`。
    pub pre_delta: f64,
}

/// 行情最优价属性
#[derive(Debug, Clone)]
pub struct MarketDataBestPrice {
    /// 申买价一。CTP：`BidPrice1`，`TThostFtdcPriceType`。
    pub bid_price1: f64,
    /// 申买量一。CTP：`BidVolume1`，`TThostFtdcVolumeType`。
    pub bid_volume1: i32,
    /// 申卖价一。CTP：`AskPrice1`，`TThostFtdcPriceType`。
    pub ask_price1: f64,
    /// 申卖量一。CTP：`AskVolume1`，`TThostFtdcVolumeType`。
    pub ask_volume1: i32,
}

/// 行情申买二、三属性
#[derive(Debug, Clone)]
pub struct MarketDataBid23 {
    /// 申买价二。CTP：`BidPrice2`，`TThostFtdcPriceType`。
    pub bid_price2: f64,
    /// 申买量二。CTP：`BidVolume2`，`TThostFtdcVolumeType`。
    pub bid_volume2: i32,
    /// 申买价三。CTP：`BidPrice3`，`TThostFtdcPriceType`。
    pub bid_price3: f64,
    /// 申买量三。CTP：`BidVolume3`，`TThostFtdcVolumeType`。
    pub bid_volume3: i32,
}

/// 行情申买四、五属性
#[derive(Debug, Clone)]
pub struct MarketDataBid45 {
    /// 申买价四。CTP：`BidPrice4`，`TThostFtdcPriceType`。
    pub bid_price4: f64,
    /// 申买量四。CTP：`BidVolume4`，`TThostFtdcVolumeType`。
    pub bid_volume4: i32,
    /// 申买价五。CTP：`BidPrice5`，`TThostFtdcPriceType`。
    pub bid_price5: f64,
    /// 申买量五。CTP：`BidVolume5`，`TThostFtdcVolumeType`。
    pub bid_volume5: i32,
}

/// 行情交易所代码属性
#[derive(Debug, Clone)]
pub struct MarketDataExchange {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 行情最新成交属性
#[derive(Debug, Clone)]
pub struct MarketDataLastMatch {
    /// 最新价。CTP：`LastPrice`，`TThostFtdcPriceType`。
    pub last_price: f64,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 成交金额。CTP：`Turnover`，`TThostFtdcMoneyType`。
    pub turnover: f64,
    /// 持仓量。CTP：`OpenInterest`，`TThostFtdcLargeVolumeType`。
    pub open_interest: f64,
}

/// 行情静态属性
#[derive(Debug, Clone)]
pub struct MarketDataStatic {
    /// 今开盘。CTP：`OpenPrice`，`TThostFtdcPriceType`。
    pub open_price: f64,
    /// 最高价。CTP：`HighestPrice`，`TThostFtdcPriceType`。
    pub highest_price: f64,
    /// 最低价。CTP：`LowestPrice`，`TThostFtdcPriceType`。
    pub lowest_price: f64,
    /// 今收盘。CTP：`ClosePrice`，`TThostFtdcPriceType`。
    pub close_price: f64,
    /// 涨停板价。CTP：`UpperLimitPrice`，`TThostFtdcPriceType`。
    pub upper_limit_price: f64,
    /// 跌停板价。CTP：`LowerLimitPrice`，`TThostFtdcPriceType`。
    pub lower_limit_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 今虚实度。CTP：`CurrDelta`，`TThostFtdcRatioType`。
    pub current_delta: f64,
}

/// 行情更新时间属性
#[derive(Debug, Clone)]
pub struct MarketDataUpdateTime {
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 最后修改毫秒。CTP：`UpdateMillisec`，`TThostFtdcMillisecType`。
    pub update_millis: i32,
    /// 业务日期。CTP：`ActionDay`，`TThostFtdcDateType`。
    pub action_day: DateText,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 质押配比参数
#[derive(Debug, Clone)]
pub struct MortgageParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 质押配比系数。CTP：`MortgageBalance`，`TThostFtdcRatioType`。
    pub mortgage_balance: f64,
    /// 开仓是否验证质押配比。CTP：`CheckMortgageRatio`，`TThostFtdcBoolType`。
    pub check_mortgage_ratio: bool,
}

/// 组播合约
#[derive(Debug, Clone)]
pub struct MulticastInstrument {
    /// 主题号。CTP：`TopicID`，`TThostFtdcInstallIDType`。
    pub topic_id: i32,
    /// 合约编号。CTP：`InstrumentNo`，`TThostFtdcInstallIDType`。
    pub instrument_no: i32,
    /// 基准价。CTP：`CodePrice`，`TThostFtdcPriceType`。
    pub code_price: f64,
    /// 合约数量乘数。CTP：`VolumeMultiple`，`TThostFtdcVolumeMultipleType`。
    pub volume_multiple: i32,
    /// 最小变动价位。CTP：`PriceTick`，`TThostFtdcPriceType`。
    pub price_tick: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 客户通知
#[derive(Debug, Clone)]
pub struct Notice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 消息正文。CTP：`Content`，`TThostFtdcContentType`。
    pub content: String,
    /// 经纪公司通知内容序列号。CTP：`SequenceLabel`，`TThostFtdcSequenceLabelType`。
    pub sequence_label: String,
}

/// 期商签到通知
#[derive(Debug, Clone)]
pub struct NotifyFutureSignIn {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// PIN密钥。CTP：`PinKey`，`TThostFtdcPasswordKeyType`。
    pub pin_key: String,
    /// MAC密钥。CTP：`MacKey`，`TThostFtdcPasswordKeyType`。
    pub mac_key: String,
}

/// 期商签退通知
#[derive(Debug, Clone)]
pub struct NotifyFutureSignOut {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 查询账户信息通知
#[derive(Debug, Clone)]
pub struct NotifyQueryAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 银行可用金额。CTP：`BankUseAmount`，`TThostFtdcTradeAmountType`。
    pub bank_use_amount: f64,
    /// 银行可取金额。CTP：`BankFetchAmount`，`TThostFtdcTradeAmountType`。
    pub bank_fetch_amount: f64,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 查询银行资金账户信息通知 要发往次席
#[derive(Debug, Clone)]
pub struct NotifyQueryFutureAccountBySec {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 银行可用金额。CTP：`BankUseAmount`，`TThostFtdcTradeAmountType`。
    pub bank_use_amount: f64,
    /// 银行可取金额。CTP：`BankFetchAmount`，`TThostFtdcTradeAmountType`。
    pub bank_fetch_amount: f64,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 次中心发起转账期货公司流水号。CTP：`SecFutureSerial`，`TThostFtdcFutureSerialType`。
    pub sec_future_serial: i32,
}

/// 交易核心向银期报盘发出密钥同步处理结果的通知
#[derive(Debug, Clone)]
pub struct NotifySyncKey {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易核心给银期报盘的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 对冲设置
#[derive(Debug, Clone)]
pub struct OffsetSetting {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 标的期货合约代码。CTP：`UnderlyingInstrID`，`TThostFtdcInstrumentIDType`。
    pub underlying_instrument_id: InstrumentId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 对冲类型。CTP：`OffsetType`，`TThostFtdcOffsetTypeType`。
    pub offset_type: OffsetType,
    /// 申请对冲的合约数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 是否对冲。CTP：`IsOffset`，`TThostFtdcBoolType`。
    pub is_offset: bool,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 交易所合约代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 交易所期权系列号。CTP：`ExchangeSerialNo`，`TThostFtdcExchangeInstIDType`。
    pub exchange_serial_no: ExchangeInstrumentId,
    /// 交易所产品代码。CTP：`ExchangeProductID`，`TThostFtdcProductIDType`。
    pub exchange_product_id: ProductId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 对冲提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 对冲设置结果。CTP：`ExecResult`，`TThostFtdcExecResultType`。
    pub exec_result: ExecResult,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerOffsetSettingSeq`，`TThostFtdcSequenceNoType`。
    pub broker_offset_setting_seq: i32,
    /// 申请来源。CTP：`ApplySrc`，`TThostFtdcApplySrcType`。
    pub apply_src: ApplySrc,
}

/// 银期开户信息
#[derive(Debug, Clone)]
pub struct OpenAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 汇钞标志。CTP：`CashExchangeCode`，`TThostFtdcCashExchangeCodeType`。
    pub cash_exchange_code: CashExchangeCode,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 当前期权合约手续费的详细内容
#[derive(Debug, Clone)]
pub struct OptionInstrCommRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 执行手续费率。CTP：`StrikeRatioByMoney`，`TThostFtdcRatioType`。
    pub strike_ratio_by_money: f64,
    /// 执行手续费。CTP：`StrikeRatioByVolume`，`TThostFtdcRatioType`。
    pub strike_ratio_by_volume: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 期权合约delta值
#[derive(Debug, Clone)]
pub struct OptionInstrDelta {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// Delta值。CTP：`Delta`，`TThostFtdcRatioType`。
    pub delta: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 当前期权合约保证金调整系数
#[derive(Debug, Clone)]
pub struct OptionInstrMarginAdjust {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机空头保证金调整系数。CTP：`SShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub s_short_margin_ratio_by_money: f64,
    /// 投机空头保证金调整系数。CTP：`SShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub s_short_margin_ratio_by_volume: f64,
    /// 保值空头保证金调整系数。CTP：`HShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub h_short_margin_ratio_by_money: f64,
    /// 保值空头保证金调整系数。CTP：`HShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub h_short_margin_ratio_by_volume: f64,
    /// 套利空头保证金调整系数。CTP：`AShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub a_short_margin_ratio_by_money: f64,
    /// 套利空头保证金调整系数。CTP：`AShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub a_short_margin_ratio_by_volume: f64,
    /// 是否跟随交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 做市商空头保证金调整系数。CTP：`MShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub m_short_margin_ratio_by_money: f64,
    /// 做市商空头保证金调整系数。CTP：`MShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub m_short_margin_ratio_by_volume: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 当前期权合约最小保证金
#[derive(Debug, Clone)]
pub struct OptionInstrMiniMargin {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 单位（手）期权合约最小保证金。CTP：`MinMargin`，`TThostFtdcMoneyType`。
    pub min_margin: f64,
    /// 取值方式。CTP：`ValueMethod`，`TThostFtdcValueMethodType`。
    pub value_method: ValueMethod,
    /// 是否跟随交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 期权交易成本
#[derive(Debug, Clone)]
pub struct OptionInstrTradeCost {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 期权合约保证金不变部分。CTP：`FixedMargin`，`TThostFtdcMoneyType`。
    pub fixed_margin: f64,
    /// 期权合约最小保证金。CTP：`MiniMargin`，`TThostFtdcMoneyType`。
    pub mini_margin: f64,
    /// 期权合约权利金。CTP：`Royalty`，`TThostFtdcMoneyType`。
    pub royalty: f64,
    /// 交易所期权合约保证金不变部分。CTP：`ExchFixedMargin`，`TThostFtdcMoneyType`。
    pub exchange_fixed_margin: f64,
    /// 交易所期权合约最小保证金。CTP：`ExchMiniMargin`，`TThostFtdcMoneyType`。
    pub exchange_mini_margin: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 投资者期权合约交易权限
#[derive(Debug, Clone)]
pub struct OptionInstrTradingRight {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 交易权限。CTP：`TradingRight`，`TThostFtdcTradingRightType`。
    pub trading_right: TradingRight,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 期权自对冲
#[derive(Debug, Clone)]
pub struct OptionSelfClose {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 期权自对冲引用。CTP：`OptionSelfCloseRef`，`TThostFtdcOrderRefType`。
    pub option_self_close_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 期权行权的头寸是否自对冲。CTP：`OptSelfCloseFlag`，`TThostFtdcOptSelfCloseFlagType`。
    pub opt_self_close_flag: OptSelfCloseFlag,
    /// 本地期权自对冲编号。CTP：`OptionSelfCloseLocalID`，`TThostFtdcOrderLocalIDType`。
    pub option_self_close_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期权自对冲提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 期权自对冲编号。CTP：`OptionSelfCloseSysID`，`TThostFtdcOrderSysIDType`。
    pub option_self_close_system_id: OrderSysId,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 自对冲结果。CTP：`ExecResult`，`TThostFtdcExecResultType`。
    pub exec_result: ExecResult,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerOptionSelfCloseSeq`，`TThostFtdcSequenceNoType`。
    pub broker_option_self_close_seq: i32,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 期权自对冲操作
#[derive(Debug, Clone)]
pub struct OptionSelfCloseAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 期权自对冲操作引用。CTP：`OptionSelfCloseActionRef`，`TThostFtdcOrderActionRefType`。
    pub option_self_close_action_ref: i32,
    /// 期权自对冲引用。CTP：`OptionSelfCloseRef`，`TThostFtdcOrderRefType`。
    pub option_self_close_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 期权自对冲操作编号。CTP：`OptionSelfCloseSysID`，`TThostFtdcOrderSysIDType`。
    pub option_self_close_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地期权自对冲编号。CTP：`OptionSelfCloseLocalID`，`TThostFtdcOrderLocalIDType`。
    pub option_self_close_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 报单
#[derive(Debug, Clone)]
pub struct Order {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 报单价格条件。CTP：`OrderPriceType`，`TThostFtdcOrderPriceTypeType`。
    pub order_price_type: OrderPriceType,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合开平标志。CTP：`CombOffsetFlag`，`TThostFtdcCombOffsetFlagType`。
    pub comb_offset_flag: CombOffsetFlag,
    /// 组合投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量。CTP：`VolumeTotalOriginal`，`TThostFtdcVolumeType`。
    pub volume_total_original: i32,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// GTD日期。CTP：`GTDDate`，`TThostFtdcDateType`。
    pub gtd_date: DateText,
    /// 成交量类型。CTP：`VolumeCondition`，`TThostFtdcVolumeConditionType`。
    pub volume_condition: VolumeCondition,
    /// 最小成交量。CTP：`MinVolume`，`TThostFtdcVolumeType`。
    pub min_volume: i32,
    /// 触发条件。CTP：`ContingentCondition`，`TThostFtdcContingentConditionType`。
    pub contingent_condition: ContingentCondition,
    /// 止损价。CTP：`StopPrice`，`TThostFtdcPriceType`。
    pub stop_price: f64,
    /// 强平原因。CTP：`ForceCloseReason`，`TThostFtdcForceCloseReasonType`。
    pub force_close_reason: ForceCloseReason,
    /// 自动挂起标志。CTP：`IsAutoSuspend`，`TThostFtdcBoolType`。
    pub is_auto_suspend: bool,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 报单来源。CTP：`OrderSource`，`TThostFtdcOrderSourceType`。
    pub order_source: OrderSource,
    /// 报单状态。CTP：`OrderStatus`，`TThostFtdcOrderStatusType`。
    pub order_status: OrderStatus,
    /// 报单类型。CTP：`OrderType`，`TThostFtdcOrderTypeType`。
    pub order_type: OrderType,
    /// 今成交数量。CTP：`VolumeTraded`，`TThostFtdcVolumeType`。
    pub volume_traded: i32,
    /// 剩余数量。CTP：`VolumeTotal`，`TThostFtdcVolumeType`。
    pub volume_total: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 委托时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 激活时间。CTP：`ActiveTime`，`TThostFtdcTimeType`。
    pub active_time: TimeText,
    /// 挂起时间。CTP：`SuspendTime`，`TThostFtdcTimeType`。
    pub suspend_time: TimeText,
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 最后修改交易所交易员代码。CTP：`ActiveTraderID`，`TThostFtdcTraderIDType`。
    pub active_trader_id: String,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 用户强平标志。CTP：`UserForceClose`，`TThostFtdcBoolType`。
    pub user_force_close: bool,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerOrderSeq`，`TThostFtdcSequenceNoType`。
    pub broker_order_seq: i32,
    /// 相关报单。CTP：`RelativeOrderSysID`，`TThostFtdcOrderSysIDType`。
    pub relative_order_system_id: OrderSysId,
    /// 郑商所成交数量。CTP：`ZCETotalTradedVolume`，`TThostFtdcVolumeType`。
    pub zce_total_traded_volume: i32,
    /// 互换单标志。CTP：`IsSwapOrder`，`TThostFtdcBoolType`。
    pub is_swap_order: bool,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 报单操作
#[derive(Debug, Clone)]
pub struct OrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单操作引用。CTP：`OrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub order_action_ref: i32,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量变化。CTP：`VolumeChange`，`TThostFtdcVolumeType`。
    pub volume_change: i32,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 预埋单
#[derive(Debug, Clone)]
pub struct ParkedOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 报单价格条件。CTP：`OrderPriceType`，`TThostFtdcOrderPriceTypeType`。
    pub order_price_type: OrderPriceType,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 组合开平标志。CTP：`CombOffsetFlag`，`TThostFtdcCombOffsetFlagType`。
    pub comb_offset_flag: CombOffsetFlag,
    /// 组合投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcCombHedgeFlagType`。
    pub comb_hedge_flag: CombHedgeFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量。CTP：`VolumeTotalOriginal`，`TThostFtdcVolumeType`。
    pub volume_total_original: i32,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// GTD日期。CTP：`GTDDate`，`TThostFtdcDateType`。
    pub gtd_date: DateText,
    /// 成交量类型。CTP：`VolumeCondition`，`TThostFtdcVolumeConditionType`。
    pub volume_condition: VolumeCondition,
    /// 最小成交量。CTP：`MinVolume`，`TThostFtdcVolumeType`。
    pub min_volume: i32,
    /// 触发条件。CTP：`ContingentCondition`，`TThostFtdcContingentConditionType`。
    pub contingent_condition: ContingentCondition,
    /// 止损价。CTP：`StopPrice`，`TThostFtdcPriceType`。
    pub stop_price: f64,
    /// 强平原因。CTP：`ForceCloseReason`，`TThostFtdcForceCloseReasonType`。
    pub force_close_reason: ForceCloseReason,
    /// 自动挂起标志。CTP：`IsAutoSuspend`，`TThostFtdcBoolType`。
    pub is_auto_suspend: bool,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 用户强平标志。CTP：`UserForceClose`，`TThostFtdcBoolType`。
    pub user_force_close: bool,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 预埋报单编号。CTP：`ParkedOrderID`，`TThostFtdcParkedOrderIDType`。
    pub parked_order_id: String,
    /// 用户类型。CTP：`UserType`，`TThostFtdcUserTypeType`。
    pub user_type: UserType,
    /// 预埋单状态。CTP：`Status`，`TThostFtdcParkedOrderStatusType`。
    pub status: ParkedOrderStatus,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 互换单标志。CTP：`IsSwapOrder`，`TThostFtdcBoolType`。
    pub is_swap_order: bool,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易编码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 输入预埋单操作
#[derive(Debug, Clone)]
pub struct ParkedOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单操作引用。CTP：`OrderActionRef`，`TThostFtdcOrderActionRefType`。
    pub order_action_ref: i32,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 价格。CTP：`LimitPrice`，`TThostFtdcPriceType`。
    pub limit_price: f64,
    /// 数量变化。CTP：`VolumeChange`，`TThostFtdcVolumeType`。
    pub volume_change: i32,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 预埋撤单单编号。CTP：`ParkedOrderActionID`，`TThostFtdcParkedOrderActionIDType`。
    pub parked_order_action_id: String,
    /// 用户类型。CTP：`UserType`，`TThostFtdcUserTypeType`。
    pub user_type: UserType,
    /// 预埋撤单状态。CTP：`Status`，`TThostFtdcParkedOrderStatusType`。
    pub status: ParkedOrderStatus,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 会员编码和经纪公司编码对照表
#[derive(Debug, Clone)]
pub struct PartBroker {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
}

/// 组保交易参数设置
#[derive(Debug, Clone)]
pub struct PortfTradeParamSetting {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 新型组保算法。CTP：`Portfolio`，`TThostFtdcPortfolioType`。
    pub portfolio: Portfolio,
    /// 撤单是否验资。CTP：`IsActionVerify`，`TThostFtdcBoolType`。
    pub is_action_verify: bool,
    /// 平仓是否验资。CTP：`IsCloseVerify`，`TThostFtdcBoolType`。
    pub is_close_verify: bool,
}

/// 浮动盈亏算法
#[derive(Debug, Clone)]
pub struct PositionProfitAlgorithm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 盈亏算法。CTP：`Algorithm`，`TThostFtdcAlgorithmType`。
    pub algorithm: Algorithm,
    /// 备注。CTP：`Memo`，`TThostFtdcMemoType`。
    pub memo: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 产品
#[derive(Debug, Clone)]
pub struct Product {
    /// 产品名称。CTP：`ProductName`，`TThostFtdcProductNameType`。
    pub product_name: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品类型。CTP：`ProductClass`，`TThostFtdcProductClassType`。
    pub product_class: ProductClass,
    /// 合约数量乘数。CTP：`VolumeMultiple`，`TThostFtdcVolumeMultipleType`。
    pub volume_multiple: i32,
    /// 最小变动价位。CTP：`PriceTick`，`TThostFtdcPriceType`。
    pub price_tick: f64,
    /// 市价单最大下单量。CTP：`MaxMarketOrderVolume`，`TThostFtdcVolumeType`。
    pub max_market_order_volume: i32,
    /// 市价单最小下单量。CTP：`MinMarketOrderVolume`，`TThostFtdcVolumeType`。
    pub min_market_order_volume: i32,
    /// 限价单最大下单量。CTP：`MaxLimitOrderVolume`，`TThostFtdcVolumeType`。
    pub max_limit_order_volume: i32,
    /// 限价单最小下单量。CTP：`MinLimitOrderVolume`，`TThostFtdcVolumeType`。
    pub min_limit_order_volume: i32,
    /// 持仓类型。CTP：`PositionType`，`TThostFtdcPositionTypeType`。
    pub position_type: PositionType,
    /// 持仓日期类型。CTP：`PositionDateType`，`TThostFtdcPositionDateTypeType`。
    pub position_date_type: PositionDateType,
    /// 平仓处理类型。CTP：`CloseDealType`，`TThostFtdcCloseDealTypeType`。
    pub close_deal_type: CloseDealType,
    /// 交易币种类型。CTP：`TradeCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub trade_currency_id: String,
    /// 质押资金可用范围。CTP：`MortgageFundUseRange`，`TThostFtdcMortgageFundUseRangeType`。
    pub mortgage_fund_use_range: MortgageFundUseRange,
    /// 合约基础商品乘数。CTP：`UnderlyingMultiple`，`TThostFtdcUnderlyingMultipleType`。
    pub underlying_multiple: f64,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 交易所产品代码。CTP：`ExchangeProductID`，`TThostFtdcInstrumentIDType`。
    pub exchange_product_id: InstrumentId,
    /// 开仓量限制粒度。CTP：`OpenLimitControlLevel`，`TThostFtdcOpenLimitControlLevelType`。
    pub open_limit_control_level: OpenLimitControlLevel,
    /// 报单频率控制粒度。CTP：`OrderFreqControlLevel`，`TThostFtdcOrderFreqControlLevelType`。
    pub order_frequency_control_level: OrderFreqControlLevel,
}

/// 产品报价汇率
#[derive(Debug, Clone)]
pub struct ProductExchRate {
    /// 报价币种类型。CTP：`QuoteCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub quote_currency_id: String,
    /// 汇率。CTP：`ExchangeRate`，`TThostFtdcExchangeRateType`。
    pub exchange_rate: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// 投资者品种/跨品种保证金产品组
#[derive(Debug, Clone)]
pub struct ProductGroup {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 产品组代码。CTP：`ProductGroupID`，`TThostFtdcInstrumentIDType`。
    pub product_group_id: InstrumentId,
}

/// 请求查询银期签约关系
#[derive(Debug, Clone)]
pub struct QryAccountRegister {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 银行编码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构编码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 服务地址和AppID的关系查询
#[derive(Debug, Clone)]
pub struct QryAddrAppIDRelation {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询批量报单操作
#[derive(Debug, Clone)]
pub struct QryBatchOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 查询经纪公司
#[derive(Debug, Clone)]
pub struct QryBroker {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询经纪公司交易算法
#[derive(Debug, Clone)]
pub struct QryBrokerTradingAlgos {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询经纪公司交易参数
#[derive(Debug, Clone)]
pub struct QryBrokerTradingParams {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
}

/// 查询经纪公司用户
#[derive(Debug, Clone)]
pub struct QryBrokerUser {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询经纪公司用户事件
#[derive(Debug, Clone)]
pub struct QryBrokerUserEvent {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户事件类型。CTP：`UserEventType`，`TThostFtdcUserEventTypeType`。
    pub user_event_type: UserEventType,
}

/// 查询经纪公司用户权限
#[derive(Debug, Clone)]
pub struct QryBrokerUserFunction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询交易所公告
#[derive(Debug, Clone)]
pub struct QryBulletin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 公告编号。CTP：`BulletinID`，`TThostFtdcBulletinIDType`。
    pub bulletin_id: i32,
    /// 序列号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 公告类型。CTP：`NewsType`，`TThostFtdcNewsTypeType`。
    pub news_type: String,
    /// 紧急程度。CTP：`NewsUrgency`，`TThostFtdcNewsUrgencyType`。
    pub news_urgency: NewsUrgency,
}

/// 查询保证金监管系统经纪公司密钥
#[derive(Debug, Clone)]
pub struct QryCFMMCBrokerKey {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 请求查询保证金监管系统经纪公司资金账户密钥
#[derive(Debug, Clone)]
pub struct QryCFMMCTradingAccountKey {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询分类合约
#[derive(Debug, Clone)]
pub struct QryClassifiedInstrument {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 合约交易状态。CTP：`TradingType`，`TThostFtdcTradingTypeType`。
    pub trading_type: TradingType,
    /// 合约分类类型。CTP：`ClassType`，`TThostFtdcClassTypeType`。
    pub class_type: ClassType,
}

/// 申请组合查询
#[derive(Debug, Clone)]
pub struct QryCombAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 组合合约安全系数查询
#[derive(Debug, Clone)]
pub struct QryCombInstrumentGuard {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 组合腿信息查询
#[derive(Debug, Clone)]
pub struct QryCombLeg {
    /// 单腿合约代码。CTP：`LegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub leg_instrument_id: InstrumentId,
}

/// 查询组合优惠比例
#[derive(Debug, Clone)]
pub struct QryCombPromotionParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询组合合约分腿
#[derive(Debug, Clone)]
pub struct QryCombinationLeg {
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 单腿合约代码。CTP：`LegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub leg_instrument_id: InstrumentId,
}

/// 请求查询投资者手续费率模板
#[derive(Debug, Clone)]
pub struct QryCommRateModel {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 手续费率模板代码。CTP：`CommModelID`，`TThostFtdcInvestorIDType`。
    pub commission_model_id: InvestorId,
}

/// 查询签约银行请求
#[derive(Debug, Clone)]
pub struct QryContractBank {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分中心代码。CTP：`BankBrchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
}

/// 查询操作员组织架构关系
#[derive(Debug, Clone)]
pub struct QryDepartmentUser {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询行情
#[derive(Debug, Clone)]
pub struct QryDepthMarketData {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 产品类型。CTP：`ProductClass`，`TThostFtdcProductClassType`。
    pub product_class: ProductClass,
}

/// 查询仓单折抵信息
#[derive(Debug, Clone)]
pub struct QryEWarrantOffset {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询错误执行宣告
#[derive(Debug, Clone)]
pub struct QryErrExecOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询错误执行宣告操作
#[derive(Debug, Clone)]
pub struct QryErrExecOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询错误报单
#[derive(Debug, Clone)]
pub struct QryErrOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询错误报单操作
#[derive(Debug, Clone)]
pub struct QryErrOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询交易所
#[derive(Debug, Clone)]
pub struct QryExchange {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 交易所申请组合查询
#[derive(Debug, Clone)]
pub struct QryExchangeCombAction {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 交易所执行宣告查询
#[derive(Debug, Clone)]
pub struct QryExchangeExecOrder {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 交易所执行宣告操作查询
#[derive(Debug, Clone)]
pub struct QryExchangeExecOrderAction {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 交易所询价查询
#[derive(Debug, Clone)]
pub struct QryExchangeForQuote {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 查询交易所保证金率
#[derive(Debug, Clone)]
pub struct QryExchangeMarginRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询交易所调整保证金率
#[derive(Debug, Clone)]
pub struct QryExchangeMarginRateAdjust {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询交易所报单
#[derive(Debug, Clone)]
pub struct QryExchangeOrder {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 查询交易所报单操作
#[derive(Debug, Clone)]
pub struct QryExchangeOrderAction {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 交易所报价查询
#[derive(Debug, Clone)]
pub struct QryExchangeQuote {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 交易所报价操作查询
#[derive(Debug, Clone)]
pub struct QryExchangeQuoteAction {
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 查询汇率
#[derive(Debug, Clone)]
pub struct QryExchangeRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 源币种。CTP：`FromCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub from_currency_id: String,
    /// 目标币种。CTP：`ToCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub to_currency_id: String,
}

/// 查询交易所状态
#[derive(Debug, Clone)]
pub struct QryExchangeSequence {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 执行宣告查询
#[derive(Debug, Clone)]
pub struct QryExecOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行宣告编号。CTP：`ExecOrderSysID`，`TThostFtdcExecOrderSysIDType`。
    pub exec_order_system_id: String,
    /// 开始时间。CTP：`InsertTimeStart`，`TThostFtdcTimeType`。
    pub insert_time_start: TimeText,
    /// 结束时间。CTP：`InsertTimeEnd`，`TThostFtdcTimeType`。
    pub insert_time_end: TimeText,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 执行宣告操作查询
#[derive(Debug, Clone)]
pub struct QryExecOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 询价查询
#[derive(Debug, Clone)]
pub struct QryForQuote {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 开始时间。CTP：`InsertTimeStart`，`TThostFtdcTimeType`。
    pub insert_time_start: TimeText,
    /// 结束时间。CTP：`InsertTimeEnd`，`TThostFtdcTimeType`。
    pub insert_time_end: TimeText,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询询价价差参数
#[derive(Debug, Clone)]
pub struct QryForQuoteParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询前置状态
#[derive(Debug, Clone)]
pub struct QryFrontStatus {
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
}

/// 套利套保申请查询
#[derive(Debug, Clone)]
pub struct QryHedgeCfm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询报单
#[derive(Debug, Clone)]
pub struct QryHisOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 开始时间。CTP：`InsertTimeStart`，`TThostFtdcTimeType`。
    pub insert_time_start: TimeText,
    /// 结束时间。CTP：`InsertTimeEnd`，`TThostFtdcTimeType`。
    pub insert_time_end: TimeText,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询IP列表
#[derive(Debug, Clone)]
pub struct QryIPList {
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 合约交易限制查询
#[derive(Debug, Clone)]
pub struct QryInstrTradeLimit {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 查询合约
#[derive(Debug, Clone)]
pub struct QryInstrument {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// 查询手续费率
#[derive(Debug, Clone)]
pub struct QryInstrumentCommissionRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询合约保证金率
#[derive(Debug, Clone)]
pub struct QryInstrumentMarginRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 报单手续费率查询
#[derive(Debug, Clone)]
pub struct QryInstrumentOrderCommRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询合约状态
#[derive(Debug, Clone)]
pub struct QryInstrumentStatus {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 查询合约交易权限
#[derive(Debug, Clone)]
pub struct QryInstrumentTradingRight {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询投资单元
#[derive(Debug, Clone)]
pub struct QryInvestUnit {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 查询投资者
#[derive(Debug, Clone)]
pub struct QryInvestor {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 投资者商品群SPMM记录查询
#[derive(Debug, Clone)]
pub struct QryInvestorCommodityGroupSPMMMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
}

/// 投资者商品组SPMM记录查询
#[derive(Debug, Clone)]
pub struct QryInvestorCommoditySPMMMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品组代码。CTP：`CommodityID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_id: String,
}

/// 查询组织架构投资者对应关系
#[derive(Debug, Clone)]
pub struct QryInvestorDepartmentFlat {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询投资者组
#[derive(Debug, Clone)]
pub struct QryInvestorGroup {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 投资者申报费阶梯收取记录查询
#[derive(Debug, Clone)]
pub struct QryInvestorInfoCommRec {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 商品代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 投资者新型组合保证金系数查询
#[derive(Debug, Clone)]
pub struct QryInvestorPortfMarginRatio {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
}

/// 投资者新组保设置查询
#[derive(Debug, Clone)]
pub struct QryInvestorPortfSetting {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者编号。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询投资者持仓
#[derive(Debug, Clone)]
pub struct QryInvestorPosition {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询组合持仓明细
#[derive(Debug, Clone)]
pub struct QryInvestorPositionCombineDetail {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 组合持仓合约编码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
}

/// 查询投资者持仓明细
#[derive(Debug, Clone)]
pub struct QryInvestorPositionDetail {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 投资者品种RCAMS保证金查询
#[derive(Debug, Clone)]
pub struct QryInvestorProdRCAMSMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
}

/// 投资者产品RULE保证金查询
#[derive(Debug, Clone)]
pub struct QryInvestorProdRULEMargin {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
}

/// 投资者产品SPBM明细查询
#[derive(Debug, Clone)]
pub struct QryInvestorProdSPBMDetail {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
}

/// 查询投资者品种/跨品种保证金
#[derive(Debug, Clone)]
pub struct QryInvestorProductGroupMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 品种/跨品种标示。CTP：`ProductGroupID`，`TThostFtdcInstrumentIDType`。
    pub product_group_id: InstrumentId,
}

/// 服务地址参数查询
#[derive(Debug, Clone)]
pub struct QryIpAddrParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询联系人
#[derive(Debug, Clone)]
pub struct QryLinkMan {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 内网地址配置查询
#[derive(Debug, Clone)]
pub struct QryLocalAddrConfig {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询禁止登录用户
#[derive(Debug, Clone)]
pub struct QryLoginForbiddenUser {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询行情报盘机
#[derive(Debug, Clone)]
pub struct QryMDTraderOffer {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 查询做市商合约手续费率
#[derive(Debug, Clone)]
pub struct QryMMInstrumentCommissionRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 做市商期权手续费率查询
#[derive(Debug, Clone)]
pub struct QryMMOptionInstrCommRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 请求查询投资者保证金率模板
#[derive(Debug, Clone)]
pub struct QryMarginModel {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 保证金率模板代码。CTP：`MarginModelID`，`TThostFtdcInvestorIDType`。
    pub margin_model_id: InvestorId,
}

/// 查询最大报单数量
#[derive(Debug, Clone)]
pub struct QryMaxOrderVolume {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 最大允许报单数量。CTP：`MaxVolume`，`TThostFtdcVolumeType`。
    pub max_volume: i32,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 根据价格查询最大报单数量
#[derive(Debug, Clone)]
pub struct QryMaxOrderVolumeWithPrice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 最大允许报单数量。CTP：`MaxVolume`，`TThostFtdcVolumeType`。
    pub max_volume: i32,
    /// 报单价格。CTP：`Price`，`TThostFtdcPriceType`。
    pub price: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询组播合约
#[derive(Debug, Clone)]
pub struct QryMulticastInstrument {
    /// 主题号。CTP：`TopicID`，`TThostFtdcInstallIDType`。
    pub topic_id: i32,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询客户通知
#[derive(Debug, Clone)]
pub struct QryNotice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
}

/// 查询对冲设置
#[derive(Debug, Clone)]
pub struct QryOffsetSetting {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 对冲类型。CTP：`OffsetType`，`TThostFtdcOffsetTypeType`。
    pub offset_type: OffsetType,
}

/// 期权手续费率查询
#[derive(Debug, Clone)]
pub struct QryOptionInstrCommRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 期权交易成本查询
#[derive(Debug, Clone)]
pub struct QryOptionInstrTradeCost {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 期权合约报价。CTP：`InputPrice`，`TThostFtdcPriceType`。
    pub input_price: f64,
    /// 标的价格,填0则用昨结算价。CTP：`UnderlyingPrice`，`TThostFtdcPriceType`。
    pub underlying_price: f64,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询期权合约交易权限
#[derive(Debug, Clone)]
pub struct QryOptionInstrTradingRight {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 期权自对冲查询
#[derive(Debug, Clone)]
pub struct QryOptionSelfClose {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 期权自对冲编号。CTP：`OptionSelfCloseSysID`，`TThostFtdcOrderSysIDType`。
    pub option_self_close_system_id: OrderSysId,
    /// 开始时间。CTP：`InsertTimeStart`，`TThostFtdcTimeType`。
    pub insert_time_start: TimeText,
    /// 结束时间。CTP：`InsertTimeEnd`，`TThostFtdcTimeType`。
    pub insert_time_end: TimeText,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 期权自对冲操作查询
#[derive(Debug, Clone)]
pub struct QryOptionSelfCloseAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 查询报单
#[derive(Debug, Clone)]
pub struct QryOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 开始时间。CTP：`InsertTimeStart`，`TThostFtdcTimeType`。
    pub insert_time_start: TimeText,
    /// 结束时间。CTP：`InsertTimeEnd`，`TThostFtdcTimeType`。
    pub insert_time_end: TimeText,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询报单操作
#[derive(Debug, Clone)]
pub struct QryOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 查询预埋单
#[derive(Debug, Clone)]
pub struct QryParkedOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询预埋撤单
#[derive(Debug, Clone)]
pub struct QryParkedOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询经纪公司会员代码
#[derive(Debug, Clone)]
pub struct QryPartBroker {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
}

/// 查询产品
#[derive(Debug, Clone)]
pub struct QryProduct {
    /// 产品类型。CTP：`ProductClass`，`TThostFtdcProductClassType`。
    pub product_class: ProductClass,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// 产品报价汇率查询
#[derive(Debug, Clone)]
pub struct QryProductExchRate {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// 查询产品组
#[derive(Debug, Clone)]
pub struct QryProductGroup {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// 报价查询
#[derive(Debug, Clone)]
pub struct QryQuote {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报价编号。CTP：`QuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub quote_system_id: OrderSysId,
    /// 开始时间。CTP：`InsertTimeStart`，`TThostFtdcTimeType`。
    pub insert_time_start: TimeText,
    /// 结束时间。CTP：`InsertTimeEnd`，`TThostFtdcTimeType`。
    pub insert_time_end: TimeText,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 报价操作查询
#[derive(Debug, Clone)]
pub struct QryQuoteAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// RCAMS产品组合信息查询
#[derive(Debug, Clone)]
pub struct QryRCAMSCombProductInfo {
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 商品组代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
}

/// RCAMS同合约风险对冲参数查询
#[derive(Debug, Clone)]
pub struct QryRCAMSInstrParameter {
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
}

/// RCAMS跨品种风险折抵参数查询
#[derive(Debug, Clone)]
pub struct QryRCAMSInterParameter {
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
    /// 产品组合代码1。CTP：`CombProduct1`，`TThostFtdcProductIDType`。
    pub comb_product1: ProductId,
    /// 产品组合代码2。CTP：`CombProduct2`，`TThostFtdcProductIDType`。
    pub comb_product2: ProductId,
}

/// RCAMS品种内风险对冲参数查询
#[derive(Debug, Clone)]
pub struct QryRCAMSIntraParameter {
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
}

/// RCAMS策略组合持仓查询
#[derive(Debug, Clone)]
pub struct QryRCAMSInvestorCombPosition {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
}

/// RCAMS空头期权风险调整参数查询
#[derive(Debug, Clone)]
pub struct QryRCAMSShortOptAdjustParam {
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
}

/// RULE合约保证金参数查询
#[derive(Debug, Clone)]
pub struct QryRULEInstrParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// RULE跨品种抵扣参数查询
#[derive(Debug, Clone)]
pub struct QryRULEInterParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
}

/// RULE品种内对锁仓折扣参数查询
#[derive(Debug, Clone)]
pub struct QryRULEIntraParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
}

/// 投资者风险结算持仓查询
#[derive(Debug, Clone)]
pub struct QryRiskSettleInvestPosition {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 风险结算产品查询
#[derive(Debug, Clone)]
pub struct QryRiskSettleProductStatus {
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
}

/// SPBM附加跨品种抵扣参数查询
#[derive(Debug, Clone)]
pub struct QrySPBMAddOnInterParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
}

/// SPBM期货合约保证金参数查询
#[derive(Debug, Clone)]
pub struct QrySPBMFutureParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
}

/// SPBM跨品种抵扣参数查询
#[derive(Debug, Clone)]
pub struct QrySPBMInterParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
}

/// SPBM品种内对锁仓折扣参数查询
#[derive(Debug, Clone)]
pub struct QrySPBMIntraParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
}

/// 投资者套餐选择查询
#[derive(Debug, Clone)]
pub struct QrySPBMInvestorPortfDef {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// SPBM期权合约保证金参数查询
#[derive(Debug, Clone)]
pub struct QrySPBMOptionParameter {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
}

/// 组合保证金套餐查询
#[derive(Debug, Clone)]
pub struct QrySPBMPortfDefinition {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 组合保证金套餐代码。CTP：`PortfolioDefID`，`TThostFtdcPortfolioDefIDType`。
    pub portfolio_definition_id: i32,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
}

/// SPMM合约参数查询
#[derive(Debug, Clone)]
pub struct QrySPMMInstParam {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// SPMM产品参数查询
#[derive(Debug, Clone)]
pub struct QrySPMMProductParam {
    /// 产品代码。CTP：`ProductID`，`TThostFtdcSPMMProductIDType`。
    pub product_id: String,
}

/// 二级代理操作员银期权限查询
#[derive(Debug, Clone)]
pub struct QrySecAgentACIDMap {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 资金账户。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 查询二级代理商资金校验模式
#[derive(Debug, Clone)]
pub struct QrySecAgentCheckMode {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
}

/// 查询二级代理商信息
#[derive(Debug, Clone)]
pub struct QrySecAgentTradeInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 境外中介机构资金帐号。CTP：`BrokerSecAgentID`，`TThostFtdcAccountIDType`。
    pub broker_sec_agent_id: AccountId,
}

/// 查询投资者结算结果
#[derive(Debug, Clone)]
pub struct QrySettlementInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 查询结算信息确认域
#[derive(Debug, Clone)]
pub struct QrySettlementInfoConfirm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 套利套保申请查询
#[derive(Debug, Clone)]
pub struct QrySpdApply {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 第一腿合约编码。CTP：`FirstLegInstrumentID`，`TThostFtdcExchangeInstIDType`。
    pub first_leg_instrument_id: ExchangeInstrumentId,
    /// 第二腿合约编码。CTP：`SecondLegInstrumentID`，`TThostFtdcExchangeInstIDType`。
    pub second_leg_instrument_id: ExchangeInstrumentId,
}

/// 期权执行偏移值查询
#[derive(Debug, Clone)]
pub struct QryStrikeOffset {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询管理用户
#[derive(Debug, Clone)]
pub struct QrySuperUser {
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询管理用户功能权限
#[derive(Debug, Clone)]
pub struct QrySuperUserFunction {
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询延时换汇同步
#[derive(Debug, Clone)]
pub struct QrySyncDelaySwap {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 延时换汇流水号。CTP：`DelaySwapSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub delay_swap_seq_no: String,
}

/// 查询出入金流水
#[derive(Debug, Clone)]
pub struct QrySyncDeposit {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 出入金流水号。CTP：`DepositSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub deposit_seq_no: String,
}

/// 查询货币质押流水
#[derive(Debug, Clone)]
pub struct QrySyncFundMortgage {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 货币质押流水号。CTP：`MortgageSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub mortgage_seq_no: String,
}

/// 查询组合合约分腿
#[derive(Debug, Clone)]
pub struct QrySyncStatus {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
}

/// 服务地址参数查询
#[derive(Debug, Clone)]
pub struct QryTGIpAddrParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
}

/// Thost终端用户功能权限查询
#[derive(Debug, Clone)]
pub struct QryThostUserFunction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询成交
#[derive(Debug, Clone)]
pub struct QryTrade {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 成交编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 开始时间。CTP：`TradeTimeStart`，`TThostFtdcTimeType`。
    pub trade_time_start: TimeText,
    /// 结束时间。CTP：`TradeTimeEnd`，`TThostFtdcTimeType`。
    pub trade_time_end: TimeText,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 查询交易员
#[derive(Debug, Clone)]
pub struct QryTrader {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 席位与交易中心对应关系维护查询
#[derive(Debug, Clone)]
pub struct QryTraderAssign {
    /// 交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 查询交易员报盘机
#[derive(Debug, Clone)]
pub struct QryTraderOffer {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
}

/// 查询资金账户
#[derive(Debug, Clone)]
pub struct QryTradingAccount {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 业务类型。CTP：`BizType`，`TThostFtdcBizTypeType`。
    pub biz_type: BizType,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
}

/// 查询交易编码
#[derive(Debug, Clone)]
pub struct QryTradingCode {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易编码类型。CTP：`ClientIDType`，`TThostFtdcClientIDTypeType`。
    pub client_id_type: ClientIDType,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 查询交易事件通知
#[derive(Debug, Clone)]
pub struct QryTradingNotice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 查询转帐银行
#[derive(Debug, Clone)]
pub struct QryTransferBank {
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分中心代码。CTP：`BankBrchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
}

/// 请求查询转帐流水
#[derive(Debug, Clone)]
pub struct QryTransferSerial {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 银行编码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 查询用户下单权限分配表
#[derive(Debug, Clone)]
pub struct QryUserRightsAssign {
    /// 应用单元代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询用户会话
#[derive(Debug, Clone)]
pub struct QryUserSession {
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 查询经纪公司资金
#[derive(Debug, Clone)]
pub struct QueryBrokerDeposit {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
}

/// 查询监控中心用户令牌
#[derive(Debug, Clone)]
pub struct QueryCFMMCTradingAccountToken {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 查询频率，每秒查询比数
#[derive(Debug, Clone)]
pub struct QueryFreq {
    /// 查询频率。CTP：`QueryFreq`，`TThostFtdcQueryFreqType`。
    pub query_frequency: i32,
    /// FTD频率。CTP：`FTDPkgFreq`，`TThostFtdcQueryFreqType`。
    pub ftd_pkg_frequency: i32,
}

/// 报价
#[derive(Debug, Clone)]
pub struct Quote {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报价引用。CTP：`QuoteRef`，`TThostFtdcOrderRefType`。
    pub quote_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 卖价格。CTP：`AskPrice`，`TThostFtdcPriceType`。
    pub ask_price: f64,
    /// 买价格。CTP：`BidPrice`，`TThostFtdcPriceType`。
    pub bid_price: f64,
    /// 卖数量。CTP：`AskVolume`，`TThostFtdcVolumeType`。
    pub ask_volume: i32,
    /// 买数量。CTP：`BidVolume`，`TThostFtdcVolumeType`。
    pub bid_volume: i32,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 卖开平标志。CTP：`AskOffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub ask_offset_flag: OffsetFlag,
    /// 买开平标志。CTP：`BidOffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub bid_offset_flag: OffsetFlag,
    /// 卖投机套保标志。CTP：`AskHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub ask_hedge_flag: HedgeFlag,
    /// 买投机套保标志。CTP：`BidHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub bid_hedge_flag: HedgeFlag,
    /// 本地报价编号。CTP：`QuoteLocalID`，`TThostFtdcOrderLocalIDType`。
    pub quote_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报价提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 报价提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 报价编号。CTP：`QuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub quote_system_id: OrderSysId,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 插入时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 报价状态。CTP：`QuoteStatus`，`TThostFtdcOrderStatusType`。
    pub quote_status: OrderStatus,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 卖方报单编号。CTP：`AskOrderSysID`，`TThostFtdcOrderSysIDType`。
    pub ask_order_system_id: OrderSysId,
    /// 买方报单编号。CTP：`BidOrderSysID`，`TThostFtdcOrderSysIDType`。
    pub bid_order_system_id: OrderSysId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报价编号。CTP：`BrokerQuoteSeq`，`TThostFtdcSequenceNoType`。
    pub broker_quote_seq: i32,
    /// 衍生卖报单引用。CTP：`AskOrderRef`，`TThostFtdcOrderRefType`。
    pub ask_order_ref: OrderRef,
    /// 衍生买报单引用。CTP：`BidOrderRef`，`TThostFtdcOrderRefType`。
    pub bid_order_ref: OrderRef,
    /// 应价编号。CTP：`ForQuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub for_quote_system_id: OrderSysId,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 资金账号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 被顶单编号。CTP：`ReplaceSysID`，`TThostFtdcOrderSysIDType`。
    pub replace_system_id: OrderSysId,
    /// 有效期类型。CTP：`TimeCondition`，`TThostFtdcTimeConditionType`。
    pub time_condition: TimeCondition,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// 报价操作
#[derive(Debug, Clone)]
pub struct QuoteAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报价操作引用。CTP：`QuoteActionRef`，`TThostFtdcOrderActionRefType`。
    pub quote_action_ref: i32,
    /// 报价引用。CTP：`QuoteRef`，`TThostFtdcOrderRefType`。
    pub quote_ref: OrderRef,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 报价操作编号。CTP：`QuoteSysID`，`TThostFtdcOrderSysIDType`。
    pub quote_system_id: OrderSysId,
    /// 操作标志。CTP：`ActionFlag`，`TThostFtdcActionFlagType`。
    pub action_flag: ActionFlag,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报价编号。CTP：`QuoteLocalID`，`TThostFtdcOrderLocalIDType`。
    pub quote_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// 报单回显字段。CTP：`OrderMemo`，`TThostFtdcOrderMemoType`。
    pub order_memo: String,
    /// session上请求计数 api自动维护。CTP：`SessionReqSeq`，`TThostFtdcSequenceNo12Type`。
    pub session_req_seq: i32,
}

/// RCAMS产品组合信息
#[derive(Debug, Clone)]
pub struct RCAMSCombProductInfo {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 商品组代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
}

/// RCAMS同合约风险对冲参数
#[derive(Debug, Clone)]
pub struct RCAMSInstrParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 同合约风险对冲比率。CTP：`HedgeRate`，`TThostFtdcHedgeRateType`。
    pub hedge_rate: f64,
}

/// RCAMS跨品种风险折抵参数
#[derive(Debug, Clone)]
pub struct RCAMSInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
    /// 优先级。CTP：`Priority`，`TThostFtdcRCAMSPriorityType`。
    pub priority: i32,
    /// 折抵率。CTP：`CreditRate`，`TThostFtdcHedgeRateType`。
    pub credit_rate: f64,
    /// 产品组合代码1。CTP：`CombProduct1`，`TThostFtdcProductIDType`。
    pub comb_product1: ProductId,
    /// 产品组合代码2。CTP：`CombProduct2`，`TThostFtdcProductIDType`。
    pub comb_product2: ProductId,
}

/// RCAMS品种内风险对冲参数
#[derive(Debug, Clone)]
pub struct RCAMSIntraParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 品种内对冲比率。CTP：`HedgeRate`，`TThostFtdcHedgeRateType`。
    pub hedge_rate: f64,
}

/// RCAMS策略组合持仓
#[derive(Debug, Clone)]
pub struct RCAMSInvestorCombPosition {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投套标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 持仓多空方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 交易所组合合约代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 持仓量。CTP：`TotalAmt`，`TThostFtdcVolumeType`。
    pub total_amount: i32,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
}

/// RCAMS空头期权风险调整参数
#[derive(Debug, Clone)]
pub struct RCAMSShortOptAdjustParam {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 投套标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 空头期权风险调整标准。CTP：`AdjustValue`，`TThostFtdcAdjustValueType`。
    pub adjust_value: f64,
}

/// RULE合约保证金参数
#[derive(Debug, Clone)]
pub struct RULEInstrParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约类型。CTP：`InstrumentClass`，`TThostFtdcInstrumentClassType`。
    pub instrument_class: InstrumentClass,
    /// 标准合约。CTP：`StdInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub std_instrument_id: InstrumentId,
    /// 投机买折算系数。CTP：`BSpecRatio`，`TThostFtdcRatioType`。
    pub b_spec_ratio: f64,
    /// 投机卖折算系数。CTP：`SSpecRatio`，`TThostFtdcRatioType`。
    pub s_spec_ratio: f64,
    /// 套保买折算系数。CTP：`BHedgeRatio`，`TThostFtdcRatioType`。
    pub b_hedge_ratio: f64,
    /// 套保卖折算系数。CTP：`SHedgeRatio`，`TThostFtdcRatioType`。
    pub s_hedge_ratio: f64,
    /// 买附加风险保证金。CTP：`BAddOnMargin`，`TThostFtdcMoneyType`。
    pub b_add_on_margin: f64,
    /// 卖附加风险保证金。CTP：`SAddOnMargin`，`TThostFtdcMoneyType`。
    pub s_add_on_margin: f64,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
}

/// RULE跨品种抵扣参数
#[derive(Debug, Clone)]
pub struct RULEInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 优先级。CTP：`SpreadId`，`TThostFtdcSpreadIdType`。
    pub spread_id: i32,
    /// 品种间对锁仓费率折扣比例。CTP：`InterRate`，`TThostFtdcRatioType`。
    pub inter_rate: f64,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
    /// 腿1比例系数。CTP：`Leg1PropFactor`，`TThostFtdcCommonIntType`。
    pub leg1_prop_factor: i32,
    /// 腿2比例系数。CTP：`Leg2PropFactor`，`TThostFtdcCommonIntType`。
    pub leg2_prop_factor: i32,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
    /// 商品群名称。CTP：`CommodityGroupName`，`TThostFtdcInstrumentNameType`。
    pub commodity_group_name: String,
}

/// RULE品种内对锁仓折扣参数
#[derive(Debug, Clone)]
pub struct RULEIntraParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 标准合约。CTP：`StdInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub std_instrument_id: InstrumentId,
    /// 标准合约保证金。CTP：`StdInstrMargin`，`TThostFtdcMoneyType`。
    pub std_instrument_margin: f64,
    /// 一般月份合约组合保证金系数。CTP：`UsualIntraRate`，`TThostFtdcRatioType`。
    pub usual_intra_rate: f64,
    /// 临近交割合约组合保证金系数。CTP：`DeliveryIntraRate`，`TThostFtdcRatioType`。
    pub delivery_intra_rate: f64,
}

/// 删除预埋单
#[derive(Debug, Clone)]
pub struct RemoveParkedOrder {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 预埋报单编号。CTP：`ParkedOrderID`，`TThostFtdcParkedOrderIDType`。
    pub parked_order_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 删除预埋撤单
#[derive(Debug, Clone)]
pub struct RemoveParkedOrderAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 预埋撤单编号。CTP：`ParkedOrderActionID`，`TThostFtdcParkedOrderActionIDType`。
    pub parked_order_action_id: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// api握手请求
#[derive(Debug, Clone)]
pub struct ReqApiHandshake {
    /// api与front通信密钥版本号。CTP：`CryptoKeyVersion`，`TThostFtdcCryptoKeyVersionType`。
    pub crypto_key_version: String,
}

/// 客户端认证请求
#[derive(Debug, Clone)]
pub struct ReqAuthenticate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 认证码。CTP：`AuthCode`，`TThostFtdcAuthCodeType`。
    pub auth_code: String,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
}

/// 转帐销户请求
#[derive(Debug, Clone)]
pub struct ReqCancelAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 汇钞标志。CTP：`CashExchangeCode`，`TThostFtdcCashExchangeCodeType`。
    pub cash_exchange_code: CashExchangeCode,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 变更银行账户请求
#[derive(Debug, Clone)]
pub struct ReqChangeAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 新银行帐号。CTP：`NewBankAccount`，`TThostFtdcBankAccountType`。
    pub new_bank_account: String,
    /// 新银行密码。CTP：`NewBankPassWord`，`TThostFtdcPasswordType`。
    pub new_bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 日终文件就绪请求
#[derive(Debug, Clone)]
pub struct ReqDayEndFileReady {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 文件业务功能。CTP：`FileBusinessCode`，`TThostFtdcFileBusinessCodeType`。
    pub file_business_code: FileBusinessCode,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
}

/// 期商签退请求
#[derive(Debug, Clone)]
pub struct ReqFutureSignOut {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
}

/// 申请短信验证码请求
#[derive(Debug, Clone)]
pub struct ReqGenSMSCode {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 手机号。CTP：`Mobile`，`TThostFtdcSMSPhoneType`。
    pub mobile: String,
}

/// 用户发出获取安全安全登陆方法请求
#[derive(Debug, Clone)]
pub struct ReqGenUserCaptcha {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 用户发出获取安全安全登陆方法请求
#[derive(Debug, Clone)]
pub struct ReqGenUserText {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 转帐开户请求
#[derive(Debug, Clone)]
pub struct ReqOpenAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 汇钞标志。CTP：`CashExchangeCode`，`TThostFtdcCashExchangeCodeType`。
    pub cash_exchange_code: CashExchangeCode,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 查询账户信息请求
#[derive(Debug, Clone)]
pub struct ReqQueryAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 次席查询银行资金账户信息请求
#[derive(Debug, Clone)]
pub struct ReqQueryBankAccountBySec {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 次中心发起转账期货公司流水号。CTP：`SecFutureSerial`，`TThostFtdcFutureSerialType`。
    pub sec_future_serial: i32,
}

/// 查询指定流水号的交易结果请求
#[derive(Debug, Clone)]
pub struct ReqQueryTradeResultBySerial {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 流水号。CTP：`Reference`，`TThostFtdcSerialType`。
    pub reference: i32,
    /// 本流水号发布者的机构类型。CTP：`RefrenceIssureType`，`TThostFtdcInstitutionTypeType`。
    pub reference_issuer_type: InstitutionType,
    /// 本流水号发布者机构编码。CTP：`RefrenceIssure`，`TThostFtdcOrganCodeType`。
    pub reference_issuer: String,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 冲正请求
#[derive(Debug, Clone)]
pub struct ReqRepeal {
    /// 冲正时间间隔。CTP：`RepealTimeInterval`，`TThostFtdcRepealTimeIntervalType`。
    pub repeal_time_interval: i32,
    /// 已经冲正次数。CTP：`RepealedTimes`，`TThostFtdcRepealedTimesType`。
    pub repealed_times: i32,
    /// 银行冲正标志。CTP：`BankRepealFlag`，`TThostFtdcBankRepealFlagType`。
    pub bank_repeal_flag: BankRepealFlag,
    /// 期商冲正标志。CTP：`BrokerRepealFlag`，`TThostFtdcBrokerRepealFlagType`。
    pub broker_repeal_flag: BrokerRepealFlag,
    /// 被冲正平台流水号。CTP：`PlateRepealSerial`，`TThostFtdcPlateSerialType`。
    pub plate_repeal_serial: i32,
    /// 被冲正银行流水号。CTP：`BankRepealSerial`，`TThostFtdcBankSerialType`。
    pub bank_repeal_serial: String,
    /// 被冲正期货流水号。CTP：`FutureRepealSerial`，`TThostFtdcFutureSerialType`。
    pub future_repeal_serial: i32,
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 期货可取金额。CTP：`FutureFetchAmount`，`TThostFtdcTradeAmountType`。
    pub future_fetch_amount: f64,
    /// 费用支付标志。CTP：`FeePayFlag`，`TThostFtdcFeePayFlagType`。
    pub fee_pay_flag: FeePayFlag,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 发送方给接收方的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 转账交易状态。CTP：`TransferStatus`，`TThostFtdcTransferStatusType`。
    pub transfer_status: TransferStatus,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 交易核心向银期报盘发出密钥同步请求
#[derive(Debug, Clone)]
pub struct ReqSyncKey {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易核心给银期报盘的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
}

/// 转账请求
#[derive(Debug, Clone)]
pub struct ReqTransfer {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 期货可取金额。CTP：`FutureFetchAmount`，`TThostFtdcTradeAmountType`。
    pub future_fetch_amount: f64,
    /// 费用支付标志。CTP：`FeePayFlag`，`TThostFtdcFeePayFlagType`。
    pub fee_pay_flag: FeePayFlag,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 发送方给接收方的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 转账交易状态。CTP：`TransferStatus`，`TThostFtdcTransferStatusType`。
    pub transfer_status: TransferStatus,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 次中心发起的转帐交易
#[derive(Debug, Clone)]
pub struct ReqTransferBySec {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 期货可取金额。CTP：`FutureFetchAmount`，`TThostFtdcTradeAmountType`。
    pub future_fetch_amount: f64,
    /// 费用支付标志。CTP：`FeePayFlag`，`TThostFtdcFeePayFlagType`。
    pub fee_pay_flag: FeePayFlag,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 发送方给接收方的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 转账交易状态。CTP：`TransferStatus`，`TThostFtdcTransferStatusType`。
    pub transfer_status: TransferStatus,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 次中心发起转账期货公司流水号。CTP：`SecFutureSerial`，`TThostFtdcFutureSerialType`。
    pub sec_future_serial: i32,
}

/// 用户发出获取安全安全登陆方法请求
#[derive(Debug, Clone)]
pub struct ReqUserAuthMethod {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 用户登录请求
#[derive(Debug, Clone)]
pub struct ReqUserLogin {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 动态密码。CTP：`OneTimePassword`，`TThostFtdcPasswordType`。
    pub one_time_password: String,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 终端IP地址。CTP：`ClientIPAddress`，`TThostFtdcIPAddressType`。
    pub client_ip_address: String,
    /// 短信验证码。CTP：`SMSCode`，`TThostFtdcSMSCodeType`。
    pub sms_code: String,
}

/// 国密用户登录请求
#[derive(Debug, Clone)]
pub struct ReqUserLoginSM {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 动态密码。CTP：`OneTimePassword`，`TThostFtdcPasswordType`。
    pub one_time_password: String,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 终端IP地址。CTP：`ClientIPAddress`，`TThostFtdcIPAddressType`。
    pub client_ip_address: String,
    /// 经纪公司名称。CTP：`BrokerName`，`TThostFtdcBrokerNameType`。
    pub broker_name: String,
    /// 认证码。CTP：`AuthCode`，`TThostFtdcAuthCodeType`。
    pub auth_code: String,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// PIN码。CTP：`PIN`，`TThostFtdcPasswordType`。
    pub pin: String,
    /// 短信验证码。CTP：`SMSCode`，`TThostFtdcSMSCodeType`。
    pub sms_code: String,
}

/// 用户发出带图形验证码的登录请求请求
#[derive(Debug, Clone)]
pub struct ReqUserLoginWithCaptcha {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// 图形验证码的文字内容。CTP：`Captcha`，`TThostFtdcPasswordType`。
    pub captcha: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 终端IP地址。CTP：`ClientIPAddress`，`TThostFtdcIPAddressType`。
    pub client_ip_address: String,
}

/// 用户发出带动态验证码的登录请求请求
#[derive(Debug, Clone)]
pub struct ReqUserLoginWithOTP {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// OTP密码。CTP：`OTPPassword`，`TThostFtdcPasswordType`。
    pub otp_password: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 终端IP地址。CTP：`ClientIPAddress`，`TThostFtdcIPAddressType`。
    pub client_ip_address: String,
}

/// 用户发出带短信验证码的登录请求请求
#[derive(Debug, Clone)]
pub struct ReqUserLoginWithText {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// 短信验证码文字内容。CTP：`Text`，`TThostFtdcPasswordType`。
    pub text: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 终端IP地址。CTP：`ClientIPAddress`，`TThostFtdcIPAddressType`。
    pub client_ip_address: String,
}

/// api给front的验证key的请求
#[derive(Debug, Clone)]
pub struct ReqVerifyApiKey {
    /// 握手回复数据长度。CTP：`ApiHandshakeDataLen`，`TThostFtdcHandshakeDataLenType`。
    pub api_handshake_data_len: i32,
    /// 握手回复数据。CTP：`ApiHandshakeData`，`TThostFtdcHandshakeDataType`。
    pub api_handshake_data: String,
}

/// 银期预约开户
#[derive(Debug, Clone)]
pub struct ReserveOpenAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcLongIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 预约开户状态。CTP：`ReserveOpenAccStas`，`TThostFtdcReserveOpenAccStasType`。
    pub reserve_open_acc_stas: ReserveOpenAccStas,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 银期预约开户确认请求
#[derive(Debug, Clone)]
pub struct ReserveOpenAccountConfirm {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcLongIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 性别。CTP：`Gender`，`TThostFtdcGenderType`。
    pub gender: Gender,
    /// 国家代码。CTP：`CountryCode`，`TThostFtdcCountryCodeType`。
    pub country_code: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 邮编。CTP：`ZipCode`，`TThostFtdcZipCodeType`。
    pub zip_code: String,
    /// 电话号码。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 手机。CTP：`MobilePhone`，`TThostFtdcMobilePhoneType`。
    pub mobile_phone: String,
    /// 传真。CTP：`Fax`，`TThostFtdcFaxType`。
    pub fax: String,
    /// 电子邮件。CTP：`EMail`，`TThostFtdcEMailType`。
    pub email: String,
    /// 资金账户状态。CTP：`MoneyAccountStatus`，`TThostFtdcMoneyAccountStatusType`。
    pub money_account_status: MoneyAccountStatus,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 预约开户银行流水号。CTP：`BankReserveOpenSeq`，`TThostFtdcBankSerialType`。
    pub bank_reserve_open_seq: String,
    /// 预约开户日期。CTP：`BookDate`，`TThostFtdcTradeDateType`。
    pub book_date: String,
    /// 预约开户验证密码。CTP：`BookPsw`，`TThostFtdcPasswordType`。
    pub book_psw: String,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 返回结果
#[derive(Debug, Clone)]
pub struct ReturnResult {
    /// 返回代码。CTP：`ReturnCode`，`TThostFtdcReturnCodeType`。
    pub return_code: String,
    /// 返回码描述。CTP：`DescrInfoForReturnCode`，`TThostFtdcDescrInfoForReturnCodeType`。
    pub descr_info_for_return_code: String,
}

/// 风控禁止的合约交易权限
#[derive(Debug, Clone)]
pub struct RiskForbiddenRight {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者编号。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约/产品代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 投资者风险结算持仓
#[derive(Debug, Clone)]
pub struct RiskSettleInvestPosition {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 持仓多空方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 持仓日期。CTP：`PositionDate`，`TThostFtdcPositionDateType`。
    pub position_date: PositionDate,
    /// 上日持仓。CTP：`YdPosition`，`TThostFtdcVolumeType`。
    pub yd_position: i32,
    /// 今日持仓。CTP：`Position`，`TThostFtdcVolumeType`。
    pub position: i32,
    /// 多头冻结。CTP：`LongFrozen`，`TThostFtdcVolumeType`。
    pub long_frozen: i32,
    /// 空头冻结。CTP：`ShortFrozen`，`TThostFtdcVolumeType`。
    pub short_frozen: i32,
    /// 开仓冻结金额。CTP：`LongFrozenAmount`，`TThostFtdcMoneyType`。
    pub long_frozen_amount: f64,
    /// 开仓冻结金额。CTP：`ShortFrozenAmount`，`TThostFtdcMoneyType`。
    pub short_frozen_amount: f64,
    /// 开仓量。CTP：`OpenVolume`，`TThostFtdcVolumeType`。
    pub open_volume: i32,
    /// 平仓量。CTP：`CloseVolume`，`TThostFtdcVolumeType`。
    pub close_volume: i32,
    /// 开仓金额。CTP：`OpenAmount`，`TThostFtdcMoneyType`。
    pub open_amount: f64,
    /// 平仓金额。CTP：`CloseAmount`，`TThostFtdcMoneyType`。
    pub close_amount: f64,
    /// 持仓成本。CTP：`PositionCost`，`TThostFtdcMoneyType`。
    pub position_cost: f64,
    /// 上次占用的保证金。CTP：`PreMargin`，`TThostFtdcMoneyType`。
    pub pre_margin: f64,
    /// 占用的保证金。CTP：`UseMargin`，`TThostFtdcMoneyType`。
    pub use_margin: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 开仓成本。CTP：`OpenCost`，`TThostFtdcMoneyType`。
    pub open_cost: f64,
    /// 交易所保证金。CTP：`ExchangeMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 组合成交形成的持仓。CTP：`CombPosition`，`TThostFtdcVolumeType`。
    pub comb_position: i32,
    /// 组合多头冻结。CTP：`CombLongFrozen`，`TThostFtdcVolumeType`。
    pub comb_long_frozen: i32,
    /// 组合空头冻结。CTP：`CombShortFrozen`，`TThostFtdcVolumeType`。
    pub comb_short_frozen: i32,
    /// 逐日盯市平仓盈亏。CTP：`CloseProfitByDate`，`TThostFtdcMoneyType`。
    pub close_profit_by_date: f64,
    /// 逐笔对冲平仓盈亏。CTP：`CloseProfitByTrade`，`TThostFtdcMoneyType`。
    pub close_profit_by_trade: f64,
    /// 今日持仓。CTP：`TodayPosition`，`TThostFtdcVolumeType`。
    pub today_position: i32,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 执行冻结。CTP：`StrikeFrozen`，`TThostFtdcVolumeType`。
    pub strike_frozen: i32,
    /// 执行冻结金额。CTP：`StrikeFrozenAmount`，`TThostFtdcMoneyType`。
    pub strike_frozen_amount: f64,
    /// 放弃执行冻结。CTP：`AbandonFrozen`，`TThostFtdcVolumeType`。
    pub abandon_frozen: i32,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行冻结的昨仓。CTP：`YdStrikeFrozen`，`TThostFtdcVolumeType`。
    pub yd_strike_frozen: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 持仓成本差值。CTP：`PositionCostOffset`，`TThostFtdcMoneyType`。
    pub position_cost_offset: f64,
    /// tas持仓手数。CTP：`TasPosition`，`TThostFtdcVolumeType`。
    pub tas_position: i32,
    /// tas持仓成本。CTP：`TasPositionCost`，`TThostFtdcMoneyType`。
    pub tas_position_cost: f64,
}

/// 风险品种
#[derive(Debug, Clone)]
pub struct RiskSettleProductStatus {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品编号。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 产品结算状态。CTP：`ProductStatus`，`TThostFtdcProductStatusType`。
    pub product_status: ProductStatus,
}

/// front发给api的握手回复
#[derive(Debug, Clone)]
pub struct RspApiHandshake {
    /// 握手回复数据长度。CTP：`FrontHandshakeDataLen`，`TThostFtdcHandshakeDataLenType`。
    pub front_handshake_data_len: i32,
    /// 握手回复数据。CTP：`FrontHandshakeData`，`TThostFtdcHandshakeDataType`。
    pub front_handshake_data: String,
    /// API认证是否开启。CTP：`IsApiAuthEnabled`，`TThostFtdcBoolType`。
    pub is_api_auth_enabled: bool,
}

/// 客户端认证响应
#[derive(Debug, Clone)]
pub struct RspAuthenticate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// App代码。CTP：`AppID`，`TThostFtdcAppIDType`。
    pub app_id: String,
    /// App类型。CTP：`AppType`，`TThostFtdcAppTypeType`。
    pub app_type: AppType,
}

/// 期商签到响应
#[derive(Debug, Clone)]
pub struct RspFutureSignIn {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// PIN密钥。CTP：`PinKey`，`TThostFtdcPasswordKeyType`。
    pub pin_key: String,
    /// MAC密钥。CTP：`MacKey`，`TThostFtdcPasswordKeyType`。
    pub mac_key: String,
}

/// 期商签退响应
#[derive(Debug, Clone)]
pub struct RspFutureSignOut {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 申请短信验证码响应
#[derive(Debug, Clone)]
pub struct RspGenSMSCode {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 生成时间。CTP：`GenTime`，`TThostFtdcTimeType`。
    pub gen_time: TimeText,
}

/// 生成的图片验证码信息
#[derive(Debug, Clone)]
pub struct RspGenUserCaptcha {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 图片信息长度。CTP：`CaptchaInfoLen`，`TThostFtdcCaptchaInfoLenType`。
    pub captcha_info_len: i32,
    /// 图片信息。CTP：`CaptchaInfo`，`TThostFtdcCaptchaInfoType`。
    pub captcha_info: String,
}

/// 短信验证码生成的回复
#[derive(Debug, Clone)]
pub struct RspGenUserText {
    /// 短信验证码序号。CTP：`UserTextSeq`，`TThostFtdcUserTextSeqType`。
    pub user_text_seq: i32,
}

/// 响应信息
#[derive(Debug, Clone)]
pub struct RspInfo {
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 查询账户信息响应
#[derive(Debug, Clone)]
pub struct RspQueryAccount {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 银行可用金额。CTP：`BankUseAmount`，`TThostFtdcTradeAmountType`。
    pub bank_use_amount: f64,
    /// 银行可取金额。CTP：`BankFetchAmount`，`TThostFtdcTradeAmountType`。
    pub bank_fetch_amount: f64,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 次席查询银行资金账户信息回报
#[derive(Debug, Clone)]
pub struct RspQueryBankAccountBySec {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 银行可用金额。CTP：`BankUseAmount`，`TThostFtdcTradeAmountType`。
    pub bank_use_amount: f64,
    /// 银行可取金额。CTP：`BankFetchAmount`，`TThostFtdcTradeAmountType`。
    pub bank_fetch_amount: f64,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 次中心发起转账期货公司流水号。CTP：`SecFutureSerial`，`TThostFtdcFutureSerialType`。
    pub sec_future_serial: i32,
}

/// 查询指定流水号的交易结果响应
#[derive(Debug, Clone)]
pub struct RspQueryTradeResultBySerial {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 流水号。CTP：`Reference`，`TThostFtdcSerialType`。
    pub reference: i32,
    /// 本流水号发布者的机构类型。CTP：`RefrenceIssureType`，`TThostFtdcInstitutionTypeType`。
    pub reference_issuer_type: InstitutionType,
    /// 本流水号发布者机构编码。CTP：`RefrenceIssure`，`TThostFtdcOrganCodeType`。
    pub reference_issuer: String,
    /// 原始返回代码。CTP：`OriginReturnCode`，`TThostFtdcReturnCodeType`。
    pub origin_return_code: String,
    /// 原始返回码描述。CTP：`OriginDescrInfoForReturnCode`，`TThostFtdcDescrInfoForReturnCodeType`。
    pub origin_descr_info_for_return_code: String,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
}

/// 冲正响应
#[derive(Debug, Clone)]
pub struct RspRepeal {
    /// 冲正时间间隔。CTP：`RepealTimeInterval`，`TThostFtdcRepealTimeIntervalType`。
    pub repeal_time_interval: i32,
    /// 已经冲正次数。CTP：`RepealedTimes`，`TThostFtdcRepealedTimesType`。
    pub repealed_times: i32,
    /// 银行冲正标志。CTP：`BankRepealFlag`，`TThostFtdcBankRepealFlagType`。
    pub bank_repeal_flag: BankRepealFlag,
    /// 期商冲正标志。CTP：`BrokerRepealFlag`，`TThostFtdcBrokerRepealFlagType`。
    pub broker_repeal_flag: BrokerRepealFlag,
    /// 被冲正平台流水号。CTP：`PlateRepealSerial`，`TThostFtdcPlateSerialType`。
    pub plate_repeal_serial: i32,
    /// 被冲正银行流水号。CTP：`BankRepealSerial`，`TThostFtdcBankSerialType`。
    pub bank_repeal_serial: String,
    /// 被冲正期货流水号。CTP：`FutureRepealSerial`，`TThostFtdcFutureSerialType`。
    pub future_repeal_serial: i32,
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 期货可取金额。CTP：`FutureFetchAmount`，`TThostFtdcTradeAmountType`。
    pub future_fetch_amount: f64,
    /// 费用支付标志。CTP：`FeePayFlag`，`TThostFtdcFeePayFlagType`。
    pub fee_pay_flag: FeePayFlag,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 发送方给接收方的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 转账交易状态。CTP：`TransferStatus`，`TThostFtdcTransferStatusType`。
    pub transfer_status: TransferStatus,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 交易核心向银期报盘发出密钥同步响应
#[derive(Debug, Clone)]
pub struct RspSyncKey {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易核心给银期报盘的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 银行发起银行资金转期货响应
#[derive(Debug, Clone)]
pub struct RspTransfer {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 期货可取金额。CTP：`FutureFetchAmount`，`TThostFtdcTradeAmountType`。
    pub future_fetch_amount: f64,
    /// 费用支付标志。CTP：`FeePayFlag`，`TThostFtdcFeePayFlagType`。
    pub fee_pay_flag: FeePayFlag,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 发送方给接收方的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 转账交易状态。CTP：`TransferStatus`，`TThostFtdcTransferStatusType`。
    pub transfer_status: TransferStatus,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 次中心发起的转帐交易回报
#[derive(Debug, Clone)]
pub struct RspTransferBySec {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 用户标识。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 验证客户证件号码标志。CTP：`VerifyCertNoFlag`，`TThostFtdcYesNoIndicatorType`。
    pub verify_cert_no_flag: YesNoIndicator,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 转帐金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 期货可取金额。CTP：`FutureFetchAmount`，`TThostFtdcTradeAmountType`。
    pub future_fetch_amount: f64,
    /// 费用支付标志。CTP：`FeePayFlag`，`TThostFtdcFeePayFlagType`。
    pub fee_pay_flag: FeePayFlag,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 发送方给接收方的消息。CTP：`Message`，`TThostFtdcAddInfoType`。
    pub message: String,
    /// 摘要。CTP：`Digest`，`TThostFtdcDigestType`。
    pub digest: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 渠道标志。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 期货单位帐号类型。CTP：`BankSecuAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_secu_acc_type: BankAccType,
    /// 期货公司银行编码。CTP：`BrokerIDByBank`，`TThostFtdcBankCodingForFutureType`。
    pub broker_id_by_bank: String,
    /// 期货单位帐号。CTP：`BankSecuAcc`，`TThostFtdcBankAccountType`。
    pub bank_secu_acc: String,
    /// 银行密码标志。CTP：`BankPwdFlag`，`TThostFtdcPwdFlagType`。
    pub bank_password_flag: PwdFlag,
    /// 期货资金密码核对标志。CTP：`SecuPwdFlag`，`TThostFtdcPwdFlagType`。
    pub secu_password_flag: PwdFlag,
    /// 交易柜员。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 转账交易状态。CTP：`TransferStatus`，`TThostFtdcTransferStatusType`。
    pub transfer_status: TransferStatus,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 次中心发起转账期货公司流水号。CTP：`SecFutureSerial`，`TThostFtdcFutureSerialType`。
    pub sec_future_serial: i32,
}

/// 用户发出获取安全安全登陆方法回复
#[derive(Debug, Clone)]
pub struct RspUserAuthMethod {
    /// 当前可以用的认证模式。CTP：`UsableAuthMethod`，`TThostFtdcCurrentAuthMethodType`。
    pub usable_auth_method: i32,
}

/// 用户登录应答
#[derive(Debug, Clone)]
pub struct RspUserLogin {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 登录成功时间。CTP：`LoginTime`，`TThostFtdcTimeType`。
    pub login_time: TimeText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易系统名称。CTP：`SystemName`，`TThostFtdcSystemNameType`。
    pub system_name: String,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 最大报单引用。CTP：`MaxOrderRef`，`TThostFtdcOrderRefType`。
    pub max_order_ref: OrderRef,
    /// 上期所时间。CTP：`SHFETime`，`TThostFtdcTimeType`。
    pub shfe_time: TimeText,
    /// 大商所时间。CTP：`DCETime`，`TThostFtdcTimeType`。
    pub dce_time: TimeText,
    /// 郑商所时间。CTP：`CZCETime`，`TThostFtdcTimeType`。
    pub czce_time: TimeText,
    /// 中金所时间。CTP：`FFEXTime`，`TThostFtdcTimeType`。
    pub ffex_time: TimeText,
    /// 能源中心时间。CTP：`INETime`，`TThostFtdcTimeType`。
    pub ine_time: TimeText,
    /// 后台版本信息。CTP：`SysVersion`，`TThostFtdcSysVersionType`。
    pub system_version: String,
    /// 广期所时间。CTP：`GFEXTime`，`TThostFtdcTimeType`。
    pub gfex_time: TimeText,
    /// 当前登录中心号。CTP：`LoginDRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub login_dr_identity_id: i32,
    /// 用户所属中心号。CTP：`UserDRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub user_dr_identity_id: i32,
    /// 上次登陆时间。CTP：`LastLoginTime`，`TThostFtdcDateTimeType`。
    pub last_login_time: String,
    /// 预留信息。CTP：`ReserveInfo`，`TThostFtdcReserveInfoType`。
    pub reserve_info: String,
}

/// 用户登录应答2
#[derive(Debug, Clone)]
pub struct RspUserLogin2 {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 登录成功时间。CTP：`LoginTime`，`TThostFtdcTimeType`。
    pub login_time: TimeText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易系统名称。CTP：`SystemName`，`TThostFtdcSystemNameType`。
    pub system_name: String,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 最大报单引用。CTP：`MaxOrderRef`，`TThostFtdcOrderRefType`。
    pub max_order_ref: OrderRef,
    /// 上期所时间。CTP：`SHFETime`，`TThostFtdcTimeType`。
    pub shfe_time: TimeText,
    /// 大商所时间。CTP：`DCETime`，`TThostFtdcTimeType`。
    pub dce_time: TimeText,
    /// 郑商所时间。CTP：`CZCETime`，`TThostFtdcTimeType`。
    pub czce_time: TimeText,
    /// 中金所时间。CTP：`FFEXTime`，`TThostFtdcTimeType`。
    pub ffex_time: TimeText,
    /// 能源中心时间。CTP：`INETime`，`TThostFtdcTimeType`。
    pub ine_time: TimeText,
    /// 随机串。CTP：`RandomString`，`TThostFtdcRandomStringType`。
    pub random_string: String,
}

/// 登录验证设置
#[derive(Debug, Clone)]
pub struct SMSVerifyConfig {
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 手机号。CTP：`Mobile`，`TThostFtdcSMSPhoneType`。
    pub mobile: String,
    /// 是否启用短信验证。CTP：`UseSMSVerify`，`TThostFtdcBoolType`。
    pub use_sms_verify: bool,
}

/// 短信验证信息通知
#[derive(Debug, Clone)]
pub struct SMSVerifyInfo {
    /// 验证码创建时间。CTP：`CreateTime`，`TThostFtdcTimeType`。
    pub create_time: TimeText,
    /// 手机号。CTP：`Mobile`，`TThostFtdcSMSPhoneType`。
    pub mobile: String,
    /// 短信验证信息内容。CTP：`SMSContent`，`TThostFtdcSMSContentType`。
    pub sms_content: String,
}

/// 短信验证信息通知
#[derive(Debug, Clone)]
pub struct SMSVerifyInfoFromSec {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 经纪公司简称。CTP：`BrokerAbbr`，`TThostFtdcBrokerAbbrType`。
    pub broker_abbr: String,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 手机号。CTP：`Mobile`，`TThostFtdcSMSPhoneType`。
    pub mobile: String,
    /// 短信验证码。CTP：`SMSCode`，`TThostFtdcSMSCodeType`。
    pub sms_code: String,
    /// 验证码创建日期。CTP：`CreateDate`，`TThostFtdcDateType`。
    pub create_date: DateText,
    /// 验证码创建时间。CTP：`CreateTime`，`TThostFtdcTimeType`。
    pub create_time: TimeText,
    /// 验证码是否被使用过。CTP：`IsUsed`，`TThostFtdcBoolType`。
    pub is_used: bool,
    /// 次席的交易中心代码。CTP：`FromSec`，`TThostFtdcDRIdentityIDType`。
    pub from_sec: i32,
}

/// SPBM附加跨品种抵扣参数
#[derive(Debug, Clone)]
pub struct SPBMAddOnInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 优先级。CTP：`SpreadId`，`TThostFtdcSpreadIdType`。
    pub spread_id: i32,
    /// 品种间对锁仓附加费率折扣比例。CTP：`AddOnInterRateZ2`，`TThostFtdcRatioType`。
    pub add_on_inter_rate_z2: f64,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
}

/// SPBM期货合约保证金参数
#[derive(Debug, Clone)]
pub struct SPBMFutureParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 期货合约因子。CTP：`Cvf`，`TThostFtdcVolumeMultipleType`。
    pub cvf: i32,
    /// 阶段标识。CTP：`TimeRange`，`TThostFtdcTimeRangeType`。
    pub time_range: TimeRange,
    /// 品种保证金标准。CTP：`MarginRate`，`TThostFtdcRatioType`。
    pub margin_rate: f64,
    /// 期货合约内部对锁仓费率折扣比例。CTP：`LockRateX`，`TThostFtdcRatioType`。
    pub lock_rate_x: f64,
    /// 提高保证金标准。CTP：`AddOnRate`，`TThostFtdcRatioType`。
    pub add_on_rate: f64,
    /// 昨结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 期货合约内部对锁仓附加费率折扣比例。CTP：`AddOnLockRateX2`，`TThostFtdcRatioType`。
    pub add_on_lock_rate_x2: f64,
}

/// SPBM跨品种抵扣参数
#[derive(Debug, Clone)]
pub struct SPBMInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 优先级。CTP：`SpreadId`，`TThostFtdcSpreadIdType`。
    pub spread_id: i32,
    /// 品种间对锁仓费率折扣比例。CTP：`InterRateZ`，`TThostFtdcRatioType`。
    pub inter_rate_z: f64,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
}

/// SPBM品种内对锁仓折扣参数
#[derive(Debug, Clone)]
pub struct SPBMIntraParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 品种内合约间对锁仓费率折扣比例。CTP：`IntraRateY`，`TThostFtdcRatioType`。
    pub intra_rate_y: f64,
    /// 品种内合约间对锁仓附加费率折扣比例。CTP：`AddOnIntraRateY2`，`TThostFtdcRatioType`。
    pub add_on_intra_rate_y2: f64,
}

/// 投资者套餐选择
#[derive(Debug, Clone)]
pub struct SPBMInvestorPortfDef {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组合保证金套餐代码。CTP：`PortfolioDefID`，`TThostFtdcPortfolioDefIDType`。
    pub portfolio_definition_id: i32,
}

/// SPBM期权合约保证金参数
#[derive(Debug, Clone)]
pub struct SPBMOptionParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 期权合约因子。CTP：`Cvf`，`TThostFtdcVolumeMultipleType`。
    pub cvf: i32,
    /// 期权冲抵价格。CTP：`DownPrice`，`TThostFtdcPriceType`。
    pub down_price: f64,
    /// Delta值。CTP：`Delta`，`TThostFtdcDeltaType`。
    pub delta: f64,
    /// 卖方期权风险转换最低值。CTP：`SlimiDelta`，`TThostFtdcDeltaType`。
    pub slim_delta: f64,
    /// 昨结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
}

/// 组合保证金套餐
#[derive(Debug, Clone)]
pub struct SPBMPortfDefinition {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 组合保证金套餐代码。CTP：`PortfolioDefID`，`TThostFtdcPortfolioDefIDType`。
    pub portfolio_definition_id: i32,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 是否启用SPBM。CTP：`IsSPBM`，`TThostFtdcBoolType`。
    pub is_spbm: bool,
}

/// SPMM合约参数
#[derive(Debug, Clone)]
pub struct SPMMInstParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// SPMM合约保证金算法。CTP：`InstMarginCalID`，`TThostFtdcInstMarginCalIDType`。
    pub instrument_margin_cal_id: InstMarginCalID,
    /// 商品组代码。CTP：`CommodityID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_id: String,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
}

/// SPMM产品参数
#[derive(Debug, Clone)]
pub struct SPMMProductParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcSPMMProductIDType`。
    pub product_id: String,
    /// 商品组代码。CTP：`CommodityID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_id: String,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
}

/// 二级代理操作员银期权限
#[derive(Debug, Clone)]
pub struct SecAgentACIDMap {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 资金账户。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 境外中介机构资金帐号。CTP：`BrokerSecAgentID`，`TThostFtdcAccountIDType`。
    pub broker_sec_agent_id: AccountId,
}

/// 二级代理商资金校验模式
#[derive(Debug, Clone)]
pub struct SecAgentCheckMode {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 币种。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 境外中介机构资金帐号。CTP：`BrokerSecAgentID`，`TThostFtdcAccountIDType`。
    pub broker_sec_agent_id: AccountId,
    /// 是否需要校验自己的资金账户。CTP：`CheckSelfAccount`，`TThostFtdcBoolType`。
    pub check_self_account: bool,
}

/// 二级代理商信息
#[derive(Debug, Clone)]
pub struct SecAgentTradeInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 境外中介机构资金帐号。CTP：`BrokerSecAgentID`，`TThostFtdcAccountIDType`。
    pub broker_sec_agent_id: AccountId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 二级代理商姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 投资者结算结果
#[derive(Debug, Clone)]
pub struct SettlementInfo {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 消息正文。CTP：`Content`，`TThostFtdcContentType`。
    pub content: String,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 投资者结算结果确认信息
#[derive(Debug, Clone)]
pub struct SettlementInfoConfirm {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 确认日期。CTP：`ConfirmDate`，`TThostFtdcDateType`。
    pub confirm_date: DateText,
    /// 确认时间。CTP：`ConfirmTime`，`TThostFtdcTimeType`。
    pub confirm_time: TimeText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 来自次席的结算结果确认
#[derive(Debug, Clone)]
pub struct SettlementInfoConfirmFromSec {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 确认日期。CTP：`ConfirmDate`，`TThostFtdcDateType`。
    pub confirm_date: DateText,
    /// 确认时间。CTP：`ConfirmTime`，`TThostFtdcTimeType`。
    pub confirm_time: TimeText,
    /// 次席的交易中心代码。CTP：`FromSec`，`TThostFtdcDRIdentityIDType`。
    pub from_sec: i32,
}

/// 结算引用
#[derive(Debug, Clone)]
pub struct SettlementRef {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
}

/// 套利申请回报
#[derive(Debug, Clone)]
pub struct SpdApply {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`FirstLegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub first_leg_instrument_id: InstrumentId,
    /// 合约代码。CTP：`SecondLegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub second_leg_instrument_id: InstrumentId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 操作用户代码。CTP：`ActiveUserID`，`TThostFtdcUserIDType`。
    pub active_user_id: UserId,
    /// 经纪公司报单编号。CTP：`BrokerOrderSeq`，`TThostFtdcSequenceNoType`。
    pub broker_order_seq: i32,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 申请状态。CTP：`ApplyStatus`，`TThostFtdcApplyStatusType`。
    pub apply_status: ApplyStatus,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 报单日期。CTP：`InsertDate`，`TThostFtdcDateType`。
    pub insert_date: DateText,
    /// 委托时间。CTP：`InsertTime`，`TThostFtdcTimeType`。
    pub insert_time: TimeText,
    /// 撤销时间。CTP：`CancelTime`，`TThostFtdcTimeType`。
    pub cancel_time: TimeText,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 报单提交状态。CTP：`OrderSubmitStatus`，`TThostFtdcOrderSubmitStatusType`。
    pub order_submit_status: OrderSubmitStatus,
    /// 报单提示序号。CTP：`NotifySequence`，`TThostFtdcSequenceNoType`。
    pub notify_sequence: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 组合定单类型。CTP：`CmbType`，`TThostFtdcCmbTypeType`。
    pub cmb_type: CmbType,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
}

/// 套利申请撤销回报
#[derive(Debug, Clone)]
pub struct SpdApplyAction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 操作日期。CTP：`ActionDate`，`TThostFtdcDateType`。
    pub action_date: DateText,
    /// 操作时间。CTP：`ActionTime`，`TThostFtdcTimeType`。
    pub action_time: TimeText,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 操作本地编号。CTP：`ActionLocalID`，`TThostFtdcOrderLocalIDType`。
    pub action_local_id: OrderLocalId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 报单操作状态。CTP：`OrderActionStatus`，`TThostFtdcOrderActionStatusType`。
    pub order_action_status: OrderActionStatus,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合同编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 请求编号。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
    /// 状态信息。CTP：`StatusMsg`，`TThostFtdcErrorMsgType`。
    pub status_msg: String,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
}

/// 指定的合约
#[derive(Debug, Clone)]
pub struct SpecificInstrument {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 当前期权合约执行偏移值的详细内容
#[derive(Debug, Clone)]
pub struct StrikeOffset {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 执行偏移值。CTP：`Offset`，`TThostFtdcMoneyType`。
    pub offset: f64,
    /// 执行偏移类型。CTP：`OffsetType`，`TThostFtdcStrikeOffsetTypeType`。
    pub offset_type: StrikeOffsetType,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 管理用户
#[derive(Debug, Clone)]
pub struct SuperUser {
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户名称。CTP：`UserName`，`TThostFtdcUserNameType`。
    pub user_name: String,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
}

/// 管理用户功能权限
#[derive(Debug, Clone)]
pub struct SuperUserFunction {
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 功能代码。CTP：`FunctionCode`，`TThostFtdcFunctionCodeType`。
    pub function_code: FunctionCode,
}

/// 延时换汇同步
#[derive(Debug, Clone)]
pub struct SyncDelaySwap {
    /// 换汇流水号。CTP：`DelaySwapSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub delay_swap_seq_no: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 源币种。CTP：`FromCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub from_currency_id: String,
    /// 源金额。CTP：`FromAmount`，`TThostFtdcMoneyType`。
    pub from_amount: f64,
    /// 源换汇冻结金额(可用冻结)。CTP：`FromFrozenSwap`，`TThostFtdcMoneyType`。
    pub from_frozen_swap: f64,
    /// 源剩余换汇额度(可提冻结)。CTP：`FromRemainSwap`，`TThostFtdcMoneyType`。
    pub from_remain_swap: f64,
    /// 目标币种。CTP：`ToCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub to_currency_id: String,
    /// 目标金额。CTP：`ToAmount`，`TThostFtdcMoneyType`。
    pub to_amount: f64,
    /// 是否手工换汇。CTP：`IsManualSwap`，`TThostFtdcBoolType`。
    pub is_manual_swap: bool,
    /// 是否将所有外币的剩余换汇额度设置为0。CTP：`IsAllRemainSetZero`，`TThostFtdcBoolType`。
    pub is_all_remain_set_zero: bool,
}

/// 换汇可提冻结
#[derive(Debug, Clone)]
pub struct SyncDelaySwapFrozen {
    /// 换汇流水号。CTP：`DelaySwapSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub delay_swap_seq_no: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 源币种。CTP：`FromCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub from_currency_id: String,
    /// 源剩余换汇额度(可提冻结)。CTP：`FromRemainSwap`，`TThostFtdcMoneyType`。
    pub from_remain_swap: f64,
    /// 是否手工换汇。CTP：`IsManualSwap`，`TThostFtdcBoolType`。
    pub is_manual_swap: bool,
}

/// 风险结算追平组合优先级
#[derive(Debug, Clone)]
pub struct SyncDeltaDceCombInstrument {
    /// 合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 成交组号。CTP：`TradeGroupID`，`TThostFtdcTradeGroupIDType`。
    pub trade_group_id: i32,
    /// 投机套保标志。CTP：`CombHedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub comb_hedge_flag: HedgeFlag,
    /// 组合类型。CTP：`CombinationType`，`TThostFtdcDceCombinationTypeType`。
    pub combination_type: DceCombinationType,
    /// 买卖。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 期权组合保证金比例。CTP：`Xparameter`，`TThostFtdcDiscountRatioType`。
    pub xparameter: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平行情
#[derive(Debug, Clone)]
pub struct SyncDeltaDepthMarketData {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 最新价。CTP：`LastPrice`，`TThostFtdcPriceType`。
    pub last_price: f64,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 昨收盘。CTP：`PreClosePrice`，`TThostFtdcPriceType`。
    pub pre_close_price: f64,
    /// 昨持仓量。CTP：`PreOpenInterest`，`TThostFtdcLargeVolumeType`。
    pub pre_open_interest: f64,
    /// 今开盘。CTP：`OpenPrice`，`TThostFtdcPriceType`。
    pub open_price: f64,
    /// 最高价。CTP：`HighestPrice`，`TThostFtdcPriceType`。
    pub highest_price: f64,
    /// 最低价。CTP：`LowestPrice`，`TThostFtdcPriceType`。
    pub lowest_price: f64,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 成交金额。CTP：`Turnover`，`TThostFtdcMoneyType`。
    pub turnover: f64,
    /// 持仓量。CTP：`OpenInterest`，`TThostFtdcLargeVolumeType`。
    pub open_interest: f64,
    /// 今收盘。CTP：`ClosePrice`，`TThostFtdcPriceType`。
    pub close_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 涨停板价。CTP：`UpperLimitPrice`，`TThostFtdcPriceType`。
    pub upper_limit_price: f64,
    /// 跌停板价。CTP：`LowerLimitPrice`，`TThostFtdcPriceType`。
    pub lower_limit_price: f64,
    /// 昨虚实度。CTP：`PreDelta`，`TThostFtdcRatioType`。
    pub pre_delta: f64,
    /// 今虚实度。CTP：`CurrDelta`，`TThostFtdcRatioType`。
    pub current_delta: f64,
    /// 最后修改时间。CTP：`UpdateTime`，`TThostFtdcTimeType`。
    pub update_time: TimeText,
    /// 最后修改毫秒。CTP：`UpdateMillisec`，`TThostFtdcMillisecType`。
    pub update_millis: i32,
    /// 申买价一。CTP：`BidPrice1`，`TThostFtdcPriceType`。
    pub bid_price1: f64,
    /// 申买量一。CTP：`BidVolume1`，`TThostFtdcVolumeType`。
    pub bid_volume1: i32,
    /// 申卖价一。CTP：`AskPrice1`，`TThostFtdcPriceType`。
    pub ask_price1: f64,
    /// 申卖量一。CTP：`AskVolume1`，`TThostFtdcVolumeType`。
    pub ask_volume1: i32,
    /// 申买价二。CTP：`BidPrice2`，`TThostFtdcPriceType`。
    pub bid_price2: f64,
    /// 申买量二。CTP：`BidVolume2`，`TThostFtdcVolumeType`。
    pub bid_volume2: i32,
    /// 申卖价二。CTP：`AskPrice2`，`TThostFtdcPriceType`。
    pub ask_price2: f64,
    /// 申卖量二。CTP：`AskVolume2`，`TThostFtdcVolumeType`。
    pub ask_volume2: i32,
    /// 申买价三。CTP：`BidPrice3`，`TThostFtdcPriceType`。
    pub bid_price3: f64,
    /// 申买量三。CTP：`BidVolume3`，`TThostFtdcVolumeType`。
    pub bid_volume3: i32,
    /// 申卖价三。CTP：`AskPrice3`，`TThostFtdcPriceType`。
    pub ask_price3: f64,
    /// 申卖量三。CTP：`AskVolume3`，`TThostFtdcVolumeType`。
    pub ask_volume3: i32,
    /// 申买价四。CTP：`BidPrice4`，`TThostFtdcPriceType`。
    pub bid_price4: f64,
    /// 申买量四。CTP：`BidVolume4`，`TThostFtdcVolumeType`。
    pub bid_volume4: i32,
    /// 申卖价四。CTP：`AskPrice4`，`TThostFtdcPriceType`。
    pub ask_price4: f64,
    /// 申卖量四。CTP：`AskVolume4`，`TThostFtdcVolumeType`。
    pub ask_volume4: i32,
    /// 申买价五。CTP：`BidPrice5`，`TThostFtdcPriceType`。
    pub bid_price5: f64,
    /// 申买量五。CTP：`BidVolume5`，`TThostFtdcVolumeType`。
    pub bid_volume5: i32,
    /// 申卖价五。CTP：`AskPrice5`，`TThostFtdcPriceType`。
    pub ask_price5: f64,
    /// 申卖量五。CTP：`AskVolume5`，`TThostFtdcVolumeType`。
    pub ask_volume5: i32,
    /// 当日均价。CTP：`AveragePrice`，`TThostFtdcPriceType`。
    pub average_price: f64,
    /// 业务日期。CTP：`ActionDay`，`TThostFtdcDateType`。
    pub action_day: DateText,
    /// 上带价。CTP：`BandingUpperPrice`，`TThostFtdcPriceType`。
    pub banding_upper_price: f64,
    /// 下带价。CTP：`BandingLowerPrice`，`TThostFtdcPriceType`。
    pub banding_lower_price: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平仓单折抵
#[derive(Debug, Clone)]
pub struct SyncDeltaEWarrantOffset {
    /// 交易日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平交易所期货保证金率
#[derive(Debug, Clone)]
pub struct SyncDeltaExchMarginRate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平现货指数
#[derive(Debug, Clone)]
pub struct SyncDeltaIndexPrice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 指数现货收盘价。CTP：`ClosePrice`，`TThostFtdcPriceType`。
    pub close_price: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平信息
#[derive(Debug, Clone)]
pub struct SyncDeltaInfo {
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
    /// 追平状态。CTP：`SyncDeltaStatus`，`TThostFtdcSyncDeltaStatusType`。
    pub sync_delta_status: SyncDeltaStatus,
    /// 追平描述。CTP：`SyncDescription`，`TThostFtdcSyncDescriptionType`。
    pub sync_description: String,
    /// 是否只有资金追平。CTP：`IsOnlyTrdDelta`，`TThostFtdcBoolType`。
    pub is_only_trd_delta: bool,
}

/// 投资者风险结算总保证金
#[derive(Debug, Clone)]
pub struct SyncDeltaInitInvestMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 追平前总风险保证金。CTP：`LastRiskTotalInvstMargin`，`TThostFtdcMoneyType`。
    pub last_risk_total_invest_margin: f64,
    /// 追平前交易所总风险保证金。CTP：`LastRiskTotalExchMargin`，`TThostFtdcMoneyType`。
    pub last_risk_total_exchange_margin: f64,
    /// 本次追平品种总保证金。CTP：`ThisSyncInvstMargin`，`TThostFtdcMoneyType`。
    pub this_sync_invest_margin: f64,
    /// 本次追平品种交易所总保证金。CTP：`ThisSyncExchMargin`，`TThostFtdcMoneyType`。
    pub this_sync_exchange_margin: f64,
    /// 本次未追平品种总保证金。CTP：`RemainRiskInvstMargin`，`TThostFtdcMoneyType`。
    pub remain_risk_invest_margin: f64,
    /// 本次未追平品种交易所总保证金。CTP：`RemainRiskExchMargin`，`TThostFtdcMoneyType`。
    pub remain_risk_exchange_margin: f64,
    /// 追平前总特殊产品风险保证金。CTP：`LastRiskSpecTotalInvstMargin`，`TThostFtdcMoneyType`。
    pub last_risk_spec_total_invest_margin: f64,
    /// 追平前总特殊产品交易所风险保证金。CTP：`LastRiskSpecTotalExchMargin`，`TThostFtdcMoneyType`。
    pub last_risk_spec_total_exchange_margin: f64,
    /// 本次追平品种特殊产品总保证金。CTP：`ThisSyncSpecInvstMargin`，`TThostFtdcMoneyType`。
    pub this_sync_spec_invest_margin: f64,
    /// 本次追平品种特殊产品交易所总保证金。CTP：`ThisSyncSpecExchMargin`，`TThostFtdcMoneyType`。
    pub this_sync_spec_exchange_margin: f64,
    /// 本次未追平品种特殊产品总保证金。CTP：`RemainRiskSpecInvstMargin`，`TThostFtdcMoneyType`。
    pub remain_risk_spec_invest_margin: f64,
    /// 本次未追平品种特殊产品交易所总保证金。CTP：`RemainRiskSpecExchMargin`，`TThostFtdcMoneyType`。
    pub remain_risk_spec_exchange_margin: f64,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平期货手续费率
#[derive(Debug, Clone)]
pub struct SyncDeltaInvestCommRate {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平投资者期货保证金率
#[derive(Debug, Clone)]
pub struct SyncDeltaInvestMarginRate {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 是否相对交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平期权标的调整保证金率
#[derive(Debug, Clone)]
pub struct SyncDeltaInvestMarginRateUL {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平组合持仓明细
#[derive(Debug, Clone)]
pub struct SyncDeltaInvestPosCombDtl {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 开仓日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组合编号。CTP：`ComTradeID`，`TThostFtdcTradeIDType`。
    pub com_trade_id: TradeId,
    /// 撮合编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 买卖。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 持仓量。CTP：`TotalAmt`，`TThostFtdcVolumeType`。
    pub total_amount: i32,
    /// 投资者保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 单腿乘数。CTP：`LegMultiple`，`TThostFtdcLegMultipleType`。
    pub leg_multiple: i32,
    /// 成交组号。CTP：`TradeGroupID`，`TThostFtdcTradeGroupIDType`。
    pub trade_group_id: i32,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平持仓明细
#[derive(Debug, Clone)]
pub struct SyncDeltaInvestPosDtl {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 买卖。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 开仓日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 成交编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 开仓价。CTP：`OpenPrice`，`TThostFtdcPriceType`。
    pub open_price: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 成交类型。CTP：`TradeType`，`TThostFtdcTradeTypeType`。
    pub trade_type: TradeType,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 逐日盯市平仓盈亏。CTP：`CloseProfitByDate`，`TThostFtdcMoneyType`。
    pub close_profit_by_date: f64,
    /// 逐笔对冲平仓盈亏。CTP：`CloseProfitByTrade`，`TThostFtdcMoneyType`。
    pub close_profit_by_trade: f64,
    /// 逐日盯市持仓盈亏。CTP：`PositionProfitByDate`，`TThostFtdcMoneyType`。
    pub position_profit_by_date: f64,
    /// 逐笔对冲持仓盈亏。CTP：`PositionProfitByTrade`，`TThostFtdcMoneyType`。
    pub position_profit_by_trade: f64,
    /// 投资者保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 昨结算价。CTP：`LastSettlementPrice`，`TThostFtdcPriceType`。
    pub last_settlement_price: f64,
    /// 结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 平仓量。CTP：`CloseVolume`，`TThostFtdcVolumeType`。
    pub close_volume: i32,
    /// 平仓金额。CTP：`CloseAmount`，`TThostFtdcMoneyType`。
    pub close_amount: f64,
    /// 先开先平剩余数量。CTP：`TimeFirstVolume`，`TThostFtdcVolumeType`。
    pub time_first_volume: i32,
    /// 特殊持仓标志。CTP：`SpecPosiType`，`TThostFtdcSpecPosiTypeType`。
    pub spec_position_type: SpecPosiType,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平投资者SPMM模板选择
#[derive(Debug, Clone)]
pub struct SyncDeltaInvestorSPMMModel {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// SPMM模板ID。CTP：`SPMMModelID`，`TThostFtdcSPMMModelIDType`。
    pub spmm_model_id: String,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平中金现货期权交易所保证金率
#[derive(Debug, Clone)]
pub struct SyncDeltaOptExchMargin {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投机空头保证金调整系数。CTP：`SShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub s_short_margin_ratio_by_money: f64,
    /// 投机空头保证金调整系数。CTP：`SShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub s_short_margin_ratio_by_volume: f64,
    /// 保值空头保证金调整系数。CTP：`HShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub h_short_margin_ratio_by_money: f64,
    /// 保值空头保证金调整系数。CTP：`HShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub h_short_margin_ratio_by_volume: f64,
    /// 套利空头保证金调整系数。CTP：`AShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub a_short_margin_ratio_by_money: f64,
    /// 套利空头保证金调整系数。CTP：`AShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub a_short_margin_ratio_by_volume: f64,
    /// 做市商空头保证金调整系数。CTP：`MShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub m_short_margin_ratio_by_money: f64,
    /// 做市商空头保证金调整系数。CTP：`MShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub m_short_margin_ratio_by_volume: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平期权手续费率
#[derive(Debug, Clone)]
pub struct SyncDeltaOptInvestCommRate {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 执行手续费率。CTP：`StrikeRatioByMoney`，`TThostFtdcRatioType`。
    pub strike_ratio_by_money: f64,
    /// 执行手续费。CTP：`StrikeRatioByVolume`，`TThostFtdcRatioType`。
    pub strike_ratio_by_volume: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平中金现货期权投资者保证金率
#[derive(Debug, Clone)]
pub struct SyncDeltaOptInvestMargin {
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机空头保证金调整系数。CTP：`SShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub s_short_margin_ratio_by_money: f64,
    /// 投机空头保证金调整系数。CTP：`SShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub s_short_margin_ratio_by_volume: f64,
    /// 保值空头保证金调整系数。CTP：`HShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub h_short_margin_ratio_by_money: f64,
    /// 保值空头保证金调整系数。CTP：`HShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub h_short_margin_ratio_by_volume: f64,
    /// 套利空头保证金调整系数。CTP：`AShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub a_short_margin_ratio_by_money: f64,
    /// 套利空头保证金调整系数。CTP：`AShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub a_short_margin_ratio_by_volume: f64,
    /// 是否跟随交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 做市商空头保证金调整系数。CTP：`MShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub m_short_margin_ratio_by_money: f64,
    /// 做市商空头保证金调整系数。CTP：`MShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub m_short_margin_ratio_by_volume: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平交叉汇率
#[derive(Debug, Clone)]
pub struct SyncDeltaProductExchRate {
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 报价币种类型。CTP：`QuoteCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub quote_currency_id: String,
    /// 汇率。CTP：`ExchangeRate`，`TThostFtdcExchangeRateType`。
    pub exchange_rate: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平产品信息
#[derive(Debug, Clone)]
pub struct SyncDeltaProductStatus {
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcInstrumentIDType`。
    pub product_id: InstrumentId,
    /// 是否允许交易。CTP：`ProductStatus`，`TThostFtdcProductStatusType`。
    pub product_status: ProductStatus,
}

/// 风险结算追平RCAMS产品组合信息
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSCombProdInfo {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 商品组代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RCAMS策略组合规则明细
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSCombRuleDtl {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 策略产品。CTP：`ProdGroup`，`TThostFtdcProductIDType`。
    pub product_group: ProductId,
    /// 策略id。CTP：`RuleId`，`TThostFtdcRuleIdType`。
    pub rule_id: String,
    /// 优先级。CTP：`Priority`，`TThostFtdcRCAMSPriorityType`。
    pub priority: i32,
    /// 投套标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 组合保证金标准。CTP：`CombMargin`，`TThostFtdcMoneyType`。
    pub comb_margin: f64,
    /// 交易所组合合约代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 单腿合约代码。CTP：`LegInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub leg_instrument_id: InstrumentId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 单腿乘数。CTP：`LegMultiple`，`TThostFtdcLegMultipleType`。
    pub leg_multiple: i32,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RCAMS同合约风险对冲参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSInstrParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcProductIDType`。
    pub product_id: ProductId,
    /// 同合约风险对冲比率。CTP：`HedgeRate`，`TThostFtdcHedgeRateType`。
    pub hedge_rate: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RCAMS跨品种风险折抵参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 商品群代码。CTP：`ProductGroupID`，`TThostFtdcProductIDType`。
    pub product_group_id: ProductId,
    /// 优先级。CTP：`Priority`，`TThostFtdcRCAMSPriorityType`。
    pub priority: i32,
    /// 折抵率。CTP：`CreditRate`，`TThostFtdcHedgeRateType`。
    pub credit_rate: f64,
    /// 产品组合代码1。CTP：`CombProduct1`，`TThostFtdcProductIDType`。
    pub comb_product1: ProductId,
    /// 产品组合代码2。CTP：`CombProduct2`，`TThostFtdcProductIDType`。
    pub comb_product2: ProductId,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RCAMS品种内风险对冲参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSIntraParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 品种内对冲比率。CTP：`HedgeRate`，`TThostFtdcHedgeRateType`。
    pub hedge_rate: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RCAMS策略组合持仓
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSInvestCombPos {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 投套标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 持仓多空方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 组合合约代码。CTP：`CombInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub comb_instrument_id: InstrumentId,
    /// 单腿编号。CTP：`LegID`，`TThostFtdcLegIDType`。
    pub leg_id: i32,
    /// 交易所组合合约代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
    /// 持仓量。CTP：`TotalAmt`，`TThostFtdcVolumeType`。
    pub total_amount: i32,
    /// 交易所保证金。CTP：`ExchMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者保证金。CTP：`Margin`，`TThostFtdcMoneyType`。
    pub margin: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RCAMS空头期权风险调整参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRCAMSSOptAdjParam {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品组合代码。CTP：`CombProductID`，`TThostFtdcProductIDType`。
    pub comb_product_id: ProductId,
    /// 投套标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 空头期权风险调整标准。CTP：`AdjustValue`，`TThostFtdcAdjustValueType`。
    pub adjust_value: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RULE合约保证金参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRULEInstrParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约类型。CTP：`InstrumentClass`，`TThostFtdcInstrumentClassType`。
    pub instrument_class: InstrumentClass,
    /// 标准合约。CTP：`StdInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub std_instrument_id: InstrumentId,
    /// 投机买折算系数。CTP：`BSpecRatio`，`TThostFtdcRatioType`。
    pub b_spec_ratio: f64,
    /// 投机卖折算系数。CTP：`SSpecRatio`，`TThostFtdcRatioType`。
    pub s_spec_ratio: f64,
    /// 套保买折算系数。CTP：`BHedgeRatio`，`TThostFtdcRatioType`。
    pub b_hedge_ratio: f64,
    /// 套保卖折算系数。CTP：`SHedgeRatio`，`TThostFtdcRatioType`。
    pub s_hedge_ratio: f64,
    /// 买附加风险保证金。CTP：`BAddOnMargin`，`TThostFtdcMoneyType`。
    pub b_add_on_margin: f64,
    /// 卖附加风险保证金。CTP：`SAddOnMargin`，`TThostFtdcMoneyType`。
    pub s_add_on_margin: f64,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RULE跨品种抵扣参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRULEInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 优先级。CTP：`SpreadId`，`TThostFtdcSpreadIdType`。
    pub spread_id: i32,
    /// 品种间对锁仓费率折扣比例。CTP：`InterRate`，`TThostFtdcRatioType`。
    pub inter_rate: f64,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
    /// 腿1比例系数。CTP：`Leg1PropFactor`，`TThostFtdcCommonIntType`。
    pub leg1_prop_factor: i32,
    /// 腿2比例系数。CTP：`Leg2PropFactor`，`TThostFtdcCommonIntType`。
    pub leg2_prop_factor: i32,
    /// 商品群号。CTP：`CommodityGroupID`，`TThostFtdcCommodityGroupIDType`。
    pub commodity_group_id: i32,
    /// 商品群名称。CTP：`CommodityGroupName`，`TThostFtdcInstrumentNameType`。
    pub commodity_group_name: String,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平RULE品种内对锁仓折扣参数
#[derive(Debug, Clone)]
pub struct SyncDeltaRULEIntraParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 标准合约。CTP：`StdInstrumentID`，`TThostFtdcInstrumentIDType`。
    pub std_instrument_id: InstrumentId,
    /// 标准合约保证金。CTP：`StdInstrMargin`，`TThostFtdcMoneyType`。
    pub std_instrument_margin: f64,
    /// 一般月份合约组合保证金系数。CTP：`UsualIntraRate`，`TThostFtdcRatioType`。
    pub usual_intra_rate: f64,
    /// 临近交割合约组合保证金系数。CTP：`DeliveryIntraRate`，`TThostFtdcRatioType`。
    pub delivery_intra_rate: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPBM附加跨品种抵扣参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMAddOnInterParam {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 优先级。CTP：`SpreadId`，`TThostFtdcSpreadIdType`。
    pub spread_id: i32,
    /// 品种间对锁仓附加费率折扣比例。CTP：`AddOnInterRateZ2`，`TThostFtdcRatioType`。
    pub add_on_inter_rate_z2: f64,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPBM期货合约保证金参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMFutureParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 期货合约因子。CTP：`Cvf`，`TThostFtdcVolumeMultipleType`。
    pub cvf: i32,
    /// 阶段标识。CTP：`TimeRange`，`TThostFtdcTimeRangeType`。
    pub time_range: TimeRange,
    /// 品种保证金标准。CTP：`MarginRate`，`TThostFtdcRatioType`。
    pub margin_rate: f64,
    /// 期货合约内部对锁仓费率折扣比例。CTP：`LockRateX`，`TThostFtdcRatioType`。
    pub lock_rate_x: f64,
    /// 提高保证金标准。CTP：`AddOnRate`，`TThostFtdcRatioType`。
    pub add_on_rate: f64,
    /// 昨结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 期货合约内部对锁仓附加费率折扣比例。CTP：`AddOnLockRateX2`，`TThostFtdcRatioType`。
    pub add_on_lock_rate_x2: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPBM跨品种抵扣参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMInterParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 优先级。CTP：`SpreadId`，`TThostFtdcSpreadIdType`。
    pub spread_id: i32,
    /// 品种间对锁仓费率折扣比例。CTP：`InterRateZ`，`TThostFtdcRatioType`。
    pub inter_rate_z: f64,
    /// 第一腿构成品种。CTP：`Leg1ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg1_product_family_code: InstrumentId,
    /// 第二腿构成品种。CTP：`Leg2ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub leg2_product_family_code: InstrumentId,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPBM品种内对锁仓折扣参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMIntraParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 品种内合约间对锁仓费率折扣比例。CTP：`IntraRateY`，`TThostFtdcRatioType`。
    pub intra_rate_y: f64,
    /// 品种内合约间对锁仓附加费率折扣比例。CTP：`AddOnIntraRateY2`，`TThostFtdcRatioType`。
    pub add_on_intra_rate_y2: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平投资者SPBM套餐选择
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMInvestPortfDef {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 组合保证金套餐代码。CTP：`PortfolioDefID`，`TThostFtdcPortfolioDefIDType`。
    pub portfolio_definition_id: i32,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPBM期权合约保证金参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMOptionParameter {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 期权合约因子。CTP：`Cvf`，`TThostFtdcVolumeMultipleType`。
    pub cvf: i32,
    /// 期权冲抵价格。CTP：`DownPrice`，`TThostFtdcPriceType`。
    pub down_price: f64,
    /// Delta值。CTP：`Delta`，`TThostFtdcDeltaType`。
    pub delta: f64,
    /// 卖方期权风险转换最低值。CTP：`SlimiDelta`，`TThostFtdcDeltaType`。
    pub slim_delta: f64,
    /// 昨结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPBM组合保证金套餐
#[derive(Debug, Clone)]
pub struct SyncDeltaSPBMPortfDefinition {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 组合保证金套餐代码。CTP：`PortfolioDefID`，`TThostFtdcPortfolioDefIDType`。
    pub portfolio_definition_id: i32,
    /// 品种代码。CTP：`ProdFamilyCode`，`TThostFtdcInstrumentIDType`。
    pub product_family_code: InstrumentId,
    /// 是否启用SPBM。CTP：`IsSPBM`，`TThostFtdcBoolType`。
    pub is_spbm: bool,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPMM合约参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPMMInstParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// SPMM合约保证金算法。CTP：`InstMarginCalID`，`TThostFtdcInstMarginCalIDType`。
    pub instrument_margin_cal_id: InstMarginCalID,
    /// 商品组代码。CTP：`CommodityID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_id: String,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPMM模板参数设置
#[derive(Debug, Clone)]
pub struct SyncDeltaSPMMModelParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// SPMM模板ID。CTP：`SPMMModelID`，`TThostFtdcSPMMModelIDType`。
    pub spmm_model_id: String,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
    /// SPMM品种内跨期优惠系数。CTP：`IntraCommodityRate`，`TThostFtdcSPMMDiscountRatioType`。
    pub intra_commodity_rate: f64,
    /// SPMM品种间优惠系数。CTP：`InterCommodityRate`，`TThostFtdcSPMMDiscountRatioType`。
    pub inter_commodity_rate: f64,
    /// SPMM期权优惠系数。CTP：`OptionDiscountRate`，`TThostFtdcSPMMDiscountRatioType`。
    pub option_discount_rate: f64,
    /// 商品群最小保证金比例。CTP：`MiniMarginRatio`，`TThostFtdcSPMMDiscountRatioType`。
    pub mini_margin_ratio: f64,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平SPMM产品相关参数
#[derive(Debug, Clone)]
pub struct SyncDeltaSPMMProductParam {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 产品代码。CTP：`ProductID`，`TThostFtdcSPMMProductIDType`。
    pub product_id: String,
    /// 商品组代码。CTP：`CommodityID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_id: String,
    /// 商品群代码。CTP：`CommodityGroupID`，`TThostFtdcSPMMProductIDType`。
    pub commodity_group_id: String,
    /// 操作标志。CTP：`ActionDirection`，`TThostFtdcActionDirectionType`。
    pub action_direction: ActionDirection,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 风险结算追平资金
#[derive(Debug, Clone)]
pub struct SyncDeltaTradingAccount {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 上次质押金额。CTP：`PreMortgage`，`TThostFtdcMoneyType`。
    pub pre_mortgage: f64,
    /// 上次信用额度。CTP：`PreCredit`，`TThostFtdcMoneyType`。
    pub pre_credit: f64,
    /// 上次存款额。CTP：`PreDeposit`，`TThostFtdcMoneyType`。
    pub pre_deposit: f64,
    /// 上次结算准备金。CTP：`PreBalance`，`TThostFtdcMoneyType`。
    pub pre_balance: f64,
    /// 上次占用的保证金。CTP：`PreMargin`，`TThostFtdcMoneyType`。
    pub pre_margin: f64,
    /// 利息基数。CTP：`InterestBase`，`TThostFtdcMoneyType`。
    pub interest_base: f64,
    /// 利息收入。CTP：`Interest`，`TThostFtdcMoneyType`。
    pub interest: f64,
    /// 入金金额。CTP：`Deposit`，`TThostFtdcMoneyType`。
    pub deposit: f64,
    /// 出金金额。CTP：`Withdraw`，`TThostFtdcMoneyType`。
    pub withdraw: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 当前保证金总额。CTP：`CurrMargin`，`TThostFtdcMoneyType`。
    pub current_margin: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 期货结算准备金。CTP：`Balance`，`TThostFtdcMoneyType`。
    pub balance: f64,
    /// 可用资金。CTP：`Available`，`TThostFtdcMoneyType`。
    pub available: f64,
    /// 可取资金。CTP：`WithdrawQuota`，`TThostFtdcMoneyType`。
    pub withdraw_quota: f64,
    /// 基本准备金。CTP：`Reserve`，`TThostFtdcMoneyType`。
    pub reserve: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 信用额度。CTP：`Credit`，`TThostFtdcMoneyType`。
    pub credit: f64,
    /// 质押金额。CTP：`Mortgage`，`TThostFtdcMoneyType`。
    pub mortgage: f64,
    /// 交易所保证金。CTP：`ExchangeMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者交割保证金。CTP：`DeliveryMargin`，`TThostFtdcMoneyType`。
    pub delivery_margin: f64,
    /// 交易所交割保证金。CTP：`ExchangeDeliveryMargin`，`TThostFtdcMoneyType`。
    pub exchange_delivery_margin: f64,
    /// 保底期货结算准备金。CTP：`ReserveBalance`，`TThostFtdcMoneyType`。
    pub reserve_balance: f64,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 上次货币质入金额。CTP：`PreFundMortgageIn`，`TThostFtdcMoneyType`。
    pub pre_fund_mortgage_in: f64,
    /// 上次货币质出金额。CTP：`PreFundMortgageOut`，`TThostFtdcMoneyType`。
    pub pre_fund_mortgage_out: f64,
    /// 货币质入金额。CTP：`FundMortgageIn`，`TThostFtdcMoneyType`。
    pub fund_mortgage_in: f64,
    /// 货币质出金额。CTP：`FundMortgageOut`，`TThostFtdcMoneyType`。
    pub fund_mortgage_out: f64,
    /// 货币质押余额。CTP：`FundMortgageAvailable`，`TThostFtdcMoneyType`。
    pub fund_mortgage_available: f64,
    /// 可质押货币金额。CTP：`MortgageableFund`，`TThostFtdcMoneyType`。
    pub mortgageable_fund: f64,
    /// 特殊产品占用保证金。CTP：`SpecProductMargin`，`TThostFtdcMoneyType`。
    pub spec_product_margin: f64,
    /// 特殊产品冻结保证金。CTP：`SpecProductFrozenMargin`，`TThostFtdcMoneyType`。
    pub spec_product_frozen_margin: f64,
    /// 特殊产品手续费。CTP：`SpecProductCommission`，`TThostFtdcMoneyType`。
    pub spec_product_commission: f64,
    /// 特殊产品冻结手续费。CTP：`SpecProductFrozenCommission`，`TThostFtdcMoneyType`。
    pub spec_product_frozen_commission: f64,
    /// 特殊产品持仓盈亏。CTP：`SpecProductPositionProfit`，`TThostFtdcMoneyType`。
    pub spec_product_position_profit: f64,
    /// 特殊产品平仓盈亏。CTP：`SpecProductCloseProfit`，`TThostFtdcMoneyType`。
    pub spec_product_close_profit: f64,
    /// 根据持仓盈亏算法计算的特殊产品持仓盈亏。CTP：`SpecProductPositionProfitByAlg`，`TThostFtdcMoneyType`。
    pub spec_product_position_profit_by_alg: f64,
    /// 特殊产品交易所保证金。CTP：`SpecProductExchangeMargin`，`TThostFtdcMoneyType`。
    pub spec_product_exchange_margin: f64,
    /// 延时换汇冻结金额。CTP：`FrozenSwap`，`TThostFtdcMoneyType`。
    pub frozen_swap: f64,
    /// 剩余换汇额度。CTP：`RemainSwap`，`TThostFtdcMoneyType`。
    pub remain_swap: f64,
    /// 期权市值。CTP：`OptionValue`，`TThostFtdcMoneyType`。
    pub option_value: f64,
    /// 追平序号。CTP：`SyncDeltaSequenceNo`，`TThostFtdcSequenceNoType`。
    pub sync_delta_sequence_no: i32,
}

/// 出入金同步
#[derive(Debug, Clone)]
pub struct SyncDeposit {
    /// 出入金流水号。CTP：`DepositSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub deposit_seq_no: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 入金金额。CTP：`Deposit`，`TThostFtdcMoneyType`。
    pub deposit: f64,
    /// 是否强制进行。CTP：`IsForce`，`TThostFtdcBoolType`。
    pub is_force: bool,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 是否是个股期权内转。CTP：`IsFromSopt`，`TThostFtdcBoolType`。
    pub is_from_sopt: bool,
    /// 资金密码。CTP：`TradingPassword`，`TThostFtdcPasswordType`。
    pub trading_password: String,
    /// 是否二级代理商的内转。CTP：`IsSecAgentTranfer`，`TThostFtdcBoolType`。
    pub is_sec_agent_transfer: bool,
}

/// 货币质押同步
#[derive(Debug, Clone)]
pub struct SyncFundMortgage {
    /// 货币质押流水号。CTP：`MortgageSeqNo`，`TThostFtdcDepositSeqNoType`。
    pub mortgage_seq_no: String,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 源币种。CTP：`FromCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub from_currency_id: String,
    /// 质押金额。CTP：`MortgageAmount`，`TThostFtdcMoneyType`。
    pub mortgage_amount: f64,
    /// 目标币种。CTP：`ToCurrencyID`，`TThostFtdcCurrencyIDType`。
    pub to_currency_id: String,
}

/// 同步SPBM参数结束
#[derive(Debug, Clone)]
pub struct SyncSPBMParameterEnd {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
}

/// 数据同步状态
#[derive(Debug, Clone)]
pub struct SyncStatus {
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 数据同步状态。CTP：`DataSyncStatus`，`TThostFtdcDataSyncStatusType`。
    pub data_sync_status: DataSyncStatus,
}

/// 正在同步中的合约手续费率
#[derive(Debug, Clone)]
pub struct SyncingInstrumentCommissionRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 开仓手续费率。CTP：`OpenRatioByMoney`，`TThostFtdcRatioType`。
    pub open_ratio_by_money: f64,
    /// 开仓手续费。CTP：`OpenRatioByVolume`，`TThostFtdcRatioType`。
    pub open_ratio_by_volume: f64,
    /// 平仓手续费率。CTP：`CloseRatioByMoney`，`TThostFtdcRatioType`。
    pub close_ratio_by_money: f64,
    /// 平仓手续费。CTP：`CloseRatioByVolume`，`TThostFtdcRatioType`。
    pub close_ratio_by_volume: f64,
    /// 平今手续费率。CTP：`CloseTodayRatioByMoney`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_money: f64,
    /// 平今手续费。CTP：`CloseTodayRatioByVolume`，`TThostFtdcRatioType`。
    pub close_today_ratio_by_volume: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 正在同步中的合约保证金率
#[derive(Debug, Clone)]
pub struct SyncingInstrumentMarginRate {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 多头保证金率。CTP：`LongMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub long_margin_ratio_by_money: f64,
    /// 多头保证金费。CTP：`LongMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub long_margin_ratio_by_volume: f64,
    /// 空头保证金率。CTP：`ShortMarginRatioByMoney`，`TThostFtdcRatioType`。
    pub short_margin_ratio_by_money: f64,
    /// 空头保证金费。CTP：`ShortMarginRatioByVolume`，`TThostFtdcMoneyType`。
    pub short_margin_ratio_by_volume: f64,
    /// 是否相对交易所收取。CTP：`IsRelative`，`TThostFtdcBoolType`。
    pub is_relative: bool,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 正在同步中的合约交易权限
#[derive(Debug, Clone)]
pub struct SyncingInstrumentTradingRight {
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 交易权限。CTP：`TradingRight`，`TThostFtdcTradingRightType`。
    pub trading_right: TradingRight,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 正在同步中的投资者
#[derive(Debug, Clone)]
pub struct SyncingInvestor {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者分组代码。CTP：`InvestorGroupID`，`TThostFtdcInvestorIDType`。
    pub investor_group_id: InvestorId,
    /// 投资者名称。CTP：`InvestorName`，`TThostFtdcPartyNameType`。
    pub investor_name: String,
    /// 证件类型。CTP：`IdentifiedCardType`，`TThostFtdcIdCardTypeType`。
    pub identified_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
    /// 联系电话。CTP：`Telephone`，`TThostFtdcTelephoneType`。
    pub telephone: String,
    /// 通讯地址。CTP：`Address`，`TThostFtdcAddressType`。
    pub address: String,
    /// 开户日期。CTP：`OpenDate`，`TThostFtdcDateType`。
    pub open_date: DateText,
    /// 手机。CTP：`Mobile`，`TThostFtdcMobileType`。
    pub mobile: String,
    /// 手续费率模板代码。CTP：`CommModelID`，`TThostFtdcInvestorIDType`。
    pub commission_model_id: InvestorId,
    /// 保证金率模板代码。CTP：`MarginModelID`，`TThostFtdcInvestorIDType`。
    pub margin_model_id: InvestorId,
    /// 是否频率控制。CTP：`IsOrderFreq`，`TThostFtdcEnumBoolType`。
    pub is_order_frequency: EnumBool,
    /// 是否开仓限制。CTP：`IsOpenVolLimit`，`TThostFtdcEnumBoolType`。
    pub is_open_vol_limit: EnumBool,
}

/// 正在同步中的投资者分组
#[derive(Debug, Clone)]
pub struct SyncingInvestorGroup {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者分组代码。CTP：`InvestorGroupID`，`TThostFtdcInvestorIDType`。
    pub investor_group_id: InvestorId,
    /// 投资者分组名称。CTP：`InvestorGroupName`，`TThostFtdcInvestorGroupNameType`。
    pub investor_group_name: String,
}

/// 正在同步中的投资者持仓
#[derive(Debug, Clone)]
pub struct SyncingInvestorPosition {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 持仓多空方向。CTP：`PosiDirection`，`TThostFtdcPosiDirectionType`。
    pub position_direction: PosiDirection,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 持仓日期。CTP：`PositionDate`，`TThostFtdcPositionDateType`。
    pub position_date: PositionDate,
    /// 上日持仓。CTP：`YdPosition`，`TThostFtdcVolumeType`。
    pub yd_position: i32,
    /// 今日持仓。CTP：`Position`，`TThostFtdcVolumeType`。
    pub position: i32,
    /// 多头冻结。CTP：`LongFrozen`，`TThostFtdcVolumeType`。
    pub long_frozen: i32,
    /// 空头冻结。CTP：`ShortFrozen`，`TThostFtdcVolumeType`。
    pub short_frozen: i32,
    /// 开仓冻结金额。CTP：`LongFrozenAmount`，`TThostFtdcMoneyType`。
    pub long_frozen_amount: f64,
    /// 开仓冻结金额。CTP：`ShortFrozenAmount`，`TThostFtdcMoneyType`。
    pub short_frozen_amount: f64,
    /// 开仓量。CTP：`OpenVolume`，`TThostFtdcVolumeType`。
    pub open_volume: i32,
    /// 平仓量。CTP：`CloseVolume`，`TThostFtdcVolumeType`。
    pub close_volume: i32,
    /// 开仓金额。CTP：`OpenAmount`，`TThostFtdcMoneyType`。
    pub open_amount: f64,
    /// 平仓金额。CTP：`CloseAmount`，`TThostFtdcMoneyType`。
    pub close_amount: f64,
    /// 持仓成本。CTP：`PositionCost`，`TThostFtdcMoneyType`。
    pub position_cost: f64,
    /// 上次占用的保证金。CTP：`PreMargin`，`TThostFtdcMoneyType`。
    pub pre_margin: f64,
    /// 占用的保证金。CTP：`UseMargin`，`TThostFtdcMoneyType`。
    pub use_margin: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 上次结算价。CTP：`PreSettlementPrice`，`TThostFtdcPriceType`。
    pub pre_settlement_price: f64,
    /// 本次结算价。CTP：`SettlementPrice`，`TThostFtdcPriceType`。
    pub settlement_price: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 开仓成本。CTP：`OpenCost`，`TThostFtdcMoneyType`。
    pub open_cost: f64,
    /// 交易所保证金。CTP：`ExchangeMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 组合成交形成的持仓。CTP：`CombPosition`，`TThostFtdcVolumeType`。
    pub comb_position: i32,
    /// 组合多头冻结。CTP：`CombLongFrozen`，`TThostFtdcVolumeType`。
    pub comb_long_frozen: i32,
    /// 组合空头冻结。CTP：`CombShortFrozen`，`TThostFtdcVolumeType`。
    pub comb_short_frozen: i32,
    /// 逐日盯市平仓盈亏。CTP：`CloseProfitByDate`，`TThostFtdcMoneyType`。
    pub close_profit_by_date: f64,
    /// 逐笔对冲平仓盈亏。CTP：`CloseProfitByTrade`，`TThostFtdcMoneyType`。
    pub close_profit_by_trade: f64,
    /// 今日持仓。CTP：`TodayPosition`，`TThostFtdcVolumeType`。
    pub today_position: i32,
    /// 保证金率。CTP：`MarginRateByMoney`，`TThostFtdcRatioType`。
    pub margin_rate_by_money: f64,
    /// 保证金率(按手数)。CTP：`MarginRateByVolume`，`TThostFtdcRatioType`。
    pub margin_rate_by_volume: f64,
    /// 执行冻结。CTP：`StrikeFrozen`，`TThostFtdcVolumeType`。
    pub strike_frozen: i32,
    /// 执行冻结金额。CTP：`StrikeFrozenAmount`，`TThostFtdcMoneyType`。
    pub strike_frozen_amount: f64,
    /// 放弃执行冻结。CTP：`AbandonFrozen`，`TThostFtdcVolumeType`。
    pub abandon_frozen: i32,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 执行冻结的昨仓。CTP：`YdStrikeFrozen`，`TThostFtdcVolumeType`。
    pub yd_strike_frozen: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 持仓成本差值。CTP：`PositionCostOffset`，`TThostFtdcMoneyType`。
    pub position_cost_offset: f64,
    /// tas持仓手数。CTP：`TasPosition`，`TThostFtdcVolumeType`。
    pub tas_position: i32,
    /// tas持仓成本。CTP：`TasPositionCost`，`TThostFtdcMoneyType`。
    pub tas_position_cost: f64,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
}

/// 正在同步中的交易账号
#[derive(Debug, Clone)]
pub struct SyncingTradingAccount {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 上次质押金额。CTP：`PreMortgage`，`TThostFtdcMoneyType`。
    pub pre_mortgage: f64,
    /// 上次信用额度。CTP：`PreCredit`，`TThostFtdcMoneyType`。
    pub pre_credit: f64,
    /// 上次存款额。CTP：`PreDeposit`，`TThostFtdcMoneyType`。
    pub pre_deposit: f64,
    /// 上次结算准备金。CTP：`PreBalance`，`TThostFtdcMoneyType`。
    pub pre_balance: f64,
    /// 上次占用的保证金。CTP：`PreMargin`，`TThostFtdcMoneyType`。
    pub pre_margin: f64,
    /// 利息基数。CTP：`InterestBase`，`TThostFtdcMoneyType`。
    pub interest_base: f64,
    /// 利息收入。CTP：`Interest`，`TThostFtdcMoneyType`。
    pub interest: f64,
    /// 入金金额。CTP：`Deposit`，`TThostFtdcMoneyType`。
    pub deposit: f64,
    /// 出金金额。CTP：`Withdraw`，`TThostFtdcMoneyType`。
    pub withdraw: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 当前保证金总额。CTP：`CurrMargin`，`TThostFtdcMoneyType`。
    pub current_margin: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 期货结算准备金。CTP：`Balance`，`TThostFtdcMoneyType`。
    pub balance: f64,
    /// 可用资金。CTP：`Available`，`TThostFtdcMoneyType`。
    pub available: f64,
    /// 可取资金。CTP：`WithdrawQuota`，`TThostFtdcMoneyType`。
    pub withdraw_quota: f64,
    /// 基本准备金。CTP：`Reserve`，`TThostFtdcMoneyType`。
    pub reserve: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 信用额度。CTP：`Credit`，`TThostFtdcMoneyType`。
    pub credit: f64,
    /// 质押金额。CTP：`Mortgage`，`TThostFtdcMoneyType`。
    pub mortgage: f64,
    /// 交易所保证金。CTP：`ExchangeMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者交割保证金。CTP：`DeliveryMargin`，`TThostFtdcMoneyType`。
    pub delivery_margin: f64,
    /// 交易所交割保证金。CTP：`ExchangeDeliveryMargin`，`TThostFtdcMoneyType`。
    pub exchange_delivery_margin: f64,
    /// 保底期货结算准备金。CTP：`ReserveBalance`，`TThostFtdcMoneyType`。
    pub reserve_balance: f64,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 上次货币质入金额。CTP：`PreFundMortgageIn`，`TThostFtdcMoneyType`。
    pub pre_fund_mortgage_in: f64,
    /// 上次货币质出金额。CTP：`PreFundMortgageOut`，`TThostFtdcMoneyType`。
    pub pre_fund_mortgage_out: f64,
    /// 货币质入金额。CTP：`FundMortgageIn`，`TThostFtdcMoneyType`。
    pub fund_mortgage_in: f64,
    /// 货币质出金额。CTP：`FundMortgageOut`，`TThostFtdcMoneyType`。
    pub fund_mortgage_out: f64,
    /// 货币质押余额。CTP：`FundMortgageAvailable`，`TThostFtdcMoneyType`。
    pub fund_mortgage_available: f64,
    /// 可质押货币金额。CTP：`MortgageableFund`，`TThostFtdcMoneyType`。
    pub mortgageable_fund: f64,
    /// 特殊产品占用保证金。CTP：`SpecProductMargin`，`TThostFtdcMoneyType`。
    pub spec_product_margin: f64,
    /// 特殊产品冻结保证金。CTP：`SpecProductFrozenMargin`，`TThostFtdcMoneyType`。
    pub spec_product_frozen_margin: f64,
    /// 特殊产品手续费。CTP：`SpecProductCommission`，`TThostFtdcMoneyType`。
    pub spec_product_commission: f64,
    /// 特殊产品冻结手续费。CTP：`SpecProductFrozenCommission`，`TThostFtdcMoneyType`。
    pub spec_product_frozen_commission: f64,
    /// 特殊产品持仓盈亏。CTP：`SpecProductPositionProfit`，`TThostFtdcMoneyType`。
    pub spec_product_position_profit: f64,
    /// 特殊产品平仓盈亏。CTP：`SpecProductCloseProfit`，`TThostFtdcMoneyType`。
    pub spec_product_close_profit: f64,
    /// 根据持仓盈亏算法计算的特殊产品持仓盈亏。CTP：`SpecProductPositionProfitByAlg`，`TThostFtdcMoneyType`。
    pub spec_product_position_profit_by_alg: f64,
    /// 特殊产品交易所保证金。CTP：`SpecProductExchangeMargin`，`TThostFtdcMoneyType`。
    pub spec_product_exchange_margin: f64,
    /// 延时换汇冻结金额。CTP：`FrozenSwap`，`TThostFtdcMoneyType`。
    pub frozen_swap: f64,
    /// 剩余换汇额度。CTP：`RemainSwap`，`TThostFtdcMoneyType`。
    pub remain_swap: f64,
    /// 期权市值。CTP：`OptionValue`，`TThostFtdcMoneyType`。
    pub option_value: f64,
}

/// 正在同步中的交易代码
#[derive(Debug, Clone)]
pub struct SyncingTradingCode {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
    /// 交易编码类型。CTP：`ClientIDType`，`TThostFtdcClientIDTypeType`。
    pub client_id_type: ClientIDType,
}

/// 服务地址参数
#[derive(Debug, Clone)]
pub struct TGIpAddrParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 服务地址。CTP：`Address`，`TThostFtdcIpAddrType`。
    pub address: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
    /// 交易中心名称。CTP：`DRIdentityName`，`TThostFtdcDRIdentityNameType`。
    pub dr_identity_name: String,
    /// 交易地址OR行情地址。CTP：`AddrSrvMode`，`TThostFtdcAddrSrvModeType`。
    pub address_srv_mode: AddrSrvMode,
    /// 地址版本。CTP：`AddrVer`，`TThostFtdcAddrVerType`。
    pub address_ver: AddrVer,
    /// 服务地址编号。CTP：`AddrNo`，`TThostFtdcCommonIntType`。
    pub address_no: i32,
    /// 服务地址名称。CTP：`AddrName`，`TThostFtdcAddrNameType`。
    pub address_name: String,
    /// 是否是国密地址。CTP：`IsSM`，`TThostFtdcBoolType`。
    pub is_sm: bool,
    /// 是否是内网地址。CTP：`IsLocalAddr`，`TThostFtdcBoolType`。
    pub is_local_address: bool,
    /// 地址补充信息。CTP：`Remark`，`TThostFtdcAddrRemarkType`。
    pub remark: String,
    /// 站点。CTP：`Site`，`TThostFtdcSiteType`。
    pub site: String,
    /// 网络运营商。CTP：`NetOperator`，`TThostFtdcNetOperatorType`。
    pub net_operator: String,
    /// 系统名称。CTP：`SysName`，`TThostFtdcAddrNameType`。
    pub system_name: String,
}

/// TGate会话查询状态
#[derive(Debug, Clone)]
pub struct TGSessionQryStatus {
    /// 最近30s的查询频率。CTP：`LastQryFreq`，`TThostFtdcCommonIntType`。
    pub last_qry_frequency: i32,
    /// 查询状态。CTP：`QryStatus`，`TThostFtdcTGSessionQryStatusType`。
    pub qry_status: TGSessionQryStatusType,
}

/// Thost终端用户功能权限
#[derive(Debug, Clone)]
pub struct ThostUserFunction {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// Thost终端功能代码。CTP：`ThostFunctionCode`，`TThostFtdcThostFunctionCodeType`。
    pub thost_function_code: i32,
}

/// 成交
#[derive(Debug, Clone)]
pub struct Trade {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 报单引用。CTP：`OrderRef`，`TThostFtdcOrderRefType`。
    pub order_ref: OrderRef,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 成交编号。CTP：`TradeID`，`TThostFtdcTradeIDType`。
    pub trade_id: TradeId,
    /// 买卖方向。CTP：`Direction`，`TThostFtdcDirectionType`。
    pub direction: Direction,
    /// 报单编号。CTP：`OrderSysID`，`TThostFtdcOrderSysIDType`。
    pub order_system_id: OrderSysId,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 交易角色。CTP：`TradingRole`，`TThostFtdcTradingRoleType`。
    pub trading_role: TradingRole,
    /// 开平标志。CTP：`OffsetFlag`，`TThostFtdcOffsetFlagType`。
    pub offset_flag: OffsetFlag,
    /// 投机套保标志。CTP：`HedgeFlag`，`TThostFtdcHedgeFlagType`。
    pub hedge_flag: HedgeFlag,
    /// 价格。CTP：`Price`，`TThostFtdcPriceType`。
    pub price: f64,
    /// 数量。CTP：`Volume`，`TThostFtdcVolumeType`。
    pub volume: i32,
    /// 成交时期。CTP：`TradeDate`，`TThostFtdcDateType`。
    pub trade_date: DateText,
    /// 成交时间。CTP：`TradeTime`，`TThostFtdcTimeType`。
    pub trade_time: TimeText,
    /// 成交类型。CTP：`TradeType`，`TThostFtdcTradeTypeType`。
    pub trade_type: TradeType,
    /// 成交价来源。CTP：`PriceSource`，`TThostFtdcPriceSourceType`。
    pub price_source: PriceSource,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 结算会员编号。CTP：`ClearingPartID`，`TThostFtdcParticipantIDType`。
    pub clearing_part_id: String,
    /// 业务单元。CTP：`BusinessUnit`，`TThostFtdcBusinessUnitType`。
    pub business_unit: BusinessUnit,
    /// 序号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 经纪公司报单编号。CTP：`BrokerOrderSeq`，`TThostFtdcSequenceNoType`。
    pub broker_order_seq: i32,
    /// 成交来源。CTP：`TradeSource`，`TThostFtdcTradeSourceType`。
    pub trade_source: TradeSource,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
    /// 合约代码。CTP：`InstrumentID`，`TThostFtdcInstrumentIDType`。
    pub instrument_id: InstrumentId,
    /// 合约在交易所的代码。CTP：`ExchangeInstID`，`TThostFtdcExchangeInstIDType`。
    pub exchange_instrument_id: ExchangeInstrumentId,
}

/// 交易参数
#[derive(Debug, Clone)]
pub struct TradeParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 参数代码。CTP：`TradeParamID`，`TThostFtdcTradeParamIDType`。
    pub trade_parameter_id: TradeParamID,
    /// 参数代码值。CTP：`TradeParamValue`，`TThostFtdcSettlementParamValueType`。
    pub trade_parameter_value: String,
    /// 备注。CTP：`Memo`，`TThostFtdcMemoType`。
    pub memo: String,
}

/// 交易所交易员
#[derive(Debug, Clone)]
pub struct Trader {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装数量。CTP：`InstallCount`，`TThostFtdcInstallCountType`。
    pub install_count: i32,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 撤单时选择席位算法。CTP：`OrderCancelAlg`，`TThostFtdcOrderCancelAlgType`。
    pub order_cancel_alg: OrderCancelAlg,
    /// 交易报盘安装数量。CTP：`TradeInstallCount`，`TThostFtdcInstallCountType`。
    pub trade_install_count: i32,
    /// 行情报盘安装数量。CTP：`MDInstallCount`，`TThostFtdcInstallCountType`。
    pub md_install_count: i32,
}

/// 席位与交易中心对应关系
#[derive(Debug, Clone)]
pub struct TraderAssign {
    /// 应用单元代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
}

/// 交易所交易员报盘机
#[derive(Debug, Clone)]
pub struct TraderOffer {
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 交易所交易员代码。CTP：`TraderID`，`TThostFtdcTraderIDType`。
    pub trader_id: String,
    /// 会员代码。CTP：`ParticipantID`，`TThostFtdcParticipantIDType`。
    pub participant_id: String,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 本地报单编号。CTP：`OrderLocalID`，`TThostFtdcOrderLocalIDType`。
    pub order_local_id: OrderLocalId,
    /// 交易所交易员连接状态。CTP：`TraderConnectStatus`，`TThostFtdcTraderConnectStatusType`。
    pub trader_connect_status: TraderConnectStatus,
    /// 发出连接请求的日期。CTP：`ConnectRequestDate`，`TThostFtdcDateType`。
    pub connect_request_date: DateText,
    /// 发出连接请求的时间。CTP：`ConnectRequestTime`，`TThostFtdcTimeType`。
    pub connect_request_time: TimeText,
    /// 上次报告日期。CTP：`LastReportDate`，`TThostFtdcDateType`。
    pub last_report_date: DateText,
    /// 上次报告时间。CTP：`LastReportTime`，`TThostFtdcTimeType`。
    pub last_report_time: TimeText,
    /// 完成连接日期。CTP：`ConnectDate`，`TThostFtdcDateType`。
    pub connect_date: DateText,
    /// 完成连接时间。CTP：`ConnectTime`，`TThostFtdcTimeType`。
    pub connect_time: TimeText,
    /// 启动日期。CTP：`StartDate`，`TThostFtdcDateType`。
    pub start_date: DateText,
    /// 启动时间。CTP：`StartTime`，`TThostFtdcTimeType`。
    pub start_time: TimeText,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 本席位最大成交编号。CTP：`MaxTradeID`，`TThostFtdcTradeIDType`。
    pub max_trade_id: TradeId,
    /// 本席位最大报单备拷。CTP：`MaxOrderMessageReference`，`TThostFtdcReturnCodeType`。
    pub max_order_message_reference: String,
    /// 撤单时选择席位算法。CTP：`OrderCancelAlg`，`TThostFtdcOrderCancelAlgType`。
    pub order_cancel_alg: OrderCancelAlg,
}

/// 资金账户
#[derive(Debug, Clone)]
pub struct TradingAccount {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 上次质押金额。CTP：`PreMortgage`，`TThostFtdcMoneyType`。
    pub pre_mortgage: f64,
    /// 上次信用额度。CTP：`PreCredit`，`TThostFtdcMoneyType`。
    pub pre_credit: f64,
    /// 上次存款额。CTP：`PreDeposit`，`TThostFtdcMoneyType`。
    pub pre_deposit: f64,
    /// 上次结算准备金。CTP：`PreBalance`，`TThostFtdcMoneyType`。
    pub pre_balance: f64,
    /// 上次占用的保证金。CTP：`PreMargin`，`TThostFtdcMoneyType`。
    pub pre_margin: f64,
    /// 利息基数。CTP：`InterestBase`，`TThostFtdcMoneyType`。
    pub interest_base: f64,
    /// 利息收入。CTP：`Interest`，`TThostFtdcMoneyType`。
    pub interest: f64,
    /// 入金金额。CTP：`Deposit`，`TThostFtdcMoneyType`。
    pub deposit: f64,
    /// 出金金额。CTP：`Withdraw`，`TThostFtdcMoneyType`。
    pub withdraw: f64,
    /// 冻结的保证金。CTP：`FrozenMargin`，`TThostFtdcMoneyType`。
    pub frozen_margin: f64,
    /// 冻结的资金。CTP：`FrozenCash`，`TThostFtdcMoneyType`。
    pub frozen_cash: f64,
    /// 冻结的手续费。CTP：`FrozenCommission`，`TThostFtdcMoneyType`。
    pub frozen_commission: f64,
    /// 当前保证金总额。CTP：`CurrMargin`，`TThostFtdcMoneyType`。
    pub current_margin: f64,
    /// 资金差额。CTP：`CashIn`，`TThostFtdcMoneyType`。
    pub cash_in: f64,
    /// 手续费。CTP：`Commission`，`TThostFtdcMoneyType`。
    pub commission: f64,
    /// 平仓盈亏。CTP：`CloseProfit`，`TThostFtdcMoneyType`。
    pub close_profit: f64,
    /// 持仓盈亏。CTP：`PositionProfit`，`TThostFtdcMoneyType`。
    pub position_profit: f64,
    /// 期货结算准备金。CTP：`Balance`，`TThostFtdcMoneyType`。
    pub balance: f64,
    /// 可用资金。CTP：`Available`，`TThostFtdcMoneyType`。
    pub available: f64,
    /// 可取资金。CTP：`WithdrawQuota`，`TThostFtdcMoneyType`。
    pub withdraw_quota: f64,
    /// 基本准备金。CTP：`Reserve`，`TThostFtdcMoneyType`。
    pub reserve: f64,
    /// 交易日。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 结算编号。CTP：`SettlementID`，`TThostFtdcSettlementIDType`。
    pub settlement_id: i32,
    /// 信用额度。CTP：`Credit`，`TThostFtdcMoneyType`。
    pub credit: f64,
    /// 质押金额。CTP：`Mortgage`，`TThostFtdcMoneyType`。
    pub mortgage: f64,
    /// 交易所保证金。CTP：`ExchangeMargin`，`TThostFtdcMoneyType`。
    pub exchange_margin: f64,
    /// 投资者交割保证金。CTP：`DeliveryMargin`，`TThostFtdcMoneyType`。
    pub delivery_margin: f64,
    /// 交易所交割保证金。CTP：`ExchangeDeliveryMargin`，`TThostFtdcMoneyType`。
    pub exchange_delivery_margin: f64,
    /// 保底期货结算准备金。CTP：`ReserveBalance`，`TThostFtdcMoneyType`。
    pub reserve_balance: f64,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 上次货币质入金额。CTP：`PreFundMortgageIn`，`TThostFtdcMoneyType`。
    pub pre_fund_mortgage_in: f64,
    /// 上次货币质出金额。CTP：`PreFundMortgageOut`，`TThostFtdcMoneyType`。
    pub pre_fund_mortgage_out: f64,
    /// 货币质入金额。CTP：`FundMortgageIn`，`TThostFtdcMoneyType`。
    pub fund_mortgage_in: f64,
    /// 货币质出金额。CTP：`FundMortgageOut`，`TThostFtdcMoneyType`。
    pub fund_mortgage_out: f64,
    /// 货币质押余额。CTP：`FundMortgageAvailable`，`TThostFtdcMoneyType`。
    pub fund_mortgage_available: f64,
    /// 可质押货币金额。CTP：`MortgageableFund`，`TThostFtdcMoneyType`。
    pub mortgageable_fund: f64,
    /// 特殊产品占用保证金。CTP：`SpecProductMargin`，`TThostFtdcMoneyType`。
    pub spec_product_margin: f64,
    /// 特殊产品冻结保证金。CTP：`SpecProductFrozenMargin`，`TThostFtdcMoneyType`。
    pub spec_product_frozen_margin: f64,
    /// 特殊产品手续费。CTP：`SpecProductCommission`，`TThostFtdcMoneyType`。
    pub spec_product_commission: f64,
    /// 特殊产品冻结手续费。CTP：`SpecProductFrozenCommission`，`TThostFtdcMoneyType`。
    pub spec_product_frozen_commission: f64,
    /// 特殊产品持仓盈亏。CTP：`SpecProductPositionProfit`，`TThostFtdcMoneyType`。
    pub spec_product_position_profit: f64,
    /// 特殊产品平仓盈亏。CTP：`SpecProductCloseProfit`，`TThostFtdcMoneyType`。
    pub spec_product_close_profit: f64,
    /// 根据持仓盈亏算法计算的特殊产品持仓盈亏。CTP：`SpecProductPositionProfitByAlg`，`TThostFtdcMoneyType`。
    pub spec_product_position_profit_by_alg: f64,
    /// 特殊产品交易所保证金。CTP：`SpecProductExchangeMargin`，`TThostFtdcMoneyType`。
    pub spec_product_exchange_margin: f64,
    /// 业务类型。CTP：`BizType`，`TThostFtdcBizTypeType`。
    pub biz_type: BizType,
    /// 延时换汇冻结金额。CTP：`FrozenSwap`，`TThostFtdcMoneyType`。
    pub frozen_swap: f64,
    /// 剩余换汇额度。CTP：`RemainSwap`，`TThostFtdcMoneyType`。
    pub remain_swap: f64,
    /// 期权市值。CTP：`OptionValue`，`TThostFtdcMoneyType`。
    pub option_value: f64,
}

/// 资金账户口令域
#[derive(Debug, Clone)]
pub struct TradingAccountPassword {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 资金账户口令变更域
#[derive(Debug, Clone)]
pub struct TradingAccountPasswordUpdate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 原来的口令。CTP：`OldPassword`，`TThostFtdcPasswordType`。
    pub old_password: String,
    /// 新的口令。CTP：`NewPassword`，`TThostFtdcPasswordType`。
    pub new_password: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 来自次席的资金账户口令变更
#[derive(Debug, Clone)]
pub struct TradingAccountPasswordUpdateFromSec {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 原来的口令。CTP：`OldPassword`，`TThostFtdcPasswordType`。
    pub old_password: String,
    /// 新的口令。CTP：`NewPassword`，`TThostFtdcPasswordType`。
    pub new_password: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 次席的交易中心代码。CTP：`FromSec`，`TThostFtdcDRIdentityIDType`。
    pub from_sec: i32,
}

/// 资金账户口令变更域
#[derive(Debug, Clone)]
pub struct TradingAccountPasswordUpdateV1 {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 原来的口令。CTP：`OldPassword`，`TThostFtdcPasswordType`。
    pub old_password: String,
    /// 新的口令。CTP：`NewPassword`，`TThostFtdcPasswordType`。
    pub new_password: String,
}

/// 资金账户基本准备金
#[derive(Debug, Clone)]
pub struct TradingAccountReserve {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 基本准备金。CTP：`Reserve`，`TThostFtdcMoneyType`。
    pub reserve: f64,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 交易编码
#[derive(Debug, Clone)]
pub struct TradingCode {
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 交易所代码。CTP：`ExchangeID`，`TThostFtdcExchangeIDType`。
    pub exchange_id: ExchangeId,
    /// 客户代码。CTP：`ClientID`，`TThostFtdcClientIDType`。
    pub client_id: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
    /// 交易编码类型。CTP：`ClientIDType`，`TThostFtdcClientIDTypeType`。
    pub client_id_type: ClientIDType,
    /// 营业部编号。CTP：`BranchID`，`TThostFtdcBranchIDType`。
    pub branch_id: String,
    /// 业务类型。CTP：`BizType`，`TThostFtdcBizTypeType`。
    pub biz_type: BizType,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 用户事件通知
#[derive(Debug, Clone)]
pub struct TradingNotice {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者范围。CTP：`InvestorRange`，`TThostFtdcInvestorRangeType`。
    pub investor_range: InvestorRange,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 序列系列号。CTP：`SequenceSeries`，`TThostFtdcSequenceSeriesType`。
    pub sequence_series: i16,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 发送时间。CTP：`SendTime`，`TThostFtdcTimeType`。
    pub send_time: TimeText,
    /// 序列号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 消息正文。CTP：`FieldContent`，`TThostFtdcContentType`。
    pub field_content: String,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 用户事件通知信息
#[derive(Debug, Clone)]
pub struct TradingNoticeInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 发送时间。CTP：`SendTime`，`TThostFtdcTimeType`。
    pub send_time: TimeText,
    /// 消息正文。CTP：`FieldContent`，`TThostFtdcContentType`。
    pub field_content: String,
    /// 序列系列号。CTP：`SequenceSeries`，`TThostFtdcSequenceSeriesType`。
    pub sequence_series: i16,
    /// 序列号。CTP：`SequenceNo`，`TThostFtdcSequenceNoType`。
    pub sequence_no: i32,
    /// 投资单元代码。CTP：`InvestUnitID`，`TThostFtdcInvestUnitIDType`。
    pub invest_unit_id: InvestUnitId,
}

/// 转帐银行
#[derive(Debug, Clone)]
pub struct TransferBank {
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分中心代码。CTP：`BankBrchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 银行名称。CTP：`BankName`，`TThostFtdcBankNameType`。
    pub bank_name: String,
    /// 是否活跃。CTP：`IsActive`，`TThostFtdcBoolType`。
    pub is_active: bool,
}

/// 银行资金转期货请求，TradeCode=202001
#[derive(Debug, Clone)]
pub struct TransferBankToFutureReq {
    /// 期货资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
    /// 密码标志。CTP：`FuturePwdFlag`，`TThostFtdcFuturePwdFlagType`。
    pub future_password_flag: FuturePwdFlag,
    /// 密码。CTP：`FutureAccPwd`，`TThostFtdcFutureAccPwdType`。
    pub future_acc_password: String,
    /// 转账金额。CTP：`TradeAmt`，`TThostFtdcMoneyType`。
    pub trade_amount: f64,
    /// 客户手续费。CTP：`CustFee`，`TThostFtdcMoneyType`。
    pub cust_fee: f64,
    /// 币种：RMB-人民币 USD-美圆 HKD-港元。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
}

/// 银行资金转期货请求响应
#[derive(Debug, Clone)]
pub struct TransferBankToFutureRsp {
    /// 响应代码。CTP：`RetCode`，`TThostFtdcRetCodeType`。
    pub ret_code: String,
    /// 响应信息。CTP：`RetInfo`，`TThostFtdcRetInfoType`。
    pub ret_info: String,
    /// 资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
    /// 转帐金额。CTP：`TradeAmt`，`TThostFtdcMoneyType`。
    pub trade_amount: f64,
    /// 应收客户手续费。CTP：`CustFee`，`TThostFtdcMoneyType`。
    pub cust_fee: f64,
    /// 币种。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
}

/// 期货资金转银行请求，TradeCode=202002
#[derive(Debug, Clone)]
pub struct TransferFutureToBankReq {
    /// 期货资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
    /// 密码标志。CTP：`FuturePwdFlag`，`TThostFtdcFuturePwdFlagType`。
    pub future_password_flag: FuturePwdFlag,
    /// 密码。CTP：`FutureAccPwd`，`TThostFtdcFutureAccPwdType`。
    pub future_acc_password: String,
    /// 转账金额。CTP：`TradeAmt`，`TThostFtdcMoneyType`。
    pub trade_amount: f64,
    /// 客户手续费。CTP：`CustFee`，`TThostFtdcMoneyType`。
    pub cust_fee: f64,
    /// 币种：RMB-人民币 USD-美圆 HKD-港元。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
}

/// 期货资金转银行请求响应
#[derive(Debug, Clone)]
pub struct TransferFutureToBankRsp {
    /// 响应代码。CTP：`RetCode`，`TThostFtdcRetCodeType`。
    pub ret_code: String,
    /// 响应信息。CTP：`RetInfo`，`TThostFtdcRetInfoType`。
    pub ret_info: String,
    /// 资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
    /// 转帐金额。CTP：`TradeAmt`，`TThostFtdcMoneyType`。
    pub trade_amount: f64,
    /// 应收客户手续费。CTP：`CustFee`，`TThostFtdcMoneyType`。
    pub cust_fee: f64,
    /// 币种。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
}

/// 银期转帐报文头
#[derive(Debug, Clone)]
pub struct TransferHeader {
    /// 版本号，常量，1.0。CTP：`Version`，`TThostFtdcVersionType`。
    pub version: String,
    /// 交易代码，必填。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 交易日期，必填，格式：yyyymmdd。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间，必填，格式：hhmmss。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 发起方流水号，N/A。CTP：`TradeSerial`，`TThostFtdcTradeSerialType`。
    pub trade_serial: String,
    /// 期货公司代码，必填。CTP：`FutureID`，`TThostFtdcFutureIDType`。
    pub future_id: String,
    /// 银行代码，根据查询银行得到，必填。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分中心代码，根据查询银行得到，必填。CTP：`BankBrchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 操作员，N/A。CTP：`OperNo`，`TThostFtdcOperNoType`。
    pub oper_no: String,
    /// 交易设备类型，N/A。CTP：`DeviceID`，`TThostFtdcDeviceIDType`。
    pub device_id: String,
    /// 记录数，N/A。CTP：`RecordNum`，`TThostFtdcRecordNumType`。
    pub record_count: String,
    /// 会话编号，N/A。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 请求编号，N/A。CTP：`RequestID`，`TThostFtdcRequestIDType`。
    pub request_id: RequestId,
}

/// 查询银行资金请求，TradeCode=204002
#[derive(Debug, Clone)]
pub struct TransferQryBankReq {
    /// 期货资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
    /// 密码标志。CTP：`FuturePwdFlag`，`TThostFtdcFuturePwdFlagType`。
    pub future_password_flag: FuturePwdFlag,
    /// 密码。CTP：`FutureAccPwd`，`TThostFtdcFutureAccPwdType`。
    pub future_acc_password: String,
    /// 币种：RMB-人民币 USD-美圆 HKD-港元。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
}

/// 查询银行资金请求响应
#[derive(Debug, Clone)]
pub struct TransferQryBankRsp {
    /// 响应代码。CTP：`RetCode`，`TThostFtdcRetCodeType`。
    pub ret_code: String,
    /// 响应信息。CTP：`RetInfo`，`TThostFtdcRetInfoType`。
    pub ret_info: String,
    /// 资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
    /// 银行余额。CTP：`TradeAmt`，`TThostFtdcMoneyType`。
    pub trade_amount: f64,
    /// 银行可用余额。CTP：`UseAmt`，`TThostFtdcMoneyType`。
    pub use_amount: f64,
    /// 银行可取余额。CTP：`FetchAmt`，`TThostFtdcMoneyType`。
    pub fetch_amount: f64,
    /// 币种。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
}

/// 查询银行交易明细请求，TradeCode=204999
#[derive(Debug, Clone)]
pub struct TransferQryDetailReq {
    /// 期货资金账户。CTP：`FutureAccount`，`TThostFtdcAccountIDType`。
    pub future_account: AccountId,
}

/// 查询银行交易明细请求响应
#[derive(Debug, Clone)]
pub struct TransferQryDetailRsp {
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcDateType`。
    pub trade_date: DateText,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 交易代码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 期货流水号。CTP：`FutureSerial`，`TThostFtdcTradeSerialNoType`。
    pub future_serial: i32,
    /// 期货公司代码。CTP：`FutureID`，`TThostFtdcFutureIDType`。
    pub future_id: String,
    /// 资金帐号。CTP：`FutureAccount`，`TThostFtdcFutureAccountType`。
    pub future_account: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcTradeSerialNoType`。
    pub bank_serial: i32,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分中心代码。CTP：`BankBrchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 银行账号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 证件号码。CTP：`CertCode`，`TThostFtdcCertCodeType`。
    pub cert_code: String,
    /// 货币代码。CTP：`CurrencyCode`，`TThostFtdcCurrencyCodeType`。
    pub currency_code: String,
    /// 发生金额。CTP：`TxAmount`，`TThostFtdcMoneyType`。
    pub tx_amount: f64,
    /// 有效标志。CTP：`Flag`，`TThostFtdcTransferValidFlagType`。
    pub flag: TransferValidFlag,
}

/// 银期转账交易流水表
#[derive(Debug, Clone)]
pub struct TransferSerial {
    /// 平台流水号。CTP：`PlateSerial`，`TThostFtdcPlateSerialType`。
    pub plate_serial: i32,
    /// 交易发起方日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易日期。CTP：`TradingDay`，`TThostFtdcDateType`。
    pub trading_day: DateText,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 交易代码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 银行编码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构编码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 银行帐号类型。CTP：`BankAccType`，`TThostFtdcBankAccTypeType`。
    pub bank_acc_type: BankAccType,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 期货公司编码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 期货公司帐号类型。CTP：`FutureAccType`，`TThostFtdcFutureAccTypeType`。
    pub future_acc_type: FutureAccType,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 期货公司流水号。CTP：`FutureSerial`，`TThostFtdcFutureSerialType`。
    pub future_serial: i32,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 交易金额。CTP：`TradeAmount`，`TThostFtdcTradeAmountType`。
    pub trade_amount: f64,
    /// 应收客户费用。CTP：`CustFee`，`TThostFtdcCustFeeType`。
    pub cust_fee: f64,
    /// 应收期货公司费用。CTP：`BrokerFee`，`TThostFtdcFutureFeeType`。
    pub broker_fee: f64,
    /// 有效标志。CTP：`AvailabilityFlag`，`TThostFtdcAvailabilityFlagType`。
    pub availability_flag: AvailabilityFlag,
    /// 操作员。CTP：`OperatorCode`，`TThostFtdcOperatorCodeType`。
    pub operator_code: String,
    /// 新银行帐号。CTP：`BankNewAccount`，`TThostFtdcBankAccountType`。
    pub bank_new_account: String,
    /// 错误代码。CTP：`ErrorID`，`TThostFtdcErrorIDType`。
    pub error_id: i32,
    /// 错误信息。CTP：`ErrorMsg`，`TThostFtdcErrorMsgType`。
    pub error_msg: String,
}

/// 客户中心权限豁免
#[derive(Debug, Clone)]
pub struct UserDRIBypass {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
}

/// 用户IP
#[derive(Debug, Clone)]
pub struct UserIP {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
    /// IP地址掩码。CTP：`IPMask`，`TThostFtdcIPAddressType`。
    pub ip_mask: String,
}

/// 用户登出请求
#[derive(Debug, Clone)]
pub struct UserLogout {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
}

/// 用户口令变更
#[derive(Debug, Clone)]
pub struct UserPasswordUpdate {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 原来的口令。CTP：`OldPassword`，`TThostFtdcPasswordType`。
    pub old_password: String,
    /// 新的口令。CTP：`NewPassword`，`TThostFtdcPasswordType`。
    pub new_password: String,
}

/// 来自次席的用户口令变更
#[derive(Debug, Clone)]
pub struct UserPasswordUpdateFromSec {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 原来的口令。CTP：`OldPassword`，`TThostFtdcPasswordType`。
    pub old_password: String,
    /// 新的口令。CTP：`NewPassword`，`TThostFtdcPasswordType`。
    pub new_password: String,
    /// 次席的交易中心代码。CTP：`FromSec`，`TThostFtdcDRIdentityIDType`。
    pub from_sec: i32,
}

/// 用户权限
#[derive(Debug, Clone)]
pub struct UserRight {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 客户权限类型。CTP：`UserRightType`，`TThostFtdcUserRightTypeType`。
    pub user_right_type: UserRightType,
    /// 是否禁止。CTP：`IsForbidden`，`TThostFtdcBoolType`。
    pub is_forbidden: bool,
}

/// 灾备中心交易权限
#[derive(Debug, Clone)]
pub struct UserRightsAssign {
    /// 应用单元代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 交易中心代码。CTP：`DRIdentityID`，`TThostFtdcDRIdentityIDType`。
    pub dr_identity_id: i32,
}

/// 用户会话
#[derive(Debug, Clone)]
pub struct UserSession {
    /// 前置编号。CTP：`FrontID`，`TThostFtdcFrontIDType`。
    pub front_id: FrontId,
    /// 会话编号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 登录日期。CTP：`LoginDate`，`TThostFtdcDateType`。
    pub login_date: DateText,
    /// 登录时间。CTP：`LoginTime`，`TThostFtdcTimeType`。
    pub login_time: TimeText,
    /// 用户端产品信息。CTP：`UserProductInfo`，`TThostFtdcProductInfoType`。
    pub user_product_info: String,
    /// 接口端产品信息。CTP：`InterfaceProductInfo`，`TThostFtdcProductInfoType`。
    pub interface_product_info: String,
    /// 协议信息。CTP：`ProtocolInfo`，`TThostFtdcProtocolInfoType`。
    pub protocol_info: String,
    /// Mac地址。CTP：`MacAddress`，`TThostFtdcMacAddressType`。
    pub mac_address: String,
    /// 登录备注。CTP：`LoginRemark`，`TThostFtdcLoginRemarkType`。
    pub login_remark: String,
    /// IP地址。CTP：`IPAddress`，`TThostFtdcIPAddressType`。
    pub ip_address: String,
}

/// 用户系统信息
#[derive(Debug, Clone)]
pub struct UserSystemInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 用户端系统内部信息长度。CTP：`ClientSystemInfoLen`，`TThostFtdcSystemInfoLenType`。
    pub client_system_info_len: i32,
    /// 用户端系统内部信息。CTP：`ClientSystemInfo`，`TThostFtdcClientSystemInfoType`。
    pub client_system_info: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 登录成功时间。CTP：`ClientLoginTime`，`TThostFtdcTimeType`。
    pub client_login_time: TimeText,
    /// App代码。CTP：`ClientAppID`，`TThostFtdcAppIDType`。
    pub client_app_id: String,
    /// 用户公网IP。CTP：`ClientPublicIP`，`TThostFtdcIPAddressType`。
    pub client_public_ip: String,
    /// 客户登录备注2。CTP：`ClientLoginRemark`，`TThostFtdcClientLoginRemarkType`。
    pub client_login_remark: String,
    /// 客户终端的MAC等标识。CTP：`MAC`，`TThostFtdcDeviceTagType`。
    pub mac: String,
}

/// 验证客户信息
#[derive(Debug, Clone)]
pub struct VerifyCustInfo {
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 验证期货资金密码
#[derive(Debug, Clone)]
pub struct VerifyFuturePassword {
    /// 业务功能码。CTP：`TradeCode`，`TThostFtdcTradeCodeType`。
    pub trade_code: String,
    /// 银行代码。CTP：`BankID`，`TThostFtdcBankIDType`。
    pub bank_id: String,
    /// 银行分支机构代码。CTP：`BankBranchID`，`TThostFtdcBankBrchIDType`。
    pub bank_branch_id: String,
    /// 期商代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 期商分支机构代码。CTP：`BrokerBranchID`，`TThostFtdcFutureBranchIDType`。
    pub broker_branch_id: String,
    /// 交易日期。CTP：`TradeDate`，`TThostFtdcTradeDateType`。
    pub trade_date: String,
    /// 交易时间。CTP：`TradeTime`，`TThostFtdcTradeTimeType`。
    pub trade_time: String,
    /// 银行流水号。CTP：`BankSerial`，`TThostFtdcBankSerialType`。
    pub bank_serial: String,
    /// 交易系统日期。CTP：`TradingDay`，`TThostFtdcTradeDateType`。
    pub trading_day: String,
    /// 银期平台消息流水号。CTP：`PlateSerial`，`TThostFtdcSerialType`。
    pub plate_serial: i32,
    /// 最后分片标志。CTP：`LastFragment`，`TThostFtdcLastFragmentType`。
    pub last_fragment: LastFragment,
    /// 会话号。CTP：`SessionID`，`TThostFtdcSessionIDType`。
    pub session_id: SessionId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 银行帐号。CTP：`BankAccount`，`TThostFtdcBankAccountType`。
    pub bank_account: String,
    /// 银行密码。CTP：`BankPassWord`，`TThostFtdcPasswordType`。
    pub bank_pass_word: String,
    /// 安装编号。CTP：`InstallID`，`TThostFtdcInstallIDType`。
    pub install_id: i32,
    /// 交易ID。CTP：`TID`，`TThostFtdcTIDType`。
    pub tid: i32,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
}

/// 验证期货资金密码和客户信息
#[derive(Debug, Clone)]
pub struct VerifyFuturePasswordAndCustInfo {
    /// 客户姓名。CTP：`CustomerName`，`TThostFtdcIndividualNameType`。
    pub customer_name: String,
    /// 证件类型。CTP：`IdCardType`，`TThostFtdcIdCardTypeType`。
    pub id_card_type: IdCardType,
    /// 证件号码。CTP：`IdentifiedCardNo`，`TThostFtdcIdentifiedCardNoType`。
    pub identified_card_no: String,
    /// 客户类型。CTP：`CustType`，`TThostFtdcCustTypeType`。
    pub cust_type: CustType,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 期货密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
    /// 币种代码。CTP：`CurrencyID`，`TThostFtdcCurrencyIDType`。
    pub currency_id: String,
    /// 长客户姓名。CTP：`LongCustomerName`，`TThostFtdcLongIndividualNameType`。
    pub long_customer_name: String,
}

/// 校验投资者密码
#[derive(Debug, Clone)]
pub struct VerifyInvestorPassword {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者代码。CTP：`InvestorID`，`TThostFtdcInvestorIDType`。
    pub investor_id: InvestorId,
    /// 密码。CTP：`Password`，`TThostFtdcPasswordType`。
    pub password: String,
}

/// 微信小程序等用户系统信息
#[derive(Debug, Clone)]
pub struct WechatUserSystemInfo {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 用户代码。CTP：`UserID`，`TThostFtdcUserIDType`。
    pub user_id: UserId,
    /// 微信小程序等用户端系统内部信息长度。CTP：`WechatCltSysInfoLen`，`TThostFtdcSystemInfoLenType`。
    pub wechat_clt_system_info_len: i32,
    /// 微信小程序等用户端系统内部信息。CTP：`WechatCltSysInfo`，`TThostFtdcClientSystemInfoType`。
    pub wechat_clt_system_info: String,
    /// 终端IP端口。CTP：`ClientIPPort`，`TThostFtdcIPPortType`。
    pub client_ip_port: i32,
    /// 登录成功时间。CTP：`ClientLoginTime`，`TThostFtdcTimeType`。
    pub client_login_time: TimeText,
    /// App代码。CTP：`ClientAppID`，`TThostFtdcAppIDType`。
    pub client_app_id: String,
    /// 用户公网IP。CTP：`ClientPublicIP`，`TThostFtdcIPAddressType`。
    pub client_public_ip: String,
    /// 客户登录备注2。CTP：`ClientLoginRemark`，`TThostFtdcClientLoginRemarkType`。
    pub client_login_remark: String,
}

/// 可提控制参数
#[derive(Debug, Clone)]
pub struct WithDrawParam {
    /// 经纪公司代码。CTP：`BrokerID`，`TThostFtdcBrokerIDType`。
    pub broker_id: BrokerId,
    /// 投资者帐号。CTP：`AccountID`，`TThostFtdcAccountIDType`。
    pub account_id: AccountId,
    /// 参数代码。CTP：`WithDrawParamID`，`TThostFtdcWithDrawParamIDType`。
    pub with_draw_parameter_id: WithDrawParamID,
    /// 参数代码值。CTP：`WithDrawParamValue`，`TThostFtdcWithDrawParamValueType`。
    pub with_draw_parameter_value: String,
}
