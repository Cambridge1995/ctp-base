use crate::session::trader::TraderSessionError;
use crate::structs::{
    CancelOffsetSetting, CombPromotionParam, HedgeCfm, HedgeCfmAction, InputHedgeCfm,
    InputHedgeCfmAction, InputOffsetSetting, InputSpdApply, InputSpdApplyAction, InstrTradeLimit,
    OffsetSetting, QryClassifiedInstrument, QryCombPromotionParam, QryHedgeCfm, QryInstrTradeLimit,
    QryOffsetSetting, QrySpdApply, SpdApply, SpdApplyAction,
};
use crate::structs::{
    CombInstrumentGuard, ExchangeMarginRate, ExchangeMarginRateAdjust, ExchangeRate,
    InstrumentCommissionRate, InstrumentMarginRate, InstrumentOrderCommRate,
    InvestorCommodityGroupSPMMMargin, InvestorCommoditySPMMMargin, InvestorInfoCommRec,
    InvestorPortfMarginRatio, InvestorPortfSetting, InvestorProdRCAMSMargin,
    InvestorProdRULEMargin, InvestorProdSPBMDetail, InvestorProductGroupMargin,
    MMInstrumentCommissionRate, MMOptionInstrCommRate, OptionInstrCommRate, OptionInstrTradeCost,
    ProductExchRate, ProductGroup, QryCombInstrumentGuard, QryExchangeMarginRate,
    QryExchangeMarginRateAdjust, QryExchangeRate, QryInstrumentCommissionRate,
    QryInstrumentMarginRate, QryInstrumentOrderCommRate, QryInvestorCommodityGroupSPMMMargin,
    QryInvestorCommoditySPMMMargin, QryInvestorInfoCommRec, QryInvestorPortfMarginRatio,
    QryInvestorPortfSetting, QryInvestorProdRCAMSMargin, QryInvestorProdRULEMargin,
    QryInvestorProdSPBMDetail, QryInvestorProductGroupMargin, QryMMInstrumentCommissionRate,
    QryMMOptionInstrCommRate, QryMaxOrderVolume, QryMaxOrderVolumeWithPrice,
    QryOptionInstrCommRate, QryOptionInstrTradeCost, QryProductExchRate, QryProductGroup,
    QryRCAMSCombProductInfo, QryRCAMSInstrParameter, QryRCAMSInterParameter,
    QryRCAMSIntraParameter, QryRCAMSInvestorCombPosition, QryRCAMSShortOptAdjustParam,
    QryRULEInstrParameter, QryRULEInterParameter, QryRULEIntraParameter,
    QryRiskSettleInvestPosition, QryRiskSettleProductStatus, QrySPBMAddOnInterParameter,
    QrySPBMFutureParameter, QrySPBMInterParameter, QrySPBMIntraParameter, QrySPBMInvestorPortfDef,
    QrySPBMOptionParameter, QrySPBMPortfDefinition, QrySPMMInstParam, QrySPMMProductParam,
    RCAMSCombProductInfo, RCAMSInstrParameter, RCAMSInterParameter, RCAMSIntraParameter,
    RCAMSInvestorCombPosition, RCAMSShortOptAdjustParam, RULEInstrParameter, RULEInterParameter,
    RULEIntraParameter, RiskSettleInvestPosition, RiskSettleProductStatus, SPBMAddOnInterParameter,
    SPBMFutureParameter, SPBMInterParameter, SPBMIntraParameter, SPBMInvestorPortfDef,
    SPBMOptionParameter, SPBMPortfDefinition, SPMMInstParam, SPMMProductParam,
};
use crate::{
    AccountRegister, BatchOrderAction, CancelAccount, ChangeAccount, CombAction, CombLeg,
    ContractBank, DepthMarketData, DisconnectReason, ErrorConditionalOrder, Exchange, ExecOrder,
    ExecOrderAction, ForQuote, ForQuoteRsp, InputBatchOrderAction, InputCombAction, InputExecOrder,
    InputExecOrderAction, InputForQuote, InputOptionSelfClose, InputOptionSelfCloseAction,
    InputOrder, InputOrderAction, InputQuote, InputQuoteAction, Instrument, InstrumentStatus,
    Investor, InvestorPosition, InvestorPositionCombineDetail, InvestorPositionDetail,
    NotifyQueryAccount, OpenAccount, OptionSelfClose, OptionSelfCloseAction, Order, OrderAction,
    ParkedOrder, ParkedOrderAction, Product, QryAccountRegister, QryCombAction, QryCombLeg,
    QryContractBank, QryDepthMarketData, QryExchange, QryExecOrder, QryForQuote, QryInstrument,
    QryInvestor, QryInvestorPosition, QryInvestorPositionCombineDetail, QryInvestorPositionDetail,
    QryOptionSelfClose, QryOrder, QryParkedOrder, QryParkedOrderAction, QryProduct, QryQuote,
    QrySettlementInfo, QryTrade, QryTradingAccount, QryTransferBank, QryTransferSerial, Quote,
    QuoteAction, RemoveParkedOrder, RemoveParkedOrderAction, ReqQueryAccount, ReqRepeal,
    ReqTransfer, RspRepeal, RspTransfer, SettlementInfo, Trade, TradingAccount, TransferBank,
    TransferSerial,
};
use std::fmt;
use std::sync::mpsc::SyncSender;
use std::time::Duration;

/// 首次连接与登录的总超时。
pub(crate) const LOGIN_TIMEOUT: Duration = Duration::from_secs(10);
/// 事件与输入队列的默认容量（[`crate::MdConfig::queue_capacity`] 为 0 时使用）。
pub const DEFAULT_QUEUE_CAPACITY: usize = 4096;
/// 行情会话登录后的事件流（owned，可直接跨线程转移）。
#[derive(Clone, Debug)]
pub enum MdSessionEvent {
    /// 连接中断；内部 loop 会自动等待 CTP 重连并重登录，是否介入由调用方决定。
    Disconnected {
        /// 断线原因。
        reason: DisconnectReason,
    },
    /// 断线后自动重连、重登录、重订阅全部完成。
    Recovered {
        /// 新会话的交易日（`yyyymmdd`）。
        trading_day: String,
    },
    /// 合约订阅成功。
    Subscribed {
        /// 合约代码。
        instrument: String,
    },
    /// 合约退订成功。
    Unsubscribed {
        /// 合约代码。
        instrument: String,
    },
    /// 询价订阅成功（期权做市相关）。
    QuoteSubscribed {
        /// 合约代码。
        instrument: String,
    },
    /// 询价退订成功。
    QuoteUnsubscribed {
        /// 合约代码。
        instrument: String,
    },
    /// 请求级错误（如流控拒绝）。坏消息不应无声无息，一律上报。
    Error {
        /// CTP 错误码。
        error_id: i32,
        /// CTP 错误描述。
        message: String,
    },
    /// owned 转换失败：CTP 发来的数据损坏被丢弃。坏消息不应无声无息，一律上报。
    ConversionFailed {
        /// 失败描述（字段与原因）。
        message: String,
    },
    /// 深度行情。SPI 直送，不经过 loop。
    Tick(Box<DepthMarketData>),
    /// 询价回报（期权做市相关）。SPI 直送，不经过 loop。
    Quote(Box<ForQuoteRsp>),
}

/// SPI 回调投递给 loop 的控制事件。
pub enum MdEvent {
    Connected,
    Disconnected(DisconnectReason),
    /// 登录成功。行情端的 FrontID/SessionID 恒为 0（会话标识是交易端概念），只携带交易日。
    LoginOk {
        trading_day: String,
    },
    LoginFailed {
        error_id: i32,
        message: String,
    },
    Subscribed(String),
    Unsubscribed(String),
    QuoteSubscribed(String),
    QuoteUnsubscribed(String),
    Error {
        error_id: i32,
        message: String,
    },
    ConversionFailed(String),
}

/// 调用方发给 loop 的命令。
pub enum Command {
    Subscribe(Vec<String>),
    Unsubscribe(Vec<String>),
    SubscribeQuote(Vec<String>),
    UnsubscribeQuote(Vec<String>),
    Shutdown,
}

/// loop 的唯一输入：SPI 控制事件与调用方命令汇入同一队列，loop 串行处理、阻塞等待。
pub enum Input {
    Spi(MdEvent),
    Command(Command),
}

/// 行情托管会话错误。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MdSessionError {
    /// 创建行情 API 失败。
    Create(String),
    /// CTP 请求发送失败。
    Request(String),
    /// 登录被前置拒绝（错误码 + 描述）。
    LoginRejected(i32, String),
    /// 连接或登录超时。
    Timeout,
    /// 事件通道已断开。
    ChannelClosed,
    /// 调度线程已停止。
    Stopped,
    /// 命令队列已满。
    QueueFull,
}

impl fmt::Display for MdSessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(message) => write!(f, "创建行情 API 失败：{message}"),
            Self::Request(message) => write!(f, "CTP 请求发送失败：{message}"),
            Self::LoginRejected(error_id, message) => {
                write!(f, "登录被前置拒绝：[{error_id}] {message}")
            }
            Self::Timeout => write!(f, "连接或登录超时（{} 秒）", LOGIN_TIMEOUT.as_secs()),
            Self::ChannelClosed => write!(f, "事件通道已断开"),
            Self::Stopped => write!(f, "行情调度线程已停止"),
            Self::QueueFull => write!(f, "命令队列已满，请稍后重试"),
        }
    }
}

impl std::error::Error for MdSessionError {}

/// 交易会话登录后的事件流（owned，可直接跨线程转移）。
#[derive(Clone, Debug)]
pub enum TraderSessionEvent {
    /// 认证、登录、结算确认全部完成，可以交易。
    Ready {
        /// 前置编号（报单对账标识之一）。
        front_id: i32,
        /// 会话编号（报单对账标识之一）。
        session_id: i32,
        /// 交易日（`yyyymmdd`）。
        trading_day: String,
    },
    /// 连接中断；内部 loop 会自动等待 CTP 重连并重走认证链，是否介入由调用方决定。
    Disconnected {
        /// 断线原因。
        reason: DisconnectReason,
    },
    /// 断线后自动重连、重认证、重登录、重确认结算全部完成。
    Recovered {
        /// 新会话的前置编号。
        front_id: i32,
        /// 新会话的会话编号。
        session_id: i32,
        /// 新会话的交易日。
        trading_day: String,
    },
    /// 登出回执（`OnRspUserLogout`）。到达场景：主动 `reconnect()` 或柜台强制登出。
    /// 交易端登出语义是断开重连（SessionID 重置），随后会收到 `Disconnected`，
    /// 链路重建完成后收到 `Recovered`，见FAQ 的 logout/重连条目。
    Logout,
    /// 请求级或流程级错误（认证/登录/结算确认被拒、流控拒绝等）。坏消息不应无声无息。
    Error {
        /// CTP 错误码；-1 表示本地发送失败。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// owned 转换失败：CTP 发来的数据损坏被丢弃。坏消息不应无声无息，一律上报。
    ConversionFailed {
        /// 失败描述（字段与原因）。
        message: String,
    },
    /// 报单请求回执（`OnRspOrderInsert`）。
    ///
    /// `error_id` 为 0 表示前置已受理（不代表交易所接受）；非 0 即 CTP 拒单路径之一。
    /// 另一条拒单路径见 [`TraderSessionEvent::OrderInsertError`]，
    /// 交易所拒单（被动撤单）见 [`TraderSessionEvent::OrderUpdate`]。
    OrderInsertResponse {
        /// 请求回声标签（对账主键是 FrontID+SessionID+OrderRef，不是它）。
        request_id: i32,
        /// 被受理/拒绝的报单；错误时前置可能不返回报单内容。
        order: Option<Box<InputOrder>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 撤单请求回执（`OnRspOrderAction`）：`error_id` 为 0 表示前置已受理。
    OrderActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的撤单指令。
        action: Option<Box<InputOrderAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 批量撤单请求回执（`OnRspBatchOrderAction`）。
    BatchOrderActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的批量撤单指令。
        action: Option<Box<InputBatchOrderAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 报单错误回报（`OnErrRtnOrderInsert`）：CTP 拒单的回报路径。
    ///
    /// 回报类回调不带 request_id，按 OrderRef 对账。
    OrderInsertError {
        /// 被拒绝的报单。
        order: Option<Box<InputOrder>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 撤单错误回报（`OnErrRtnOrderAction`）：撤单被交易所拒绝（如已全成交）。
    OrderActionError {
        /// 被拒绝的撤单指令（交易所回送的处理后形态）。
        action: Option<Box<OrderAction>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 批量撤单错误回报（`OnErrRtnBatchOrderAction`）。
    BatchOrderActionError {
        /// 被拒绝的批量撤单指令（交易所回送的处理后形态）。
        action: Option<Box<BatchOrderAction>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 条件单错误回报（`OnErrRtnConditionalOrder`）；错误码在结构体的 `error_id` 字段中。
    ConditionalOrderError {
        /// 出错条件单。
        order: Option<Box<ErrorConditionalOrder>>,
    },
    /// 报单状态回报（`OnRtnOrder`）。
    ///
    /// 含交易所拒单（被动撤单：`order_status` 为 `Canceled` 且 `status_msg` 带原因）。
    /// 对账主键：FrontID + SessionID + OrderRef，结构体内自带。
    OrderUpdate(Box<Order>),
    /// 成交回报（`OnRtnTrade`）。
    Execution(Box<Trade>),
    /// 查询被异步拒绝，携带请求回声标签，供调用方结束对应批次。
    QueryError {
        /// 请求回声标签；原样保留，调用方据此关联在途批次。
        request_id: i32,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 查询响应的一行，收到后立即公开，不等待批次结束。
    ///
    /// 调用方自行按 request_id 聚合；`row` 为 None 表示无载荷，
    /// 即使 `is_last` 为 true 也照常发送，便于调用方结束自己的批次。
    QueryRow {
        /// 请求回声标签；调用方查询方法的返回值，原样保留。
        request_id: i32,
        /// 是否该次查询的最后一行。
        is_last: bool,
        /// CTP 错误码；0 表示成功。
        error_id: i32,
        /// 错误描述。
        message: String,
        /// 单行载荷；变体标识具体查询接口。
        row: Option<QueryRow>,
    },
    /// 合约状态推送（`OnRtnInstrumentStatus`，公共流，非查询响应）。
    InstrumentStatus(Box<InstrumentStatus>),

    // ---- 批次四：高级指令（行权/期权自对冲/组合/报价询价/预埋单）----
    /// 行权申报回执（`OnRspExecOrderInsert`）：`error_id` 为 0 表示前置已受理。
    ExecOrderInsertResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的行权申报。
        order: Option<Box<InputExecOrder>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 行权操作（撤销）回执（`OnRspExecOrderAction`）。
    ExecOrderActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的行权操作。
        action: Option<Box<InputExecOrderAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 行权申报错误回报（`OnErrRtnExecOrderInsert`），按 ExecOrderRef 对账。
    ExecOrderInsertError {
        /// 被拒绝的行权申报。
        order: Option<Box<InputExecOrder>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 行权操作错误回报（`OnErrRtnExecOrderAction`）。
    ExecOrderActionError {
        /// 被拒绝的行权操作（交易所回送的处理后形态）。
        action: Option<Box<ExecOrderAction>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 行权状态回报（`OnRtnExecOrder`）。
    ExecOrderReport(Box<ExecOrder>),
    /// 期权自对冲申报回执（`OnRspOptionSelfCloseInsert`）。
    OptionSelfCloseInsertResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的自对冲申报。
        order: Option<Box<InputOptionSelfClose>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期权自对冲操作回执（`OnRspOptionSelfCloseAction`）。
    OptionSelfCloseActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的自对冲操作。
        action: Option<Box<InputOptionSelfCloseAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期权自对冲申报错误回报（`OnErrRtnOptionSelfCloseInsert`）。
    OptionSelfCloseInsertError {
        /// 被拒绝的自对冲申报。
        order: Option<Box<InputOptionSelfClose>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期权自对冲操作错误回报（`OnErrRtnOptionSelfCloseAction`）。
    OptionSelfCloseActionError {
        /// 被拒绝的自对冲操作（交易所回送的处理后形态）。
        action: Option<Box<OptionSelfCloseAction>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期权自对冲状态回报（`OnRtnOptionSelfClose`）。
    OptionSelfCloseReport(Box<OptionSelfClose>),
    /// 组合申请回执（`OnRspCombActionInsert`）。
    CombActionInsertResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的组合申请。
        action: Option<Box<InputCombAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 组合申请错误回报（`OnErrRtnCombActionInsert`）。
    CombActionInsertError {
        /// 被拒绝的组合申请。
        action: Option<Box<InputCombAction>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 组合状态回报（`OnRtnCombAction`）。
    CombActionReport(Box<CombAction>),
    /// 报价申报回执（`OnRspQuoteInsert`）。
    QuoteInsertResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的报价。
        quote: Option<Box<InputQuote>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 报价操作回执（`OnRspQuoteAction`）。
    QuoteActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的报价操作。
        action: Option<Box<InputQuoteAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 报价申报错误回报（`OnErrRtnQuoteInsert`）。
    QuoteInsertError {
        /// 被拒绝的报价。
        quote: Option<Box<InputQuote>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 报价操作错误回报（`OnErrRtnQuoteAction`）。
    QuoteActionError {
        /// 被拒绝的报价操作（交易所回送的处理后形态）。
        action: Option<Box<QuoteAction>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 报价状态回报（`OnRtnQuote`）。
    QuoteReport(Box<Quote>),
    /// 询价申报回执（`OnRspForQuoteInsert`）。
    ForQuoteInsertResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的询价。
        for_quote: Option<Box<InputForQuote>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 询价申报错误回报（`OnErrRtnForQuoteInsert`）。
    ForQuoteInsertError {
        /// 被拒绝的询价。
        for_quote: Option<Box<InputForQuote>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 询价回报（`OnRtnForQuoteRsp`，期权做市相关）。
    ForQuoteReport(Box<ForQuoteRsp>),
    /// 预埋单申报回执（`OnRspParkedOrderInsert`）。
    ParkedOrderInsertResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的预埋单。
        order: Option<Box<ParkedOrder>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 预埋撤单回执（`OnRspParkedOrderAction`）。
    ParkedActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的预埋撤单。
        action: Option<Box<ParkedOrderAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 删除预埋单回执（`OnRspRemoveParkedOrder`）。
    RemoveParkedOrderResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的删除请求。
        removed: Option<Box<RemoveParkedOrder>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 删除预埋撤单回执（`OnRspRemoveParkedOrderAction`）。
    RemoveParkedOrderActionResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的删除请求。
        removed: Option<Box<RemoveParkedOrderAction>>,
        /// CTP 错误码；0 表示已受理。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 预埋单状态回报（`OnRtnParkedOrder`）。
    ParkedOrderReport(Box<ParkedOrder>),
    /// 预埋撤单状态回报（`OnRtnParkedOrderAction`）。
    ParkedOrderActionReport(Box<ParkedOrderAction>),

    // ---- 批次五：银期转账 ----
    /// 期商发起入金（银行→期货）应答（`OnRspFromBankToFutureByFuture`）。
    BankToFutureResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的转账请求。
        transfer: Option<Box<ReqTransfer>>,
        /// CTP 错误码；0 表示已受理（处理结果看 `BankToFutureByFutureReport`）。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期商发起出金（期货→银行）应答（`OnRspFromFutureToBankByFuture`）。
    FutureToBankResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的转账请求。
        transfer: Option<Box<ReqTransfer>>,
        /// CTP 错误码；0 表示已受理（处理结果看 `FutureToBankByFutureReport`）。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 查询银行余额应答（`OnRspQueryBankAccountMoneyByFuture`）。
    QueryBankAccountMoneyResponse {
        /// 请求回声标签。
        request_id: i32,
        /// 被受理/拒绝的查询请求。
        query: Option<Box<ReqQueryAccount>>,
        /// CTP 错误码；0 表示已受理（结果看 `QueryBankBalanceReport`）。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期商入金错误回报（`OnErrRtnBankToFutureByFuture`）。
    BankToFutureError {
        /// 被拒绝的转账请求。
        transfer: Option<Box<ReqTransfer>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 期商出金错误回报（`OnErrRtnFutureToBankByFuture`）。
    FutureToBankError {
        /// 被拒绝的转账请求。
        transfer: Option<Box<ReqTransfer>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 手工冲正入金错误回报（`OnErrRtnRepealBankToFutureByFutureManual`）。
    RepealBankToFutureError {
        /// 被拒绝的冲正请求。
        repeal: Option<Box<ReqRepeal>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 手工冲正出金错误回报（`OnErrRtnRepealFutureToBankByFutureManual`）。
    RepealFutureToBankError {
        /// 被拒绝的冲正请求。
        repeal: Option<Box<ReqRepeal>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 查询银行余额错误回报（`OnErrRtnQueryBankBalanceByFuture`）。
    QueryBankBalanceError {
        /// 被拒绝的查询请求。
        query: Option<Box<ReqQueryAccount>>,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    /// 银行端发起入金结果回报（`OnRtnFromBankToFutureByBank`）。
    BankToFutureByBankReport(Box<RspTransfer>),
    /// 银行端发起出金结果回报（`OnRtnFromFutureToBankByBank`）。
    FutureToBankByBankReport(Box<RspTransfer>),
    /// 银行端入金冲正回报（`OnRtnRepealFromBankToFutureByBank`）。
    RepealBankToFutureByBankReport(Box<RspRepeal>),
    /// 银行端出金冲正回报（`OnRtnRepealFromFutureToBankByBank`）。
    RepealFutureToBankByBankReport(Box<RspRepeal>),
    /// 期商入金处理结果回报（`OnRtnFromBankToFutureByFuture`）。
    BankToFutureByFutureReport(Box<RspTransfer>),
    /// 期商出金处理结果回报（`OnRtnFutureToBankByFuture`）。
    FutureToBankByFutureReport(Box<RspTransfer>),
    /// 手工冲正入金回报（`OnRtnRepealFromBankToFutureByFutureManual`）。
    RepealBankToFutureManualReport(Box<RspRepeal>),
    /// 手工冲正出金回报（`OnRtnRepealFromFutureToBankByFutureManual`）。
    RepealFutureToBankManualReport(Box<RspRepeal>),
    /// 期商入金冲正回报（`OnRtnRepealFromBankToFutureByFuture`）。
    RepealBankToFutureReport(Box<RspRepeal>),
    /// 期商出金冲正回报（`OnRtnRepealFromFutureToBankByFuture`）。
    RepealFutureToBankReport(Box<RspRepeal>),
    /// 银行余额回报（`OnRtnQueryBankBalanceByFuture`）。
    QueryBankBalanceReport(Box<NotifyQueryAccount>),
    /// 银行端开户回报（`OnRtnOpenAccountByBank`）。
    OpenAccountReport(Box<OpenAccount>),
    /// 银行端销户回报（`OnRtnCancelAccountByBank`）。
    CancelAccountReport(Box<CancelAccount>),
    /// 银行端变更账户回报（`OnRtnChangeAccountByBank`）。
    ChangeAccountReport(Box<ChangeAccount>),

    // ---- 批次七：对冲设置、SPD、套保确认 ----
    /// 对冲设置直接应答（`OnRspOffsetSetting`）；受理不等于状态通知已完成。
    SetOffsetResponse {
        request_id: i32,
        setting: Option<Box<InputOffsetSetting>>,
        error_id: i32,
        message: String,
    },
    /// 对冲设置撤销直接应答（`OnRspCancelOffsetSetting`）。
    CancelOffsetSettingResponse {
        request_id: i32,
        setting: Option<Box<InputOffsetSetting>>,
        error_id: i32,
        message: String,
    },
    /// 对冲设置状态通知（`OnRtnOffsetSetting`）。
    OffsetSettingReport(Box<OffsetSetting>),
    /// 对冲设置错误回报（`OnErrRtnOffsetSetting`）；无外层 request_id。
    OffsetSettingError {
        setting: Option<Box<InputOffsetSetting>>,
        error_id: i32,
        message: String,
    },
    /// 对冲设置撤销错误回报（`OnErrRtnCancelOffsetSetting`）。
    CancelOffsetSettingError {
        setting: Option<Box<CancelOffsetSetting>>,
        error_id: i32,
        message: String,
    },
    /// SPD 套利确认直接应答（`OnRspSpdApply`）；后续状态看 `SpdApplyReport`。
    SpdApplyResponse {
        request_id: i32,
        apply: Option<Box<InputSpdApply>>,
        error_id: i32,
        message: String,
    },
    /// SPD 套利确认撤销直接应答（`OnRspSpdApplyAction`）。
    SpdApplyActionResponse {
        request_id: i32,
        action: Option<Box<InputSpdApplyAction>>,
        error_id: i32,
        message: String,
    },
    /// SPD 套利确认状态通知（`OnRtnSpdApply`）。
    SpdApplyReport(Box<SpdApply>),
    /// SPD 套利确认错误回报（`OnErrRtnSpdApply`）；按原申请标识关联。
    SpdApplyError {
        apply: Option<Box<InputSpdApply>>,
        error_id: i32,
        message: String,
    },
    /// SPD 套利确认撤销错误回报（`OnErrRtnSpdApplyAction`）。
    SpdApplyActionError {
        action: Option<Box<SpdApplyAction>>,
        error_id: i32,
        message: String,
    },
    /// 套保确认直接应答（`OnRspHedgeCfm`）；后续状态看 `HedgeCfmReport`。
    HedgeCfmResponse {
        request_id: i32,
        apply: Option<Box<InputHedgeCfm>>,
        error_id: i32,
        message: String,
    },
    /// 套保确认撤销直接应答（`OnRspHedgeCfmAction`）。
    HedgeCfmActionResponse {
        request_id: i32,
        action: Option<Box<InputHedgeCfmAction>>,
        error_id: i32,
        message: String,
    },
    /// 套保确认状态通知（`OnRtnHedgeCfm`）。
    HedgeCfmReport(Box<HedgeCfm>),
    /// 套保确认错误回报（`OnErrRtnHedgeCfm`）；按原申请标识关联。
    HedgeCfmError {
        apply: Option<Box<InputHedgeCfm>>,
        error_id: i32,
        message: String,
    },
    /// 套保确认撤销错误回报（`OnErrRtnHedgeCfmAction`）。
    HedgeCfmActionError {
        action: Option<Box<HedgeCfmAction>>,
        error_id: i32,
        message: String,
    },
}

/// 查询批次聚合完成的行集（批次三：核心查询）。
#[derive(Clone, Debug)]
pub enum QueryRows {
    /// 报单查询结果。
    Orders(Vec<Order>),
    /// 成交查询结果。
    Trades(Vec<Trade>),
    /// 持仓查询结果。
    Positions(Vec<InvestorPosition>),
    /// 持仓明细查询结果。
    PositionDetails(Vec<InvestorPositionDetail>),
    /// 资金账户查询结果。
    TradingAccounts(Vec<TradingAccount>),
    /// 合约查询结果。
    Instruments(Vec<Instrument>),
    /// 交易所查询结果。
    Exchanges(Vec<Exchange>),
    /// 产品查询结果。
    Products(Vec<Product>),
    /// 深度行情查询结果。
    DepthMarketData(Vec<DepthMarketData>),
    /// 组合持仓明细查询结果。
    PositionCombineDetails(Vec<InvestorPositionCombineDetail>),
    /// 组合合约腿查询结果。
    CombLegs(Vec<CombLeg>),
    /// 投资者信息查询结果。
    Investors(Vec<Investor>),
    /// 结算单查询结果。
    SettlementInfo(Vec<SettlementInfo>),
    /// 行权查询结果。
    ExecOrders(Vec<ExecOrder>),
    /// 期权自对冲查询结果。
    OptionSelfCloses(Vec<OptionSelfClose>),
    /// 组合申请查询结果。
    CombActions(Vec<CombAction>),
    /// 报价查询结果。
    Quotes(Vec<Quote>),
    /// 询价查询结果。
    ForQuotes(Vec<ForQuote>),
    /// 预埋单查询结果。
    ParkedOrders(Vec<ParkedOrder>),
    /// 预埋撤单查询结果。
    ParkedOrderActions(Vec<ParkedOrderAction>),
    /// 转帐银行查询结果。
    TransferBanks(Vec<TransferBank>),
    /// 转账流水查询结果。
    TransferSerials(Vec<TransferSerial>),
    /// 银期账户登记查询结果。
    AccountRegisters(Vec<AccountRegister>),
    /// 签约银行查询结果。
    ContractBanks(Vec<ContractBank>),
    /// 最大报单数量查询结果。
    MaxOrderVolumes(Vec<QryMaxOrderVolume>),
    /// 合约保证金率查询结果。
    InstrumentMarginRates(Vec<InstrumentMarginRate>),
    /// 合约手续费率查询结果。
    InstrumentCommissionRates(Vec<InstrumentCommissionRate>),
    /// 做市商合约手续费率查询结果。
    MMInstrumentCommissionRates(Vec<MMInstrumentCommissionRate>),
    /// 做市商期权合约手续费率查询结果。
    MMOptionInstrCommRates(Vec<MMOptionInstrCommRate>),
    /// 报单手续费率查询结果。
    InstrumentOrderCommRates(Vec<InstrumentOrderCommRate>),
    /// 期权交易成本查询结果。
    OptionInstrTradeCosts(Vec<OptionInstrTradeCost>),
    /// 期权合约手续费率查询结果。
    OptionInstrCommRates(Vec<OptionInstrCommRate>),
    /// 交易所保证金率查询结果。
    ExchangeMarginRates(Vec<ExchangeMarginRate>),
    /// 交易所调整保证金率查询结果。
    ExchangeMarginRateAdjusts(Vec<ExchangeMarginRateAdjust>),
    /// 汇率查询结果。
    ExchangeRates(Vec<ExchangeRate>),
    /// 产品报价汇率查询结果。
    ProductExchRates(Vec<ProductExchRate>),
    /// 产品组查询结果。
    ProductGroups(Vec<ProductGroup>),
    /// 投资者品种/跨品种保证金查询结果。
    InvestorProductGroupMargins(Vec<InvestorProductGroupMargin>),
    /// 组合合约安全系数查询结果。
    CombInstrumentGuards(Vec<CombInstrumentGuard>),
    /// 根据价格查询最大报单数量的结果。
    MaxOrderVolumeWithPrices(Vec<QryMaxOrderVolumeWithPrice>),
    /// SPBM 期货合约参数查询结果。
    SPBMFutureParameters(Vec<SPBMFutureParameter>),
    /// SPBM 期权合约参数查询结果。
    SPBMOptionParameters(Vec<SPBMOptionParameter>),
    /// SPBM 品种内参数查询结果。
    SPBMIntraParameters(Vec<SPBMIntraParameter>),
    /// SPBM 跨品种参数查询结果。
    SPBMInterParameters(Vec<SPBMInterParameter>),
    /// SPBM 组合保证金套餐查询结果。
    SPBMPortfDefinitions(Vec<SPBMPortfDefinition>),
    /// 投资者 SPBM 套餐选择查询结果。
    SPBMInvestorPortfDefs(Vec<SPBMInvestorPortfDef>),
    /// SPBM 附加跨品种抵扣参数查询结果。
    SPBMAddOnInterParameters(Vec<SPBMAddOnInterParameter>),
    /// 投资者组合保证金系数查询结果。
    InvestorPortfMarginRatios(Vec<InvestorPortfMarginRatio>),
    /// 投资者产品 SPBM 明细查询结果。
    InvestorProdSPBMDetails(Vec<InvestorProdSPBMDetail>),
    /// SPMM 合约参数查询结果。
    SPMMInstParams(Vec<SPMMInstParam>),
    /// SPMM 品种参数查询结果。
    SPMMProductParams(Vec<SPMMProductParam>),
    /// 投资者商品组 SPMM 保证金查询结果。
    InvestorCommoditySPMMMargins(Vec<InvestorCommoditySPMMMargin>),
    /// 投资者商品群 SPMM 保证金查询结果。
    InvestorCommodityGroupSPMMMargins(Vec<InvestorCommodityGroupSPMMMargin>),
    /// RCAMS 组合产品信息查询结果。
    RCAMSCombProductInfos(Vec<RCAMSCombProductInfo>),
    /// RCAMS 合约参数查询结果。
    RCAMSInstrParameters(Vec<RCAMSInstrParameter>),
    /// RCAMS 品种内参数查询结果。
    RCAMSIntraParameters(Vec<RCAMSIntraParameter>),
    /// RCAMS 跨品种参数查询结果。
    RCAMSInterParameters(Vec<RCAMSInterParameter>),
    /// RCAMS 空头期权风险调整参数查询结果。
    RCAMSShortOptAdjustParams(Vec<RCAMSShortOptAdjustParam>),
    /// RCAMS 策略组合持仓查询结果。
    RCAMSInvestorCombPositions(Vec<RCAMSInvestorCombPosition>),
    /// 投资者品种 RCAMS 保证金查询结果。
    InvestorProdRCAMSMargins(Vec<InvestorProdRCAMSMargin>),
    /// RULE 合约参数查询结果。
    RULEInstrParameters(Vec<RULEInstrParameter>),
    /// RULE 品种内参数查询结果。
    RULEIntraParameters(Vec<RULEIntraParameter>),
    /// RULE 跨品种参数查询结果。
    RULEInterParameters(Vec<RULEInterParameter>),
    /// 投资者产品 RULE 保证金查询结果。
    InvestorProdRULEMargins(Vec<InvestorProdRULEMargin>),
    /// 投资者组合保证金开关查询结果。
    InvestorPortfSettings(Vec<InvestorPortfSetting>),
    /// 投资者风险结算持仓查询结果。
    RiskSettleInvestPositions(Vec<RiskSettleInvestPosition>),
    /// 风险结算产品状态查询结果。
    RiskSettleProductStatuses(Vec<RiskSettleProductStatus>),
    /// 投资者申报费阶梯收取记录查询结果。
    InvestorInfoCommRecs(Vec<InvestorInfoCommRec>),
    OffsetSettings(Vec<OffsetSetting>),
    SpdApplies(Vec<SpdApply>),
    HedgeCfms(Vec<HedgeCfm>),
    InstrTradeLimits(Vec<InstrTradeLimit>),
    ClassifiedInstruments(Vec<Instrument>),
    CombPromotionParams(Vec<CombPromotionParam>),
}

impl QueryRows {
    /// 聚合行数。
    pub fn len(&self) -> usize {
        match self {
            Self::Orders(rows) => rows.len(),
            Self::Trades(rows) => rows.len(),
            Self::Positions(rows) => rows.len(),
            Self::PositionDetails(rows) => rows.len(),
            Self::TradingAccounts(rows) => rows.len(),
            Self::Instruments(rows) => rows.len(),
            Self::Exchanges(rows) => rows.len(),
            Self::Products(rows) => rows.len(),
            Self::DepthMarketData(rows) => rows.len(),
            Self::PositionCombineDetails(rows) => rows.len(),
            Self::CombLegs(rows) => rows.len(),
            Self::Investors(rows) => rows.len(),
            Self::SettlementInfo(rows) => rows.len(),
            Self::ExecOrders(rows) => rows.len(),
            Self::OptionSelfCloses(rows) => rows.len(),
            Self::CombActions(rows) => rows.len(),
            Self::Quotes(rows) => rows.len(),
            Self::ForQuotes(rows) => rows.len(),
            Self::ParkedOrders(rows) => rows.len(),
            Self::ParkedOrderActions(rows) => rows.len(),
            Self::TransferBanks(rows) => rows.len(),
            Self::TransferSerials(rows) => rows.len(),
            Self::AccountRegisters(rows) => rows.len(),
            Self::ContractBanks(rows) => rows.len(),
            Self::MaxOrderVolumes(rows) => rows.len(),
            Self::InstrumentMarginRates(rows) => rows.len(),
            Self::InstrumentCommissionRates(rows) => rows.len(),
            Self::MMInstrumentCommissionRates(rows) => rows.len(),
            Self::MMOptionInstrCommRates(rows) => rows.len(),
            Self::InstrumentOrderCommRates(rows) => rows.len(),
            Self::OptionInstrTradeCosts(rows) => rows.len(),
            Self::OptionInstrCommRates(rows) => rows.len(),
            Self::ExchangeMarginRates(rows) => rows.len(),
            Self::ExchangeMarginRateAdjusts(rows) => rows.len(),
            Self::ExchangeRates(rows) => rows.len(),
            Self::ProductExchRates(rows) => rows.len(),
            Self::ProductGroups(rows) => rows.len(),
            Self::InvestorProductGroupMargins(rows) => rows.len(),
            Self::CombInstrumentGuards(rows) => rows.len(),
            Self::MaxOrderVolumeWithPrices(rows) => rows.len(),
            Self::SPBMFutureParameters(rows) => rows.len(),
            Self::SPBMOptionParameters(rows) => rows.len(),
            Self::SPBMIntraParameters(rows) => rows.len(),
            Self::SPBMInterParameters(rows) => rows.len(),
            Self::SPBMPortfDefinitions(rows) => rows.len(),
            Self::SPBMInvestorPortfDefs(rows) => rows.len(),
            Self::SPBMAddOnInterParameters(rows) => rows.len(),
            Self::InvestorPortfMarginRatios(rows) => rows.len(),
            Self::InvestorProdSPBMDetails(rows) => rows.len(),
            Self::SPMMInstParams(rows) => rows.len(),
            Self::SPMMProductParams(rows) => rows.len(),
            Self::InvestorCommoditySPMMMargins(rows) => rows.len(),
            Self::InvestorCommodityGroupSPMMMargins(rows) => rows.len(),
            Self::RCAMSCombProductInfos(rows) => rows.len(),
            Self::RCAMSInstrParameters(rows) => rows.len(),
            Self::RCAMSIntraParameters(rows) => rows.len(),
            Self::RCAMSInterParameters(rows) => rows.len(),
            Self::RCAMSShortOptAdjustParams(rows) => rows.len(),
            Self::RCAMSInvestorCombPositions(rows) => rows.len(),
            Self::InvestorProdRCAMSMargins(rows) => rows.len(),
            Self::RULEInstrParameters(rows) => rows.len(),
            Self::RULEIntraParameters(rows) => rows.len(),
            Self::RULEInterParameters(rows) => rows.len(),
            Self::InvestorProdRULEMargins(rows) => rows.len(),
            Self::InvestorPortfSettings(rows) => rows.len(),
            Self::RiskSettleInvestPositions(rows) => rows.len(),
            Self::RiskSettleProductStatuses(rows) => rows.len(),
            Self::InvestorInfoCommRecs(rows) => rows.len(),
            Self::OffsetSettings(rows) => rows.len(),
            Self::SpdApplies(rows) => rows.len(),
            Self::HedgeCfms(rows) => rows.len(),
            Self::InstrTradeLimits(rows) => rows.len(),
            Self::ClassifiedInstruments(rows) => rows.len(),
            Self::CombPromotionParams(rows) => rows.len(),
        }
    }

    /// 批次是否为空（全部行被过滤或无数据）。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// 交易 SPI 回调投递给 loop 的控制事件。
pub enum TraderEvent {
    Connected,
    Disconnected(DisconnectReason),
    AuthenticateOk,
    LoginOk {
        front_id: i32,
        session_id: i32,
        trading_day: String,
    },
    SettlementConfirmed,
    /// 认证/登录/结算确认被拒：首登期间结束握手，重连期间上报错误事件。
    Failed {
        error_id: i32,
        message: String,
    },
    /// 通用请求级错误（OnRspError）。
    ///
    /// 携带回声标签：这是查询被异步拒绝的路径（对应 OnRsp*Qry* 不会到达），
    /// 原样透传给调用方，由调用方结束受影响的在途批次。
    Error {
        /// 请求回声标签。
        request_id: i32,
        /// CTP 错误码。
        error_id: i32,
        /// 错误描述。
        message: String,
    },
    ConversionFailed(String),
    Logout,
    /// 无需 loop 决策的数据回报：SPI 转换完成后原样透传给调用方事件流。
    Report(TraderSessionEvent),
    /// 查询响应的一行。
    ///
    /// `row` 为 None 表示该行被过滤（无载荷）；批次仍须按 `is_last` 结束，
    /// 空载荷不等于批次结束被忽略（老 session 在此踩过坑）。
    QueryRow {
        /// 批次标识。
        request_id: i32,
        /// 是否批次最后一行。
        is_last: bool,
        /// CTP 错误码；0 表示成功。
        error_id: i32,
        /// 错误描述。
        message: String,
        /// 单行载荷。
        row: Option<QueryRow>,
    },
}

/// 查询请求的种类（内部命令载荷；公开方法按具体类型暴露）。
pub enum QueryKind {
    /// 查询报单。
    Order(QryOrder),
    /// 查询成交。
    Trade(QryTrade),
    /// 查询持仓。
    InvestorPosition(QryInvestorPosition),
    /// 查询持仓明细。
    InvestorPositionDetail(QryInvestorPositionDetail),
    /// 查询资金账户。
    TradingAccount(QryTradingAccount),
    /// 查询合约。
    Instrument(QryInstrument),
    /// 查询交易所。
    Exchange(QryExchange),
    /// 查询产品。
    Product(QryProduct),
    /// 查询深度行情。
    DepthMarketData(QryDepthMarketData),
    /// 查询组合持仓明细。
    InvestorPositionCombineDetail(QryInvestorPositionCombineDetail),
    /// 查询组合合约腿。
    CombLeg(QryCombLeg),
    /// 查询投资者信息。
    Investor(QryInvestor),
    /// 查询结算单。
    SettlementInfo(QrySettlementInfo),
    /// 查询行权。
    ExecOrder(QryExecOrder),
    /// 查询期权自对冲。
    OptionSelfClose(QryOptionSelfClose),
    /// 查询组合申请。
    CombAction(QryCombAction),
    /// 查询报价。
    Quote(QryQuote),
    /// 查询询价。
    ForQuote(QryForQuote),
    /// 查询预埋单。
    ParkedOrder(QryParkedOrder),
    /// 查询预埋撤单。
    ParkedOrderAction(QryParkedOrderAction),
    /// 查询转帐银行。
    TransferBank(QryTransferBank),
    /// 查询转账流水。
    TransferSerial(QryTransferSerial),
    /// 查询银期账户登记。
    AccountRegister(QryAccountRegister),
    /// 查询签约银行。
    ContractBank(QryContractBank),
    MaxOrderVolume(QryMaxOrderVolume),
    InstrumentMarginRate(QryInstrumentMarginRate),
    InstrumentCommissionRate(QryInstrumentCommissionRate),
    MMInstrumentCommissionRate(QryMMInstrumentCommissionRate),
    MMOptionInstrCommRate(QryMMOptionInstrCommRate),
    InstrumentOrderCommRate(QryInstrumentOrderCommRate),
    OptionInstrTradeCost(QryOptionInstrTradeCost),
    OptionInstrCommRate(QryOptionInstrCommRate),
    ExchangeMarginRate(QryExchangeMarginRate),
    ExchangeMarginRateAdjust(QryExchangeMarginRateAdjust),
    ExchangeRate(QryExchangeRate),
    ProductExchRate(QryProductExchRate),
    ProductGroup(QryProductGroup),
    InvestorProductGroupMargin(QryInvestorProductGroupMargin),
    CombInstrumentGuard(QryCombInstrumentGuard),
    MaxOrderVolumeWithPrice(QryMaxOrderVolumeWithPrice),
    SPBMFutureParameter(QrySPBMFutureParameter),
    SPBMOptionParameter(QrySPBMOptionParameter),
    SPBMIntraParameter(QrySPBMIntraParameter),
    SPBMInterParameter(QrySPBMInterParameter),
    SPBMPortfDefinition(QrySPBMPortfDefinition),
    SPBMInvestorPortfDef(QrySPBMInvestorPortfDef),
    SPBMAddOnInterParameter(QrySPBMAddOnInterParameter),
    InvestorPortfMarginRatio(QryInvestorPortfMarginRatio),
    InvestorProdSPBMDetail(QryInvestorProdSPBMDetail),
    SPMMInstParam(QrySPMMInstParam),
    SPMMProductParam(QrySPMMProductParam),
    InvestorCommoditySPMMMargin(QryInvestorCommoditySPMMMargin),
    InvestorCommodityGroupSPMMMargin(QryInvestorCommodityGroupSPMMMargin),
    RCAMSCombProductInfo(QryRCAMSCombProductInfo),
    RCAMSInstrParameter(QryRCAMSInstrParameter),
    RCAMSIntraParameter(QryRCAMSIntraParameter),
    RCAMSInterParameter(QryRCAMSInterParameter),
    RCAMSShortOptAdjustParam(QryRCAMSShortOptAdjustParam),
    RCAMSInvestorCombPosition(QryRCAMSInvestorCombPosition),
    InvestorProdRCAMSMargin(QryInvestorProdRCAMSMargin),
    RULEInstrParameter(QryRULEInstrParameter),
    RULEIntraParameter(QryRULEIntraParameter),
    RULEInterParameter(QryRULEInterParameter),
    InvestorProdRULEMargin(QryInvestorProdRULEMargin),
    InvestorPortfSetting(QryInvestorPortfSetting),
    RiskSettleInvestPosition(QryRiskSettleInvestPosition),
    RiskSettleProductStatus(QryRiskSettleProductStatus),
    InvestorInfoCommRec(QryInvestorInfoCommRec),
    OffsetSetting(QryOffsetSetting),
    SpdApply(QrySpdApply),
    HedgeCfm(QryHedgeCfm),
    InstrTradeLimit(QryInstrTradeLimit),
    ClassifiedInstrument(QryClassifiedInstrument),
    CombPromotionParam(QryCombPromotionParam),
}

/// 查询响应的单行载荷（SPI → loop → 调用方）。
#[derive(Clone, Debug)]
pub enum QueryRow {
    /// 报单行。
    Order(Box<Order>),
    /// 成交行。
    Trade(Box<Trade>),
    /// 持仓行。
    InvestorPosition(Box<InvestorPosition>),
    /// 持仓明细行。
    InvestorPositionDetail(Box<InvestorPositionDetail>),
    /// 资金账户行。
    TradingAccount(Box<TradingAccount>),
    /// 合约行。
    Instrument(Box<Instrument>),
    /// 交易所行。
    Exchange(Box<Exchange>),
    /// 产品行。
    Product(Box<Product>),
    /// 深度行情行。
    DepthMarketData(Box<DepthMarketData>),
    /// 组合持仓明细行。
    InvestorPositionCombineDetail(Box<InvestorPositionCombineDetail>),
    /// 组合合约腿行。
    CombLeg(Box<CombLeg>),
    /// 投资者信息行。
    Investor(Box<Investor>),
    /// 结算单行。
    SettlementInfo(Box<SettlementInfo>),
    /// 行权行。
    ExecOrder(Box<ExecOrder>),
    /// 期权自对冲行。
    OptionSelfClose(Box<OptionSelfClose>),
    /// 组合申请行。
    CombAction(Box<CombAction>),
    /// 报价行。
    Quote(Box<Quote>),
    /// 询价行。
    ForQuote(Box<ForQuote>),
    /// 预埋单行。
    ParkedOrder(Box<ParkedOrder>),
    /// 预埋撤单行。
    ParkedOrderAction(Box<ParkedOrderAction>),
    /// 转帐银行行。
    TransferBank(Box<TransferBank>),
    /// 转账流水行。
    TransferSerial(Box<TransferSerial>),
    /// 银期账户登记行。
    AccountRegister(Box<AccountRegister>),
    /// 签约银行行。
    ContractBank(Box<ContractBank>),
    MaxOrderVolume(Box<QryMaxOrderVolume>),
    InstrumentMarginRate(Box<InstrumentMarginRate>),
    InstrumentCommissionRate(Box<InstrumentCommissionRate>),
    MMInstrumentCommissionRate(Box<MMInstrumentCommissionRate>),
    MMOptionInstrCommRate(Box<MMOptionInstrCommRate>),
    InstrumentOrderCommRate(Box<InstrumentOrderCommRate>),
    OptionInstrTradeCost(Box<OptionInstrTradeCost>),
    OptionInstrCommRate(Box<OptionInstrCommRate>),
    ExchangeMarginRate(Box<ExchangeMarginRate>),
    ExchangeMarginRateAdjust(Box<ExchangeMarginRateAdjust>),
    ExchangeRate(Box<ExchangeRate>),
    ProductExchRate(Box<ProductExchRate>),
    ProductGroup(Box<ProductGroup>),
    InvestorProductGroupMargin(Box<InvestorProductGroupMargin>),
    CombInstrumentGuard(Box<CombInstrumentGuard>),
    MaxOrderVolumeWithPrice(Box<QryMaxOrderVolumeWithPrice>),
    SPBMFutureParameter(Box<SPBMFutureParameter>),
    SPBMOptionParameter(Box<SPBMOptionParameter>),
    SPBMIntraParameter(Box<SPBMIntraParameter>),
    SPBMInterParameter(Box<SPBMInterParameter>),
    SPBMPortfDefinition(Box<SPBMPortfDefinition>),
    SPBMInvestorPortfDef(Box<SPBMInvestorPortfDef>),
    SPBMAddOnInterParameter(Box<SPBMAddOnInterParameter>),
    InvestorPortfMarginRatio(Box<InvestorPortfMarginRatio>),
    InvestorProdSPBMDetail(Box<InvestorProdSPBMDetail>),
    SPMMInstParam(Box<SPMMInstParam>),
    SPMMProductParam(Box<SPMMProductParam>),
    InvestorCommoditySPMMMargin(Box<InvestorCommoditySPMMMargin>),
    InvestorCommodityGroupSPMMMargin(Box<InvestorCommodityGroupSPMMMargin>),
    RCAMSCombProductInfo(Box<RCAMSCombProductInfo>),
    RCAMSInstrParameter(Box<RCAMSInstrParameter>),
    RCAMSIntraParameter(Box<RCAMSIntraParameter>),
    RCAMSInterParameter(Box<RCAMSInterParameter>),
    RCAMSShortOptAdjustParam(Box<RCAMSShortOptAdjustParam>),
    RCAMSInvestorCombPosition(Box<RCAMSInvestorCombPosition>),
    InvestorProdRCAMSMargin(Box<InvestorProdRCAMSMargin>),
    RULEInstrParameter(Box<RULEInstrParameter>),
    RULEIntraParameter(Box<RULEIntraParameter>),
    RULEInterParameter(Box<RULEInterParameter>),
    InvestorProdRULEMargin(Box<InvestorProdRULEMargin>),
    InvestorPortfSetting(Box<InvestorPortfSetting>),
    RiskSettleInvestPosition(Box<RiskSettleInvestPosition>),
    RiskSettleProductStatus(Box<RiskSettleProductStatus>),
    InvestorInfoCommRec(Box<InvestorInfoCommRec>),
    OffsetSetting(Box<OffsetSetting>),
    SpdApply(Box<SpdApply>),
    HedgeCfm(Box<HedgeCfm>),
    InstrTradeLimit(Box<InstrTradeLimit>),
    ClassifiedInstrument(Box<Instrument>),
    CombPromotionParam(Box<CombPromotionParam>),
}

/// 调用方发给交易 loop 的命令。
pub enum TraderCommand {
    /// 报单：OrderRef 与经纪商/投资者/用户代码由 loop 填充，reply 回送分配的 OrderRef 文本。
    InsertOrder {
        /// 报单内容；`order_ref`、`broker_id`、`investor_id`、`user_id` 会被覆盖。
        order: Box<InputOrder>,
        /// 结果回执：分配的 OrderRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 撤单：OrderActionRef 与 FrontID/SessionID 由 loop 填充，reply 回送分配的 OrderActionRef。
    CancelOrder {
        /// 撤单内容；`order_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<InputOrderAction>,
        /// 结果回执：分配的 OrderActionRef 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 批量撤单：与单笔撤单共用同一 OrderActionRef 递增序列。
    BatchOrderAction {
        /// 批量撤单内容；`order_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<InputBatchOrderAction>,
        /// 结果回执：分配的 OrderActionRef 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 行权申报：ExecOrderRef 由 loop 分配，reply 回送分配的引用文本。
    ExecOrderInsert {
        /// 申报内容；`exec_order_ref` 与身份字段会被覆盖。
        order: Box<InputExecOrder>,
        /// 结果回执：分配的 ExecOrderRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 行权操作（撤销）：ExecOrderActionRef 与 FrontID/SessionID 由 loop 填充。
    ExecOrderAction {
        /// 操作内容；`exec_order_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<InputExecOrderAction>,
        /// 结果回执：分配的 ExecOrderActionRef 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 期权自对冲申报：OptionSelfCloseRef 由 loop 分配。
    OptionSelfCloseInsert {
        /// 申报内容；`option_self_close_ref` 与身份字段会被覆盖。
        order: Box<InputOptionSelfClose>,
        /// 结果回执：分配的 OptionSelfCloseRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 期权自对冲操作（撤销）：OptionSelfCloseActionRef 与 FrontID/SessionID 由 loop 填充。
    OptionSelfCloseAction {
        /// 操作内容；`option_self_close_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<InputOptionSelfCloseAction>,
        /// 结果回执：分配的 OptionSelfCloseActionRef 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 组合申请：CombActionRef 由 loop 分配，FrontID/SessionID 填当前会话。
    CombActionInsert {
        /// 申请内容；`comb_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<InputCombAction>,
        /// 结果回执：分配的 CombActionRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 报价申报：QuoteRef 由 loop 分配。
    QuoteInsert {
        /// 申报内容；`quote_ref` 与身份字段会被覆盖。
        quote: Box<InputQuote>,
        /// 结果回执：分配的 QuoteRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 报价操作（撤销）：QuoteActionRef 与 FrontID/SessionID 由 loop 填充。
    QuoteAction {
        /// 操作内容；`quote_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<InputQuoteAction>,
        /// 结果回执：分配的 QuoteActionRef 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 询价申报：ForQuoteRef 由 loop 分配。
    ForQuoteInsert {
        /// 申报内容；`for_quote_ref` 与身份字段会被覆盖。
        for_quote: Box<InputForQuote>,
        /// 结果回执：分配的 ForQuoteRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 预埋单申报：OrderRef 由 loop 分配（与报单共用递增序列）。
    ParkedOrderInsert {
        /// 申报内容；`order_ref` 与身份字段会被覆盖。
        order: Box<ParkedOrder>,
        /// 结果回执：分配的 OrderRef 或失败原因。
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 预埋撤单：OrderActionRef 与 FrontID/SessionID 由 loop 填充。
    ParkedOrderAction {
        /// 撤单内容；`order_action_ref`、`front_id`、`session_id`、身份字段会被覆盖。
        action: Box<ParkedOrderAction>,
        /// 结果回执：分配的 OrderActionRef 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 删除预埋单：按 CTP 分配的 ParkedOrderID 定位。
    RemoveParkedOrder {
        /// 删除内容；身份字段会被覆盖。
        removed: Box<RemoveParkedOrder>,
        /// 结果回执：分配的 request_id 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 删除预埋撤单：按 CTP 分配的 ParkedOrderActionID 定位。
    RemoveParkedOrderAction {
        /// 删除内容；身份字段会被覆盖。
        removed: Box<RemoveParkedOrderAction>,
        /// 结果回执：分配的 request_id 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 期商发起入金（银行→期货）：业务功能码与身份字段由 loop 填充。
    BankToFuture {
        /// 转账内容；`trade_code`、`broker_id`、`account_id`、`user_id` 会被覆盖。
        transfer: Box<ReqTransfer>,
        /// 结果回执：分配的 request_id 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 期商发起出金（期货→银行）：业务功能码与身份字段由 loop 填充。
    FutureToBank {
        /// 转账内容；`trade_code`、`broker_id`、`account_id`、`user_id` 会被覆盖。
        transfer: Box<ReqTransfer>,
        /// 结果回执：分配的 request_id 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 查询银行余额：身份字段由 loop 填充。
    QueryBankBalance {
        /// 查询内容；`broker_id`、`account_id`、`user_id` 会被覆盖。
        query: Box<ReqQueryAccount>,
        /// 结果回执：分配的 request_id 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 查询：request_id 由 loop 分配，每个响应行立即以 `QueryRow` 事件返回。
    Query {
        /// 查询种类与请求内容。
        kind: Box<QueryKind>,
        /// 结果回执：分配的 request_id 或失败原因。
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 对冲设置；保留结构体内 RequestID 与业务维度。
    SetOffset {
        setting: Box<InputOffsetSetting>,
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 撤销对冲设置；按原报入维度发送。
    CancelOffsetSetting {
        setting: Box<InputOffsetSetting>,
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// SPD 套利确认；回执返回分配的 OrderRef 文本。
    SpdApply {
        apply: Box<InputSpdApply>,
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// SPD 撤销；保留原 OrderRef/OrderSysID。
    SpdApplyAction {
        action: Box<InputSpdApplyAction>,
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 套保确认；回执返回分配的 OrderRef 文本。
    HedgeCfm {
        apply: Box<InputHedgeCfm>,
        reply: SyncSender<Result<String, TraderSessionError>>,
    },
    /// 套保确认撤销；保留原 OrderRef/OrderSysID。
    HedgeCfmAction {
        action: Box<InputHedgeCfmAction>,
        reply: SyncSender<Result<i32, TraderSessionError>>,
    },
    /// 主动重连：利用官方记载的"登出=断开重连"行为触发链路重建（FAQ 的 logout/重连条目）。
    Reconnect,
    Shutdown,
}

/// 交易 loop 的唯一输入：SPI 控制事件与调用方命令汇入同一队列，串行处理、阻塞等待。
pub enum TraderInput {
    Spi(TraderEvent),
    Command(TraderCommand),
}
