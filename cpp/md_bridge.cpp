#include "cpp/bridge.h"

#include <utility>
#include <vector>

namespace ctp_base::md {

MdSpiBridge::MdSpiBridge(rust::Box<MdSpiSink> sink) : sink_(std::move(sink)) {}

void MdSpiBridge::OnFrontConnected() {
    connected(*sink_);
}

void MdSpiBridge::OnFrontDisconnected(int nReason) {
    disconnected(*sink_, nReason);
}

void MdSpiBridge::OnHeartBeatWarning(int nTimeLapse) {
    heartbeat(*sink_, nTimeLapse);
}

void MdSpiBridge::OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    login_response(*sink_, reinterpret_cast<const std::uint8_t*>(pRspUserLogin), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    logout_response(*sink_, reinterpret_cast<const std::uint8_t*>(pUserLogout), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField* pMulticastInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    multicast_instrument_response(*sink_, reinterpret_cast<const std::uint8_t*>(pMulticastInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspError(CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    error(*sink_, reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    subscribe_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSpecificInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    unsubscribe_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSpecificInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    subscribe_quote_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSpecificInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast) {
    unsubscribe_quote_response(*sink_, reinterpret_cast<const std::uint8_t*>(pSpecificInstrument), reinterpret_cast<const std::uint8_t*>(pRspInfo), nRequestID, bIsLast);
}

void MdSpiBridge::OnRtnDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData) {
    tick(*sink_, reinterpret_cast<const std::uint8_t*>(pDepthMarketData));
}

void MdSpiBridge::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) {
    for_quote_response(*sink_, reinterpret_cast<const std::uint8_t*>(pForQuoteRsp));
}

MdApiBridge::MdApiBridge(CThostFtdcMdApi* api, std::unique_ptr<MdSpiBridge> spi)
    : api_(api), spi_(std::move(spi)) {
    api_->RegisterSpi(spi_.get());
}

MdApiBridge::~MdApiBridge() {
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

rust::String MdApiBridge::version() const {
    const char* value = CThostFtdcMdApi::GetApiVersion();
    return value == nullptr ? rust::String() : rust::String(value);
}

void MdApiBridge::init() const { initialized_ = true; api_->Init(); }

std::int32_t MdApiBridge::join() const { return api_->Join(); }

rust::String MdApiBridge::trading_day() const {
    const char* value = api_->GetTradingDay();
    return value == nullptr ? rust::String() : rust::String(value);
}

void MdApiBridge::register_front(rust::Str address) const {
    const std::string value(address);
    api_->RegisterFront(const_cast<char*>(value.c_str()));
}

void MdApiBridge::register_name_server(rust::Str address) const {
    const std::string value(address);
    api_->RegisterNameServer(const_cast<char*>(value.c_str()));
}

void MdApiBridge::register_fens_user_info(std::uint8_t* request) const {
    api_->RegisterFensUserInfo(reinterpret_cast<CThostFtdcFensUserInfoField*>(request));
}

namespace {
int subscribe_impl(CThostFtdcMdApi* api, rust::Vec<rust::String> instruments,
                   int (CThostFtdcMdApi::*method)(char**, int)) {
    std::vector<char*> pointers;
    pointers.reserve(instruments.size());
    for (auto& instrument : instruments) {
        pointers.push_back(const_cast<char*>(instrument.c_str()));
    }
    return (api->*method)(pointers.data(), static_cast<int>(pointers.size()));
}
} // namespace

std::int32_t MdApiBridge::subscribe(rust::Vec<rust::String> instruments) const {
    return subscribe_impl(api_, std::move(instruments), &CThostFtdcMdApi::SubscribeMarketData);
}

std::int32_t MdApiBridge::unsubscribe(rust::Vec<rust::String> instruments) const {
    return subscribe_impl(api_, std::move(instruments), &CThostFtdcMdApi::UnSubscribeMarketData);
}

std::int32_t MdApiBridge::subscribe_quote(rust::Vec<rust::String> instruments) const {
    return subscribe_impl(api_, std::move(instruments), &CThostFtdcMdApi::SubscribeForQuoteRsp);
}

std::int32_t MdApiBridge::unsubscribe_quote(rust::Vec<rust::String> instruments) const {
    return subscribe_impl(api_, std::move(instruments), &CThostFtdcMdApi::UnSubscribeForQuoteRsp);
}

std::int32_t MdApiBridge::req_user_login(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLogin(
        reinterpret_cast<CThostFtdcReqUserLoginField*>(request), request_id);
}

std::int32_t MdApiBridge::logout(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqUserLogout(reinterpret_cast<CThostFtdcUserLogoutField*>(request), request_id);
}

std::int32_t MdApiBridge::query_multicast_instrument(std::uint8_t* request, std::int32_t request_id) const {
    return api_->ReqQryMulticastInstrument(
        reinterpret_cast<CThostFtdcQryMulticastInstrumentField*>(request), request_id);
}

std::unique_ptr<MdSpiBridge> create_md_spi(rust::Box<MdSpiSink> sink) {
    return std::make_unique<MdSpiBridge>(std::move(sink));
}

std::unique_ptr<MdApiBridge> create_md_api(rust::Box<MdSpiSink> sink, rust::Str flow_path, bool using_udp, bool multicast, bool production_mode) {
    const std::string path(flow_path);
    CThostFtdcMdApi* api = CThostFtdcMdApi::CreateFtdcMdApi(path.c_str(), using_udp, multicast, production_mode);
    (void)using_udp;
    (void)multicast;
    (void)production_mode;
    if (api == nullptr) {
        return nullptr;
    }
    return std::make_unique<MdApiBridge>(api, create_md_spi(std::move(sink)));
}

void emit_depth_market_data(MdSpiBridge& spi, std::uint8_t* data) {
    spi.OnRtnDepthMarketData(reinterpret_cast<CThostFtdcDepthMarketDataField*>(data));
}

void emit_md_front_connected(MdSpiBridge& spi) {
    spi.OnFrontConnected();
}

void emit_md_callback(MdSpiBridge& spi, std::uint32_t callback_index, std::uint8_t* first, std::uint8_t* second, std::int32_t request_id, bool last) {
    switch (callback_index) {
    case 0: spi.OnFrontConnected(); break;
    case 1: spi.OnFrontDisconnected(request_id); break;
    case 2: spi.OnHeartBeatWarning(request_id); break;
    case 3: spi.OnRspUserLogin(reinterpret_cast<CThostFtdcRspUserLoginField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 4: spi.OnRspUserLogout(reinterpret_cast<CThostFtdcUserLogoutField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 5: spi.OnRspQryMulticastInstrument(reinterpret_cast<CThostFtdcMulticastInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 6: spi.OnRspError(reinterpret_cast<CThostFtdcRspInfoField*>(first), request_id, last); break;
    case 7: spi.OnRspSubMarketData(reinterpret_cast<CThostFtdcSpecificInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 8: spi.OnRspUnSubMarketData(reinterpret_cast<CThostFtdcSpecificInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 9: spi.OnRspSubForQuoteRsp(reinterpret_cast<CThostFtdcSpecificInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 10: spi.OnRspUnSubForQuoteRsp(reinterpret_cast<CThostFtdcSpecificInstrumentField*>(first), reinterpret_cast<CThostFtdcRspInfoField*>(second), request_id, last); break;
    case 11: spi.OnRtnDepthMarketData(reinterpret_cast<CThostFtdcDepthMarketDataField*>(first)); break;
    case 12: spi.OnRtnForQuoteRsp(reinterpret_cast<CThostFtdcForQuoteRspField*>(first)); break;
    default: break;
    }
}

} // namespace ctp_base::md
