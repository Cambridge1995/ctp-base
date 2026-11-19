// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use crate::spi::TraderSpi;
use crate::sys::convert::owned_params;
use crate::{
    AccountRegister, BatchOrderAction, BrokerTradingAlgos, BrokerTradingParams, Bulletin,
    CFMMCTradingAccountKey, CFMMCTradingAccountToken, CancelAccount, CancelOffsetSetting,
    ChangeAccount, CombAction, CombInstrumentGuard, CombLeg, CombPromotionParam, ContractBank,
    DepthMarketData, EWarrantOffset, ErrorConditionalOrder, Exchange, ExchangeMarginRate,
    ExchangeMarginRateAdjust, ExchangeRate, ExecOrder, ExecOrderAction, ForQuote, ForQuoteRsp,
    HedgeCfm, HedgeCfmAction, InputBatchOrderAction, InputCombAction, InputExecOrder,
    InputExecOrderAction, InputForQuote, InputHedgeCfm, InputHedgeCfmAction, InputOffsetSetting,
    InputOptionSelfClose, InputOptionSelfCloseAction, InputOrder, InputOrderAction, InputQuote,
    InputQuoteAction, InputSpdApply, InputSpdApplyAction, Instrument, InstrumentCommissionRate,
    InstrumentMarginRate, InstrumentOrderCommRate, InstrumentStatus, InvestUnit, Investor,
    InvestorCommodityGroupSPMMMargin, InvestorCommoditySPMMMargin, InvestorInfoCommRec,
    InvestorPortfMarginRatio, InvestorPortfSetting, InvestorPosition,
    InvestorPositionCombineDetail, InvestorPositionDetail, InvestorProdRCAMSMargin,
    InvestorProdRULEMargin, InvestorProdSPBMDetail, InvestorProductGroupMargin,
    MMInstrumentCommissionRate, MMOptionInstrCommRate, Notice, NotifyQueryAccount, OffsetSetting,
    OpenAccount, OptionInstrCommRate, OptionInstrTradeCost, OptionSelfClose, OptionSelfCloseAction,
    Order, OrderAction, ParkedOrder, ParkedOrderAction, Product, ProductExchRate, ProductGroup,
    QryMaxOrderVolume, QueryCFMMCTradingAccountToken, Quote, QuoteAction, RCAMSCombProductInfo,
    RCAMSInstrParameter, RCAMSInterParameter, RCAMSIntraParameter, RCAMSInvestorCombPosition,
    RCAMSShortOptAdjustParam, RULEInstrParameter, RULEInterParameter, RULEIntraParameter,
    RemoveParkedOrder, RemoveParkedOrderAction, ReqQueryAccount, ReqRepeal, ReqTransfer,
    RiskSettleInvestPosition, RiskSettleProductStatus, RspAuthenticate, RspGenSMSCode,
    RspGenUserCaptcha, RspGenUserText, RspInfo, RspRepeal, RspTransfer, RspUserAuthMethod,
    RspUserLogin, SPBMAddOnInterParameter, SPBMFutureParameter, SPBMInterParameter,
    SPBMIntraParameter, SPBMInvestorPortfDef, SPBMOptionParameter, SPBMPortfDefinition,
    SPMMInstParam, SPMMProductParam, SecAgentACIDMap, SecAgentCheckMode, SecAgentTradeInfo,
    SettlementInfo, SettlementInfoConfirm, SpdApply, SpdApplyAction, Trade, TraderOffer,
    TradingAccount, TradingAccountPasswordUpdate, TradingCode, TradingNotice, TradingNoticeInfo,
    TransferBank, TransferSerial, UserLogout, UserPasswordUpdate, UserSession,
};

pub struct TraderSpiSink {
    spi: Box<dyn TraderSpi>,
}

impl TraderSpiSink {
    pub fn new(spi: Box<dyn TraderSpi>) -> Self {
        Self { spi }
    }
}

#[inline]
fn invoke(callback: impl FnOnce()) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback));
}

pub(crate) fn connected(sink: &mut TraderSpiSink) {
    invoke(|| sink.spi.connected());
}

pub(crate) fn disconnected(sink: &mut TraderSpiSink, reason: i32) {
    invoke(|| sink.spi.disconnected(reason.into()));
}

pub(crate) fn heartbeat(sink: &mut TraderSpiSink, time_lapse: i32) {
    invoke(|| sink.spi.heartbeat(time_lapse));
}

pub(crate) unsafe fn authenticate_response(
    sink: &mut TraderSpiSink,
    rsp_authenticate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_authenticate: RspAuthenticate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .authenticate_response(rsp_authenticate, rsp_info, request_id, is_last)
    });
}

pub(crate) fn private_seq_no_report(sink: &mut TraderSpiSink, seq_no: i32) {
    invoke(|| sink.spi.private_seq_no_report(seq_no));
}

pub(crate) unsafe fn login_response(
    sink: &mut TraderSpiSink,
    rsp_user_login: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_user_login: RspUserLogin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .login_response(rsp_user_login, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn logout_response(
    sink: &mut TraderSpiSink,
    user_logout: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, user_logout: UserLogout,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .logout_response(user_logout, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn user_password_update_response(
    sink: &mut TraderSpiSink,
    user_password_update: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, user_password_update: UserPasswordUpdate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .user_password_update_response(user_password_update, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn trading_account_password_update_response(
    sink: &mut TraderSpiSink,
    trading_account_password_update: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trading_account_password_update: TradingAccountPasswordUpdate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.trading_account_password_update_response(
            trading_account_password_update,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn user_auth_method_response(
    sink: &mut TraderSpiSink,
    rsp_user_auth_method: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_user_auth_method: RspUserAuthMethod,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .user_auth_method_response(rsp_user_auth_method, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn gen_user_captcha_response(
    sink: &mut TraderSpiSink,
    rsp_gen_user_captcha: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_gen_user_captcha: RspGenUserCaptcha,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .gen_user_captcha_response(rsp_gen_user_captcha, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn gen_user_text_response(
    sink: &mut TraderSpiSink,
    rsp_gen_user_text: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_gen_user_text: RspGenUserText,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .gen_user_text_response(rsp_gen_user_text, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn order_insert_response(
    sink: &mut TraderSpiSink,
    input_order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_order: InputOrder,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .order_insert_response(input_order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn parked_order_insert_response(
    sink: &mut TraderSpiSink,
    parked_order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, parked_order: ParkedOrder,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .parked_order_insert_response(parked_order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn parked_action_response(
    sink: &mut TraderSpiSink,
    parked_order_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, parked_order_action: ParkedOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .parked_action_response(parked_order_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn order_action_response(
    sink: &mut TraderSpiSink,
    input_order_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_order_action: InputOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .order_action_response(input_order_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn max_order_volume_response(
    sink: &mut TraderSpiSink,
    qry_max_order_volume: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, qry_max_order_volume: QryMaxOrderVolume,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .max_order_volume_response(qry_max_order_volume, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn settlement_confirm_response(
    sink: &mut TraderSpiSink,
    settlement_info_confirm: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, settlement_info_confirm: SettlementInfoConfirm,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .settlement_confirm_response(settlement_info_confirm, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn remove_parked_order_response(
    sink: &mut TraderSpiSink,
    remove_parked_order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, remove_parked_order: RemoveParkedOrder,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .remove_parked_order_response(remove_parked_order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn remove_parked_order_action_response(
    sink: &mut TraderSpiSink,
    remove_parked_order_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, remove_parked_order_action: RemoveParkedOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.remove_parked_order_action_response(
            remove_parked_order_action,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn exec_order_insert_response(
    sink: &mut TraderSpiSink,
    input_exec_order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_exec_order: InputExecOrder,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exec_order_insert_response(input_exec_order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn exec_order_action_response(
    sink: &mut TraderSpiSink,
    input_exec_order_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_exec_order_action: InputExecOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exec_order_action_response(input_exec_order_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn for_quote_insert_response(
    sink: &mut TraderSpiSink,
    input_for_quote: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_for_quote: InputForQuote,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .for_quote_insert_response(input_for_quote, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn quote_insert_response(
    sink: &mut TraderSpiSink,
    input_quote: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_quote: InputQuote,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .quote_insert_response(input_quote, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn quote_action_response(
    sink: &mut TraderSpiSink,
    input_quote_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_quote_action: InputQuoteAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .quote_action_response(input_quote_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn batch_order_action_response(
    sink: &mut TraderSpiSink,
    input_batch_order_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_batch_order_action: InputBatchOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.batch_order_action_response(
            input_batch_order_action,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn option_self_close_insert_response(
    sink: &mut TraderSpiSink,
    input_option_self_close: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_option_self_close: InputOptionSelfClose,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.option_self_close_insert_response(
            input_option_self_close,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn option_self_close_action_response(
    sink: &mut TraderSpiSink,
    input_option_self_close_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_option_self_close_action: InputOptionSelfCloseAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.option_self_close_action_response(
            input_option_self_close_action,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn comb_action_insert_response(
    sink: &mut TraderSpiSink,
    input_comb_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_comb_action: InputCombAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .comb_action_insert_response(input_comb_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn order_response(
    sink: &mut TraderSpiSink,
    order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, order: Order,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .order_response(order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn trade_response(
    sink: &mut TraderSpiSink,
    trade: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trade: Trade,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .trade_response(trade, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn investor_position_response(
    sink: &mut TraderSpiSink,
    investor_position: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_position: InvestorPosition,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .investor_position_response(investor_position, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn trading_account_response(
    sink: &mut TraderSpiSink,
    trading_account: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trading_account: TradingAccount,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .trading_account_response(trading_account, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn investor_response(
    sink: &mut TraderSpiSink,
    investor: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor: Investor,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .investor_response(investor, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn trading_code_response(
    sink: &mut TraderSpiSink,
    trading_code: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trading_code: TradingCode,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .trading_code_response(trading_code, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn instrument_margin_rate_response(
    sink: &mut TraderSpiSink,
    instrument_margin_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, instrument_margin_rate: InstrumentMarginRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.instrument_margin_rate_response(
            instrument_margin_rate,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn instrument_commission_rate_response(
    sink: &mut TraderSpiSink,
    instrument_commission_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, instrument_commission_rate: InstrumentCommissionRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.instrument_commission_rate_response(
            instrument_commission_rate,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn user_session_response(
    sink: &mut TraderSpiSink,
    user_session: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, user_session: UserSession,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .user_session_response(user_session, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn exchange_response(
    sink: &mut TraderSpiSink,
    exchange: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, exchange: Exchange,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exchange_response(exchange, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn product_response(
    sink: &mut TraderSpiSink,
    product: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, product: Product,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .product_response(product, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn instrument_response(
    sink: &mut TraderSpiSink,
    instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, instrument: Instrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .instrument_response(instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn depth_market_data_response(
    sink: &mut TraderSpiSink,
    depth_market_data: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, depth_market_data: DepthMarketData,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .depth_market_data_response(depth_market_data, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn trader_offer_response(
    sink: &mut TraderSpiSink,
    trader_offer: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trader_offer: TraderOffer,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .trader_offer_response(trader_offer, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn settlement_info_response(
    sink: &mut TraderSpiSink,
    settlement_info: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, settlement_info: SettlementInfo,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .settlement_info_response(settlement_info, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn transfer_bank_response(
    sink: &mut TraderSpiSink,
    transfer_bank: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, transfer_bank: TransferBank,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .transfer_bank_response(transfer_bank, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn investor_position_detail_response(
    sink: &mut TraderSpiSink,
    investor_position_detail: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_position_detail: InvestorPositionDetail,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_position_detail_response(
            investor_position_detail,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn notice_response(
    sink: &mut TraderSpiSink,
    notice: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, notice: Notice,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .notice_response(notice, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn settlement_status(
    sink: &mut TraderSpiSink,
    settlement_info_confirm: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, settlement_info_confirm: SettlementInfoConfirm,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .settlement_status(settlement_info_confirm, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn investor_position_combine_detail_response(
    sink: &mut TraderSpiSink,
    investor_position_combine_detail: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_position_combine_detail: InvestorPositionCombineDetail,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_position_combine_detail_response(
            investor_position_combine_detail,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn cfmmc_trading_account_key_response(
    sink: &mut TraderSpiSink,
    cfmmc_trading_account_key: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, cfmmc_trading_account_key: CFMMCTradingAccountKey,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.cfmmc_trading_account_key_response(
            cfmmc_trading_account_key,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn e_warrant_offset_response(
    sink: &mut TraderSpiSink,
    e_warrant_offset: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, e_warrant_offset: EWarrantOffset,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .e_warrant_offset_response(e_warrant_offset, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn investor_product_group_margin_response(
    sink: &mut TraderSpiSink,
    investor_product_group_margin: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_product_group_margin: InvestorProductGroupMargin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_product_group_margin_response(
            investor_product_group_margin,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn exchange_margin_rate_response(
    sink: &mut TraderSpiSink,
    exchange_margin_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, exchange_margin_rate: ExchangeMarginRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exchange_margin_rate_response(exchange_margin_rate, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn exchange_margin_rate_adjust_response(
    sink: &mut TraderSpiSink,
    exchange_margin_rate_adjust: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, exchange_margin_rate_adjust: ExchangeMarginRateAdjust,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.exchange_margin_rate_adjust_response(
            exchange_margin_rate_adjust,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn exchange_rate_response(
    sink: &mut TraderSpiSink,
    exchange_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, exchange_rate: ExchangeRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exchange_rate_response(exchange_rate, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn sec_agent_acid_map_response(
    sink: &mut TraderSpiSink,
    sec_agent_acid_map: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, sec_agent_acid_map: SecAgentACIDMap,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .sec_agent_acid_map_response(sec_agent_acid_map, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn product_exchange_rate_response(
    sink: &mut TraderSpiSink,
    product_exch_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, product_exch_rate: ProductExchRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .product_exchange_rate_response(product_exch_rate, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn product_group_response(
    sink: &mut TraderSpiSink,
    product_group: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, product_group: ProductGroup,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .product_group_response(product_group, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn mm_instrument_commission_rate_response(
    sink: &mut TraderSpiSink,
    mm_instrument_commission_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, mm_instrument_commission_rate: MMInstrumentCommissionRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.mm_instrument_commission_rate_response(
            mm_instrument_commission_rate,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn mm_option_instrument_commission_rate_response(
    sink: &mut TraderSpiSink,
    mm_option_instr_comm_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, mm_option_instr_comm_rate: MMOptionInstrCommRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.mm_option_instrument_commission_rate_response(
            mm_option_instr_comm_rate,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn instrument_order_commission_rate_response(
    sink: &mut TraderSpiSink,
    instrument_order_comm_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, instrument_order_comm_rate: InstrumentOrderCommRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.instrument_order_commission_rate_response(
            instrument_order_comm_rate,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn sec_agent_trading_account_response(
    sink: &mut TraderSpiSink,
    trading_account: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trading_account: TradingAccount,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .sec_agent_trading_account_response(trading_account, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn sec_agent_check_mode_response(
    sink: &mut TraderSpiSink,
    sec_agent_check_mode: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, sec_agent_check_mode: SecAgentCheckMode,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .sec_agent_check_mode_response(sec_agent_check_mode, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn sec_agent_trade_info_response(
    sink: &mut TraderSpiSink,
    sec_agent_trade_info: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, sec_agent_trade_info: SecAgentTradeInfo,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .sec_agent_trade_info_response(sec_agent_trade_info, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn option_instrument_trade_cost_response(
    sink: &mut TraderSpiSink,
    option_instr_trade_cost: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, option_instr_trade_cost: OptionInstrTradeCost,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.option_instrument_trade_cost_response(
            option_instr_trade_cost,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn option_instrument_commission_rate_response(
    sink: &mut TraderSpiSink,
    option_instr_comm_rate: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, option_instr_comm_rate: OptionInstrCommRate,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.option_instrument_commission_rate_response(
            option_instr_comm_rate,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn exec_order_response(
    sink: &mut TraderSpiSink,
    exec_order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, exec_order: ExecOrder,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exec_order_response(exec_order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn for_quote_query_response(
    sink: &mut TraderSpiSink,
    for_quote: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, for_quote: ForQuote,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .for_quote_query_response(for_quote, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn quote_response(
    sink: &mut TraderSpiSink,
    quote: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, quote: Quote,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .quote_response(quote, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn option_self_close_response(
    sink: &mut TraderSpiSink,
    option_self_close: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, option_self_close: OptionSelfClose,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .option_self_close_response(option_self_close, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn invest_unit_response(
    sink: &mut TraderSpiSink,
    invest_unit: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, invest_unit: InvestUnit,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .invest_unit_response(invest_unit, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn comb_instrument_guard_response(
    sink: &mut TraderSpiSink,
    comb_instrument_guard: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, comb_instrument_guard: CombInstrumentGuard,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.comb_instrument_guard_response(
            comb_instrument_guard,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn comb_action_response(
    sink: &mut TraderSpiSink,
    comb_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, comb_action: CombAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .comb_action_response(comb_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn transfer_serial_response(
    sink: &mut TraderSpiSink,
    transfer_serial: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, transfer_serial: TransferSerial,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .transfer_serial_response(transfer_serial, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn account_register_response(
    sink: &mut TraderSpiSink,
    account_register: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, account_register: AccountRegister,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .account_register_response(account_register, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn error(
    sink: &mut TraderSpiSink,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_info: RspInfo);
    invoke(|| sink.spi.error(rsp_info, request_id, is_last));
}

pub(crate) unsafe fn order_update(sink: &mut TraderSpiSink, order: *const u8) {
    owned_params!(sink, order: Order);
    invoke(|| sink.spi.order_update(order));
}

pub(crate) unsafe fn execution(sink: &mut TraderSpiSink, trade: *const u8) {
    owned_params!(sink, trade: Trade);
    invoke(|| sink.spi.execution(trade));
}

pub(crate) unsafe fn order_insert_error(
    sink: &mut TraderSpiSink,
    input_order: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_order: InputOrder,
        rsp_info: RspInfo);
    invoke(|| sink.spi.order_insert_error(input_order, rsp_info));
}

pub(crate) unsafe fn order_action_error(
    sink: &mut TraderSpiSink,
    order_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, order_action: OrderAction,
        rsp_info: RspInfo);
    invoke(|| sink.spi.order_action_error(order_action, rsp_info));
}

pub(crate) unsafe fn instrument_status(sink: &mut TraderSpiSink, instrument_status: *const u8) {
    owned_params!(sink, instrument_status: InstrumentStatus);
    invoke(|| sink.spi.instrument_status(instrument_status));
}

pub(crate) unsafe fn bulletin(sink: &mut TraderSpiSink, bulletin: *const u8) {
    owned_params!(sink, bulletin: Bulletin);
    invoke(|| sink.spi.bulletin(bulletin));
}

pub(crate) unsafe fn notice_received(sink: &mut TraderSpiSink, trading_notice_info: *const u8) {
    owned_params!(sink, trading_notice_info: TradingNoticeInfo);
    invoke(|| sink.spi.notice_received(trading_notice_info));
}

pub(crate) unsafe fn conditional_order_error(
    sink: &mut TraderSpiSink,
    error_conditional_order: *const u8,
) {
    owned_params!(sink, error_conditional_order: ErrorConditionalOrder);
    invoke(|| sink.spi.conditional_order_error(error_conditional_order));
}

pub(crate) unsafe fn exec_order_report(sink: &mut TraderSpiSink, exec_order: *const u8) {
    owned_params!(sink, exec_order: ExecOrder);
    invoke(|| sink.spi.exec_order_report(exec_order));
}

pub(crate) unsafe fn exec_order_insert_error(
    sink: &mut TraderSpiSink,
    input_exec_order: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_exec_order: InputExecOrder,
        rsp_info: RspInfo);
    invoke(|| sink.spi.exec_order_insert_error(input_exec_order, rsp_info));
}

pub(crate) unsafe fn exec_order_action_error(
    sink: &mut TraderSpiSink,
    exec_order_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, exec_order_action: ExecOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .exec_order_action_error(exec_order_action, rsp_info)
    });
}

pub(crate) unsafe fn for_quote_insert_error(
    sink: &mut TraderSpiSink,
    input_for_quote: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_for_quote: InputForQuote,
        rsp_info: RspInfo);
    invoke(|| sink.spi.for_quote_insert_error(input_for_quote, rsp_info));
}

pub(crate) unsafe fn quote_report(sink: &mut TraderSpiSink, quote: *const u8) {
    owned_params!(sink, quote: Quote);
    invoke(|| sink.spi.quote_report(quote));
}

pub(crate) unsafe fn quote_insert_error(
    sink: &mut TraderSpiSink,
    input_quote: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_quote: InputQuote,
        rsp_info: RspInfo);
    invoke(|| sink.spi.quote_insert_error(input_quote, rsp_info));
}

pub(crate) unsafe fn quote_action_error(
    sink: &mut TraderSpiSink,
    quote_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, quote_action: QuoteAction,
        rsp_info: RspInfo);
    invoke(|| sink.spi.quote_action_error(quote_action, rsp_info));
}

pub(crate) unsafe fn for_quote_response(sink: &mut TraderSpiSink, for_quote_rsp: *const u8) {
    owned_params!(sink, for_quote_rsp: ForQuoteRsp);
    invoke(|| sink.spi.for_quote_response(for_quote_rsp));
}

pub(crate) unsafe fn cfmmc_account_token(
    sink: &mut TraderSpiSink,
    cfmmc_trading_account_token: *const u8,
) {
    owned_params!(sink, cfmmc_trading_account_token: CFMMCTradingAccountToken);
    invoke(|| sink.spi.cfmmc_account_token(cfmmc_trading_account_token));
}

pub(crate) unsafe fn batch_order_action_error(
    sink: &mut TraderSpiSink,
    batch_order_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, batch_order_action: BatchOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .batch_order_action_error(batch_order_action, rsp_info)
    });
}

pub(crate) unsafe fn option_self_close_report(
    sink: &mut TraderSpiSink,
    option_self_close: *const u8,
) {
    owned_params!(sink, option_self_close: OptionSelfClose);
    invoke(|| sink.spi.option_self_close_report(option_self_close));
}

pub(crate) unsafe fn option_self_close_insert_error(
    sink: &mut TraderSpiSink,
    input_option_self_close: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_option_self_close: InputOptionSelfClose,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .option_self_close_insert_error(input_option_self_close, rsp_info)
    });
}

pub(crate) unsafe fn option_self_close_action_error(
    sink: &mut TraderSpiSink,
    option_self_close_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, option_self_close_action: OptionSelfCloseAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .option_self_close_action_error(option_self_close_action, rsp_info)
    });
}

pub(crate) unsafe fn comb_action_report(sink: &mut TraderSpiSink, comb_action: *const u8) {
    owned_params!(sink, comb_action: CombAction);
    invoke(|| sink.spi.comb_action_report(comb_action));
}

pub(crate) unsafe fn comb_action_insert_error(
    sink: &mut TraderSpiSink,
    input_comb_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_comb_action: InputCombAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .comb_action_insert_error(input_comb_action, rsp_info)
    });
}

pub(crate) unsafe fn contract_bank_response(
    sink: &mut TraderSpiSink,
    contract_bank: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, contract_bank: ContractBank,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .contract_bank_response(contract_bank, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn parked_order_response(
    sink: &mut TraderSpiSink,
    parked_order: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, parked_order: ParkedOrder,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .parked_order_response(parked_order, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn parked_order_actions(
    sink: &mut TraderSpiSink,
    parked_order_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, parked_order_action: ParkedOrderAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .parked_order_actions(parked_order_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn notice_history_response(
    sink: &mut TraderSpiSink,
    trading_notice: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, trading_notice: TradingNotice,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .notice_history_response(trading_notice, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn broker_trading_parameters_response(
    sink: &mut TraderSpiSink,
    broker_trading_params: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, broker_trading_params: BrokerTradingParams,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.broker_trading_parameters_response(
            broker_trading_params,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn broker_trading_algos_response(
    sink: &mut TraderSpiSink,
    broker_trading_algos: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, broker_trading_algos: BrokerTradingAlgos,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .broker_trading_algos_response(broker_trading_algos, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn query_cfmmc_trading_account_token_response(
    sink: &mut TraderSpiSink,
    query_cfmmc_trading_account_token: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, query_cfmmc_trading_account_token: QueryCFMMCTradingAccountToken,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.query_cfmmc_trading_account_token_response(
            query_cfmmc_trading_account_token,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn from_bank_to_future_by_bank_report(
    sink: &mut TraderSpiSink,
    rsp_transfer: *const u8,
) {
    owned_params!(sink, rsp_transfer: RspTransfer);
    invoke(|| sink.spi.from_bank_to_future_by_bank_report(rsp_transfer));
}

pub(crate) unsafe fn from_future_to_bank_by_bank_report(
    sink: &mut TraderSpiSink,
    rsp_transfer: *const u8,
) {
    owned_params!(sink, rsp_transfer: RspTransfer);
    invoke(|| sink.spi.from_future_to_bank_by_bank_report(rsp_transfer));
}

pub(crate) unsafe fn repeal_from_bank_to_future_by_bank_report(
    sink: &mut TraderSpiSink,
    rsp_repeal: *const u8,
) {
    owned_params!(sink, rsp_repeal: RspRepeal);
    invoke(|| {
        sink.spi
            .repeal_from_bank_to_future_by_bank_report(rsp_repeal)
    });
}

pub(crate) unsafe fn repeal_from_future_to_bank_by_bank_report(
    sink: &mut TraderSpiSink,
    rsp_repeal: *const u8,
) {
    owned_params!(sink, rsp_repeal: RspRepeal);
    invoke(|| {
        sink.spi
            .repeal_from_future_to_bank_by_bank_report(rsp_repeal)
    });
}

pub(crate) unsafe fn from_bank_to_future_by_future_report(
    sink: &mut TraderSpiSink,
    rsp_transfer: *const u8,
) {
    owned_params!(sink, rsp_transfer: RspTransfer);
    invoke(|| sink.spi.from_bank_to_future_by_future_report(rsp_transfer));
}

pub(crate) unsafe fn from_future_to_bank_by_future_report(
    sink: &mut TraderSpiSink,
    rsp_transfer: *const u8,
) {
    owned_params!(sink, rsp_transfer: RspTransfer);
    invoke(|| sink.spi.from_future_to_bank_by_future_report(rsp_transfer));
}

pub(crate) unsafe fn repeal_from_bank_to_future_by_future_manual_report(
    sink: &mut TraderSpiSink,
    rsp_repeal: *const u8,
) {
    owned_params!(sink, rsp_repeal: RspRepeal);
    invoke(|| {
        sink.spi
            .repeal_from_bank_to_future_by_future_manual_report(rsp_repeal)
    });
}

pub(crate) unsafe fn repeal_from_future_to_bank_by_future_manual_report(
    sink: &mut TraderSpiSink,
    rsp_repeal: *const u8,
) {
    owned_params!(sink, rsp_repeal: RspRepeal);
    invoke(|| {
        sink.spi
            .repeal_from_future_to_bank_by_future_manual_report(rsp_repeal)
    });
}

pub(crate) unsafe fn query_bank_balance_by_future_report(
    sink: &mut TraderSpiSink,
    notify_query_account: *const u8,
) {
    owned_params!(sink, notify_query_account: NotifyQueryAccount);
    invoke(|| {
        sink.spi
            .query_bank_balance_by_future_report(notify_query_account)
    });
}

pub(crate) unsafe fn bank_to_future_by_future_error(
    sink: &mut TraderSpiSink,
    req_transfer: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, req_transfer: ReqTransfer,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .bank_to_future_by_future_error(req_transfer, rsp_info)
    });
}

pub(crate) unsafe fn future_to_bank_by_future_error(
    sink: &mut TraderSpiSink,
    req_transfer: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, req_transfer: ReqTransfer,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .future_to_bank_by_future_error(req_transfer, rsp_info)
    });
}

pub(crate) unsafe fn repeal_bank_to_future_by_future_manual_error(
    sink: &mut TraderSpiSink,
    req_repeal: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, req_repeal: ReqRepeal,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .repeal_bank_to_future_by_future_manual_error(req_repeal, rsp_info)
    });
}

pub(crate) unsafe fn repeal_future_to_bank_by_future_manual_error(
    sink: &mut TraderSpiSink,
    req_repeal: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, req_repeal: ReqRepeal,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .repeal_future_to_bank_by_future_manual_error(req_repeal, rsp_info)
    });
}

pub(crate) unsafe fn query_bank_balance_by_future_error(
    sink: &mut TraderSpiSink,
    req_query_account: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, req_query_account: ReqQueryAccount,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .query_bank_balance_by_future_error(req_query_account, rsp_info)
    });
}

pub(crate) unsafe fn repeal_from_bank_to_future_by_future_report(
    sink: &mut TraderSpiSink,
    rsp_repeal: *const u8,
) {
    owned_params!(sink, rsp_repeal: RspRepeal);
    invoke(|| {
        sink.spi
            .repeal_from_bank_to_future_by_future_report(rsp_repeal)
    });
}

pub(crate) unsafe fn repeal_from_future_to_bank_by_future_report(
    sink: &mut TraderSpiSink,
    rsp_repeal: *const u8,
) {
    owned_params!(sink, rsp_repeal: RspRepeal);
    invoke(|| {
        sink.spi
            .repeal_from_future_to_bank_by_future_report(rsp_repeal)
    });
}

pub(crate) unsafe fn from_bank_to_future_by_future_response(
    sink: &mut TraderSpiSink,
    req_transfer: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, req_transfer: ReqTransfer,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .from_bank_to_future_by_future_response(req_transfer, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn from_future_to_bank_by_future_response(
    sink: &mut TraderSpiSink,
    req_transfer: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, req_transfer: ReqTransfer,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .from_future_to_bank_by_future_response(req_transfer, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn query_bank_account_money_by_future_response(
    sink: &mut TraderSpiSink,
    req_query_account: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, req_query_account: ReqQueryAccount,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.query_bank_account_money_by_future_response(
            req_query_account,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn open_account_by_bank_report(
    sink: &mut TraderSpiSink,
    open_account: *const u8,
) {
    owned_params!(sink, open_account: OpenAccount);
    invoke(|| sink.spi.open_account_by_bank_report(open_account));
}

pub(crate) unsafe fn cancel_account_by_bank_report(
    sink: &mut TraderSpiSink,
    cancel_account: *const u8,
) {
    owned_params!(sink, cancel_account: CancelAccount);
    invoke(|| sink.spi.cancel_account_by_bank_report(cancel_account));
}

pub(crate) unsafe fn change_account_by_bank_report(
    sink: &mut TraderSpiSink,
    change_account: *const u8,
) {
    owned_params!(sink, change_account: ChangeAccount);
    invoke(|| sink.spi.change_account_by_bank_report(change_account));
}

pub(crate) unsafe fn classified_instrument_response(
    sink: &mut TraderSpiSink,
    instrument: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, instrument: Instrument,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .classified_instrument_response(instrument, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn comb_promotion_parameter_response(
    sink: &mut TraderSpiSink,
    comb_promotion_param: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, comb_promotion_param: CombPromotionParam,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.comb_promotion_parameter_response(
            comb_promotion_param,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn risk_settle_investor_position_response(
    sink: &mut TraderSpiSink,
    risk_settle_invest_position: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, risk_settle_invest_position: RiskSettleInvestPosition,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.risk_settle_investor_position_response(
            risk_settle_invest_position,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn risk_settle_product_status_response(
    sink: &mut TraderSpiSink,
    risk_settle_product_status: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, risk_settle_product_status: RiskSettleProductStatus,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.risk_settle_product_status_response(
            risk_settle_product_status,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn spbm_future_parameter_response(
    sink: &mut TraderSpiSink,
    spbm_future_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_future_parameter: SPBMFutureParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.spbm_future_parameter_response(
            spbm_future_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn spbm_option_parameter_response(
    sink: &mut TraderSpiSink,
    spbm_option_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_option_parameter: SPBMOptionParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.spbm_option_parameter_response(
            spbm_option_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn spbm_intra_parameter_response(
    sink: &mut TraderSpiSink,
    spbm_intra_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_intra_parameter: SPBMIntraParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spbm_intra_parameter_response(spbm_intra_parameter, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spbm_inter_parameter_response(
    sink: &mut TraderSpiSink,
    spbm_inter_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_inter_parameter: SPBMInterParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spbm_inter_parameter_response(spbm_inter_parameter, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spbm_portfolio_definition_response(
    sink: &mut TraderSpiSink,
    spbm_portf_definition: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_portf_definition: SPBMPortfDefinition,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.spbm_portfolio_definition_response(
            spbm_portf_definition,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn spbm_investor_portfolio_definition_response(
    sink: &mut TraderSpiSink,
    spbm_investor_portf_def: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_investor_portf_def: SPBMInvestorPortfDef,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.spbm_investor_portfolio_definition_response(
            spbm_investor_portf_def,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_portfolio_margin_ratio_response(
    sink: &mut TraderSpiSink,
    investor_portf_margin_ratio: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_portf_margin_ratio: InvestorPortfMarginRatio,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_portfolio_margin_ratio_response(
            investor_portf_margin_ratio,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_product_spbm_detail_response(
    sink: &mut TraderSpiSink,
    investor_prod_spbm_detail: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_prod_spbm_detail: InvestorProdSPBMDetail,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_product_spbm_detail_response(
            investor_prod_spbm_detail,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_commodity_spmm_margin_response(
    sink: &mut TraderSpiSink,
    investor_commodity_spmm_margin: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_commodity_spmm_margin: InvestorCommoditySPMMMargin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_commodity_spmm_margin_response(
            investor_commodity_spmm_margin,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_commodity_group_spmm_margin_response(
    sink: &mut TraderSpiSink,
    investor_commodity_group_spmm_margin: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_commodity_group_spmm_margin: InvestorCommodityGroupSPMMMargin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_commodity_group_spmm_margin_response(
            investor_commodity_group_spmm_margin,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn spmm_instrument_parameter_response(
    sink: &mut TraderSpiSink,
    spmm_inst_param: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spmm_inst_param: SPMMInstParam,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spmm_instrument_parameter_response(spmm_inst_param, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spmm_product_parameter_response(
    sink: &mut TraderSpiSink,
    spmm_product_param: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spmm_product_param: SPMMProductParam,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spmm_product_parameter_response(spmm_product_param, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spbm_add_on_inter_parameter_response(
    sink: &mut TraderSpiSink,
    spbm_add_on_inter_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spbm_add_on_inter_parameter: SPBMAddOnInterParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.spbm_add_on_inter_parameter_response(
            spbm_add_on_inter_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rcams_comb_product_info_response(
    sink: &mut TraderSpiSink,
    rcams_comb_product_info: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rcams_comb_product_info: RCAMSCombProductInfo,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rcams_comb_product_info_response(
            rcams_comb_product_info,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rcams_instrument_parameter_response(
    sink: &mut TraderSpiSink,
    rcams_instr_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rcams_instr_parameter: RCAMSInstrParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rcams_instrument_parameter_response(
            rcams_instr_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rcams_intra_parameter_response(
    sink: &mut TraderSpiSink,
    rcams_intra_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rcams_intra_parameter: RCAMSIntraParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rcams_intra_parameter_response(
            rcams_intra_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rcams_inter_parameter_response(
    sink: &mut TraderSpiSink,
    rcams_inter_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rcams_inter_parameter: RCAMSInterParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rcams_inter_parameter_response(
            rcams_inter_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rcams_short_opt_adjust_parameter_response(
    sink: &mut TraderSpiSink,
    rcams_short_opt_adjust_param: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rcams_short_opt_adjust_param: RCAMSShortOptAdjustParam,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rcams_short_opt_adjust_parameter_response(
            rcams_short_opt_adjust_param,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rcams_investor_comb_position_response(
    sink: &mut TraderSpiSink,
    rcams_investor_comb_position: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rcams_investor_comb_position: RCAMSInvestorCombPosition,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rcams_investor_comb_position_response(
            rcams_investor_comb_position,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_product_rcams_margin_response(
    sink: &mut TraderSpiSink,
    investor_prod_rcams_margin: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_prod_rcams_margin: InvestorProdRCAMSMargin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_product_rcams_margin_response(
            investor_prod_rcams_margin,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rule_instrument_parameter_response(
    sink: &mut TraderSpiSink,
    rule_instr_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rule_instr_parameter: RULEInstrParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.rule_instrument_parameter_response(
            rule_instr_parameter,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn rule_intra_parameter_response(
    sink: &mut TraderSpiSink,
    rule_intra_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rule_intra_parameter: RULEIntraParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .rule_intra_parameter_response(rule_intra_parameter, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn rule_inter_parameter_response(
    sink: &mut TraderSpiSink,
    rule_inter_parameter: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rule_inter_parameter: RULEInterParameter,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .rule_inter_parameter_response(rule_inter_parameter, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn investor_product_rule_margin_response(
    sink: &mut TraderSpiSink,
    investor_prod_rule_margin: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_prod_rule_margin: InvestorProdRULEMargin,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_product_rule_margin_response(
            investor_prod_rule_margin,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_portfolio_setting_response(
    sink: &mut TraderSpiSink,
    investor_portf_setting: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_portf_setting: InvestorPortfSetting,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_portfolio_setting_response(
            investor_portf_setting,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn investor_info_commission_record_response(
    sink: &mut TraderSpiSink,
    investor_info_comm_rec: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, investor_info_comm_rec: InvestorInfoCommRec,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi.investor_info_commission_record_response(
            investor_info_comm_rec,
            rsp_info,
            request_id,
            is_last,
        )
    });
}

pub(crate) unsafe fn comb_leg_response(
    sink: &mut TraderSpiSink,
    comb_leg: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, comb_leg: CombLeg,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .comb_leg_response(comb_leg, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn set_offset_response(
    sink: &mut TraderSpiSink,
    input_offset_setting: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_offset_setting: InputOffsetSetting,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .set_offset_response(input_offset_setting, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn cancel_offset_setting_response(
    sink: &mut TraderSpiSink,
    input_offset_setting: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_offset_setting: InputOffsetSetting,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .cancel_offset_setting_response(input_offset_setting, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn offset_setting_report(sink: &mut TraderSpiSink, offset_setting: *const u8) {
    owned_params!(sink, offset_setting: OffsetSetting);
    invoke(|| sink.spi.offset_setting_report(offset_setting));
}

pub(crate) unsafe fn offset_setting_error(
    sink: &mut TraderSpiSink,
    input_offset_setting: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_offset_setting: InputOffsetSetting,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .offset_setting_error(input_offset_setting, rsp_info)
    });
}

pub(crate) unsafe fn cancel_offset_setting_error(
    sink: &mut TraderSpiSink,
    cancel_offset_setting: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, cancel_offset_setting: CancelOffsetSetting,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .cancel_offset_setting_error(cancel_offset_setting, rsp_info)
    });
}

pub(crate) unsafe fn offset_settings_response(
    sink: &mut TraderSpiSink,
    offset_setting: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, offset_setting: OffsetSetting,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .offset_settings_response(offset_setting, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn gen_sms_code_response(
    sink: &mut TraderSpiSink,
    rsp_gen_sms_code: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, rsp_gen_sms_code: RspGenSMSCode,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .gen_sms_code_response(rsp_gen_sms_code, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spd_apply_response(
    sink: &mut TraderSpiSink,
    input_spd_apply: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_spd_apply: InputSpdApply,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spd_apply_response(input_spd_apply, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spd_apply_action_response(
    sink: &mut TraderSpiSink,
    input_spd_apply_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_spd_apply_action: InputSpdApplyAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spd_apply_action_response(input_spd_apply_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spd_apply_query_response(
    sink: &mut TraderSpiSink,
    spd_apply: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, spd_apply: SpdApply,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .spd_apply_query_response(spd_apply, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn spd_apply_report(sink: &mut TraderSpiSink, spd_apply: *const u8) {
    owned_params!(sink, spd_apply: SpdApply);
    invoke(|| sink.spi.spd_apply_report(spd_apply));
}

pub(crate) unsafe fn spd_apply_error(
    sink: &mut TraderSpiSink,
    input_spd_apply: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_spd_apply: InputSpdApply,
        rsp_info: RspInfo);
    invoke(|| sink.spi.spd_apply_error(input_spd_apply, rsp_info));
}

pub(crate) unsafe fn spd_apply_action_error(
    sink: &mut TraderSpiSink,
    spd_apply_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, spd_apply_action: SpdApplyAction,
        rsp_info: RspInfo);
    invoke(|| sink.spi.spd_apply_action_error(spd_apply_action, rsp_info));
}

pub(crate) unsafe fn hedge_cfm_response(
    sink: &mut TraderSpiSink,
    input_hedge_cfm: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_hedge_cfm: InputHedgeCfm,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .hedge_cfm_response(input_hedge_cfm, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn hedge_cfm_action_response(
    sink: &mut TraderSpiSink,
    input_hedge_cfm_action: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, input_hedge_cfm_action: InputHedgeCfmAction,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .hedge_cfm_action_response(input_hedge_cfm_action, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn hedge_cfm_query_response(
    sink: &mut TraderSpiSink,
    hedge_cfm: *const u8,
    rsp_info: *const u8,
    request_id: i32,
    is_last: bool,
) {
    owned_params!(sink, hedge_cfm: HedgeCfm,
        rsp_info: RspInfo);
    invoke(|| {
        sink.spi
            .hedge_cfm_query_response(hedge_cfm, rsp_info, request_id, is_last)
    });
}

pub(crate) unsafe fn hedge_cfm_report(sink: &mut TraderSpiSink, hedge_cfm: *const u8) {
    owned_params!(sink, hedge_cfm: HedgeCfm);
    invoke(|| sink.spi.hedge_cfm_report(hedge_cfm));
}

pub(crate) unsafe fn hedge_cfm_error(
    sink: &mut TraderSpiSink,
    input_hedge_cfm: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, input_hedge_cfm: InputHedgeCfm,
        rsp_info: RspInfo);
    invoke(|| sink.spi.hedge_cfm_error(input_hedge_cfm, rsp_info));
}

pub(crate) unsafe fn hedge_cfm_action_error(
    sink: &mut TraderSpiSink,
    hedge_cfm_action: *const u8,
    rsp_info: *const u8,
) {
    owned_params!(sink, hedge_cfm_action: HedgeCfmAction,
        rsp_info: RspInfo);
    invoke(|| sink.spi.hedge_cfm_action_error(hedge_cfm_action, rsp_info));
}
