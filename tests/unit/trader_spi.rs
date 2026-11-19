// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use super::trader::test_trader_spi;
use ctp_base::TraderSpi;
use ctp_base::{
    AccountRegister, BatchOrderAction, BrokerTradingAlgos, BrokerTradingParams, Bulletin,
    CFMMCTradingAccountKey, CFMMCTradingAccountToken, CancelAccount, CancelOffsetSetting,
    ChangeAccount, CombAction, CombInstrumentGuard, CombLeg, CombPromotionParam, ContractBank,
    DepthMarketData, DisconnectReason, EWarrantOffset, ErrorConditionalOrder, Exchange,
    ExchangeMarginRate, ExchangeMarginRateAdjust, ExchangeRate, ExecOrder, ExecOrderAction,
    ForQuote, ForQuoteRsp, HedgeCfm, HedgeCfmAction, InputBatchOrderAction, InputCombAction,
    InputExecOrder, InputExecOrderAction, InputForQuote, InputHedgeCfm, InputHedgeCfmAction,
    InputOffsetSetting, InputOptionSelfClose, InputOptionSelfCloseAction, InputOrder,
    InputOrderAction, InputQuote, InputQuoteAction, InputSpdApply, InputSpdApplyAction, Instrument,
    InstrumentCommissionRate, InstrumentMarginRate, InstrumentOrderCommRate, InstrumentStatus,
    InvestUnit, Investor, InvestorCommodityGroupSPMMMargin, InvestorCommoditySPMMMargin,
    InvestorInfoCommRec, InvestorPortfMarginRatio, InvestorPortfSetting, InvestorPosition,
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
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct RecordingSpi(Arc<Mutex<Vec<&'static str>>>);

impl RecordingSpi {
    fn push(&self, name: &'static str) {
        self.0.lock().unwrap().push(name);
    }
}

impl TraderSpi for RecordingSpi {
    fn connected(&mut self) {
        self.push("connected");
    }
    fn disconnected(&mut self, _reason: DisconnectReason) {
        self.push("disconnected");
    }
    fn heartbeat(&mut self, _time_lapse: i32) {
        self.push("heartbeat");
    }
    fn authenticate_response(
        &mut self,
        _rsp_authenticate: Option<RspAuthenticate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("authenticate_response");
    }
    fn private_seq_no_report(&mut self, _seq_no: i32) {
        self.push("private_seq_no_report");
    }
    fn login_response(
        &mut self,
        _rsp_user_login: Option<RspUserLogin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("login_response");
    }
    fn logout_response(
        &mut self,
        _user_logout: Option<UserLogout>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("logout_response");
    }
    fn user_password_update_response(
        &mut self,
        _user_password_update: Option<UserPasswordUpdate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("user_password_update_response");
    }
    fn trading_account_password_update_response(
        &mut self,
        _trading_account_password_update: Option<TradingAccountPasswordUpdate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("trading_account_password_update_response");
    }
    fn user_auth_method_response(
        &mut self,
        _rsp_user_auth_method: Option<RspUserAuthMethod>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("user_auth_method_response");
    }
    fn gen_user_captcha_response(
        &mut self,
        _rsp_gen_user_captcha: Option<RspGenUserCaptcha>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("gen_user_captcha_response");
    }
    fn gen_user_text_response(
        &mut self,
        _rsp_gen_user_text: Option<RspGenUserText>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("gen_user_text_response");
    }
    fn order_insert_response(
        &mut self,
        _input_order: Option<InputOrder>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("order_insert_response");
    }
    fn parked_order_insert_response(
        &mut self,
        _parked_order: Option<ParkedOrder>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("parked_order_insert_response");
    }
    fn parked_action_response(
        &mut self,
        _parked_order_action: Option<ParkedOrderAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("parked_action_response");
    }
    fn order_action_response(
        &mut self,
        _input_order_action: Option<InputOrderAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("order_action_response");
    }
    fn max_order_volume_response(
        &mut self,
        _qry_max_order_volume: Option<QryMaxOrderVolume>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("max_order_volume_response");
    }
    fn settlement_confirm_response(
        &mut self,
        _settlement_info_confirm: Option<SettlementInfoConfirm>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("settlement_confirm_response");
    }
    fn remove_parked_order_response(
        &mut self,
        _remove_parked_order: Option<RemoveParkedOrder>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("remove_parked_order_response");
    }
    fn remove_parked_order_action_response(
        &mut self,
        _remove_parked_order_action: Option<RemoveParkedOrderAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("remove_parked_order_action_response");
    }
    fn exec_order_insert_response(
        &mut self,
        _input_exec_order: Option<InputExecOrder>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exec_order_insert_response");
    }
    fn exec_order_action_response(
        &mut self,
        _input_exec_order_action: Option<InputExecOrderAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exec_order_action_response");
    }
    fn for_quote_insert_response(
        &mut self,
        _input_for_quote: Option<InputForQuote>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("for_quote_insert_response");
    }
    fn quote_insert_response(
        &mut self,
        _input_quote: Option<InputQuote>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("quote_insert_response");
    }
    fn quote_action_response(
        &mut self,
        _input_quote_action: Option<InputQuoteAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("quote_action_response");
    }
    fn batch_order_action_response(
        &mut self,
        _input_batch_order_action: Option<InputBatchOrderAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("batch_order_action_response");
    }
    fn option_self_close_insert_response(
        &mut self,
        _input_option_self_close: Option<InputOptionSelfClose>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("option_self_close_insert_response");
    }
    fn option_self_close_action_response(
        &mut self,
        _input_option_self_close_action: Option<InputOptionSelfCloseAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("option_self_close_action_response");
    }
    fn comb_action_insert_response(
        &mut self,
        _input_comb_action: Option<InputCombAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("comb_action_insert_response");
    }
    fn order_response(
        &mut self,
        _order: Option<Order>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("order_response");
    }
    fn trade_response(
        &mut self,
        _trade: Option<Trade>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("trade_response");
    }
    fn investor_position_response(
        &mut self,
        _investor_position: Option<InvestorPosition>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_position_response");
    }
    fn trading_account_response(
        &mut self,
        _trading_account: Option<TradingAccount>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("trading_account_response");
    }
    fn investor_response(
        &mut self,
        _investor: Option<Investor>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_response");
    }
    fn trading_code_response(
        &mut self,
        _trading_code: Option<TradingCode>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("trading_code_response");
    }
    fn instrument_margin_rate_response(
        &mut self,
        _instrument_margin_rate: Option<InstrumentMarginRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("instrument_margin_rate_response");
    }
    fn instrument_commission_rate_response(
        &mut self,
        _instrument_commission_rate: Option<InstrumentCommissionRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("instrument_commission_rate_response");
    }
    fn user_session_response(
        &mut self,
        _user_session: Option<UserSession>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("user_session_response");
    }
    fn exchange_response(
        &mut self,
        _exchange: Option<Exchange>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exchange_response");
    }
    fn product_response(
        &mut self,
        _product: Option<Product>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("product_response");
    }
    fn instrument_response(
        &mut self,
        _instrument: Option<Instrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("instrument_response");
    }
    fn depth_market_data_response(
        &mut self,
        _depth_market_data: Option<DepthMarketData>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("depth_market_data_response");
    }
    fn trader_offer_response(
        &mut self,
        _trader_offer: Option<TraderOffer>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("trader_offer_response");
    }
    fn settlement_info_response(
        &mut self,
        _settlement_info: Option<SettlementInfo>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("settlement_info_response");
    }
    fn transfer_bank_response(
        &mut self,
        _transfer_bank: Option<TransferBank>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("transfer_bank_response");
    }
    fn investor_position_detail_response(
        &mut self,
        _investor_position_detail: Option<InvestorPositionDetail>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_position_detail_response");
    }
    fn notice_response(
        &mut self,
        _notice: Option<Notice>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("notice_response");
    }
    fn settlement_status(
        &mut self,
        _settlement_info_confirm: Option<SettlementInfoConfirm>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("settlement_status");
    }
    fn investor_position_combine_detail_response(
        &mut self,
        _investor_position_combine_detail: Option<InvestorPositionCombineDetail>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_position_combine_detail_response");
    }
    fn cfmmc_trading_account_key_response(
        &mut self,
        _cfmmc_trading_account_key: Option<CFMMCTradingAccountKey>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("cfmmc_trading_account_key_response");
    }
    fn e_warrant_offset_response(
        &mut self,
        _e_warrant_offset: Option<EWarrantOffset>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("e_warrant_offset_response");
    }
    fn investor_product_group_margin_response(
        &mut self,
        _investor_product_group_margin: Option<InvestorProductGroupMargin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_product_group_margin_response");
    }
    fn exchange_margin_rate_response(
        &mut self,
        _exchange_margin_rate: Option<ExchangeMarginRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exchange_margin_rate_response");
    }
    fn exchange_margin_rate_adjust_response(
        &mut self,
        _exchange_margin_rate_adjust: Option<ExchangeMarginRateAdjust>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exchange_margin_rate_adjust_response");
    }
    fn exchange_rate_response(
        &mut self,
        _exchange_rate: Option<ExchangeRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exchange_rate_response");
    }
    fn sec_agent_acid_map_response(
        &mut self,
        _sec_agent_acid_map: Option<SecAgentACIDMap>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("sec_agent_acid_map_response");
    }
    fn product_exchange_rate_response(
        &mut self,
        _product_exch_rate: Option<ProductExchRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("product_exchange_rate_response");
    }
    fn product_group_response(
        &mut self,
        _product_group: Option<ProductGroup>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("product_group_response");
    }
    fn mm_instrument_commission_rate_response(
        &mut self,
        _mm_instrument_commission_rate: Option<MMInstrumentCommissionRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("mm_instrument_commission_rate_response");
    }
    fn mm_option_instrument_commission_rate_response(
        &mut self,
        _mm_option_instr_comm_rate: Option<MMOptionInstrCommRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("mm_option_instrument_commission_rate_response");
    }
    fn instrument_order_commission_rate_response(
        &mut self,
        _instrument_order_comm_rate: Option<InstrumentOrderCommRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("instrument_order_commission_rate_response");
    }
    fn sec_agent_trading_account_response(
        &mut self,
        _trading_account: Option<TradingAccount>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("sec_agent_trading_account_response");
    }
    fn sec_agent_check_mode_response(
        &mut self,
        _sec_agent_check_mode: Option<SecAgentCheckMode>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("sec_agent_check_mode_response");
    }
    fn sec_agent_trade_info_response(
        &mut self,
        _sec_agent_trade_info: Option<SecAgentTradeInfo>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("sec_agent_trade_info_response");
    }
    fn option_instrument_trade_cost_response(
        &mut self,
        _option_instr_trade_cost: Option<OptionInstrTradeCost>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("option_instrument_trade_cost_response");
    }
    fn option_instrument_commission_rate_response(
        &mut self,
        _option_instr_comm_rate: Option<OptionInstrCommRate>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("option_instrument_commission_rate_response");
    }
    fn exec_order_response(
        &mut self,
        _exec_order: Option<ExecOrder>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("exec_order_response");
    }
    fn for_quote_query_response(
        &mut self,
        _for_quote: Option<ForQuote>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("for_quote_query_response");
    }
    fn quote_response(
        &mut self,
        _quote: Option<Quote>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("quote_response");
    }
    fn option_self_close_response(
        &mut self,
        _option_self_close: Option<OptionSelfClose>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("option_self_close_response");
    }
    fn invest_unit_response(
        &mut self,
        _invest_unit: Option<InvestUnit>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("invest_unit_response");
    }
    fn comb_instrument_guard_response(
        &mut self,
        _comb_instrument_guard: Option<CombInstrumentGuard>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("comb_instrument_guard_response");
    }
    fn comb_action_response(
        &mut self,
        _comb_action: Option<CombAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("comb_action_response");
    }
    fn transfer_serial_response(
        &mut self,
        _transfer_serial: Option<TransferSerial>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("transfer_serial_response");
    }
    fn account_register_response(
        &mut self,
        _account_register: Option<AccountRegister>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("account_register_response");
    }
    fn error(&mut self, _rsp_info: Option<RspInfo>, _request_id: i32, _is_last: bool) {
        self.push("error");
    }
    fn order_update(&mut self, _order: Option<Order>) {
        self.push("order_update");
    }
    fn execution(&mut self, _trade: Option<Trade>) {
        self.push("execution");
    }
    fn order_insert_error(&mut self, _input_order: Option<InputOrder>, _rsp_info: Option<RspInfo>) {
        self.push("order_insert_error");
    }
    fn order_action_error(
        &mut self,
        _order_action: Option<OrderAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("order_action_error");
    }
    fn instrument_status(&mut self, _instrument_status: Option<InstrumentStatus>) {
        self.push("instrument_status");
    }
    fn bulletin(&mut self, _bulletin: Option<Bulletin>) {
        self.push("bulletin");
    }
    fn notice_received(&mut self, _trading_notice_info: Option<TradingNoticeInfo>) {
        self.push("notice_received");
    }
    fn conditional_order_error(&mut self, _error_conditional_order: Option<ErrorConditionalOrder>) {
        self.push("conditional_order_error");
    }
    fn exec_order_report(&mut self, _exec_order: Option<ExecOrder>) {
        self.push("exec_order_report");
    }
    fn exec_order_insert_error(
        &mut self,
        _input_exec_order: Option<InputExecOrder>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("exec_order_insert_error");
    }
    fn exec_order_action_error(
        &mut self,
        _exec_order_action: Option<ExecOrderAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("exec_order_action_error");
    }
    fn for_quote_insert_error(
        &mut self,
        _input_for_quote: Option<InputForQuote>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("for_quote_insert_error");
    }
    fn quote_report(&mut self, _quote: Option<Quote>) {
        self.push("quote_report");
    }
    fn quote_insert_error(&mut self, _input_quote: Option<InputQuote>, _rsp_info: Option<RspInfo>) {
        self.push("quote_insert_error");
    }
    fn quote_action_error(
        &mut self,
        _quote_action: Option<QuoteAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("quote_action_error");
    }
    fn for_quote_response(&mut self, _for_quote_rsp: Option<ForQuoteRsp>) {
        self.push("for_quote_response");
    }
    fn cfmmc_account_token(
        &mut self,
        _cfmmc_trading_account_token: Option<CFMMCTradingAccountToken>,
    ) {
        self.push("cfmmc_account_token");
    }
    fn batch_order_action_error(
        &mut self,
        _batch_order_action: Option<BatchOrderAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("batch_order_action_error");
    }
    fn option_self_close_report(&mut self, _option_self_close: Option<OptionSelfClose>) {
        self.push("option_self_close_report");
    }
    fn option_self_close_insert_error(
        &mut self,
        _input_option_self_close: Option<InputOptionSelfClose>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("option_self_close_insert_error");
    }
    fn option_self_close_action_error(
        &mut self,
        _option_self_close_action: Option<OptionSelfCloseAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("option_self_close_action_error");
    }
    fn comb_action_report(&mut self, _comb_action: Option<CombAction>) {
        self.push("comb_action_report");
    }
    fn comb_action_insert_error(
        &mut self,
        _input_comb_action: Option<InputCombAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("comb_action_insert_error");
    }
    fn contract_bank_response(
        &mut self,
        _contract_bank: Option<ContractBank>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("contract_bank_response");
    }
    fn parked_order_response(
        &mut self,
        _parked_order: Option<ParkedOrder>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("parked_order_response");
    }
    fn parked_order_actions(
        &mut self,
        _parked_order_action: Option<ParkedOrderAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("parked_order_actions");
    }
    fn notice_history_response(
        &mut self,
        _trading_notice: Option<TradingNotice>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("notice_history_response");
    }
    fn broker_trading_parameters_response(
        &mut self,
        _broker_trading_params: Option<BrokerTradingParams>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("broker_trading_parameters_response");
    }
    fn broker_trading_algos_response(
        &mut self,
        _broker_trading_algos: Option<BrokerTradingAlgos>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("broker_trading_algos_response");
    }
    fn query_cfmmc_trading_account_token_response(
        &mut self,
        _query_cfmmc_trading_account_token: Option<QueryCFMMCTradingAccountToken>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("query_cfmmc_trading_account_token_response");
    }
    fn from_bank_to_future_by_bank_report(&mut self, _rsp_transfer: Option<RspTransfer>) {
        self.push("from_bank_to_future_by_bank_report");
    }
    fn from_future_to_bank_by_bank_report(&mut self, _rsp_transfer: Option<RspTransfer>) {
        self.push("from_future_to_bank_by_bank_report");
    }
    fn repeal_from_bank_to_future_by_bank_report(&mut self, _rsp_repeal: Option<RspRepeal>) {
        self.push("repeal_from_bank_to_future_by_bank_report");
    }
    fn repeal_from_future_to_bank_by_bank_report(&mut self, _rsp_repeal: Option<RspRepeal>) {
        self.push("repeal_from_future_to_bank_by_bank_report");
    }
    fn from_bank_to_future_by_future_report(&mut self, _rsp_transfer: Option<RspTransfer>) {
        self.push("from_bank_to_future_by_future_report");
    }
    fn from_future_to_bank_by_future_report(&mut self, _rsp_transfer: Option<RspTransfer>) {
        self.push("from_future_to_bank_by_future_report");
    }
    fn repeal_from_bank_to_future_by_future_manual_report(
        &mut self,
        _rsp_repeal: Option<RspRepeal>,
    ) {
        self.push("repeal_from_bank_to_future_by_future_manual_report");
    }
    fn repeal_from_future_to_bank_by_future_manual_report(
        &mut self,
        _rsp_repeal: Option<RspRepeal>,
    ) {
        self.push("repeal_from_future_to_bank_by_future_manual_report");
    }
    fn query_bank_balance_by_future_report(
        &mut self,
        _notify_query_account: Option<NotifyQueryAccount>,
    ) {
        self.push("query_bank_balance_by_future_report");
    }
    fn bank_to_future_by_future_error(
        &mut self,
        _req_transfer: Option<ReqTransfer>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("bank_to_future_by_future_error");
    }
    fn future_to_bank_by_future_error(
        &mut self,
        _req_transfer: Option<ReqTransfer>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("future_to_bank_by_future_error");
    }
    fn repeal_bank_to_future_by_future_manual_error(
        &mut self,
        _req_repeal: Option<ReqRepeal>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("repeal_bank_to_future_by_future_manual_error");
    }
    fn repeal_future_to_bank_by_future_manual_error(
        &mut self,
        _req_repeal: Option<ReqRepeal>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("repeal_future_to_bank_by_future_manual_error");
    }
    fn query_bank_balance_by_future_error(
        &mut self,
        _req_query_account: Option<ReqQueryAccount>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("query_bank_balance_by_future_error");
    }
    fn repeal_from_bank_to_future_by_future_report(&mut self, _rsp_repeal: Option<RspRepeal>) {
        self.push("repeal_from_bank_to_future_by_future_report");
    }
    fn repeal_from_future_to_bank_by_future_report(&mut self, _rsp_repeal: Option<RspRepeal>) {
        self.push("repeal_from_future_to_bank_by_future_report");
    }
    fn from_bank_to_future_by_future_response(
        &mut self,
        _req_transfer: Option<ReqTransfer>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("from_bank_to_future_by_future_response");
    }
    fn from_future_to_bank_by_future_response(
        &mut self,
        _req_transfer: Option<ReqTransfer>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("from_future_to_bank_by_future_response");
    }
    fn query_bank_account_money_by_future_response(
        &mut self,
        _req_query_account: Option<ReqQueryAccount>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("query_bank_account_money_by_future_response");
    }
    fn open_account_by_bank_report(&mut self, _open_account: Option<OpenAccount>) {
        self.push("open_account_by_bank_report");
    }
    fn cancel_account_by_bank_report(&mut self, _cancel_account: Option<CancelAccount>) {
        self.push("cancel_account_by_bank_report");
    }
    fn change_account_by_bank_report(&mut self, _change_account: Option<ChangeAccount>) {
        self.push("change_account_by_bank_report");
    }
    fn classified_instrument_response(
        &mut self,
        _instrument: Option<Instrument>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("classified_instrument_response");
    }
    fn comb_promotion_parameter_response(
        &mut self,
        _comb_promotion_param: Option<CombPromotionParam>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("comb_promotion_parameter_response");
    }
    fn risk_settle_investor_position_response(
        &mut self,
        _risk_settle_invest_position: Option<RiskSettleInvestPosition>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("risk_settle_investor_position_response");
    }
    fn risk_settle_product_status_response(
        &mut self,
        _risk_settle_product_status: Option<RiskSettleProductStatus>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("risk_settle_product_status_response");
    }
    fn spbm_future_parameter_response(
        &mut self,
        _spbm_future_parameter: Option<SPBMFutureParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_future_parameter_response");
    }
    fn spbm_option_parameter_response(
        &mut self,
        _spbm_option_parameter: Option<SPBMOptionParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_option_parameter_response");
    }
    fn spbm_intra_parameter_response(
        &mut self,
        _spbm_intra_parameter: Option<SPBMIntraParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_intra_parameter_response");
    }
    fn spbm_inter_parameter_response(
        &mut self,
        _spbm_inter_parameter: Option<SPBMInterParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_inter_parameter_response");
    }
    fn spbm_portfolio_definition_response(
        &mut self,
        _spbm_portf_definition: Option<SPBMPortfDefinition>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_portfolio_definition_response");
    }
    fn spbm_investor_portfolio_definition_response(
        &mut self,
        _spbm_investor_portf_def: Option<SPBMInvestorPortfDef>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_investor_portfolio_definition_response");
    }
    fn investor_portfolio_margin_ratio_response(
        &mut self,
        _investor_portf_margin_ratio: Option<InvestorPortfMarginRatio>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_portfolio_margin_ratio_response");
    }
    fn investor_product_spbm_detail_response(
        &mut self,
        _investor_prod_spbm_detail: Option<InvestorProdSPBMDetail>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_product_spbm_detail_response");
    }
    fn investor_commodity_spmm_margin_response(
        &mut self,
        _investor_commodity_spmm_margin: Option<InvestorCommoditySPMMMargin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_commodity_spmm_margin_response");
    }
    fn investor_commodity_group_spmm_margin_response(
        &mut self,
        _investor_commodity_group_spmm_margin: Option<InvestorCommodityGroupSPMMMargin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_commodity_group_spmm_margin_response");
    }
    fn spmm_instrument_parameter_response(
        &mut self,
        _spmm_inst_param: Option<SPMMInstParam>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spmm_instrument_parameter_response");
    }
    fn spmm_product_parameter_response(
        &mut self,
        _spmm_product_param: Option<SPMMProductParam>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spmm_product_parameter_response");
    }
    fn spbm_add_on_inter_parameter_response(
        &mut self,
        _spbm_add_on_inter_parameter: Option<SPBMAddOnInterParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spbm_add_on_inter_parameter_response");
    }
    fn rcams_comb_product_info_response(
        &mut self,
        _rcams_comb_product_info: Option<RCAMSCombProductInfo>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rcams_comb_product_info_response");
    }
    fn rcams_instrument_parameter_response(
        &mut self,
        _rcams_instr_parameter: Option<RCAMSInstrParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rcams_instrument_parameter_response");
    }
    fn rcams_intra_parameter_response(
        &mut self,
        _rcams_intra_parameter: Option<RCAMSIntraParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rcams_intra_parameter_response");
    }
    fn rcams_inter_parameter_response(
        &mut self,
        _rcams_inter_parameter: Option<RCAMSInterParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rcams_inter_parameter_response");
    }
    fn rcams_short_opt_adjust_parameter_response(
        &mut self,
        _rcams_short_opt_adjust_param: Option<RCAMSShortOptAdjustParam>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rcams_short_opt_adjust_parameter_response");
    }
    fn rcams_investor_comb_position_response(
        &mut self,
        _rcams_investor_comb_position: Option<RCAMSInvestorCombPosition>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rcams_investor_comb_position_response");
    }
    fn investor_product_rcams_margin_response(
        &mut self,
        _investor_prod_rcams_margin: Option<InvestorProdRCAMSMargin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_product_rcams_margin_response");
    }
    fn rule_instrument_parameter_response(
        &mut self,
        _rule_instr_parameter: Option<RULEInstrParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rule_instrument_parameter_response");
    }
    fn rule_intra_parameter_response(
        &mut self,
        _rule_intra_parameter: Option<RULEIntraParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rule_intra_parameter_response");
    }
    fn rule_inter_parameter_response(
        &mut self,
        _rule_inter_parameter: Option<RULEInterParameter>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("rule_inter_parameter_response");
    }
    fn investor_product_rule_margin_response(
        &mut self,
        _investor_prod_rule_margin: Option<InvestorProdRULEMargin>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_product_rule_margin_response");
    }
    fn investor_portfolio_setting_response(
        &mut self,
        _investor_portf_setting: Option<InvestorPortfSetting>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_portfolio_setting_response");
    }
    fn investor_info_commission_record_response(
        &mut self,
        _investor_info_comm_rec: Option<InvestorInfoCommRec>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("investor_info_commission_record_response");
    }
    fn comb_leg_response(
        &mut self,
        _comb_leg: Option<CombLeg>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("comb_leg_response");
    }
    fn set_offset_response(
        &mut self,
        _input_offset_setting: Option<InputOffsetSetting>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("set_offset_response");
    }
    fn cancel_offset_setting_response(
        &mut self,
        _input_offset_setting: Option<InputOffsetSetting>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("cancel_offset_setting_response");
    }
    fn offset_setting_report(&mut self, _offset_setting: Option<OffsetSetting>) {
        self.push("offset_setting_report");
    }
    fn offset_setting_error(
        &mut self,
        _input_offset_setting: Option<InputOffsetSetting>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("offset_setting_error");
    }
    fn cancel_offset_setting_error(
        &mut self,
        _cancel_offset_setting: Option<CancelOffsetSetting>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("cancel_offset_setting_error");
    }
    fn offset_settings_response(
        &mut self,
        _offset_setting: Option<OffsetSetting>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("offset_settings_response");
    }
    fn gen_sms_code_response(
        &mut self,
        _rsp_gen_sms_code: Option<RspGenSMSCode>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("gen_sms_code_response");
    }
    fn spd_apply_response(
        &mut self,
        _input_spd_apply: Option<InputSpdApply>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spd_apply_response");
    }
    fn spd_apply_action_response(
        &mut self,
        _input_spd_apply_action: Option<InputSpdApplyAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spd_apply_action_response");
    }
    fn spd_apply_query_response(
        &mut self,
        _spd_apply: Option<SpdApply>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("spd_apply_query_response");
    }
    fn spd_apply_report(&mut self, _spd_apply: Option<SpdApply>) {
        self.push("spd_apply_report");
    }
    fn spd_apply_error(
        &mut self,
        _input_spd_apply: Option<InputSpdApply>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("spd_apply_error");
    }
    fn spd_apply_action_error(
        &mut self,
        _spd_apply_action: Option<SpdApplyAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("spd_apply_action_error");
    }
    fn hedge_cfm_response(
        &mut self,
        _input_hedge_cfm: Option<InputHedgeCfm>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("hedge_cfm_response");
    }
    fn hedge_cfm_action_response(
        &mut self,
        _input_hedge_cfm_action: Option<InputHedgeCfmAction>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("hedge_cfm_action_response");
    }
    fn hedge_cfm_query_response(
        &mut self,
        _hedge_cfm: Option<HedgeCfm>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.push("hedge_cfm_query_response");
    }
    fn hedge_cfm_report(&mut self, _hedge_cfm: Option<HedgeCfm>) {
        self.push("hedge_cfm_report");
    }
    fn hedge_cfm_error(
        &mut self,
        _input_hedge_cfm: Option<InputHedgeCfm>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("hedge_cfm_error");
    }
    fn hedge_cfm_action_error(
        &mut self,
        _hedge_cfm_action: Option<HedgeCfmAction>,
        _rsp_info: Option<RspInfo>,
    ) {
        self.push("hedge_cfm_action_error");
    }
}

#[test]
fn all_trader_callbacks_reach_caller_spi() {
    let spi = RecordingSpi::default();
    let seen = spi.0.clone();
    let mut bridge = test_trader_spi(spi);
    for index in 0..178 {
        bridge.emit_callback(index);
    }
    assert_eq!(
        *seen.lock().unwrap(),
        vec![
            "connected",
            "disconnected",
            "heartbeat",
            "authenticate_response",
            "private_seq_no_report",
            "login_response",
            "logout_response",
            "user_password_update_response",
            "trading_account_password_update_response",
            "user_auth_method_response",
            "gen_user_captcha_response",
            "gen_user_text_response",
            "order_insert_response",
            "parked_order_insert_response",
            "parked_action_response",
            "order_action_response",
            "max_order_volume_response",
            "settlement_confirm_response",
            "remove_parked_order_response",
            "remove_parked_order_action_response",
            "exec_order_insert_response",
            "exec_order_action_response",
            "for_quote_insert_response",
            "quote_insert_response",
            "quote_action_response",
            "batch_order_action_response",
            "option_self_close_insert_response",
            "option_self_close_action_response",
            "comb_action_insert_response",
            "order_response",
            "trade_response",
            "investor_position_response",
            "trading_account_response",
            "investor_response",
            "trading_code_response",
            "instrument_margin_rate_response",
            "instrument_commission_rate_response",
            "user_session_response",
            "exchange_response",
            "product_response",
            "instrument_response",
            "depth_market_data_response",
            "trader_offer_response",
            "settlement_info_response",
            "transfer_bank_response",
            "investor_position_detail_response",
            "notice_response",
            "settlement_status",
            "investor_position_combine_detail_response",
            "cfmmc_trading_account_key_response",
            "e_warrant_offset_response",
            "investor_product_group_margin_response",
            "exchange_margin_rate_response",
            "exchange_margin_rate_adjust_response",
            "exchange_rate_response",
            "sec_agent_acid_map_response",
            "product_exchange_rate_response",
            "product_group_response",
            "mm_instrument_commission_rate_response",
            "mm_option_instrument_commission_rate_response",
            "instrument_order_commission_rate_response",
            "sec_agent_trading_account_response",
            "sec_agent_check_mode_response",
            "sec_agent_trade_info_response",
            "option_instrument_trade_cost_response",
            "option_instrument_commission_rate_response",
            "exec_order_response",
            "for_quote_query_response",
            "quote_response",
            "option_self_close_response",
            "invest_unit_response",
            "comb_instrument_guard_response",
            "comb_action_response",
            "transfer_serial_response",
            "account_register_response",
            "error",
            "order_update",
            "execution",
            "order_insert_error",
            "order_action_error",
            "instrument_status",
            "bulletin",
            "notice_received",
            "conditional_order_error",
            "exec_order_report",
            "exec_order_insert_error",
            "exec_order_action_error",
            "for_quote_insert_error",
            "quote_report",
            "quote_insert_error",
            "quote_action_error",
            "for_quote_response",
            "cfmmc_account_token",
            "batch_order_action_error",
            "option_self_close_report",
            "option_self_close_insert_error",
            "option_self_close_action_error",
            "comb_action_report",
            "comb_action_insert_error",
            "contract_bank_response",
            "parked_order_response",
            "parked_order_actions",
            "notice_history_response",
            "broker_trading_parameters_response",
            "broker_trading_algos_response",
            "query_cfmmc_trading_account_token_response",
            "from_bank_to_future_by_bank_report",
            "from_future_to_bank_by_bank_report",
            "repeal_from_bank_to_future_by_bank_report",
            "repeal_from_future_to_bank_by_bank_report",
            "from_bank_to_future_by_future_report",
            "from_future_to_bank_by_future_report",
            "repeal_from_bank_to_future_by_future_manual_report",
            "repeal_from_future_to_bank_by_future_manual_report",
            "query_bank_balance_by_future_report",
            "bank_to_future_by_future_error",
            "future_to_bank_by_future_error",
            "repeal_bank_to_future_by_future_manual_error",
            "repeal_future_to_bank_by_future_manual_error",
            "query_bank_balance_by_future_error",
            "repeal_from_bank_to_future_by_future_report",
            "repeal_from_future_to_bank_by_future_report",
            "from_bank_to_future_by_future_response",
            "from_future_to_bank_by_future_response",
            "query_bank_account_money_by_future_response",
            "open_account_by_bank_report",
            "cancel_account_by_bank_report",
            "change_account_by_bank_report",
            "classified_instrument_response",
            "comb_promotion_parameter_response",
            "risk_settle_investor_position_response",
            "risk_settle_product_status_response",
            "spbm_future_parameter_response",
            "spbm_option_parameter_response",
            "spbm_intra_parameter_response",
            "spbm_inter_parameter_response",
            "spbm_portfolio_definition_response",
            "spbm_investor_portfolio_definition_response",
            "investor_portfolio_margin_ratio_response",
            "investor_product_spbm_detail_response",
            "investor_commodity_spmm_margin_response",
            "investor_commodity_group_spmm_margin_response",
            "spmm_instrument_parameter_response",
            "spmm_product_parameter_response",
            "spbm_add_on_inter_parameter_response",
            "rcams_comb_product_info_response",
            "rcams_instrument_parameter_response",
            "rcams_intra_parameter_response",
            "rcams_inter_parameter_response",
            "rcams_short_opt_adjust_parameter_response",
            "rcams_investor_comb_position_response",
            "investor_product_rcams_margin_response",
            "rule_instrument_parameter_response",
            "rule_intra_parameter_response",
            "rule_inter_parameter_response",
            "investor_product_rule_margin_response",
            "investor_portfolio_setting_response",
            "investor_info_commission_record_response",
            "comb_leg_response",
            "set_offset_response",
            "cancel_offset_setting_response",
            "offset_setting_report",
            "offset_setting_error",
            "cancel_offset_setting_error",
            "offset_settings_response",
            "gen_sms_code_response",
            "spd_apply_response",
            "spd_apply_action_response",
            "spd_apply_query_response",
            "spd_apply_report",
            "spd_apply_error",
            "spd_apply_action_error",
            "hedge_cfm_response",
            "hedge_cfm_action_response",
            "hedge_cfm_query_response",
            "hedge_cfm_report",
            "hedge_cfm_error",
            "hedge_cfm_action_error"
        ]
    );
}
