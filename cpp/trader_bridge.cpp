#include "cpp/bridge.h"

#include <utility>
#include <vector>

namespace ctp_base::trader {

TraderSpiBridge::TraderSpiBridge(rust::Box<TraderSpiSink> sink) : sink_(std::move(sink)) {}

void TraderSpiBridge::OnFrontConnected() {
    connected(*sink_);
}

void TraderSpiBridge::OnFrontDisconnected(int nReason) {
    disconnected(*sink_, nReason);
}

void TraderSpiBridge::OnHeartBeatWarning(int nTimeLapse) {
    heartbeat(*sink_, nTimeLapse);
}

void TraderSpiBridge::OnRspAuthenticate(CThostFtdcRspAuthenticateField* pRspAuthenticateField, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    authenticate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspAuthenticateField), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnPrivateSeqNo(int nSeqNo) {
    private_seq_no_report(*sink_, nSeqNo);
}

void TraderSpiBridge::OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    login_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspUserLogin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    logout_response(*sink_, reinterpret_cast<const std::uint8_t*>(pUserLogout), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    user_password_update_response(*sink_, reinterpret_cast<const std::uint8_t*>(pUserPasswordUpdate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField* pTradingAccountPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    trading_account_password_update_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTradingAccountPasswordUpdate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField* pRspUserAuthMethod, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    user_auth_method_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspUserAuthMethod), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField* pRspGenUserCaptcha, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    gen_user_captcha_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspGenUserCaptcha), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspGenUserText(CThostFtdcRspGenUserTextField* pRspGenUserText, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    gen_user_text_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspGenUserText), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    order_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspParkedOrderInsert(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    parked_order_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pParkedOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    parked_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pParkedOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspOrderAction(CThostFtdcInputOrderActionField* pInputOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    order_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField* pQryMaxOrderVolume, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    max_order_volume_response(*sink_, reinterpret_cast<const std::uint8_t*>(pQryMaxOrderVolume), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    settlement_confirm_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSettlementInfoConfirm), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    remove_parked_order_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRemoveParkedOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    remove_parked_order_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRemoveParkedOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exec_order_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputExecOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspExecOrderAction(CThostFtdcInputExecOrderActionField* pInputExecOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exec_order_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputExecOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    for_quote_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputForQuote), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    quote_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputQuote), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQuoteAction(CThostFtdcInputQuoteActionField* pInputQuoteAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    quote_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputQuoteAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    batch_order_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputBatchOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    option_self_close_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOptionSelfClose), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField* pInputOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    option_self_close_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOptionSelfCloseAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    comb_action_insert_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputCombAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryOrder(CThostFtdcOrderField* pOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    order_response(*sink_, reinterpret_cast<const std::uint8_t*>(pOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTrade(CThostFtdcTradeField* pTrade, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    trade_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTrade), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorPosition(CThostFtdcInvestorPositionField* pInvestorPosition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_position_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorPosition), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    trading_account_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTradingAccount), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestor(CThostFtdcInvestorField* pInvestor, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestor), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTradingCode(CThostFtdcTradingCodeField* pTradingCode, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    trading_code_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTradingCode), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField* pInstrumentMarginRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    instrument_margin_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInstrumentMarginRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField* pInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    instrument_commission_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInstrumentCommissionRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryUserSession(CThostFtdcUserSessionField* pUserSession, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    user_session_response(*sink_, reinterpret_cast<const std::uint8_t*>(pUserSession), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryExchange(CThostFtdcExchangeField* pExchange, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exchange_response(*sink_, reinterpret_cast<const std::uint8_t*>(pExchange), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryProduct(CThostFtdcProductField* pProduct, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    product_response(*sink_, reinterpret_cast<const std::uint8_t*>(pProduct), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    instrument_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    depth_market_data_response(*sink_, reinterpret_cast<const std::uint8_t*>(pDepthMarketData), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTraderOffer(CThostFtdcTraderOfferField* pTraderOffer, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    trader_offer_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTraderOffer), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySettlementInfo(CThostFtdcSettlementInfoField* pSettlementInfo, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    settlement_info_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSettlementInfo), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTransferBank(CThostFtdcTransferBankField* pTransferBank, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    transfer_bank_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTransferBank), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField* pInvestorPositionDetail, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_position_detail_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorPositionDetail), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryNotice(CThostFtdcNoticeField* pNotice, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    notice_response(*sink_, reinterpret_cast<const std::uint8_t*>(pNotice), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    settlement_status(*sink_, reinterpret_cast<const std::uint8_t*>(pSettlementInfoConfirm), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField* pInvestorPositionCombineDetail, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_position_combine_detail_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorPositionCombineDetail), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField* pCFMMCTradingAccountKey, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    cfmmc_trading_account_key_response(*sink_, reinterpret_cast<const std::uint8_t*>(pCFMMCTradingAccountKey), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField* pEWarrantOffset, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    e_warrant_offset_response(*sink_, reinterpret_cast<const std::uint8_t*>(pEWarrantOffset), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField* pInvestorProductGroupMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_product_group_margin_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorProductGroupMargin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField* pExchangeMarginRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exchange_margin_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pExchangeMarginRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField* pExchangeMarginRateAdjust, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exchange_margin_rate_adjust_response(*sink_, reinterpret_cast<const std::uint8_t*>(pExchangeMarginRateAdjust), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryExchangeRate(CThostFtdcExchangeRateField* pExchangeRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exchange_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pExchangeRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField* pSecAgentACIDMap, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    sec_agent_acid_map_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSecAgentACIDMap), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryProductExchRate(CThostFtdcProductExchRateField* pProductExchRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    product_exchange_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pProductExchRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryProductGroup(CThostFtdcProductGroupField* pProductGroup, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    product_group_response(*sink_, reinterpret_cast<const std::uint8_t*>(pProductGroup), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField* pMMInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    mm_instrument_commission_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pMMInstrumentCommissionRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField* pMMOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    mm_option_instrument_commission_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pMMOptionInstrCommRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField* pInstrumentOrderCommRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    instrument_order_commission_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInstrumentOrderCommRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    sec_agent_trading_account_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTradingAccount), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField* pSecAgentCheckMode, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    sec_agent_check_mode_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSecAgentCheckMode), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField* pSecAgentTradeInfo, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    sec_agent_trade_info_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSecAgentTradeInfo), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField* pOptionInstrTradeCost, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    option_instrument_trade_cost_response(*sink_, reinterpret_cast<const std::uint8_t*>(pOptionInstrTradeCost), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField* pOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    option_instrument_commission_rate_response(*sink_, reinterpret_cast<const std::uint8_t*>(pOptionInstrCommRate), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryExecOrder(CThostFtdcExecOrderField* pExecOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    exec_order_response(*sink_, reinterpret_cast<const std::uint8_t*>(pExecOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryForQuote(CThostFtdcForQuoteField* pForQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    for_quote_query_response(*sink_, reinterpret_cast<const std::uint8_t*>(pForQuote), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryQuote(CThostFtdcQuoteField* pQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    quote_response(*sink_, reinterpret_cast<const std::uint8_t*>(pQuote), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    option_self_close_response(*sink_, reinterpret_cast<const std::uint8_t*>(pOptionSelfClose), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestUnit(CThostFtdcInvestUnitField* pInvestUnit, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    invest_unit_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestUnit), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField* pCombInstrumentGuard, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    comb_instrument_guard_response(*sink_, reinterpret_cast<const std::uint8_t*>(pCombInstrumentGuard), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryCombAction(CThostFtdcCombActionField* pCombAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    comb_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pCombAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTransferSerial(CThostFtdcTransferSerialField* pTransferSerial, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    transfer_serial_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTransferSerial), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryAccountregister(CThostFtdcAccountregisterField* pAccountregister, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    account_register_response(*sink_, reinterpret_cast<const std::uint8_t*>(pAccountregister), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspError(CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    error(*sink_, reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnOrder(CThostFtdcOrderField* pOrder) {
    order_update(*sink_, reinterpret_cast<const std::uint8_t*>(pOrder));
}

void TraderSpiBridge::OnRtnTrade(CThostFtdcTradeField* pTrade) {
    execution(*sink_, reinterpret_cast<const std::uint8_t*>(pTrade));
}

void TraderSpiBridge::OnErrRtnOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo) {
    order_insert_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnOrderAction(CThostFtdcOrderActionField* pOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    order_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField* pInstrumentStatus) {
    instrument_status(*sink_, reinterpret_cast<const std::uint8_t*>(pInstrumentStatus));
}

void TraderSpiBridge::OnRtnBulletin(CThostFtdcBulletinField* pBulletin) {
    bulletin(*sink_, reinterpret_cast<const std::uint8_t*>(pBulletin));
}

void TraderSpiBridge::OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField* pTradingNoticeInfo) {
    notice_received(*sink_, reinterpret_cast<const std::uint8_t*>(pTradingNoticeInfo));
}

void TraderSpiBridge::OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField* pErrorConditionalOrder) {
    conditional_order_error(*sink_, reinterpret_cast<const std::uint8_t*>(pErrorConditionalOrder));
}

void TraderSpiBridge::OnRtnExecOrder(CThostFtdcExecOrderField* pExecOrder) {
    exec_order_report(*sink_, reinterpret_cast<const std::uint8_t*>(pExecOrder));
}

void TraderSpiBridge::OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo) {
    exec_order_insert_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputExecOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField* pExecOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    exec_order_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pExecOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo) {
    for_quote_insert_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputForQuote), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRtnQuote(CThostFtdcQuoteField* pQuote) {
    quote_report(*sink_, reinterpret_cast<const std::uint8_t*>(pQuote));
}

void TraderSpiBridge::OnErrRtnQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo) {
    quote_insert_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputQuote), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnQuoteAction(CThostFtdcQuoteActionField* pQuoteAction, CThostFtdcRspInfoField* pRspInfo) {
    quote_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pQuoteAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) {
    for_quote_response(*sink_, reinterpret_cast<const std::uint8_t*>(pForQuoteRsp));
}

void TraderSpiBridge::OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField* pCFMMCTradingAccountToken) {
    cfmmc_account_token(*sink_, reinterpret_cast<const std::uint8_t*>(pCFMMCTradingAccountToken));
}

void TraderSpiBridge::OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField* pBatchOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    batch_order_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pBatchOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose) {
    option_self_close_report(*sink_, reinterpret_cast<const std::uint8_t*>(pOptionSelfClose));
}

void TraderSpiBridge::OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo) {
    option_self_close_insert_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOptionSelfClose), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField* pOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo) {
    option_self_close_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pOptionSelfCloseAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRtnCombAction(CThostFtdcCombActionField* pCombAction) {
    comb_action_report(*sink_, reinterpret_cast<const std::uint8_t*>(pCombAction));
}

void TraderSpiBridge::OnErrRtnCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo) {
    comb_action_insert_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputCombAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRspQryContractBank(CThostFtdcContractBankField* pContractBank, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    contract_bank_response(*sink_, reinterpret_cast<const std::uint8_t*>(pContractBank), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryParkedOrder(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    parked_order_response(*sink_, reinterpret_cast<const std::uint8_t*>(pParkedOrder), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    parked_order_actions(*sink_, reinterpret_cast<const std::uint8_t*>(pParkedOrderAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryTradingNotice(CThostFtdcTradingNoticeField* pTradingNotice, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    notice_history_response(*sink_, reinterpret_cast<const std::uint8_t*>(pTradingNotice), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField* pBrokerTradingParams, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    broker_trading_parameters_response(*sink_, reinterpret_cast<const std::uint8_t*>(pBrokerTradingParams), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField* pBrokerTradingAlgos, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    broker_trading_algos_response(*sink_, reinterpret_cast<const std::uint8_t*>(pBrokerTradingAlgos), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField* pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    query_cfmmc_trading_account_token_response(*sink_, reinterpret_cast<const std::uint8_t*>(pQueryCFMMCTradingAccountToken), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField* pRspTransfer) {
    from_bank_to_future_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspTransfer));
}

void TraderSpiBridge::OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField* pRspTransfer) {
    from_future_to_bank_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspTransfer));
}

void TraderSpiBridge::OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField* pRspRepeal) {
    repeal_from_bank_to_future_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspRepeal));
}

void TraderSpiBridge::OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField* pRspRepeal) {
    repeal_from_future_to_bank_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspRepeal));
}

void TraderSpiBridge::OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField* pRspTransfer) {
    from_bank_to_future_by_future_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspTransfer));
}

void TraderSpiBridge::OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField* pRspTransfer) {
    from_future_to_bank_by_future_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspTransfer));
}

void TraderSpiBridge::OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField* pRspRepeal) {
    repeal_from_bank_to_future_by_future_manual_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspRepeal));
}

void TraderSpiBridge::OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField* pRspRepeal) {
    repeal_from_future_to_bank_by_future_manual_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspRepeal));
}

void TraderSpiBridge::OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField* pNotifyQueryAccount) {
    query_bank_balance_by_future_report(*sink_, reinterpret_cast<const std::uint8_t*>(pNotifyQueryAccount));
}

void TraderSpiBridge::OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) {
    bank_to_future_by_future_error(*sink_, reinterpret_cast<const std::uint8_t*>(pReqTransfer), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) {
    future_to_bank_by_future_error(*sink_, reinterpret_cast<const std::uint8_t*>(pReqTransfer), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) {
    repeal_bank_to_future_by_future_manual_error(*sink_, reinterpret_cast<const std::uint8_t*>(pReqRepeal), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) {
    repeal_future_to_bank_by_future_manual_error(*sink_, reinterpret_cast<const std::uint8_t*>(pReqRepeal), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo) {
    query_bank_balance_by_future_error(*sink_, reinterpret_cast<const std::uint8_t*>(pReqQueryAccount), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField* pRspRepeal) {
    repeal_from_bank_to_future_by_future_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspRepeal));
}

void TraderSpiBridge::OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField* pRspRepeal) {
    repeal_from_future_to_bank_by_future_report(*sink_, reinterpret_cast<const std::uint8_t*>(pRspRepeal));
}

void TraderSpiBridge::OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    from_bank_to_future_by_future_response(*sink_, reinterpret_cast<const std::uint8_t*>(pReqTransfer), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    from_future_to_bank_by_future_response(*sink_, reinterpret_cast<const std::uint8_t*>(pReqTransfer), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    query_bank_account_money_by_future_response(*sink_, reinterpret_cast<const std::uint8_t*>(pReqQueryAccount), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnOpenAccountByBank(CThostFtdcOpenAccountField* pOpenAccount) {
    open_account_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pOpenAccount));
}

void TraderSpiBridge::OnRtnCancelAccountByBank(CThostFtdcCancelAccountField* pCancelAccount) {
    cancel_account_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pCancelAccount));
}

void TraderSpiBridge::OnRtnChangeAccountByBank(CThostFtdcChangeAccountField* pChangeAccount) {
    change_account_by_bank_report(*sink_, reinterpret_cast<const std::uint8_t*>(pChangeAccount));
}

void TraderSpiBridge::OnRspQryClassifiedInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    classified_instrument_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryCombPromotionParam(CThostFtdcCombPromotionParamField* pCombPromotionParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    comb_promotion_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pCombPromotionParam), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRiskSettleInvstPosition(CThostFtdcRiskSettleInvstPositionField* pRiskSettleInvstPosition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    risk_settle_investor_position_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRiskSettleInvstPosition), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRiskSettleProductStatus(CThostFtdcRiskSettleProductStatusField* pRiskSettleProductStatus, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    risk_settle_product_status_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRiskSettleProductStatus), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMFutureParameter(CThostFtdcSPBMFutureParameterField* pSPBMFutureParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_future_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMFutureParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMOptionParameter(CThostFtdcSPBMOptionParameterField* pSPBMOptionParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_option_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMOptionParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMIntraParameter(CThostFtdcSPBMIntraParameterField* pSPBMIntraParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_intra_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMIntraParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMInterParameter(CThostFtdcSPBMInterParameterField* pSPBMInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_inter_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMInterParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMPortfDefinition(CThostFtdcSPBMPortfDefinitionField* pSPBMPortfDefinition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_portfolio_definition_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMPortfDefinition), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMInvestorPortfDef(CThostFtdcSPBMInvestorPortfDefField* pSPBMInvestorPortfDef, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_investor_portfolio_definition_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMInvestorPortfDef), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorPortfMarginRatio(CThostFtdcInvestorPortfMarginRatioField* pInvestorPortfMarginRatio, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_portfolio_margin_ratio_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorPortfMarginRatio), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorProdSPBMDetail(CThostFtdcInvestorProdSPBMDetailField* pInvestorProdSPBMDetail, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_product_spbm_detail_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorProdSPBMDetail), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorCommoditySPMMMargin(CThostFtdcInvestorCommoditySPMMMarginField* pInvestorCommoditySPMMMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_commodity_spmm_margin_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorCommoditySPMMMargin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorCommodityGroupSPMMMargin(CThostFtdcInvestorCommodityGroupSPMMMarginField* pInvestorCommodityGroupSPMMMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_commodity_group_spmm_margin_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorCommodityGroupSPMMMargin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPMMInstParam(CThostFtdcSPMMInstParamField* pSPMMInstParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spmm_instrument_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPMMInstParam), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPMMProductParam(CThostFtdcSPMMProductParamField* pSPMMProductParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spmm_product_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPMMProductParam), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySPBMAddOnInterParameter(CThostFtdcSPBMAddOnInterParameterField* pSPBMAddOnInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spbm_add_on_inter_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSPBMAddOnInterParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRCAMSCombProductInfo(CThostFtdcRCAMSCombProductInfoField* pRCAMSCombProductInfo, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rcams_comb_product_info_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRCAMSCombProductInfo), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRCAMSInstrParameter(CThostFtdcRCAMSInstrParameterField* pRCAMSInstrParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rcams_instrument_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRCAMSInstrParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRCAMSIntraParameter(CThostFtdcRCAMSIntraParameterField* pRCAMSIntraParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rcams_intra_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRCAMSIntraParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRCAMSInterParameter(CThostFtdcRCAMSInterParameterField* pRCAMSInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rcams_inter_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRCAMSInterParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRCAMSShortOptAdjustParam(CThostFtdcRCAMSShortOptAdjustParamField* pRCAMSShortOptAdjustParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rcams_short_opt_adjust_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRCAMSShortOptAdjustParam), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRCAMSInvestorCombPosition(CThostFtdcRCAMSInvestorCombPositionField* pRCAMSInvestorCombPosition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rcams_investor_comb_position_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRCAMSInvestorCombPosition), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorProdRCAMSMargin(CThostFtdcInvestorProdRCAMSMarginField* pInvestorProdRCAMSMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_product_rcams_margin_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorProdRCAMSMargin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRULEInstrParameter(CThostFtdcRULEInstrParameterField* pRULEInstrParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rule_instrument_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRULEInstrParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRULEIntraParameter(CThostFtdcRULEIntraParameterField* pRULEIntraParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rule_intra_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRULEIntraParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryRULEInterParameter(CThostFtdcRULEInterParameterField* pRULEInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    rule_inter_parameter_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRULEInterParameter), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorProdRULEMargin(CThostFtdcInvestorProdRULEMarginField* pInvestorProdRULEMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_product_rule_margin_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorProdRULEMargin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorPortfSetting(CThostFtdcInvestorPortfSettingField* pInvestorPortfSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_portfolio_setting_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorPortfSetting), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryInvestorInfoCommRec(CThostFtdcInvestorInfoCommRecField* pInvestorInfoCommRec, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    investor_info_commission_record_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInvestorInfoCommRec), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryCombLeg(CThostFtdcCombLegField* pCombLeg, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    comb_leg_response(*sink_, reinterpret_cast<const std::uint8_t*>(pCombLeg), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    set_offset_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOffsetSetting), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspCancelOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    cancel_offset_setting_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOffsetSetting), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting) {
    offset_setting_report(*sink_, reinterpret_cast<const std::uint8_t*>(pOffsetSetting));
}

void TraderSpiBridge::OnErrRtnOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo) {
    offset_setting_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputOffsetSetting), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnCancelOffsetSetting(CThostFtdcCancelOffsetSettingField* pCancelOffsetSetting, CThostFtdcRspInfoField* pRspInfo) {
    cancel_offset_setting_error(*sink_, reinterpret_cast<const std::uint8_t*>(pCancelOffsetSetting), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRspQryOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    offset_settings_response(*sink_, reinterpret_cast<const std::uint8_t*>(pOffsetSetting), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspGenSMSCode(CThostFtdcRspGenSMSCodeField* pRspGenSMSCode, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    gen_sms_code_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspGenSMSCode), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspSpdApply(CThostFtdcInputSpdApplyField* pInputSpdApply, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spd_apply_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputSpdApply), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspSpdApplyAction(CThostFtdcInputSpdApplyActionField* pInputSpdApplyAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spd_apply_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputSpdApplyAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQrySpdApply(CThostFtdcSpdApplyField* pSpdApply, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    spd_apply_query_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSpdApply), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnSpdApply(CThostFtdcSpdApplyField* pSpdApply) {
    spd_apply_report(*sink_, reinterpret_cast<const std::uint8_t*>(pSpdApply));
}

void TraderSpiBridge::OnErrRtnSpdApply(CThostFtdcInputSpdApplyField* pInputSpdApply, CThostFtdcRspInfoField* pRspInfo) {
    spd_apply_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputSpdApply), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnSpdApplyAction(CThostFtdcSpdApplyActionField* pSpdApplyAction, CThostFtdcRspInfoField* pRspInfo) {
    spd_apply_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pSpdApplyAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnRspHedgeCfm(CThostFtdcInputHedgeCfmField* pInputHedgeCfm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    hedge_cfm_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputHedgeCfm), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspHedgeCfmAction(CThostFtdcInputHedgeCfmActionField* pInputHedgeCfmAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    hedge_cfm_action_response(*sink_, reinterpret_cast<const std::uint8_t*>(pInputHedgeCfmAction), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRspQryHedgeCfm(CThostFtdcHedgeCfmField* pHedgeCfm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    hedge_cfm_query_response(*sink_, reinterpret_cast<const std::uint8_t*>(pHedgeCfm), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void TraderSpiBridge::OnRtnHedgeCfm(CThostFtdcHedgeCfmField* pHedgeCfm) {
    hedge_cfm_report(*sink_, reinterpret_cast<const std::uint8_t*>(pHedgeCfm));
}

void TraderSpiBridge::OnErrRtnHedgeCfm(CThostFtdcInputHedgeCfmField* pInputHedgeCfm, CThostFtdcRspInfoField* pRspInfo) {
    hedge_cfm_error(*sink_, reinterpret_cast<const std::uint8_t*>(pInputHedgeCfm), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

void TraderSpiBridge::OnErrRtnHedgeCfmAction(CThostFtdcHedgeCfmActionField* pHedgeCfmAction, CThostFtdcRspInfoField* pRspInfo) {
    hedge_cfm_action_error(*sink_, reinterpret_cast<const std::uint8_t*>(pHedgeCfmAction), reinterpret_cast<const std::uint8_t*>(pRspInfo));
}

TraderApiBridge::TraderApiBridge(CThostFtdcTraderApi* api, std::unique_ptr<TraderSpiBridge> spi)
    : api_(api), spi_(std::move(spi)) {
    api_->RegisterSpi(spi_.get());
}

TraderApiBridge::~TraderApiBridge() {
    // 未调用 Init 的实例没有工作线程，Release 内部 join 空线程在新版
    // glibc 与 macOS 上会崩溃；Windows 的 Release 对未初始化实例安全，
    // 且必须调用才能关闭流量文件句柄。
#ifdef _WIN32
    api_->Release();
#else
    if (initialized_) {
        api_->Release();
    }
#endif
    spi_.reset();
}

std::unique_ptr<TraderSpiBridge> create_trader_spi(rust::Box<TraderSpiSink> sink) {
    return std::make_unique<TraderSpiBridge>(std::move(sink));
}

std::unique_ptr<TraderApiBridge> create_trader_api(rust::Box<TraderSpiSink> sink, rust::Str flow_path, bool using_udp, bool multicast, bool production_mode) {
    const std::string path(flow_path);
    CThostFtdcTraderApi* api = CThostFtdcTraderApi::CreateFtdcTraderApi(path.c_str());
    (void)using_udp;
    (void)multicast;
    (void)production_mode;
    if (api == nullptr) {
        return nullptr;
    }
    return std::make_unique<TraderApiBridge>(api, create_trader_spi(std::move(sink)));
}

rust::String TraderApiBridge::version() const {
    const char* value = CThostFtdcTraderApi::GetApiVersion();
    return value == nullptr ? rust::String() : rust::String(value);
}

void TraderApiBridge::init() const { initialized_ = true; api_->Init(); }

std::int32_t TraderApiBridge::join() const { return api_->Join(); }

rust::String TraderApiBridge::trading_day() const {
    const char* value = api_->GetTradingDay();
    return value == nullptr ? rust::String() : rust::String(value);
}

void TraderApiBridge::register_front(rust::Str address) const {
    const std::string value(address);
    api_->RegisterFront(const_cast<char*>(value.c_str()));
}

void TraderApiBridge::register_name_server(rust::Str address) const {
    const std::string value(address);
    api_->RegisterNameServer(const_cast<char*>(value.c_str()));
}

void TraderApiBridge::front_info(std::uint8_t* info) const {
    api_->GetFrontInfo(reinterpret_cast<CThostFtdcFrontInfoField*>(info));
}

void TraderApiBridge::register_fens_user_info(std::uint8_t* request) const {
    api_->RegisterFensUserInfo(reinterpret_cast<CThostFtdcFensUserInfoField*>(request));
}

void TraderApiBridge::subscribe_private(std::int32_t resume) const {
    api_->SubscribePrivateTopic(static_cast<THOST_TE_RESUME_TYPE>(resume));
}

void TraderApiBridge::subscribe_public(std::int32_t resume) const {
    api_->SubscribePublicTopic(static_cast<THOST_TE_RESUME_TYPE>(resume));
}

std::int32_t TraderApiBridge::authenticate(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqAuthenticate(reinterpret_cast<CThostFtdcReqAuthenticateField*>(request), request_id);
}

std::int32_t TraderApiBridge::login(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLogin(reinterpret_cast<CThostFtdcReqUserLoginField*>(request), request_id);
}

std::int32_t TraderApiBridge::logout(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLogout(reinterpret_cast<CThostFtdcUserLogoutField*>(request), request_id);
}

std::int32_t TraderApiBridge::confirm_settlement(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqSettlementInfoConfirm(reinterpret_cast<CThostFtdcSettlementInfoConfirmField*>(request), request_id);
}

std::int32_t TraderApiBridge::settlement_confirmation(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQrySettlementInfoConfirm(reinterpret_cast<CThostFtdcQrySettlementInfoConfirmField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_settlement_info(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQrySettlementInfo(reinterpret_cast<CThostFtdcQrySettlementInfoField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_investor_position_detail(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryInvestorPositionDetail(reinterpret_cast<CThostFtdcQryInvestorPositionDetailField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_product(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryProduct(reinterpret_cast<CThostFtdcQryProductField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_exchange(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryExchange(reinterpret_cast<CThostFtdcQryExchangeField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_trading_code(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryTradingCode(reinterpret_cast<CThostFtdcQryTradingCodeField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_instrument_margin_rate(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryInstrumentMarginRate(reinterpret_cast<CThostFtdcQryInstrumentMarginRateField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_instrument_commission_rate(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryInstrumentCommissionRate(reinterpret_cast<CThostFtdcQryInstrumentCommissionRateField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_instrument_order_comm_rate(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryInstrumentOrderCommRate(reinterpret_cast<CThostFtdcQryInstrumentOrderCommRateField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_max_order_volume(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryMaxOrderVolume(reinterpret_cast<CThostFtdcQryMaxOrderVolumeField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_depth_market_data(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryDepthMarketData(reinterpret_cast<CThostFtdcQryDepthMarketDataField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_order(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryOrder(reinterpret_cast<CThostFtdcQryOrderField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_trade(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryTrade(reinterpret_cast<CThostFtdcQryTradeField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_investor_position(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryInvestorPosition(reinterpret_cast<CThostFtdcQryInvestorPositionField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_trading_account(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryTradingAccount(reinterpret_cast<CThostFtdcQryTradingAccountField*>(request), request_id);
}

std::int32_t TraderApiBridge::query_instrument(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryInstrument(reinterpret_cast<CThostFtdcQryInstrumentField*>(request), request_id);
}

std::int32_t TraderApiBridge::insert_order(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqOrderInsert(reinterpret_cast<CThostFtdcInputOrderField*>(request), request_id);
}

std::int32_t TraderApiBridge::cancel_order(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqOrderAction(reinterpret_cast<CThostFtdcInputOrderActionField*>(request), request_id);
}

void TraderApiBridge::register_user_system_info(std::uint8_t* request) const {
    api_->RegisterUserSystemInfo(reinterpret_cast<CThostFtdcUserSystemInfoField*>(request));
}

void TraderApiBridge::submit_user_system_info(std::uint8_t* request) const {
    api_->SubmitUserSystemInfo(reinterpret_cast<CThostFtdcUserSystemInfoField*>(request));
}

void TraderApiBridge::register_wechat_user_system_info(std::uint8_t* request) const {
    api_->RegisterWechatUserSystemInfo(reinterpret_cast<CThostFtdcWechatUserSystemInfoField*>(request));
}

void TraderApiBridge::submit_wechat_user_system_info(std::uint8_t* request) const {
    api_->SubmitWechatUserSystemInfo(reinterpret_cast<CThostFtdcWechatUserSystemInfoField*>(request));
}

std::int32_t TraderApiBridge::user_password_update(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserPasswordUpdate(reinterpret_cast<CThostFtdcUserPasswordUpdateField*>(request), request_id);
}

std::int32_t TraderApiBridge::trading_account_password_update(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqTradingAccountPasswordUpdate(reinterpret_cast<CThostFtdcTradingAccountPasswordUpdateField*>(request), request_id);
}

std::int32_t TraderApiBridge::user_auth_method(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserAuthMethod(reinterpret_cast<CThostFtdcReqUserAuthMethodField*>(request), request_id);
}

std::int32_t TraderApiBridge::gen_user_captcha(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqGenUserCaptcha(reinterpret_cast<CThostFtdcReqGenUserCaptchaField*>(request), request_id);
}

std::int32_t TraderApiBridge::gen_user_text(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqGenUserText(reinterpret_cast<CThostFtdcReqGenUserTextField*>(request), request_id);
}

std::int32_t TraderApiBridge::user_login_with_captcha(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLoginWithCaptcha(reinterpret_cast<CThostFtdcReqUserLoginWithCaptchaField*>(request), request_id);
}

std::int32_t TraderApiBridge::user_login_with_text(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLoginWithText(reinterpret_cast<CThostFtdcReqUserLoginWithTextField*>(request), request_id);
}

std::int32_t TraderApiBridge::user_login_with_otp(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLoginWithOTP(reinterpret_cast<CThostFtdcReqUserLoginWithOTPField*>(request), request_id);
}

void emit_trader_order(TraderSpiBridge& spi, std::uint8_t* data) {
    spi.OnRtnOrder(reinterpret_cast<CThostFtdcOrderField*>(data));
}

void emit_trader_callback(TraderSpiBridge& spi, std::uint32_t callback_index, std::uint8_t* first, std::uint8_t* second, std::int32_t request_id, bool last) {
    switch (callback_index) {
    case 0: spi.OnFrontConnected(); break;
    case 1: spi.OnFrontDisconnected(request_id); break;
    case 2: spi.OnHeartBeatWarning(request_id); break;
    case 3: spi.OnRspAuthenticate(reinterpret_cast<CThostFtdcRspAuthenticateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 4: spi.OnRtnPrivateSeqNo(request_id); break;
    case 5: spi.OnRspUserLogin(reinterpret_cast<CThostFtdcRspUserLoginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 6: spi.OnRspUserLogout(reinterpret_cast<CThostFtdcUserLogoutField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 7: spi.OnRspUserPasswordUpdate(reinterpret_cast<CThostFtdcUserPasswordUpdateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 8: spi.OnRspTradingAccountPasswordUpdate(reinterpret_cast<CThostFtdcTradingAccountPasswordUpdateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 9: spi.OnRspUserAuthMethod(reinterpret_cast<CThostFtdcRspUserAuthMethodField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 10: spi.OnRspGenUserCaptcha(reinterpret_cast<CThostFtdcRspGenUserCaptchaField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 11: spi.OnRspGenUserText(reinterpret_cast<CThostFtdcRspGenUserTextField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 12: spi.OnRspOrderInsert(reinterpret_cast<CThostFtdcInputOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 13: spi.OnRspParkedOrderInsert(reinterpret_cast<CThostFtdcParkedOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 14: spi.OnRspParkedOrderAction(reinterpret_cast<CThostFtdcParkedOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 15: spi.OnRspOrderAction(reinterpret_cast<CThostFtdcInputOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 16: spi.OnRspQryMaxOrderVolume(reinterpret_cast<CThostFtdcQryMaxOrderVolumeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 17: spi.OnRspSettlementInfoConfirm(reinterpret_cast<CThostFtdcSettlementInfoConfirmField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 18: spi.OnRspRemoveParkedOrder(reinterpret_cast<CThostFtdcRemoveParkedOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 19: spi.OnRspRemoveParkedOrderAction(reinterpret_cast<CThostFtdcRemoveParkedOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 20: spi.OnRspExecOrderInsert(reinterpret_cast<CThostFtdcInputExecOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 21: spi.OnRspExecOrderAction(reinterpret_cast<CThostFtdcInputExecOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 22: spi.OnRspForQuoteInsert(reinterpret_cast<CThostFtdcInputForQuoteField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 23: spi.OnRspQuoteInsert(reinterpret_cast<CThostFtdcInputQuoteField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 24: spi.OnRspQuoteAction(reinterpret_cast<CThostFtdcInputQuoteActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 25: spi.OnRspBatchOrderAction(reinterpret_cast<CThostFtdcInputBatchOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 26: spi.OnRspOptionSelfCloseInsert(reinterpret_cast<CThostFtdcInputOptionSelfCloseField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 27: spi.OnRspOptionSelfCloseAction(reinterpret_cast<CThostFtdcInputOptionSelfCloseActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 28: spi.OnRspCombActionInsert(reinterpret_cast<CThostFtdcInputCombActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 29: spi.OnRspQryOrder(reinterpret_cast<CThostFtdcOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 30: spi.OnRspQryTrade(reinterpret_cast<CThostFtdcTradeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 31: spi.OnRspQryInvestorPosition(reinterpret_cast<CThostFtdcInvestorPositionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 32: spi.OnRspQryTradingAccount(reinterpret_cast<CThostFtdcTradingAccountField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 33: spi.OnRspQryInvestor(reinterpret_cast<CThostFtdcInvestorField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 34: spi.OnRspQryTradingCode(reinterpret_cast<CThostFtdcTradingCodeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 35: spi.OnRspQryInstrumentMarginRate(reinterpret_cast<CThostFtdcInstrumentMarginRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 36: spi.OnRspQryInstrumentCommissionRate(reinterpret_cast<CThostFtdcInstrumentCommissionRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 37: spi.OnRspQryUserSession(reinterpret_cast<CThostFtdcUserSessionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 38: spi.OnRspQryExchange(reinterpret_cast<CThostFtdcExchangeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 39: spi.OnRspQryProduct(reinterpret_cast<CThostFtdcProductField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 40: spi.OnRspQryInstrument(reinterpret_cast<CThostFtdcInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 41: spi.OnRspQryDepthMarketData(reinterpret_cast<CThostFtdcDepthMarketDataField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 42: spi.OnRspQryTraderOffer(reinterpret_cast<CThostFtdcTraderOfferField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 43: spi.OnRspQrySettlementInfo(reinterpret_cast<CThostFtdcSettlementInfoField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 44: spi.OnRspQryTransferBank(reinterpret_cast<CThostFtdcTransferBankField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 45: spi.OnRspQryInvestorPositionDetail(reinterpret_cast<CThostFtdcInvestorPositionDetailField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 46: spi.OnRspQryNotice(reinterpret_cast<CThostFtdcNoticeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 47: spi.OnRspQrySettlementInfoConfirm(reinterpret_cast<CThostFtdcSettlementInfoConfirmField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 48: spi.OnRspQryInvestorPositionCombineDetail(reinterpret_cast<CThostFtdcInvestorPositionCombineDetailField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 49: spi.OnRspQryCFMMCTradingAccountKey(reinterpret_cast<CThostFtdcCFMMCTradingAccountKeyField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 50: spi.OnRspQryEWarrantOffset(reinterpret_cast<CThostFtdcEWarrantOffsetField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 51: spi.OnRspQryInvestorProductGroupMargin(reinterpret_cast<CThostFtdcInvestorProductGroupMarginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 52: spi.OnRspQryExchangeMarginRate(reinterpret_cast<CThostFtdcExchangeMarginRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 53: spi.OnRspQryExchangeMarginRateAdjust(reinterpret_cast<CThostFtdcExchangeMarginRateAdjustField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 54: spi.OnRspQryExchangeRate(reinterpret_cast<CThostFtdcExchangeRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 55: spi.OnRspQrySecAgentACIDMap(reinterpret_cast<CThostFtdcSecAgentACIDMapField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 56: spi.OnRspQryProductExchRate(reinterpret_cast<CThostFtdcProductExchRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 57: spi.OnRspQryProductGroup(reinterpret_cast<CThostFtdcProductGroupField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 58: spi.OnRspQryMMInstrumentCommissionRate(reinterpret_cast<CThostFtdcMMInstrumentCommissionRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 59: spi.OnRspQryMMOptionInstrCommRate(reinterpret_cast<CThostFtdcMMOptionInstrCommRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 60: spi.OnRspQryInstrumentOrderCommRate(reinterpret_cast<CThostFtdcInstrumentOrderCommRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 61: spi.OnRspQrySecAgentTradingAccount(reinterpret_cast<CThostFtdcTradingAccountField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 62: spi.OnRspQrySecAgentCheckMode(reinterpret_cast<CThostFtdcSecAgentCheckModeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 63: spi.OnRspQrySecAgentTradeInfo(reinterpret_cast<CThostFtdcSecAgentTradeInfoField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 64: spi.OnRspQryOptionInstrTradeCost(reinterpret_cast<CThostFtdcOptionInstrTradeCostField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 65: spi.OnRspQryOptionInstrCommRate(reinterpret_cast<CThostFtdcOptionInstrCommRateField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 66: spi.OnRspQryExecOrder(reinterpret_cast<CThostFtdcExecOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 67: spi.OnRspQryForQuote(reinterpret_cast<CThostFtdcForQuoteField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 68: spi.OnRspQryQuote(reinterpret_cast<CThostFtdcQuoteField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 69: spi.OnRspQryOptionSelfClose(reinterpret_cast<CThostFtdcOptionSelfCloseField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 70: spi.OnRspQryInvestUnit(reinterpret_cast<CThostFtdcInvestUnitField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 71: spi.OnRspQryCombInstrumentGuard(reinterpret_cast<CThostFtdcCombInstrumentGuardField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 72: spi.OnRspQryCombAction(reinterpret_cast<CThostFtdcCombActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 73: spi.OnRspQryTransferSerial(reinterpret_cast<CThostFtdcTransferSerialField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 74: spi.OnRspQryAccountregister(reinterpret_cast<CThostFtdcAccountregisterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 75: spi.OnRspError(reinterpret_cast<CThostFtdcRspInfoField*>(first), request_id, last); break;
    case 76: spi.OnRtnOrder(reinterpret_cast<CThostFtdcOrderField*>(first)); break;
    case 77: spi.OnRtnTrade(reinterpret_cast<CThostFtdcTradeField*>(first)); break;
    case 78: spi.OnErrRtnOrderInsert(reinterpret_cast<CThostFtdcInputOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 79: spi.OnErrRtnOrderAction(reinterpret_cast<CThostFtdcOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 80: spi.OnRtnInstrumentStatus(reinterpret_cast<CThostFtdcInstrumentStatusField*>(first)); break;
    case 81: spi.OnRtnBulletin(reinterpret_cast<CThostFtdcBulletinField*>(first)); break;
    case 82: spi.OnRtnTradingNotice(reinterpret_cast<CThostFtdcTradingNoticeInfoField*>(first)); break;
    case 83: spi.OnRtnErrorConditionalOrder(reinterpret_cast<CThostFtdcErrorConditionalOrderField*>(first)); break;
    case 84: spi.OnRtnExecOrder(reinterpret_cast<CThostFtdcExecOrderField*>(first)); break;
    case 85: spi.OnErrRtnExecOrderInsert(reinterpret_cast<CThostFtdcInputExecOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 86: spi.OnErrRtnExecOrderAction(reinterpret_cast<CThostFtdcExecOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 87: spi.OnErrRtnForQuoteInsert(reinterpret_cast<CThostFtdcInputForQuoteField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 88: spi.OnRtnQuote(reinterpret_cast<CThostFtdcQuoteField*>(first)); break;
    case 89: spi.OnErrRtnQuoteInsert(reinterpret_cast<CThostFtdcInputQuoteField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 90: spi.OnErrRtnQuoteAction(reinterpret_cast<CThostFtdcQuoteActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 91: spi.OnRtnForQuoteRsp(reinterpret_cast<CThostFtdcForQuoteRspField*>(first)); break;
    case 92: spi.OnRtnCFMMCTradingAccountToken(reinterpret_cast<CThostFtdcCFMMCTradingAccountTokenField*>(first)); break;
    case 93: spi.OnErrRtnBatchOrderAction(reinterpret_cast<CThostFtdcBatchOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 94: spi.OnRtnOptionSelfClose(reinterpret_cast<CThostFtdcOptionSelfCloseField*>(first)); break;
    case 95: spi.OnErrRtnOptionSelfCloseInsert(reinterpret_cast<CThostFtdcInputOptionSelfCloseField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 96: spi.OnErrRtnOptionSelfCloseAction(reinterpret_cast<CThostFtdcOptionSelfCloseActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 97: spi.OnRtnCombAction(reinterpret_cast<CThostFtdcCombActionField*>(first)); break;
    case 98: spi.OnErrRtnCombActionInsert(reinterpret_cast<CThostFtdcInputCombActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 99: spi.OnRspQryContractBank(reinterpret_cast<CThostFtdcContractBankField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 100: spi.OnRspQryParkedOrder(reinterpret_cast<CThostFtdcParkedOrderField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 101: spi.OnRspQryParkedOrderAction(reinterpret_cast<CThostFtdcParkedOrderActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 102: spi.OnRspQryTradingNotice(reinterpret_cast<CThostFtdcTradingNoticeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 103: spi.OnRspQryBrokerTradingParams(reinterpret_cast<CThostFtdcBrokerTradingParamsField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 104: spi.OnRspQryBrokerTradingAlgos(reinterpret_cast<CThostFtdcBrokerTradingAlgosField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 105: spi.OnRspQueryCFMMCTradingAccountToken(reinterpret_cast<CThostFtdcQueryCFMMCTradingAccountTokenField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 106: spi.OnRtnFromBankToFutureByBank(reinterpret_cast<CThostFtdcRspTransferField*>(first)); break;
    case 107: spi.OnRtnFromFutureToBankByBank(reinterpret_cast<CThostFtdcRspTransferField*>(first)); break;
    case 108: spi.OnRtnRepealFromBankToFutureByBank(reinterpret_cast<CThostFtdcRspRepealField*>(first)); break;
    case 109: spi.OnRtnRepealFromFutureToBankByBank(reinterpret_cast<CThostFtdcRspRepealField*>(first)); break;
    case 110: spi.OnRtnFromBankToFutureByFuture(reinterpret_cast<CThostFtdcRspTransferField*>(first)); break;
    case 111: spi.OnRtnFromFutureToBankByFuture(reinterpret_cast<CThostFtdcRspTransferField*>(first)); break;
    case 112: spi.OnRtnRepealFromBankToFutureByFutureManual(reinterpret_cast<CThostFtdcRspRepealField*>(first)); break;
    case 113: spi.OnRtnRepealFromFutureToBankByFutureManual(reinterpret_cast<CThostFtdcRspRepealField*>(first)); break;
    case 114: spi.OnRtnQueryBankBalanceByFuture(reinterpret_cast<CThostFtdcNotifyQueryAccountField*>(first)); break;
    case 115: spi.OnErrRtnBankToFutureByFuture(reinterpret_cast<CThostFtdcReqTransferField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 116: spi.OnErrRtnFutureToBankByFuture(reinterpret_cast<CThostFtdcReqTransferField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 117: spi.OnErrRtnRepealBankToFutureByFutureManual(reinterpret_cast<CThostFtdcReqRepealField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 118: spi.OnErrRtnRepealFutureToBankByFutureManual(reinterpret_cast<CThostFtdcReqRepealField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 119: spi.OnErrRtnQueryBankBalanceByFuture(reinterpret_cast<CThostFtdcReqQueryAccountField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 120: spi.OnRtnRepealFromBankToFutureByFuture(reinterpret_cast<CThostFtdcRspRepealField*>(first)); break;
    case 121: spi.OnRtnRepealFromFutureToBankByFuture(reinterpret_cast<CThostFtdcRspRepealField*>(first)); break;
    case 122: spi.OnRspFromBankToFutureByFuture(reinterpret_cast<CThostFtdcReqTransferField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 123: spi.OnRspFromFutureToBankByFuture(reinterpret_cast<CThostFtdcReqTransferField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 124: spi.OnRspQueryBankAccountMoneyByFuture(reinterpret_cast<CThostFtdcReqQueryAccountField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 125: spi.OnRtnOpenAccountByBank(reinterpret_cast<CThostFtdcOpenAccountField*>(first)); break;
    case 126: spi.OnRtnCancelAccountByBank(reinterpret_cast<CThostFtdcCancelAccountField*>(first)); break;
    case 127: spi.OnRtnChangeAccountByBank(reinterpret_cast<CThostFtdcChangeAccountField*>(first)); break;
    case 128: spi.OnRspQryClassifiedInstrument(reinterpret_cast<CThostFtdcInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 129: spi.OnRspQryCombPromotionParam(reinterpret_cast<CThostFtdcCombPromotionParamField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 130: spi.OnRspQryRiskSettleInvstPosition(reinterpret_cast<CThostFtdcRiskSettleInvstPositionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 131: spi.OnRspQryRiskSettleProductStatus(reinterpret_cast<CThostFtdcRiskSettleProductStatusField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 132: spi.OnRspQrySPBMFutureParameter(reinterpret_cast<CThostFtdcSPBMFutureParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 133: spi.OnRspQrySPBMOptionParameter(reinterpret_cast<CThostFtdcSPBMOptionParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 134: spi.OnRspQrySPBMIntraParameter(reinterpret_cast<CThostFtdcSPBMIntraParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 135: spi.OnRspQrySPBMInterParameter(reinterpret_cast<CThostFtdcSPBMInterParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 136: spi.OnRspQrySPBMPortfDefinition(reinterpret_cast<CThostFtdcSPBMPortfDefinitionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 137: spi.OnRspQrySPBMInvestorPortfDef(reinterpret_cast<CThostFtdcSPBMInvestorPortfDefField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 138: spi.OnRspQryInvestorPortfMarginRatio(reinterpret_cast<CThostFtdcInvestorPortfMarginRatioField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 139: spi.OnRspQryInvestorProdSPBMDetail(reinterpret_cast<CThostFtdcInvestorProdSPBMDetailField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 140: spi.OnRspQryInvestorCommoditySPMMMargin(reinterpret_cast<CThostFtdcInvestorCommoditySPMMMarginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 141: spi.OnRspQryInvestorCommodityGroupSPMMMargin(reinterpret_cast<CThostFtdcInvestorCommodityGroupSPMMMarginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 142: spi.OnRspQrySPMMInstParam(reinterpret_cast<CThostFtdcSPMMInstParamField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 143: spi.OnRspQrySPMMProductParam(reinterpret_cast<CThostFtdcSPMMProductParamField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 144: spi.OnRspQrySPBMAddOnInterParameter(reinterpret_cast<CThostFtdcSPBMAddOnInterParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 145: spi.OnRspQryRCAMSCombProductInfo(reinterpret_cast<CThostFtdcRCAMSCombProductInfoField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 146: spi.OnRspQryRCAMSInstrParameter(reinterpret_cast<CThostFtdcRCAMSInstrParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 147: spi.OnRspQryRCAMSIntraParameter(reinterpret_cast<CThostFtdcRCAMSIntraParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 148: spi.OnRspQryRCAMSInterParameter(reinterpret_cast<CThostFtdcRCAMSInterParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 149: spi.OnRspQryRCAMSShortOptAdjustParam(reinterpret_cast<CThostFtdcRCAMSShortOptAdjustParamField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 150: spi.OnRspQryRCAMSInvestorCombPosition(reinterpret_cast<CThostFtdcRCAMSInvestorCombPositionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 151: spi.OnRspQryInvestorProdRCAMSMargin(reinterpret_cast<CThostFtdcInvestorProdRCAMSMarginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 152: spi.OnRspQryRULEInstrParameter(reinterpret_cast<CThostFtdcRULEInstrParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 153: spi.OnRspQryRULEIntraParameter(reinterpret_cast<CThostFtdcRULEIntraParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 154: spi.OnRspQryRULEInterParameter(reinterpret_cast<CThostFtdcRULEInterParameterField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 155: spi.OnRspQryInvestorProdRULEMargin(reinterpret_cast<CThostFtdcInvestorProdRULEMarginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 156: spi.OnRspQryInvestorPortfSetting(reinterpret_cast<CThostFtdcInvestorPortfSettingField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 157: spi.OnRspQryInvestorInfoCommRec(reinterpret_cast<CThostFtdcInvestorInfoCommRecField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 158: spi.OnRspQryCombLeg(reinterpret_cast<CThostFtdcCombLegField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 159: spi.OnRspOffsetSetting(reinterpret_cast<CThostFtdcInputOffsetSettingField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 160: spi.OnRspCancelOffsetSetting(reinterpret_cast<CThostFtdcInputOffsetSettingField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 161: spi.OnRtnOffsetSetting(reinterpret_cast<CThostFtdcOffsetSettingField*>(first)); break;
    case 162: spi.OnErrRtnOffsetSetting(reinterpret_cast<CThostFtdcInputOffsetSettingField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 163: spi.OnErrRtnCancelOffsetSetting(reinterpret_cast<CThostFtdcCancelOffsetSettingField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 164: spi.OnRspQryOffsetSetting(reinterpret_cast<CThostFtdcOffsetSettingField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 165: spi.OnRspGenSMSCode(reinterpret_cast<CThostFtdcRspGenSMSCodeField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 166: spi.OnRspSpdApply(reinterpret_cast<CThostFtdcInputSpdApplyField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 167: spi.OnRspSpdApplyAction(reinterpret_cast<CThostFtdcInputSpdApplyActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 168: spi.OnRspQrySpdApply(reinterpret_cast<CThostFtdcSpdApplyField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 169: spi.OnRtnSpdApply(reinterpret_cast<CThostFtdcSpdApplyField*>(first)); break;
    case 170: spi.OnErrRtnSpdApply(reinterpret_cast<CThostFtdcInputSpdApplyField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 171: spi.OnErrRtnSpdApplyAction(reinterpret_cast<CThostFtdcSpdApplyActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 172: spi.OnRspHedgeCfm(reinterpret_cast<CThostFtdcInputHedgeCfmField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 173: spi.OnRspHedgeCfmAction(reinterpret_cast<CThostFtdcInputHedgeCfmActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 174: spi.OnRspQryHedgeCfm(reinterpret_cast<CThostFtdcHedgeCfmField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 175: spi.OnRtnHedgeCfm(reinterpret_cast<CThostFtdcHedgeCfmField*>(first)); break;
    case 176: spi.OnErrRtnHedgeCfm(reinterpret_cast<CThostFtdcInputHedgeCfmField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    case 177: spi.OnErrRtnHedgeCfmAction(reinterpret_cast<CThostFtdcHedgeCfmActionField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second)); break;
    default: break;
    }
}

} // namespace ctp_base::trader
