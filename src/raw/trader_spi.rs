// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use crate::sys::convert::ConversionError;
use crate::{
    AccountRegister, BatchOrderAction, BrokerTradingAlgos, BrokerTradingParams, Bulletin,
    CFMMCTradingAccountKey, CFMMCTradingAccountToken, CancelAccount, CancelOffsetSetting,
    ChangeAccount, CombAction, CombInstrumentGuard, CombLeg, CombPromotionParam, ContractBank,
    DepthMarketData, DisconnectReason, EWarrantOffset, ErrorConditionalOrder, Exchange,
    ExchangeMarginRate, ExchangeMarginRateAdjust, ExchangeRate, ExecOrder, ExecOrderAction,
    ForQuote, ForQuoteRsp, HedgeCfm, HedgeCfmAction, InputBatchOrderAction, InputCombAction,
    InputExecOrder, InputExecOrderAction, InputForQuote, InputHedgeCfm, InputHedgeCfmAction,
    InputOffsetSetting, InputOptionSelfClose, InputOptionSelfCloseAction, InputOrder,
    InputOrderAction, InputQuote, InputQuoteAction, InputSpdApply, InputSpdApplyAction,
    InstrTradeLimit, Instrument, InstrumentCommissionRate, InstrumentMarginRate,
    InstrumentOrderCommRate, InstrumentStatus, InvestUnit, Investor,
    InvestorCommodityGroupSPMMMargin, InvestorCommoditySPMMMargin, InvestorInfoCommRec,
    InvestorPortfMarginRatio, InvestorPortfSetting, InvestorPosition,
    InvestorPositionCombineDetail, InvestorPositionDetail, InvestorProdRCAMSMargin,
    InvestorProdRULEMargin, InvestorProdSPBMDetail, InvestorProductGroupMargin,
    MMInstrumentCommissionRate, MMOptionInstrCommRate, Notice, NotifyQueryAccount, OffsetSetting,
    OpenAccount, OptionInstrCommRate, OptionInstrTradeCost, OptionSelfClose, OptionSelfCloseAction,
    Order, OrderAction, ParkedOrder, ParkedOrderAction, Product, ProductExchRate, ProductGroup,
    QryMaxOrderVolume, QryMaxOrderVolumeWithPrice, QueryCFMMCTradingAccountToken, Quote,
    QuoteAction, RCAMSCombProductInfo, RCAMSInstrParameter, RCAMSInterParameter,
    RCAMSIntraParameter, RCAMSInvestorCombPosition, RCAMSShortOptAdjustParam, RULEInstrParameter,
    RULEInterParameter, RULEIntraParameter, RemoveParkedOrder, RemoveParkedOrderAction,
    ReqQueryAccount, ReqRepeal, ReqTransfer, RiskSettleInvestPosition, RiskSettleProductStatus,
    RspAuthenticate, RspGenSMSCode, RspGenUserCaptcha, RspGenUserText, RspInfo, RspRepeal,
    RspTransfer, RspUserAuthMethod, RspUserLogin, SPBMAddOnInterParameter, SPBMFutureParameter,
    SPBMInterParameter, SPBMIntraParameter, SPBMInvestorPortfDef, SPBMOptionParameter,
    SPBMPortfDefinition, SPMMInstParam, SPMMProductParam, SecAgentACIDMap, SecAgentCheckMode,
    SecAgentTradeInfo, SettlementInfo, SettlementInfoConfirm, SpdApply, SpdApplyAction, Trade,
    TraderOffer, TradingAccount, TradingAccountPasswordUpdate, TradingCode, TradingNotice,
    TradingNoticeInfo, TransferBank, TransferSerial, UserLogout, UserPasswordUpdate, UserSession,
};

/// 原生回调线程中的调用方 SPI；只需覆盖关心的方法。
/// 回调参数已经 owned 化，可直接保存或转移；慢 IO 和长时间阻塞应由调用方转交。
#[allow(unused_variables)]
#[allow(clippy::wrong_self_convention)]
pub trait TraderSpi: Send {
    /// owned 转换失败时调用；失败的原回调不会继续分派。
    fn conversion_error(&mut self, error: ConversionError) {}
    fn connected(&mut self) {}
    fn disconnected(&mut self, reason: DisconnectReason) {}
    fn heartbeat(&mut self, time_lapse: i32) {}
    fn authenticate_response(
        &mut self,
        rsp_authenticate: Option<RspAuthenticate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn private_seq_no_report(&mut self, seq_no: i32) {}
    fn login_response(
        &mut self,
        rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn logout_response(
        &mut self,
        user_logout: Option<UserLogout>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn user_password_update_response(
        &mut self,
        user_password_update: Option<UserPasswordUpdate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn trading_account_password_update_response(
        &mut self,
        trading_account_password_update: Option<TradingAccountPasswordUpdate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn user_auth_method_response(
        &mut self,
        rsp_user_auth_method: Option<RspUserAuthMethod>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn gen_user_captcha_response(
        &mut self,
        rsp_gen_user_captcha: Option<RspGenUserCaptcha>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn gen_user_text_response(
        &mut self,
        rsp_gen_user_text: Option<RspGenUserText>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn order_insert_response(
        &mut self,
        input_order: Option<InputOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn parked_order_insert_response(
        &mut self,
        parked_order: Option<ParkedOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn parked_action_response(
        &mut self,
        parked_order_action: Option<ParkedOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn order_action_response(
        &mut self,
        input_order_action: Option<InputOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn max_order_volume_response(
        &mut self,
        qry_max_order_volume: Option<QryMaxOrderVolume>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn settlement_confirm_response(
        &mut self,
        settlement_info_confirm: Option<SettlementInfoConfirm>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn remove_parked_order_response(
        &mut self,
        remove_parked_order: Option<RemoveParkedOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn remove_parked_order_action_response(
        &mut self,
        remove_parked_order_action: Option<RemoveParkedOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn max_order_volume_with_price_query_response(
        &mut self,
        qry_max_order_volume_with_price: Option<QryMaxOrderVolumeWithPrice>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exec_order_insert_response(
        &mut self,
        input_exec_order: Option<InputExecOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exec_order_action_response(
        &mut self,
        input_exec_order_action: Option<InputExecOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn for_quote_insert_response(
        &mut self,
        input_for_quote: Option<InputForQuote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn quote_insert_response(
        &mut self,
        input_quote: Option<InputQuote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn quote_action_response(
        &mut self,
        input_quote_action: Option<InputQuoteAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn batch_order_action_response(
        &mut self,
        input_batch_order_action: Option<InputBatchOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn option_self_close_insert_response(
        &mut self,
        input_option_self_close: Option<InputOptionSelfClose>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn option_self_close_action_response(
        &mut self,
        input_option_self_close_action: Option<InputOptionSelfCloseAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn comb_action_insert_response(
        &mut self,
        input_comb_action: Option<InputCombAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn order_response(
        &mut self,
        order: Option<Order>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn trade_response(
        &mut self,
        trade: Option<Trade>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_position_response(
        &mut self,
        investor_position: Option<InvestorPosition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn trading_account_response(
        &mut self,
        trading_account: Option<TradingAccount>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_response(
        &mut self,
        investor: Option<Investor>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn trading_code_response(
        &mut self,
        trading_code: Option<TradingCode>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn instrument_margin_rate_response(
        &mut self,
        instrument_margin_rate: Option<InstrumentMarginRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn instrument_commission_rate_response(
        &mut self,
        instrument_commission_rate: Option<InstrumentCommissionRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn user_session_response(
        &mut self,
        user_session: Option<UserSession>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exchange_response(
        &mut self,
        exchange: Option<Exchange>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn product_response(
        &mut self,
        product: Option<Product>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn instrument_response(
        &mut self,
        instrument: Option<Instrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn depth_market_data_response(
        &mut self,
        depth_market_data: Option<DepthMarketData>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn trader_offer_response(
        &mut self,
        trader_offer: Option<TraderOffer>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn settlement_info_response(
        &mut self,
        settlement_info: Option<SettlementInfo>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn transfer_bank_response(
        &mut self,
        transfer_bank: Option<TransferBank>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_position_detail_response(
        &mut self,
        investor_position_detail: Option<InvestorPositionDetail>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn notice_response(
        &mut self,
        notice: Option<Notice>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn settlement_status(
        &mut self,
        settlement_info_confirm: Option<SettlementInfoConfirm>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_position_combine_detail_response(
        &mut self,
        investor_position_combine_detail: Option<InvestorPositionCombineDetail>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn cfmmc_trading_account_key_response(
        &mut self,
        cfmmc_trading_account_key: Option<CFMMCTradingAccountKey>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn e_warrant_offset_response(
        &mut self,
        e_warrant_offset: Option<EWarrantOffset>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_product_group_margin_response(
        &mut self,
        investor_product_group_margin: Option<InvestorProductGroupMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exchange_margin_rate_response(
        &mut self,
        exchange_margin_rate: Option<ExchangeMarginRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exchange_margin_rate_adjust_response(
        &mut self,
        exchange_margin_rate_adjust: Option<ExchangeMarginRateAdjust>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exchange_rate_response(
        &mut self,
        exchange_rate: Option<ExchangeRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn sec_agent_acid_map_response(
        &mut self,
        sec_agent_acid_map: Option<SecAgentACIDMap>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn product_exchange_rate_response(
        &mut self,
        product_exch_rate: Option<ProductExchRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn product_group_response(
        &mut self,
        product_group: Option<ProductGroup>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn mm_instrument_commission_rate_response(
        &mut self,
        mm_instrument_commission_rate: Option<MMInstrumentCommissionRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn mm_option_instrument_commission_rate_response(
        &mut self,
        mm_option_instr_comm_rate: Option<MMOptionInstrCommRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn instrument_order_commission_rate_response(
        &mut self,
        instrument_order_comm_rate: Option<InstrumentOrderCommRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn sec_agent_trading_account_response(
        &mut self,
        trading_account: Option<TradingAccount>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn sec_agent_check_mode_response(
        &mut self,
        sec_agent_check_mode: Option<SecAgentCheckMode>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn sec_agent_trade_info_response(
        &mut self,
        sec_agent_trade_info: Option<SecAgentTradeInfo>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn option_instrument_trade_cost_response(
        &mut self,
        option_instr_trade_cost: Option<OptionInstrTradeCost>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn option_instrument_commission_rate_response(
        &mut self,
        option_instr_comm_rate: Option<OptionInstrCommRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn exec_order_response(
        &mut self,
        exec_order: Option<ExecOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn for_quote_query_response(
        &mut self,
        for_quote: Option<ForQuote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn quote_response(
        &mut self,
        quote: Option<Quote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn option_self_close_response(
        &mut self,
        option_self_close: Option<OptionSelfClose>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn invest_unit_response(
        &mut self,
        invest_unit: Option<InvestUnit>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn comb_instrument_guard_response(
        &mut self,
        comb_instrument_guard: Option<CombInstrumentGuard>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn comb_action_response(
        &mut self,
        comb_action: Option<CombAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn transfer_serial_response(
        &mut self,
        transfer_serial: Option<TransferSerial>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn account_register_response(
        &mut self,
        account_register: Option<AccountRegister>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn error(&mut self, rsp_info: Option<RspInfo>, request_id: i32, is_last: bool) {}
    fn order_update(&mut self, order: Option<Order>) {}
    fn execution(&mut self, trade: Option<Trade>) {}
    fn order_insert_error(&mut self, input_order: Option<InputOrder>, rsp_info: Option<RspInfo>) {}
    fn order_action_error(&mut self, order_action: Option<OrderAction>, rsp_info: Option<RspInfo>) {
    }
    fn instrument_status(&mut self, instrument_status: Option<InstrumentStatus>) {}
    fn bulletin(&mut self, bulletin: Option<Bulletin>) {}
    fn notice_received(&mut self, trading_notice_info: Option<TradingNoticeInfo>) {}
    fn conditional_order_error(&mut self, error_conditional_order: Option<ErrorConditionalOrder>) {}
    fn exec_order_report(&mut self, exec_order: Option<ExecOrder>) {}
    fn exec_order_insert_error(
        &mut self,
        input_exec_order: Option<InputExecOrder>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn exec_order_action_error(
        &mut self,
        exec_order_action: Option<ExecOrderAction>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn for_quote_insert_error(
        &mut self,
        input_for_quote: Option<InputForQuote>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn quote_report(&mut self, quote: Option<Quote>) {}
    fn quote_insert_error(&mut self, input_quote: Option<InputQuote>, rsp_info: Option<RspInfo>) {}
    fn quote_action_error(&mut self, quote_action: Option<QuoteAction>, rsp_info: Option<RspInfo>) {
    }
    fn for_quote_response(&mut self, for_quote_rsp: Option<ForQuoteRsp>) {}
    fn cfmmc_account_token(
        &mut self,
        cfmmc_trading_account_token: Option<CFMMCTradingAccountToken>,
    ) {
    }
    fn batch_order_action_error(
        &mut self,
        batch_order_action: Option<BatchOrderAction>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn option_self_close_report(&mut self, option_self_close: Option<OptionSelfClose>) {}
    fn option_self_close_insert_error(
        &mut self,
        input_option_self_close: Option<InputOptionSelfClose>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn option_self_close_action_error(
        &mut self,
        option_self_close_action: Option<OptionSelfCloseAction>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn comb_action_report(&mut self, comb_action: Option<CombAction>) {}
    fn comb_action_insert_error(
        &mut self,
        input_comb_action: Option<InputCombAction>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn contract_bank_response(
        &mut self,
        contract_bank: Option<ContractBank>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn parked_order_response(
        &mut self,
        parked_order: Option<ParkedOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn parked_order_actions(
        &mut self,
        parked_order_action: Option<ParkedOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn notice_history_response(
        &mut self,
        trading_notice: Option<TradingNotice>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn broker_trading_parameters_response(
        &mut self,
        broker_trading_params: Option<BrokerTradingParams>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn broker_trading_algos_response(
        &mut self,
        broker_trading_algos: Option<BrokerTradingAlgos>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn query_cfmmc_trading_account_token_response(
        &mut self,
        query_cfmmc_trading_account_token: Option<QueryCFMMCTradingAccountToken>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn from_bank_to_future_by_bank_report(&mut self, rsp_transfer: Option<RspTransfer>) {}
    fn from_future_to_bank_by_bank_report(&mut self, rsp_transfer: Option<RspTransfer>) {}
    fn repeal_from_bank_to_future_by_bank_report(&mut self, rsp_repeal: Option<RspRepeal>) {}
    fn repeal_from_future_to_bank_by_bank_report(&mut self, rsp_repeal: Option<RspRepeal>) {}
    fn from_bank_to_future_by_future_report(&mut self, rsp_transfer: Option<RspTransfer>) {}
    fn from_future_to_bank_by_future_report(&mut self, rsp_transfer: Option<RspTransfer>) {}
    fn repeal_from_bank_to_future_by_future_manual_report(
        &mut self,
        rsp_repeal: Option<RspRepeal>,
    ) {
    }
    fn repeal_from_future_to_bank_by_future_manual_report(
        &mut self,
        rsp_repeal: Option<RspRepeal>,
    ) {
    }
    fn query_bank_balance_by_future_report(
        &mut self,
        notify_query_account: Option<NotifyQueryAccount>,
    ) {
    }
    fn bank_to_future_by_future_error(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn future_to_bank_by_future_error(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn repeal_bank_to_future_by_future_manual_error(
        &mut self,
        req_repeal: Option<ReqRepeal>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn repeal_future_to_bank_by_future_manual_error(
        &mut self,
        req_repeal: Option<ReqRepeal>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn query_bank_balance_by_future_error(
        &mut self,
        req_query_account: Option<ReqQueryAccount>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn repeal_from_bank_to_future_by_future_report(&mut self, rsp_repeal: Option<RspRepeal>) {}
    fn repeal_from_future_to_bank_by_future_report(&mut self, rsp_repeal: Option<RspRepeal>) {}
    fn from_bank_to_future_by_future_response(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn from_future_to_bank_by_future_response(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn query_bank_account_money_by_future_response(
        &mut self,
        req_query_account: Option<ReqQueryAccount>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn open_account_by_bank_report(&mut self, open_account: Option<OpenAccount>) {}
    fn cancel_account_by_bank_report(&mut self, cancel_account: Option<CancelAccount>) {}
    fn change_account_by_bank_report(&mut self, change_account: Option<ChangeAccount>) {}
    fn classified_instrument_response(
        &mut self,
        instrument: Option<Instrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn comb_promotion_parameter_response(
        &mut self,
        comb_promotion_param: Option<CombPromotionParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn risk_settle_investor_position_response(
        &mut self,
        risk_settle_invest_position: Option<RiskSettleInvestPosition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn risk_settle_product_status_response(
        &mut self,
        risk_settle_product_status: Option<RiskSettleProductStatus>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_future_parameter_response(
        &mut self,
        spbm_future_parameter: Option<SPBMFutureParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_option_parameter_response(
        &mut self,
        spbm_option_parameter: Option<SPBMOptionParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_intra_parameter_response(
        &mut self,
        spbm_intra_parameter: Option<SPBMIntraParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_inter_parameter_response(
        &mut self,
        spbm_inter_parameter: Option<SPBMInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_portfolio_definition_response(
        &mut self,
        spbm_portf_definition: Option<SPBMPortfDefinition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_investor_portfolio_definition_response(
        &mut self,
        spbm_investor_portf_def: Option<SPBMInvestorPortfDef>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_portfolio_margin_ratio_response(
        &mut self,
        investor_portf_margin_ratio: Option<InvestorPortfMarginRatio>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_product_spbm_detail_response(
        &mut self,
        investor_prod_spbm_detail: Option<InvestorProdSPBMDetail>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_commodity_spmm_margin_response(
        &mut self,
        investor_commodity_spmm_margin: Option<InvestorCommoditySPMMMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_commodity_group_spmm_margin_response(
        &mut self,
        investor_commodity_group_spmm_margin: Option<InvestorCommodityGroupSPMMMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spmm_instrument_parameter_response(
        &mut self,
        spmm_inst_param: Option<SPMMInstParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spmm_product_parameter_response(
        &mut self,
        spmm_product_param: Option<SPMMProductParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spbm_add_on_inter_parameter_response(
        &mut self,
        spbm_add_on_inter_parameter: Option<SPBMAddOnInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rcams_comb_product_info_response(
        &mut self,
        rcams_comb_product_info: Option<RCAMSCombProductInfo>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rcams_instrument_parameter_response(
        &mut self,
        rcams_instr_parameter: Option<RCAMSInstrParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rcams_intra_parameter_response(
        &mut self,
        rcams_intra_parameter: Option<RCAMSIntraParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rcams_inter_parameter_response(
        &mut self,
        rcams_inter_parameter: Option<RCAMSInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rcams_short_opt_adjust_parameter_response(
        &mut self,
        rcams_short_opt_adjust_param: Option<RCAMSShortOptAdjustParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rcams_investor_comb_position_response(
        &mut self,
        rcams_investor_comb_position: Option<RCAMSInvestorCombPosition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_product_rcams_margin_response(
        &mut self,
        investor_prod_rcams_margin: Option<InvestorProdRCAMSMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rule_instrument_parameter_response(
        &mut self,
        rule_instr_parameter: Option<RULEInstrParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rule_intra_parameter_response(
        &mut self,
        rule_intra_parameter: Option<RULEIntraParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn rule_inter_parameter_response(
        &mut self,
        rule_inter_parameter: Option<RULEInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_product_rule_margin_response(
        &mut self,
        investor_prod_rule_margin: Option<InvestorProdRULEMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_portfolio_setting_response(
        &mut self,
        investor_portf_setting: Option<InvestorPortfSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn investor_info_commission_record_response(
        &mut self,
        investor_info_comm_rec: Option<InvestorInfoCommRec>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn comb_leg_response(
        &mut self,
        comb_leg: Option<CombLeg>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn set_offset_response(
        &mut self,
        input_offset_setting: Option<InputOffsetSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn cancel_offset_setting_response(
        &mut self,
        input_offset_setting: Option<InputOffsetSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn offset_setting_report(&mut self, offset_setting: Option<OffsetSetting>) {}
    fn offset_setting_error(
        &mut self,
        input_offset_setting: Option<InputOffsetSetting>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn cancel_offset_setting_error(
        &mut self,
        cancel_offset_setting: Option<CancelOffsetSetting>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn offset_settings_response(
        &mut self,
        offset_setting: Option<OffsetSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn gen_sms_code_response(
        &mut self,
        rsp_gen_sms_code: Option<RspGenSMSCode>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spd_apply_response(
        &mut self,
        input_spd_apply: Option<InputSpdApply>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spd_apply_action_response(
        &mut self,
        input_spd_apply_action: Option<InputSpdApplyAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spd_apply_query_response(
        &mut self,
        spd_apply: Option<SpdApply>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn spd_apply_report(&mut self, spd_apply: Option<SpdApply>) {}
    fn spd_apply_error(
        &mut self,
        input_spd_apply: Option<InputSpdApply>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn spd_apply_action_error(
        &mut self,
        spd_apply_action: Option<SpdApplyAction>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn hedge_cfm_response(
        &mut self,
        input_hedge_cfm: Option<InputHedgeCfm>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn hedge_cfm_action_response(
        &mut self,
        input_hedge_cfm_action: Option<InputHedgeCfmAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn hedge_cfm_query_response(
        &mut self,
        hedge_cfm: Option<HedgeCfm>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
    fn hedge_cfm_report(&mut self, hedge_cfm: Option<HedgeCfm>) {}
    fn hedge_cfm_error(
        &mut self,
        input_hedge_cfm: Option<InputHedgeCfm>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn hedge_cfm_action_error(
        &mut self,
        hedge_cfm_action: Option<HedgeCfmAction>,
        rsp_info: Option<RspInfo>,
    ) {
    }
    fn parked_order_report(&mut self, parked_order: Option<ParkedOrder>) {}
    fn parked_order_action_report(&mut self, parked_order_action: Option<ParkedOrderAction>) {}
    fn instrument_trade_limit_query_response(
        &mut self,
        instr_trade_limit: Option<InstrTradeLimit>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
    }
}
