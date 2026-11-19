#pragma once

namespace ctp_base::md { class MdApiBridge; class MdSpiBridge; }
namespace ctp_base::trader { class TraderApiBridge; class TraderSpiBridge; }

#include "ctp-base/src/sys/ffi/md.rs.h"
#include "ctp-base/src/sys/ffi/trader.rs.h"
#include "ThostFtdcMdApi.h"
#include "ThostFtdcTraderApi.h"
#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>

namespace ctp_base::md {

class MdSpiBridge final : public CThostFtdcMdSpi {
public:
    explicit MdSpiBridge(rust::Box<MdSpiSink> sink);

    void OnFrontConnected() override;
    void OnFrontDisconnected(int nReason) override;
    void OnHeartBeatWarning(int nTimeLapse) override;
    void OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField* pMulticastInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspError(CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData) override;
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) override;

private:
    rust::Box<MdSpiSink> sink_;
};

class MdApiBridge final {
public:
    MdApiBridge(CThostFtdcMdApi* api, std::unique_ptr<MdSpiBridge> spi);
    ~MdApiBridge();

    MdApiBridge(const MdApiBridge&) = delete;
    MdApiBridge& operator=(const MdApiBridge&) = delete;

    rust::String version() const;
    void init() const;
    std::int32_t join() const;
    rust::String trading_day() const;
    void register_front(rust::Str address) const;
    void register_name_server(rust::Str address) const;
    void register_fens_user_info(std::uint8_t* request) const;
    std::int32_t subscribe(rust::Vec<rust::String> instruments) const;
    std::int32_t unsubscribe(rust::Vec<rust::String> instruments) const;
    std::int32_t subscribe_quote(rust::Vec<rust::String> instruments) const;
    std::int32_t unsubscribe_quote(rust::Vec<rust::String> instruments) const;
    std::int32_t req_user_login(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t logout(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_multicast_instrument(std::uint8_t* request, std::int32_t request_id) const;

private:
    CThostFtdcMdApi* api_;
    std::unique_ptr<MdSpiBridge> spi_;
    // CTP 未调用 Init 时内部工作线程不存在，Release 会 join 空线程，新版 glibc 与 macOS 会直接崩溃。
    mutable bool initialized_ = false;
};

std::unique_ptr<MdSpiBridge> create_md_spi(rust::Box<MdSpiSink> sink);
std::unique_ptr<MdApiBridge> create_md_api(rust::Box<MdSpiSink> sink, rust::Str flow_path, bool using_udp, bool multicast, bool production_mode);
void emit_depth_market_data(MdSpiBridge& spi, std::uint8_t* data);
void emit_md_front_connected(MdSpiBridge& spi);
void emit_md_callback(MdSpiBridge& spi, std::uint32_t callback_index, std::uint8_t* first, std::uint8_t* second, std::int32_t request_id, bool last);

} // namespace ctp_base::md

namespace ctp_base::trader {

class TraderSpiBridge final : public CThostFtdcTraderSpi {
public:
    explicit TraderSpiBridge(rust::Box<TraderSpiSink> sink);

    void OnFrontConnected() override;
    void OnFrontDisconnected(int nReason) override;
    void OnHeartBeatWarning(int nTimeLapse) override;
    void OnRspAuthenticate(CThostFtdcRspAuthenticateField* pRspAuthenticateField, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnPrivateSeqNo(int nSeqNo) override;
    void OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField* pTradingAccountPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField* pRspUserAuthMethod, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField* pRspGenUserCaptcha, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspGenUserText(CThostFtdcRspGenUserTextField* pRspGenUserText, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspParkedOrderInsert(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspOrderAction(CThostFtdcInputOrderActionField* pInputOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField* pQryMaxOrderVolume, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspExecOrderAction(CThostFtdcInputExecOrderActionField* pInputExecOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQuoteAction(CThostFtdcInputQuoteActionField* pInputQuoteAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField* pInputOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryOrder(CThostFtdcOrderField* pOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTrade(CThostFtdcTradeField* pTrade, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorPosition(CThostFtdcInvestorPositionField* pInvestorPosition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestor(CThostFtdcInvestorField* pInvestor, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTradingCode(CThostFtdcTradingCodeField* pTradingCode, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField* pInstrumentMarginRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField* pInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryUserSession(CThostFtdcUserSessionField* pUserSession, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryExchange(CThostFtdcExchangeField* pExchange, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryProduct(CThostFtdcProductField* pProduct, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTraderOffer(CThostFtdcTraderOfferField* pTraderOffer, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySettlementInfo(CThostFtdcSettlementInfoField* pSettlementInfo, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTransferBank(CThostFtdcTransferBankField* pTransferBank, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField* pInvestorPositionDetail, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryNotice(CThostFtdcNoticeField* pNotice, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField* pInvestorPositionCombineDetail, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField* pCFMMCTradingAccountKey, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField* pEWarrantOffset, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField* pInvestorProductGroupMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField* pExchangeMarginRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField* pExchangeMarginRateAdjust, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryExchangeRate(CThostFtdcExchangeRateField* pExchangeRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField* pSecAgentACIDMap, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryProductExchRate(CThostFtdcProductExchRateField* pProductExchRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryProductGroup(CThostFtdcProductGroupField* pProductGroup, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField* pMMInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField* pMMOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField* pInstrumentOrderCommRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField* pSecAgentCheckMode, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField* pSecAgentTradeInfo, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField* pOptionInstrTradeCost, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField* pOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryExecOrder(CThostFtdcExecOrderField* pExecOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryForQuote(CThostFtdcForQuoteField* pForQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryQuote(CThostFtdcQuoteField* pQuote, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestUnit(CThostFtdcInvestUnitField* pInvestUnit, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField* pCombInstrumentGuard, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryCombAction(CThostFtdcCombActionField* pCombAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTransferSerial(CThostFtdcTransferSerialField* pTransferSerial, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryAccountregister(CThostFtdcAccountregisterField* pAccountregister, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspError(CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnOrder(CThostFtdcOrderField* pOrder) override;
    void OnRtnTrade(CThostFtdcTradeField* pTrade) override;
    void OnErrRtnOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnOrderAction(CThostFtdcOrderActionField* pOrderAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField* pInstrumentStatus) override;
    void OnRtnBulletin(CThostFtdcBulletinField* pBulletin) override;
    void OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField* pTradingNoticeInfo) override;
    void OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField* pErrorConditionalOrder) override;
    void OnRtnExecOrder(CThostFtdcExecOrderField* pExecOrder) override;
    void OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField* pExecOrderAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRtnQuote(CThostFtdcQuoteField* pQuote) override;
    void OnErrRtnQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnQuoteAction(CThostFtdcQuoteActionField* pQuoteAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) override;
    void OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField* pCFMMCTradingAccountToken) override;
    void OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField* pBatchOrderAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose) override;
    void OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField* pOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRtnCombAction(CThostFtdcCombActionField* pCombAction) override;
    void OnErrRtnCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRspQryContractBank(CThostFtdcContractBankField* pContractBank, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryParkedOrder(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryTradingNotice(CThostFtdcTradingNoticeField* pTradingNotice, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField* pBrokerTradingParams, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField* pBrokerTradingAlgos, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField* pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField* pRspTransfer) override;
    void OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField* pRspTransfer) override;
    void OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField* pRspRepeal) override;
    void OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField* pRspRepeal) override;
    void OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField* pRspTransfer) override;
    void OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField* pRspTransfer) override;
    void OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField* pRspRepeal) override;
    void OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField* pRspRepeal) override;
    void OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField* pNotifyQueryAccount) override;
    void OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField* pRspRepeal) override;
    void OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField* pRspRepeal) override;
    void OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnOpenAccountByBank(CThostFtdcOpenAccountField* pOpenAccount) override;
    void OnRtnCancelAccountByBank(CThostFtdcCancelAccountField* pCancelAccount) override;
    void OnRtnChangeAccountByBank(CThostFtdcChangeAccountField* pChangeAccount) override;
    void OnRspQryClassifiedInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryCombPromotionParam(CThostFtdcCombPromotionParamField* pCombPromotionParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRiskSettleInvstPosition(CThostFtdcRiskSettleInvstPositionField* pRiskSettleInvstPosition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRiskSettleProductStatus(CThostFtdcRiskSettleProductStatusField* pRiskSettleProductStatus, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMFutureParameter(CThostFtdcSPBMFutureParameterField* pSPBMFutureParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMOptionParameter(CThostFtdcSPBMOptionParameterField* pSPBMOptionParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMIntraParameter(CThostFtdcSPBMIntraParameterField* pSPBMIntraParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMInterParameter(CThostFtdcSPBMInterParameterField* pSPBMInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMPortfDefinition(CThostFtdcSPBMPortfDefinitionField* pSPBMPortfDefinition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMInvestorPortfDef(CThostFtdcSPBMInvestorPortfDefField* pSPBMInvestorPortfDef, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorPortfMarginRatio(CThostFtdcInvestorPortfMarginRatioField* pInvestorPortfMarginRatio, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorProdSPBMDetail(CThostFtdcInvestorProdSPBMDetailField* pInvestorProdSPBMDetail, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorCommoditySPMMMargin(CThostFtdcInvestorCommoditySPMMMarginField* pInvestorCommoditySPMMMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorCommodityGroupSPMMMargin(CThostFtdcInvestorCommodityGroupSPMMMarginField* pInvestorCommodityGroupSPMMMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPMMInstParam(CThostFtdcSPMMInstParamField* pSPMMInstParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPMMProductParam(CThostFtdcSPMMProductParamField* pSPMMProductParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySPBMAddOnInterParameter(CThostFtdcSPBMAddOnInterParameterField* pSPBMAddOnInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRCAMSCombProductInfo(CThostFtdcRCAMSCombProductInfoField* pRCAMSCombProductInfo, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRCAMSInstrParameter(CThostFtdcRCAMSInstrParameterField* pRCAMSInstrParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRCAMSIntraParameter(CThostFtdcRCAMSIntraParameterField* pRCAMSIntraParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRCAMSInterParameter(CThostFtdcRCAMSInterParameterField* pRCAMSInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRCAMSShortOptAdjustParam(CThostFtdcRCAMSShortOptAdjustParamField* pRCAMSShortOptAdjustParam, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRCAMSInvestorCombPosition(CThostFtdcRCAMSInvestorCombPositionField* pRCAMSInvestorCombPosition, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorProdRCAMSMargin(CThostFtdcInvestorProdRCAMSMarginField* pInvestorProdRCAMSMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRULEInstrParameter(CThostFtdcRULEInstrParameterField* pRULEInstrParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRULEIntraParameter(CThostFtdcRULEIntraParameterField* pRULEIntraParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryRULEInterParameter(CThostFtdcRULEInterParameterField* pRULEInterParameter, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorProdRULEMargin(CThostFtdcInvestorProdRULEMarginField* pInvestorProdRULEMargin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorPortfSetting(CThostFtdcInvestorPortfSettingField* pInvestorPortfSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryInvestorInfoCommRec(CThostFtdcInvestorInfoCommRecField* pInvestorInfoCommRec, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryCombLeg(CThostFtdcCombLegField* pCombLeg, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspCancelOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting) override;
    void OnErrRtnOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnCancelOffsetSetting(CThostFtdcCancelOffsetSettingField* pCancelOffsetSetting, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRspQryOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspGenSMSCode(CThostFtdcRspGenSMSCodeField* pRspGenSMSCode, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspSpdApply(CThostFtdcInputSpdApplyField* pInputSpdApply, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspSpdApplyAction(CThostFtdcInputSpdApplyActionField* pInputSpdApplyAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQrySpdApply(CThostFtdcSpdApplyField* pSpdApply, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnSpdApply(CThostFtdcSpdApplyField* pSpdApply) override;
    void OnErrRtnSpdApply(CThostFtdcInputSpdApplyField* pInputSpdApply, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnSpdApplyAction(CThostFtdcSpdApplyActionField* pSpdApplyAction, CThostFtdcRspInfoField* pRspInfo) override;
    void OnRspHedgeCfm(CThostFtdcInputHedgeCfmField* pInputHedgeCfm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspHedgeCfmAction(CThostFtdcInputHedgeCfmActionField* pInputHedgeCfmAction, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRspQryHedgeCfm(CThostFtdcHedgeCfmField* pHedgeCfm, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) override;
    void OnRtnHedgeCfm(CThostFtdcHedgeCfmField* pHedgeCfm) override;
    void OnErrRtnHedgeCfm(CThostFtdcInputHedgeCfmField* pInputHedgeCfm, CThostFtdcRspInfoField* pRspInfo) override;
    void OnErrRtnHedgeCfmAction(CThostFtdcHedgeCfmActionField* pHedgeCfmAction, CThostFtdcRspInfoField* pRspInfo) override;

private:
    rust::Box<TraderSpiSink> sink_;
};

class TraderApiBridge final {
public:
    TraderApiBridge(CThostFtdcTraderApi* api, std::unique_ptr<TraderSpiBridge> spi);
    ~TraderApiBridge();

    TraderApiBridge(const TraderApiBridge&) = delete;
    TraderApiBridge& operator=(const TraderApiBridge&) = delete;

    rust::String version() const;
    void init() const;
    std::int32_t join() const;
    rust::String trading_day() const;
    void register_front(rust::Str address) const;
    void register_name_server(rust::Str address) const;
    void front_info(std::uint8_t* info) const;
    void register_fens_user_info(std::uint8_t* request) const;
    void subscribe_private(std::int32_t resume) const;
    void subscribe_public(std::int32_t resume) const;
    std::int32_t authenticate(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t login(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t logout(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t confirm_settlement(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t settlement_confirmation(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_settlement_info(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_investor_position_detail(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_product(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_exchange(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_trading_code(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_instrument_margin_rate(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_instrument_commission_rate(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_instrument_order_comm_rate(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_max_order_volume(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_depth_market_data(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_order(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_trade(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_investor_position(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_trading_account(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t query_instrument(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t insert_order(std::uint8_t* request, std::int32_t request_id) const;
    std::int32_t cancel_order(std::uint8_t* request, std::int32_t request_id) const;

private:
    CThostFtdcTraderApi* api_;
    std::unique_ptr<TraderSpiBridge> spi_;
    // CTP 未调用 Init 时内部工作线程不存在，Release 会 join 空线程，新版 glibc 与 macOS 会直接崩溃。
    mutable bool initialized_ = false;
};

std::unique_ptr<TraderSpiBridge> create_trader_spi(rust::Box<TraderSpiSink> sink);
std::unique_ptr<TraderApiBridge> create_trader_api(rust::Box<TraderSpiSink> sink, rust::Str flow_path, bool using_udp, bool multicast, bool production_mode);
void emit_trader_order(TraderSpiBridge& spi, std::uint8_t* data);
void emit_trader_callback(TraderSpiBridge& spi, std::uint32_t callback_index, std::uint8_t* first, std::uint8_t* second, std::int32_t request_id, bool last);

} // namespace ctp_base::trader

