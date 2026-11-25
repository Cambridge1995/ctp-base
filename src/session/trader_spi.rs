//! 交易 SPI 适配器：只转换并入队，不做任何决策。
//!
//! 批次一实现会话链（认证/登录/结算/登出/错误路径）；
//! 批次二实现交易指令（报撤单回执 + 订单/成交/错单回报）；
//! 批次三~七实现查询、高级指令、银期转账、保证金/费率、风险监控、
//! 对冲设置、SPD/套保确认回调；其余回调按产品取舍保留默认实现。

pub(super) use crate::session::enums::{QueryRow, TraderEvent, TraderInput, TraderSessionEvent};
pub(super) use crate::structs::{
    CancelOffsetSetting, CombPromotionParam, HedgeCfm, HedgeCfmAction, InputHedgeCfm,
    InputHedgeCfmAction, InputOffsetSetting, InputSpdApply, InputSpdApplyAction, InstrTradeLimit,
    OffsetSetting, SpdApply, SpdApplyAction,
};
pub(super) use crate::structs::{
    CombInstrumentGuard, ExchangeMarginRate, ExchangeMarginRateAdjust, ExchangeRate,
    InstrumentCommissionRate, InstrumentMarginRate, InstrumentOrderCommRate,
    InvestorCommodityGroupSPMMMargin, InvestorCommoditySPMMMargin, InvestorInfoCommRec,
    InvestorPortfMarginRatio, InvestorPortfSetting, InvestorProdRCAMSMargin,
    InvestorProdRULEMargin, InvestorProdSPBMDetail, InvestorProductGroupMargin,
    MMInstrumentCommissionRate, MMOptionInstrCommRate, OptionInstrCommRate, OptionInstrTradeCost,
    ProductExchRate, ProductGroup, QryMaxOrderVolume, QryMaxOrderVolumeWithPrice,
    RCAMSCombProductInfo, RCAMSInstrParameter, RCAMSInterParameter, RCAMSIntraParameter,
    RCAMSInvestorCombPosition, RCAMSShortOptAdjustParam, RULEInstrParameter, RULEInterParameter,
    RULEIntraParameter, RiskSettleInvestPosition, RiskSettleProductStatus, SPBMAddOnInterParameter,
    SPBMFutureParameter, SPBMInterParameter, SPBMIntraParameter, SPBMInvestorPortfDef,
    SPBMOptionParameter, SPBMPortfDefinition, SPMMInstParam, SPMMProductParam,
};
pub(super) use crate::{
    AccountRegister, BatchOrderAction, CancelAccount, ChangeAccount, CombAction, CombLeg,
    ContractBank, ConversionError, DepthMarketData, DisconnectReason, ErrorConditionalOrder,
    Exchange, ExecOrder, ExecOrderAction, ForQuote, ForQuoteRsp, InputBatchOrderAction,
    InputCombAction, InputExecOrder, InputExecOrderAction, InputForQuote, InputOptionSelfClose,
    InputOptionSelfCloseAction, InputOrder, InputOrderAction, InputQuote, InputQuoteAction,
    Instrument, InstrumentStatus, Investor, InvestorPosition, InvestorPositionCombineDetail,
    InvestorPositionDetail, NotifyQueryAccount, OpenAccount, OptionSelfClose,
    OptionSelfCloseAction, Order, OrderAction, ParkedOrder, ParkedOrderAction, Product, Quote,
    QuoteAction, RemoveParkedOrder, RemoveParkedOrderAction, ReqQueryAccount, ReqRepeal,
    ReqTransfer, RspAuthenticate, RspInfo, RspRepeal, RspTransfer, RspUserLogin, SettlementInfo,
    SettlementInfoConfirm, Trade, TraderSpi, TradingAccount, TransferBank, TransferSerial,
    UserLogout,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::SyncSender;

/// 交易 SPI 适配器。
pub(crate) struct TraderSpiImpl {
    inputs: SyncSender<TraderInput>,
    dropped: Arc<AtomicU64>,
}

impl TraderSpiImpl {
    pub(crate) fn new(inputs: SyncSender<TraderInput>, dropped: Arc<AtomicU64>) -> Self {
        Self { inputs, dropped }
    }

    /// 非阻塞投递控制事件；队列满时丢弃并计数（回调线程绝不阻塞）。
    fn send(&mut self, event: TraderEvent) {
        if self.inputs.try_send(TraderInput::Spi(event)).is_err() {
            self.dropped.fetch_add(1, Ordering::AcqRel);
        }
    }
}

/// 从响应信息提取成功/失败：error_id 非 0 为失败。
fn check(info: Option<RspInfo>) -> Result<(), (i32, String)> {
    match info {
        Some(info) if info.error_id != 0 => Err((info.error_id, info.error_msg)),
        _ => Ok(()),
    }
}

/// 拆解响应信息为 (error_id, message)；无信息视为成功。
fn info_parts(info: Option<RspInfo>) -> (i32, String) {
    match info {
        Some(info) => (info.error_id, info.error_msg),
        None => (0, String::new()),
    }
}

impl TraderSpiImpl {
    /// 投递查询响应的一行；空载荷（被过滤的行）也必须投递，批次靠 is_last 结束。
    fn send_query_row(
        &mut self,
        row: Option<QueryRow>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        let (error_id, message) = info_parts(rsp_info);
        self.send(TraderEvent::QueryRow {
            request_id,
            is_last,
            error_id,
            message,
            row,
        });
    }

    /// 应答/错误类回调公共处理：拆解错误信息、载荷装箱后按构造器透传。
    fn respond<T>(
        &mut self,
        payload: Option<T>,
        rsp_info: Option<RspInfo>,
        event: impl FnOnce(Option<Box<T>>, i32, String) -> TraderSessionEvent,
    ) {
        let (error_id, message) = info_parts(rsp_info);
        self.send(TraderEvent::Report(event(
            payload.map(Box::new),
            error_id,
            message,
        )));
    }

    /// 数据回报公共处理：空指针不产生事件，非空装箱透传。
    fn pass<T>(&mut self, payload: Option<T>, event: impl FnOnce(Box<T>) -> TraderSessionEvent) {
        if let Some(payload) = payload {
            self.send(TraderEvent::Report(event(Box::new(payload))));
        }
    }
}

impl TraderSpi for TraderSpiImpl {
    /// owned 转换失败：数据损坏被丢弃，计入丢弃数并上报事件，不得无声无息。
    fn conversion_error(&mut self, error: ConversionError) {
        self.dropped.fetch_add(1, Ordering::AcqRel);
        self.send(TraderEvent::ConversionFailed(format!("{error:?}")));
    }

    fn connected(&mut self) {
        self.send(TraderEvent::Connected);
    }

    fn disconnected(&mut self, reason: DisconnectReason) {
        self.send(TraderEvent::Disconnected(reason));
    }

    fn authenticate_response(
        &mut self,
        _rsp: Option<RspAuthenticate>,
        info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match check(info) {
            Ok(()) => self.send(TraderEvent::AuthenticateOk),
            Err((error_id, message)) => self.send(TraderEvent::Failed { error_id, message }),
        }
    }

    fn login_response(
        &mut self,
        rsp_user_login: Option<RspUserLogin>,
        rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match check(rsp_info) {
            Ok(()) => match rsp_user_login {
                Some(login) => self.send(TraderEvent::LoginOk {
                    front_id: login.front_id.get(),
                    session_id: login.session_id.get(),
                    trading_day: login.trading_day.as_str().to_owned(),
                }),
                None => self.send(TraderEvent::LoginOk {
                    front_id: 0,
                    session_id: 0,
                    trading_day: String::new(),
                }),
            },
            Err((error_id, message)) => self.send(TraderEvent::Failed { error_id, message }),
        }
    }

    fn logout_response(
        &mut self,
        _user_logout: Option<UserLogout>,
        _rsp_info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        self.send(TraderEvent::Logout);
    }

    fn order_insert_response(
        &mut self,
        input_order: Option<InputOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_order, rsp_info, |order, error_id, message| {
            TraderSessionEvent::OrderInsertResponse {
                request_id,
                order,
                error_id,
                message,
            }
        });
    }

    fn parked_order_insert_response(
        &mut self,
        parked_order: Option<ParkedOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(parked_order, rsp_info, |order, error_id, message| {
            TraderSessionEvent::ParkedOrderInsertResponse {
                request_id,
                order,
                error_id,
                message,
            }
        });
    }

    // 批次二：交易指令。响应带 request_id（回声），回报类不带（按 OrderRef 对账）。

    fn parked_action_response(
        &mut self,
        parked_order_action: Option<ParkedOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            parked_order_action,
            rsp_info,
            |action, error_id, message| TraderSessionEvent::ParkedActionResponse {
                request_id,
                action,
                error_id,
                message,
            },
        );
    }

    fn order_action_response(
        &mut self,
        input_order_action: Option<InputOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_order_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::OrderActionResponse {
                request_id,
                action,
                error_id,
                message,
            }
        });
    }

    fn max_order_volume_response(
        &mut self,
        payload: Option<QryMaxOrderVolume>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::MaxOrderVolume),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn settlement_confirm_response(
        &mut self,
        _confirm: Option<SettlementInfoConfirm>,
        info: Option<RspInfo>,
        _request_id: i32,
        _is_last: bool,
    ) {
        match check(info) {
            Ok(()) => self.send(TraderEvent::SettlementConfirmed),
            Err((error_id, message)) => self.send(TraderEvent::Failed { error_id, message }),
        }
    }

    fn remove_parked_order_response(
        &mut self,
        remove_parked_order: Option<RemoveParkedOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            remove_parked_order,
            rsp_info,
            |removed, error_id, message| TraderSessionEvent::RemoveParkedOrderResponse {
                request_id,
                removed,
                error_id,
                message,
            },
        );
    }

    fn remove_parked_order_action_response(
        &mut self,
        remove_parked_order_action: Option<RemoveParkedOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            remove_parked_order_action,
            rsp_info,
            |removed, error_id, message| TraderSessionEvent::RemoveParkedOrderActionResponse {
                request_id,
                removed,
                error_id,
                message,
            },
        );
    }

    fn max_order_volume_with_price_query_response(
        &mut self,
        payload: Option<QryMaxOrderVolumeWithPrice>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::MaxOrderVolumeWithPrice),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn exec_order_insert_response(
        &mut self,
        input_exec_order: Option<InputExecOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_exec_order, rsp_info, |order, error_id, message| {
            TraderSessionEvent::ExecOrderInsertResponse {
                request_id,
                order,
                error_id,
                message,
            }
        });
    }

    fn exec_order_action_response(
        &mut self,
        input_exec_order_action: Option<InputExecOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            input_exec_order_action,
            rsp_info,
            |action, error_id, message| TraderSessionEvent::ExecOrderActionResponse {
                request_id,
                action,
                error_id,
                message,
            },
        );
    }

    // 批次三：核心查询。多行响应逐行投递，loop 立即公开，调用方按需聚合；
    // 空载荷行（被过滤）同样投递，否则批次无法结束。

    fn for_quote_insert_response(
        &mut self,
        input_for_quote: Option<InputForQuote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_for_quote, rsp_info, |for_quote, error_id, message| {
            TraderSessionEvent::ForQuoteInsertResponse {
                request_id,
                for_quote,
                error_id,
                message,
            }
        });
    }

    fn quote_insert_response(
        &mut self,
        input_quote: Option<InputQuote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_quote, rsp_info, |quote, error_id, message| {
            TraderSessionEvent::QuoteInsertResponse {
                request_id,
                quote,
                error_id,
                message,
            }
        });
    }

    fn quote_action_response(
        &mut self,
        input_quote_action: Option<InputQuoteAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_quote_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::QuoteActionResponse {
                request_id,
                action,
                error_id,
                message,
            }
        });
    }

    fn batch_order_action_response(
        &mut self,
        input_batch_order_action: Option<InputBatchOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            input_batch_order_action,
            rsp_info,
            |action, error_id, message| TraderSessionEvent::BatchOrderActionResponse {
                request_id,
                action,
                error_id,
                message,
            },
        );
    }

    fn option_self_close_insert_response(
        &mut self,
        input_option_self_close: Option<InputOptionSelfClose>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            input_option_self_close,
            rsp_info,
            |order, error_id, message| TraderSessionEvent::OptionSelfCloseInsertResponse {
                request_id,
                order,
                error_id,
                message,
            },
        );
    }

    fn option_self_close_action_response(
        &mut self,
        input_option_self_close_action: Option<InputOptionSelfCloseAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(
            input_option_self_close_action,
            rsp_info,
            |action, error_id, message| TraderSessionEvent::OptionSelfCloseActionResponse {
                request_id,
                action,
                error_id,
                message,
            },
        );
    }

    fn comb_action_insert_response(
        &mut self,
        input_comb_action: Option<InputCombAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(input_comb_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::CombActionInsertResponse {
                request_id,
                action,
                error_id,
                message,
            }
        });
    }

    fn order_response(
        &mut self,
        order: Option<Order>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            order.map(Box::new).map(QueryRow::Order),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn trade_response(
        &mut self,
        trade: Option<Trade>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            trade.map(Box::new).map(QueryRow::Trade),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_position_response(
        &mut self,
        investor_position: Option<InvestorPosition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            investor_position
                .map(Box::new)
                .map(QueryRow::InvestorPosition),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn trading_account_response(
        &mut self,
        trading_account: Option<TradingAccount>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            trading_account.map(Box::new).map(QueryRow::TradingAccount),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_response(
        &mut self,
        investor: Option<Investor>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            investor.map(Box::new).map(QueryRow::Investor),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn instrument_margin_rate_response(
        &mut self,
        payload: Option<InstrumentMarginRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InstrumentMarginRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn instrument_commission_rate_response(
        &mut self,
        payload: Option<InstrumentCommissionRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::InstrumentCommissionRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    // 批次四：高级指令（行权/期权自对冲/组合/报价询价/预埋单）。

    fn exchange_response(
        &mut self,
        exchange: Option<Exchange>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            exchange.map(Box::new).map(QueryRow::Exchange),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn product_response(
        &mut self,
        product: Option<Product>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            product.map(Box::new).map(QueryRow::Product),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn instrument_response(
        &mut self,
        instrument: Option<Instrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            instrument.map(Box::new).map(QueryRow::Instrument),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn depth_market_data_response(
        &mut self,
        depth_market_data: Option<DepthMarketData>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            depth_market_data
                .map(Box::new)
                .map(QueryRow::DepthMarketData),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn settlement_info_response(
        &mut self,
        settlement_info: Option<SettlementInfo>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            settlement_info.map(Box::new).map(QueryRow::SettlementInfo),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn transfer_bank_response(
        &mut self,
        transfer_bank: Option<TransferBank>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            transfer_bank.map(Box::new).map(QueryRow::TransferBank),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_position_detail_response(
        &mut self,
        investor_position_detail: Option<InvestorPositionDetail>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            investor_position_detail
                .map(Box::new)
                .map(QueryRow::InvestorPositionDetail),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_position_combine_detail_response(
        &mut self,
        investor_position_combine_detail: Option<InvestorPositionCombineDetail>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            investor_position_combine_detail
                .map(Box::new)
                .map(QueryRow::InvestorPositionCombineDetail),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_product_group_margin_response(
        &mut self,
        payload: Option<InvestorProductGroupMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::InvestorProductGroupMargin),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn exchange_margin_rate_response(
        &mut self,
        payload: Option<ExchangeMarginRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::ExchangeMarginRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn exchange_margin_rate_adjust_response(
        &mut self,
        payload: Option<ExchangeMarginRateAdjust>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::ExchangeMarginRateAdjust),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn exchange_rate_response(
        &mut self,
        payload: Option<ExchangeRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::ExchangeRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn product_exchange_rate_response(
        &mut self,
        payload: Option<ProductExchRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::ProductExchRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn product_group_response(
        &mut self,
        payload: Option<ProductGroup>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::ProductGroup),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn mm_instrument_commission_rate_response(
        &mut self,
        payload: Option<MMInstrumentCommissionRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::MMInstrumentCommissionRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn mm_option_instrument_commission_rate_response(
        &mut self,
        payload: Option<MMOptionInstrCommRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::MMOptionInstrCommRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn instrument_order_commission_rate_response(
        &mut self,
        payload: Option<InstrumentOrderCommRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InstrumentOrderCommRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn option_instrument_trade_cost_response(
        &mut self,
        payload: Option<OptionInstrTradeCost>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::OptionInstrTradeCost),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn option_instrument_commission_rate_response(
        &mut self,
        payload: Option<OptionInstrCommRate>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::OptionInstrCommRate),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn exec_order_response(
        &mut self,
        exec_order: Option<ExecOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            exec_order.map(Box::new).map(QueryRow::ExecOrder),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn for_quote_query_response(
        &mut self,
        for_quote: Option<ForQuote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            for_quote.map(Box::new).map(QueryRow::ForQuote),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn quote_response(
        &mut self,
        quote: Option<Quote>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            quote.map(Box::new).map(QueryRow::Quote),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn option_self_close_response(
        &mut self,
        option_self_close: Option<OptionSelfClose>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            option_self_close
                .map(Box::new)
                .map(QueryRow::OptionSelfClose),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn comb_instrument_guard_response(
        &mut self,
        payload: Option<CombInstrumentGuard>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::CombInstrumentGuard),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn comb_action_response(
        &mut self,
        comb_action: Option<CombAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            comb_action.map(Box::new).map(QueryRow::CombAction),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn transfer_serial_response(
        &mut self,
        transfer_serial: Option<TransferSerial>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            transfer_serial.map(Box::new).map(QueryRow::TransferSerial),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn account_register_response(
        &mut self,
        account_register: Option<AccountRegister>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            account_register
                .map(Box::new)
                .map(QueryRow::AccountRegister),
            rsp_info,
            request_id,
            is_last,
        );
    }

    // 批次四查询响应：逐行投递，loop 立即公开，调用方按需聚合。

    /// 通用请求级错误（如流控拒绝、查询异步拒绝）：转成控制事件上报。
    fn error(&mut self, rsp_info: Option<RspInfo>, request_id: i32, _is_last: bool) {
        if let Some(info) = rsp_info
            && info.error_id != 0
        {
            self.send(TraderEvent::Error {
                request_id,
                error_id: info.error_id,
                message: info.error_msg,
            });
        }
    }

    /// 报单状态回报：含交易所拒单（被动撤单），空指针不产生事件。
    fn order_update(&mut self, order: Option<Order>) {
        self.pass(order, TraderSessionEvent::OrderUpdate);
    }

    /// 成交回报：空指针不产生事件。
    fn execution(&mut self, trade: Option<Trade>) {
        self.pass(trade, TraderSessionEvent::Execution);
    }

    fn order_insert_error(&mut self, input_order: Option<InputOrder>, rsp_info: Option<RspInfo>) {
        self.respond(input_order, rsp_info, |order, error_id, message| {
            TraderSessionEvent::OrderInsertError {
                order,
                error_id,
                message,
            }
        });
    }

    fn order_action_error(&mut self, order_action: Option<OrderAction>, rsp_info: Option<RspInfo>) {
        self.respond(order_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::OrderActionError {
                action,
                error_id,
                message,
            }
        });
    }

    /// 合约状态推送（公共流）：空指针不产生事件。
    fn instrument_status(&mut self, instrument_status: Option<InstrumentStatus>) {
        self.pass(instrument_status, TraderSessionEvent::InstrumentStatus);
    }

    fn conditional_order_error(&mut self, error_conditional_order: Option<ErrorConditionalOrder>) {
        self.send(TraderEvent::Report(
            TraderSessionEvent::ConditionalOrderError {
                order: error_conditional_order.map(Box::new),
            },
        ));
    }

    // 批次五：银期转账。应答带 request_id（回声），错误与回报不带（按流水对账）。

    fn exec_order_report(&mut self, exec_order: Option<ExecOrder>) {
        self.pass(exec_order, TraderSessionEvent::ExecOrderReport);
    }

    fn exec_order_insert_error(
        &mut self,
        input_exec_order: Option<InputExecOrder>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(input_exec_order, rsp_info, |order, error_id, message| {
            TraderSessionEvent::ExecOrderInsertError {
                order,
                error_id,
                message,
            }
        });
    }

    fn exec_order_action_error(
        &mut self,
        exec_order_action: Option<ExecOrderAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(exec_order_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::ExecOrderActionError {
                action,
                error_id,
                message,
            }
        });
    }

    fn for_quote_insert_error(
        &mut self,
        input_for_quote: Option<InputForQuote>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(input_for_quote, rsp_info, |for_quote, error_id, message| {
            TraderSessionEvent::ForQuoteInsertError {
                for_quote,
                error_id,
                message,
            }
        });
    }

    fn quote_report(&mut self, quote: Option<Quote>) {
        self.pass(quote, TraderSessionEvent::QuoteReport);
    }

    fn quote_insert_error(&mut self, input_quote: Option<InputQuote>, rsp_info: Option<RspInfo>) {
        self.respond(input_quote, rsp_info, |quote, error_id, message| {
            TraderSessionEvent::QuoteInsertError {
                quote,
                error_id,
                message,
            }
        });
    }

    fn quote_action_error(&mut self, quote_action: Option<QuoteAction>, rsp_info: Option<RspInfo>) {
        self.respond(quote_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::QuoteActionError {
                action,
                error_id,
                message,
            }
        });
    }

    /// 询价回报（期权做市相关）。
    fn for_quote_response(&mut self, for_quote_rsp: Option<ForQuoteRsp>) {
        self.pass(for_quote_rsp, TraderSessionEvent::ForQuoteReport);
    }

    // 转账与开销户回报：空指针不产生事件。

    fn batch_order_action_error(
        &mut self,
        batch_order_action: Option<BatchOrderAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(batch_order_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::BatchOrderActionError {
                action,
                error_id,
                message,
            }
        });
    }

    fn option_self_close_report(&mut self, option_self_close: Option<OptionSelfClose>) {
        self.pass(option_self_close, TraderSessionEvent::OptionSelfCloseReport);
    }

    fn option_self_close_insert_error(
        &mut self,
        input_option_self_close: Option<InputOptionSelfClose>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(
            input_option_self_close,
            rsp_info,
            |order, error_id, message| TraderSessionEvent::OptionSelfCloseInsertError {
                order,
                error_id,
                message,
            },
        );
    }

    fn option_self_close_action_error(
        &mut self,
        option_self_close_action: Option<OptionSelfCloseAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(
            option_self_close_action,
            rsp_info,
            |action, error_id, message| TraderSessionEvent::OptionSelfCloseActionError {
                action,
                error_id,
                message,
            },
        );
    }

    fn comb_action_report(&mut self, comb_action: Option<CombAction>) {
        self.pass(comb_action, TraderSessionEvent::CombActionReport);
    }

    fn comb_action_insert_error(
        &mut self,
        input_comb_action: Option<InputCombAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(input_comb_action, rsp_info, |action, error_id, message| {
            TraderSessionEvent::CombActionInsertError {
                action,
                error_id,
                message,
            }
        });
    }

    fn contract_bank_response(
        &mut self,
        contract_bank: Option<ContractBank>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            contract_bank.map(Box::new).map(QueryRow::ContractBank),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn parked_order_response(
        &mut self,
        parked_order: Option<ParkedOrder>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            parked_order.map(Box::new).map(QueryRow::ParkedOrder),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn parked_order_actions(
        &mut self,
        parked_order_action: Option<ParkedOrderAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            parked_order_action
                .map(Box::new)
                .map(QueryRow::ParkedOrderAction),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn from_bank_to_future_by_bank_report(&mut self, rsp_transfer: Option<RspTransfer>) {
        self.pass(rsp_transfer, TraderSessionEvent::BankToFutureByBankReport);
    }

    fn from_future_to_bank_by_bank_report(&mut self, rsp_transfer: Option<RspTransfer>) {
        self.pass(rsp_transfer, TraderSessionEvent::FutureToBankByBankReport);
    }

    fn repeal_from_bank_to_future_by_bank_report(&mut self, rsp_repeal: Option<RspRepeal>) {
        self.pass(
            rsp_repeal,
            TraderSessionEvent::RepealBankToFutureByBankReport,
        );
    }

    fn repeal_from_future_to_bank_by_bank_report(&mut self, rsp_repeal: Option<RspRepeal>) {
        self.pass(
            rsp_repeal,
            TraderSessionEvent::RepealFutureToBankByBankReport,
        );
    }

    fn from_bank_to_future_by_future_report(&mut self, rsp_transfer: Option<RspTransfer>) {
        self.pass(rsp_transfer, TraderSessionEvent::BankToFutureByFutureReport);
    }

    // 批次五查询响应：逐行投递，loop 立即公开，调用方按需聚合。

    fn from_future_to_bank_by_future_report(&mut self, rsp_transfer: Option<RspTransfer>) {
        self.pass(rsp_transfer, TraderSessionEvent::FutureToBankByFutureReport);
    }

    fn repeal_from_bank_to_future_by_future_manual_report(
        &mut self,
        rsp_repeal: Option<RspRepeal>,
    ) {
        self.pass(
            rsp_repeal,
            TraderSessionEvent::RepealBankToFutureManualReport,
        );
    }

    fn repeal_from_future_to_bank_by_future_manual_report(
        &mut self,
        rsp_repeal: Option<RspRepeal>,
    ) {
        self.pass(
            rsp_repeal,
            TraderSessionEvent::RepealFutureToBankManualReport,
        );
    }

    fn query_bank_balance_by_future_report(
        &mut self,
        notify_query_account: Option<NotifyQueryAccount>,
    ) {
        self.pass(
            notify_query_account,
            TraderSessionEvent::QueryBankBalanceReport,
        );
    }

    // 批次六：保证金、费率与风险监控查询响应。

    fn bank_to_future_by_future_error(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(req_transfer, rsp_info, |transfer, error_id, message| {
            TraderSessionEvent::BankToFutureError {
                transfer,
                error_id,
                message,
            }
        });
    }

    fn future_to_bank_by_future_error(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(req_transfer, rsp_info, |transfer, error_id, message| {
            TraderSessionEvent::FutureToBankError {
                transfer,
                error_id,
                message,
            }
        });
    }

    fn repeal_bank_to_future_by_future_manual_error(
        &mut self,
        req_repeal: Option<ReqRepeal>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(req_repeal, rsp_info, |repeal, error_id, message| {
            TraderSessionEvent::RepealBankToFutureError {
                repeal,
                error_id,
                message,
            }
        });
    }

    fn repeal_future_to_bank_by_future_manual_error(
        &mut self,
        req_repeal: Option<ReqRepeal>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(req_repeal, rsp_info, |repeal, error_id, message| {
            TraderSessionEvent::RepealFutureToBankError {
                repeal,
                error_id,
                message,
            }
        });
    }

    fn query_bank_balance_by_future_error(
        &mut self,
        req_query_account: Option<ReqQueryAccount>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(req_query_account, rsp_info, |query, error_id, message| {
            TraderSessionEvent::QueryBankBalanceError {
                query,
                error_id,
                message,
            }
        });
    }

    fn repeal_from_bank_to_future_by_future_report(&mut self, rsp_repeal: Option<RspRepeal>) {
        self.pass(rsp_repeal, TraderSessionEvent::RepealBankToFutureReport);
    }

    fn repeal_from_future_to_bank_by_future_report(&mut self, rsp_repeal: Option<RspRepeal>) {
        self.pass(rsp_repeal, TraderSessionEvent::RepealFutureToBankReport);
    }

    fn from_bank_to_future_by_future_response(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(req_transfer, rsp_info, |transfer, error_id, message| {
            TraderSessionEvent::BankToFutureResponse {
                request_id,
                transfer,
                error_id,
                message,
            }
        });
    }

    fn from_future_to_bank_by_future_response(
        &mut self,
        req_transfer: Option<ReqTransfer>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(req_transfer, rsp_info, |transfer, error_id, message| {
            TraderSessionEvent::FutureToBankResponse {
                request_id,
                transfer,
                error_id,
                message,
            }
        });
    }

    fn query_bank_account_money_by_future_response(
        &mut self,
        req_query_account: Option<ReqQueryAccount>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(req_query_account, rsp_info, |query, error_id, message| {
            TraderSessionEvent::QueryBankAccountMoneyResponse {
                request_id,
                query,
                error_id,
                message,
            }
        });
    }

    fn open_account_by_bank_report(&mut self, open_account: Option<OpenAccount>) {
        self.pass(open_account, TraderSessionEvent::OpenAccountReport);
    }

    fn cancel_account_by_bank_report(&mut self, cancel_account: Option<CancelAccount>) {
        self.pass(cancel_account, TraderSessionEvent::CancelAccountReport);
    }

    fn change_account_by_bank_report(&mut self, change_account: Option<ChangeAccount>) {
        self.pass(change_account, TraderSessionEvent::ChangeAccountReport);
    }

    fn classified_instrument_response(
        &mut self,
        payload: Option<Instrument>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::ClassifiedInstrument),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn comb_promotion_parameter_response(
        &mut self,
        payload: Option<CombPromotionParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::CombPromotionParam),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn risk_settle_investor_position_response(
        &mut self,
        payload: Option<RiskSettleInvestPosition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::RiskSettleInvestPosition),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn risk_settle_product_status_response(
        &mut self,
        payload: Option<RiskSettleProductStatus>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RiskSettleProductStatus),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_future_parameter_response(
        &mut self,
        payload: Option<SPBMFutureParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMFutureParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_option_parameter_response(
        &mut self,
        payload: Option<SPBMOptionParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMOptionParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_intra_parameter_response(
        &mut self,
        payload: Option<SPBMIntraParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMIntraParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_inter_parameter_response(
        &mut self,
        payload: Option<SPBMInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMInterParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_portfolio_definition_response(
        &mut self,
        payload: Option<SPBMPortfDefinition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMPortfDefinition),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_investor_portfolio_definition_response(
        &mut self,
        payload: Option<SPBMInvestorPortfDef>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMInvestorPortfDef),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_portfolio_margin_ratio_response(
        &mut self,
        payload: Option<InvestorPortfMarginRatio>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::InvestorPortfMarginRatio),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_product_spbm_detail_response(
        &mut self,
        payload: Option<InvestorProdSPBMDetail>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InvestorProdSPBMDetail),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_commodity_spmm_margin_response(
        &mut self,
        payload: Option<InvestorCommoditySPMMMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::InvestorCommoditySPMMMargin),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_commodity_group_spmm_margin_response(
        &mut self,
        payload: Option<InvestorCommodityGroupSPMMMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::InvestorCommodityGroupSPMMMargin),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spmm_instrument_parameter_response(
        &mut self,
        payload: Option<SPMMInstParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPMMInstParam),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spmm_product_parameter_response(
        &mut self,
        payload: Option<SPMMProductParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPMMProductParam),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spbm_add_on_inter_parameter_response(
        &mut self,
        payload: Option<SPBMAddOnInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SPBMAddOnInterParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rcams_comb_product_info_response(
        &mut self,
        payload: Option<RCAMSCombProductInfo>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RCAMSCombProductInfo),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rcams_instrument_parameter_response(
        &mut self,
        payload: Option<RCAMSInstrParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RCAMSInstrParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rcams_intra_parameter_response(
        &mut self,
        payload: Option<RCAMSIntraParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RCAMSIntraParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rcams_inter_parameter_response(
        &mut self,
        payload: Option<RCAMSInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RCAMSInterParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rcams_short_opt_adjust_parameter_response(
        &mut self,
        payload: Option<RCAMSShortOptAdjustParam>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::RCAMSShortOptAdjustParam),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rcams_investor_comb_position_response(
        &mut self,
        payload: Option<RCAMSInvestorCombPosition>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload
                .map(Box::new)
                .map(QueryRow::RCAMSInvestorCombPosition),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_product_rcams_margin_response(
        &mut self,
        payload: Option<InvestorProdRCAMSMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InvestorProdRCAMSMargin),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rule_instrument_parameter_response(
        &mut self,
        payload: Option<RULEInstrParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RULEInstrParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rule_intra_parameter_response(
        &mut self,
        payload: Option<RULEIntraParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RULEIntraParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn rule_inter_parameter_response(
        &mut self,
        payload: Option<RULEInterParameter>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::RULEInterParameter),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_product_rule_margin_response(
        &mut self,
        payload: Option<InvestorProdRULEMargin>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InvestorProdRULEMargin),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_portfolio_setting_response(
        &mut self,
        payload: Option<InvestorPortfSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InvestorPortfSetting),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn investor_info_commission_record_response(
        &mut self,
        payload: Option<InvestorInfoCommRec>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InvestorInfoCommRec),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn comb_leg_response(
        &mut self,
        comb_leg: Option<CombLeg>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            comb_leg.map(Box::new).map(QueryRow::CombLeg),
            rsp_info,
            request_id,
            is_last,
        );
    }

    // 批次七：对冲设置、SPD、套保确认与新增查询。

    fn set_offset_response(
        &mut self,
        payload: Option<InputOffsetSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(payload, rsp_info, |setting, error_id, message| {
            TraderSessionEvent::SetOffsetResponse {
                request_id,
                setting,
                error_id,
                message,
            }
        });
    }

    fn cancel_offset_setting_response(
        &mut self,
        payload: Option<InputOffsetSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(payload, rsp_info, |setting, error_id, message| {
            TraderSessionEvent::CancelOffsetSettingResponse {
                request_id,
                setting,
                error_id,
                message,
            }
        });
    }

    fn offset_setting_report(&mut self, payload: Option<OffsetSetting>) {
        self.pass(payload, TraderSessionEvent::OffsetSettingReport);
    }

    fn offset_setting_error(
        &mut self,
        payload: Option<InputOffsetSetting>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(payload, rsp_info, |setting, error_id, message| {
            TraderSessionEvent::OffsetSettingError {
                setting,
                error_id,
                message,
            }
        });
    }

    fn cancel_offset_setting_error(
        &mut self,
        payload: Option<CancelOffsetSetting>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(payload, rsp_info, |setting, error_id, message| {
            TraderSessionEvent::CancelOffsetSettingError {
                setting,
                error_id,
                message,
            }
        });
    }

    fn offset_settings_response(
        &mut self,
        payload: Option<OffsetSetting>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::OffsetSetting),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spd_apply_response(
        &mut self,
        payload: Option<InputSpdApply>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(payload, rsp_info, |apply, error_id, message| {
            TraderSessionEvent::SpdApplyResponse {
                request_id,
                apply,
                error_id,
                message,
            }
        });
    }

    fn spd_apply_action_response(
        &mut self,
        payload: Option<InputSpdApplyAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(payload, rsp_info, |action, error_id, message| {
            TraderSessionEvent::SpdApplyActionResponse {
                request_id,
                action,
                error_id,
                message,
            }
        });
    }

    fn spd_apply_query_response(
        &mut self,
        payload: Option<SpdApply>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::SpdApply),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn spd_apply_report(&mut self, payload: Option<SpdApply>) {
        self.pass(payload, TraderSessionEvent::SpdApplyReport);
    }

    fn spd_apply_error(&mut self, payload: Option<InputSpdApply>, rsp_info: Option<RspInfo>) {
        self.respond(payload, rsp_info, |apply, error_id, message| {
            TraderSessionEvent::SpdApplyError {
                apply,
                error_id,
                message,
            }
        });
    }

    fn spd_apply_action_error(
        &mut self,
        payload: Option<SpdApplyAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(payload, rsp_info, |action, error_id, message| {
            TraderSessionEvent::SpdApplyActionError {
                action,
                error_id,
                message,
            }
        });
    }

    fn hedge_cfm_response(
        &mut self,
        payload: Option<InputHedgeCfm>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(payload, rsp_info, |apply, error_id, message| {
            TraderSessionEvent::HedgeCfmResponse {
                request_id,
                apply,
                error_id,
                message,
            }
        });
    }

    fn hedge_cfm_action_response(
        &mut self,
        payload: Option<InputHedgeCfmAction>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        _is_last: bool,
    ) {
        self.respond(payload, rsp_info, |action, error_id, message| {
            TraderSessionEvent::HedgeCfmActionResponse {
                request_id,
                action,
                error_id,
                message,
            }
        });
    }

    fn hedge_cfm_query_response(
        &mut self,
        payload: Option<HedgeCfm>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::HedgeCfm),
            rsp_info,
            request_id,
            is_last,
        );
    }

    fn hedge_cfm_report(&mut self, payload: Option<HedgeCfm>) {
        self.pass(payload, TraderSessionEvent::HedgeCfmReport);
    }

    fn hedge_cfm_error(&mut self, payload: Option<InputHedgeCfm>, rsp_info: Option<RspInfo>) {
        self.respond(payload, rsp_info, |apply, error_id, message| {
            TraderSessionEvent::HedgeCfmError {
                apply,
                error_id,
                message,
            }
        });
    }

    fn hedge_cfm_action_error(
        &mut self,
        payload: Option<HedgeCfmAction>,
        rsp_info: Option<RspInfo>,
    ) {
        self.respond(payload, rsp_info, |action, error_id, message| {
            TraderSessionEvent::HedgeCfmActionError {
                action,
                error_id,
                message,
            }
        });
    }

    fn parked_order_report(&mut self, parked_order: Option<ParkedOrder>) {
        self.pass(parked_order, TraderSessionEvent::ParkedOrderReport);
    }

    fn parked_order_action_report(&mut self, parked_order_action: Option<ParkedOrderAction>) {
        self.pass(
            parked_order_action,
            TraderSessionEvent::ParkedOrderActionReport,
        );
    }

    fn instrument_trade_limit_query_response(
        &mut self,
        payload: Option<InstrTradeLimit>,
        rsp_info: Option<RspInfo>,
        request_id: i32,
        is_last: bool,
    ) {
        self.send_query_row(
            payload.map(Box::new).map(QueryRow::InstrTradeLimit),
            rsp_info,
            request_id,
            is_last,
        );
    }
}
