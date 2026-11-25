use super::enums::{TraderEvent, TraderInput, TraderSessionEvent};
use super::trader_spi::*;
use crate::sys::convert::FromSys;
use crate::{
    BatchOrderAction, ErrorConditionalOrder, InputBatchOrderAction, InputOrder, Order, OrderAction,
    Trade, sys,
};
use std::os::raw::c_char;
use std::sync::mpsc::{Receiver, sync_channel};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

fn harness(capacity: usize) -> (TraderSpiImpl, Receiver<TraderInput>, Arc<AtomicU64>) {
    let (sender, receiver) = sync_channel(capacity);
    let dropped = Arc::new(AtomicU64::new(0));
    (
        TraderSpiImpl::new(sender, dropped.clone()),
        receiver,
        dropped,
    )
}

/// 取下一个透传数据事件；非透传事件（流程控制）直接判失败。
fn next_report(receiver: &Receiver<TraderInput>) -> TraderSessionEvent {
    match receiver.try_recv() {
        Ok(TraderInput::Spi(TraderEvent::Report(event))) => event,
        _ => panic!("期望透传数据事件"),
    }
}

fn info(error_id: i32, message: &str) -> RspInfo {
    RspInfo {
        error_id,
        error_msg: message.to_owned(),
    }
}

fn from_default<T>() -> T
where
    T: FromSys,
    T::Raw: Default,
{
    T::from_sys(&T::Raw::default()).unwrap()
}

fn write_ref<const N: usize>(field: &mut [c_char; N], value: &str) {
    for (slot, byte) in field.iter_mut().zip(value.bytes()) {
        *slot = byte as c_char;
    }
}

fn sample_input_order(reference: &str) -> InputOrder {
    let mut raw = sys::CThostFtdcInputOrderField::default();
    write_ref(&mut raw.OrderRef, reference);
    InputOrder::from_sys(&raw).unwrap()
}

#[test]
fn order_insert_response_carries_echo_payload_and_result() {
    let (mut spi, receiver, _) = harness(8);
    spi.order_insert_response(Some(sample_input_order("7")), Some(info(0, "")), 42, true);
    match next_report(&receiver) {
        TraderSessionEvent::OrderInsertResponse {
            request_id,
            order,
            error_id,
            ..
        } => {
            assert_eq!(request_id, 42);
            assert_eq!(error_id, 0);
            assert_eq!(order.unwrap().order_ref.as_str(), "7");
        }
        event => panic!("期望报单回执事件，实际 {event:?}"),
    }
}

#[test]
fn order_insert_response_rejection_keeps_error() {
    let (mut spi, receiver, _) = harness(8);
    spi.order_insert_response(None, Some(info(26, "不允许重复报单")), 43, true);
    match next_report(&receiver) {
        TraderSessionEvent::OrderInsertResponse {
            request_id,
            order,
            error_id,
            message,
        } => {
            assert_eq!(request_id, 43);
            assert_eq!(error_id, 26);
            assert_eq!(message, "不允许重复报单");
            assert!(order.is_none());
        }
        event => panic!("期望报单回执事件，实际 {event:?}"),
    }
}

#[test]
fn order_insert_error_is_return_path_without_request_id() {
    let (mut spi, receiver, _) = harness(8);
    spi.order_insert_error(Some(sample_input_order("9")), Some(info(50, "报单被拒绝")));
    match next_report(&receiver) {
        TraderSessionEvent::OrderInsertError {
            order,
            error_id,
            message,
        } => {
            assert_eq!(order.unwrap().order_ref.as_str(), "9");
            assert_eq!(error_id, 50);
            assert_eq!(message, "报单被拒绝");
        }
        event => panic!("期望报单错误回报，实际 {event:?}"),
    }
}

#[test]
fn order_action_response_and_error_are_distinct() {
    let (mut spi, receiver, _) = harness(8);
    spi.order_action_response(None, Some(info(0, "")), 44, true);
    match next_report(&receiver) {
        TraderSessionEvent::OrderActionResponse {
            request_id,
            error_id,
            ..
        } => {
            assert_eq!(request_id, 44);
            assert_eq!(error_id, 0);
        }
        event => panic!("期望撤单回执事件，实际 {event:?}"),
    }

    let mut raw = sys::CThostFtdcOrderActionField::default();
    write_ref(&mut raw.OrderRef, "9");
    let action = OrderAction::from_sys(&raw).unwrap();
    spi.order_action_error(Some(action), Some(info(51, "已全成交或已撤销")));
    match next_report(&receiver) {
        TraderSessionEvent::OrderActionError {
            action,
            error_id,
            message,
        } => {
            assert_eq!(action.unwrap().order_ref.as_str(), "9");
            assert_eq!(error_id, 51);
            assert_eq!(message, "已全成交或已撤销");
        }
        event => panic!("期望撤单错误回报，实际 {event:?}"),
    }
}

#[test]
fn batch_order_action_response_and_error_are_distinct() {
    let (mut spi, receiver, _) = harness(8);
    let raw = sys::CThostFtdcInputBatchOrderActionField {
        OrderActionRef: 3,
        ..Default::default()
    };
    let action = InputBatchOrderAction::from_sys(&raw).unwrap();
    spi.batch_order_action_response(Some(action), Some(info(0, "")), 45, true);
    match next_report(&receiver) {
        TraderSessionEvent::BatchOrderActionResponse {
            request_id,
            action,
            error_id,
            ..
        } => {
            assert_eq!(request_id, 45);
            assert_eq!(error_id, 0);
            assert_eq!(action.unwrap().order_action_ref, 3);
        }
        event => panic!("期望批量撤单回执事件，实际 {event:?}"),
    }

    let raw = sys::CThostFtdcBatchOrderActionField::default();
    let action = BatchOrderAction::from_sys(&raw).unwrap();
    spi.batch_order_action_error(Some(action), Some(info(52, "批量撤单被拒绝")));
    match next_report(&receiver) {
        TraderSessionEvent::BatchOrderActionError { error_id, .. } => {
            assert_eq!(error_id, 52);
        }
        event => panic!("期望批量撤单错误回报，实际 {event:?}"),
    }
}

#[test]
fn conditional_order_error_passes_through() {
    let (mut spi, receiver, _) = harness(8);
    let mut raw = sys::CThostFtdcErrorConditionalOrderField::default();
    write_ref(&mut raw.OrderRef, "11");
    raw.ErrorID = 53;
    let order = ErrorConditionalOrder::from_sys(&raw).unwrap();
    spi.conditional_order_error(Some(order));
    match next_report(&receiver) {
        TraderSessionEvent::ConditionalOrderError { order } => {
            let order = order.unwrap();
            assert_eq!(order.order_ref.as_str(), "11");
            assert_eq!(order.error_id, 53);
        }
        event => panic!("期望条件单错误回报，实际 {event:?}"),
    }
}

#[test]
fn order_update_and_execution_pass_payload_and_skip_null() {
    let (mut spi, receiver, _) = harness(8);
    spi.order_update(None);
    assert!(receiver.try_recv().is_err(), "空报单回报不应产生事件");

    let mut raw_order = sys::CThostFtdcOrderField::default();
    write_ref(&mut raw_order.OrderRef, "12");
    spi.order_update(Some(Order::from_sys(&raw_order).unwrap()));
    match next_report(&receiver) {
        TraderSessionEvent::OrderUpdate(order) => {
            assert_eq!(order.order_ref.as_str(), "12");
        }
        event => panic!("期望报单状态回报，实际 {event:?}"),
    }

    spi.execution(None);
    assert!(receiver.try_recv().is_err(), "空成交回报不应产生事件");
    let raw_trade = sys::CThostFtdcTradeField::default();
    spi.execution(Some(Trade::from_sys(&raw_trade).unwrap()));
    match next_report(&receiver) {
        TraderSessionEvent::Execution(_) => {}
        event => panic!("期望成交回报，实际 {event:?}"),
    }
}

#[test]
fn full_queue_drops_report_and_counts() {
    let (mut spi, receiver, dropped) = harness(1);
    spi.order_update(Some(
        Order::from_sys(&sys::CThostFtdcOrderField::default()).unwrap(),
    ));
    // 队列已满（容量 1，无人接收），第二条回报被丢弃并计数
    spi.order_update(Some(
        Order::from_sys(&sys::CThostFtdcOrderField::default()).unwrap(),
    ));
    assert_eq!(dropped.load(Ordering::Acquire), 1);
    drop(receiver);
}

#[test]
fn query_response_row_emits_even_without_payload() {
    let (mut spi, receiver, _) = harness(8);
    // 批次三要点：载荷为空（被过滤）也必须发事件，否则批次无法按 is_last 结束
    spi.order_response(None, Some(info(0, "")), 77, false);
    match receiver.try_recv() {
        Ok(TraderInput::Spi(TraderEvent::QueryRow {
            request_id,
            is_last,
            row,
            ..
        })) => {
            assert_eq!(request_id, 77);
            assert!(!is_last);
            assert!(row.is_none());
        }
        _ => panic!("期望查询行事件"),
    }
}

#[test]
fn query_response_wraps_typed_row() {
    let (mut spi, receiver, _) = harness(8);
    let mut raw = sys::CThostFtdcOrderField::default();
    write_ref(&mut raw.OrderRef, "21");
    spi.order_response(
        Some(Order::from_sys(&raw).unwrap()),
        Some(info(0, "")),
        78,
        true,
    );
    match receiver.try_recv() {
        Ok(TraderInput::Spi(TraderEvent::QueryRow {
            request_id,
            is_last,
            row: Some(QueryRow::Order(order)),
            ..
        })) => {
            assert_eq!(request_id, 78);
            assert!(is_last);
            assert_eq!(order.order_ref.as_str(), "21");
        }
        _ => panic!("期望订单查询行"),
    }
}

#[test]
fn query_response_error_keeps_error_info() {
    let (mut spi, receiver, _) = harness(8);
    spi.trading_account_response(None, Some(info(7, "查询失败")), 79, true);
    match receiver.try_recv() {
        Ok(TraderInput::Spi(TraderEvent::QueryRow {
            request_id,
            is_last,
            error_id,
            message,
            row,
        })) => {
            assert_eq!(request_id, 79);
            assert!(is_last);
            assert_eq!(error_id, 7);
            assert_eq!(message, "查询失败");
            assert!(row.is_none());
        }
        _ => panic!("期望查询错误行"),
    }
}

#[test]
fn instrument_status_passes_through() {
    let (mut spi, receiver, _) = harness(8);
    let mut raw = sys::CThostFtdcInstrumentStatusField::default();
    write_ref(&mut raw.InstrumentID, "ag2612");
    let status = InstrumentStatus::from_sys(&raw).unwrap();
    spi.instrument_status(Some(status));
    match next_report(&receiver) {
        TraderSessionEvent::InstrumentStatus(status) => {
            assert_eq!(status.instrument_id.as_str(), "ag2612");
        }
        event => panic!("期望合约状态推送，实际 {event:?}"),
    }
}

// ---- 批次四：高级指令 ----

#[test]
fn exec_order_insert_response_carries_echo_and_payload() {
    let (mut spi, receiver, _) = harness(8);
    let mut raw = sys::CThostFtdcInputExecOrderField::default();
    write_ref(&mut raw.ExecOrderRef, "31");
    let order = InputExecOrder::from_sys(&raw).unwrap();
    spi.exec_order_insert_response(Some(order), Some(info(0, "")), 55, true);
    match next_report(&receiver) {
        TraderSessionEvent::ExecOrderInsertResponse {
            request_id,
            order,
            error_id,
            ..
        } => {
            assert_eq!(request_id, 55);
            assert_eq!(error_id, 0);
            assert_eq!(order.unwrap().exec_order_ref.as_str(), "31");
        }
        event => panic!("期望行权申报回执，实际 {event:?}"),
    }
}

#[test]
fn option_self_close_action_error_maps_to_event() {
    let (mut spi, receiver, _) = harness(8);
    let mut raw = sys::CThostFtdcOptionSelfCloseActionField::default();
    write_ref(&mut raw.OptionSelfCloseRef, "32");
    let action = OptionSelfCloseAction::from_sys(&raw).unwrap();
    spi.option_self_close_action_error(Some(action), Some(info(60, "撤销被拒绝")));
    match next_report(&receiver) {
        TraderSessionEvent::OptionSelfCloseActionError {
            action,
            error_id,
            message,
        } => {
            assert_eq!(action.unwrap().option_self_close_ref.as_str(), "32");
            assert_eq!(error_id, 60);
            assert_eq!(message, "撤销被拒绝");
        }
        event => panic!("期望自对冲操作错误回报，实际 {event:?}"),
    }
}

#[test]
fn advanced_reports_pass_payload_and_skip_null() {
    let (mut spi, receiver, _) = harness(8);
    spi.exec_order_report(None);
    spi.option_self_close_report(None);
    spi.comb_action_report(None);
    spi.quote_report(None);
    spi.for_quote_response(None);
    spi.parked_order_report(None);
    spi.parked_order_action_report(None);
    assert!(receiver.try_recv().is_err(), "空回报不应产生事件");

    let mut raw = sys::CThostFtdcCombActionField::default();
    write_ref(&mut raw.CombActionRef, "33");
    spi.comb_action_report(Some(CombAction::from_sys(&raw).unwrap()));
    match next_report(&receiver) {
        TraderSessionEvent::CombActionReport(action) => {
            assert_eq!(action.comb_action_ref.as_str(), "33");
        }
        event => panic!("期望组合回报，实际 {event:?}"),
    }

    let mut raw = sys::CThostFtdcForQuoteRspField::default();
    write_ref(&mut raw.InstrumentID, "ag2612");
    spi.for_quote_response(Some(ForQuoteRsp::from_sys(&raw).unwrap()));
    match next_report(&receiver) {
        TraderSessionEvent::ForQuoteReport(rsp) => {
            assert_eq!(rsp.instrument_id.as_str(), "ag2612");
        }
        event => panic!("期望询价回报，实际 {event:?}"),
    }
}

#[test]
fn parked_family_responses_carry_echo() {
    let (mut spi, receiver, _) = harness(8);
    spi.parked_order_insert_response(None, Some(info(0, "")), 61, true);
    match next_report(&receiver) {
        TraderSessionEvent::ParkedOrderInsertResponse {
            request_id,
            error_id,
            order,
            ..
        } => {
            assert_eq!(request_id, 61);
            assert_eq!(error_id, 0);
            assert!(order.is_none());
        }
        event => panic!("期望预埋单回执，实际 {event:?}"),
    }
    spi.remove_parked_order_action_response(None, Some(info(62, "删除失败")), 62, true);
    match next_report(&receiver) {
        TraderSessionEvent::RemoveParkedOrderActionResponse {
            request_id,
            error_id,
            message,
            ..
        } => {
            assert_eq!(request_id, 62);
            assert_eq!(error_id, 62);
            assert_eq!(message, "删除失败");
        }
        event => panic!("期望删除预埋操作回执，实际 {event:?}"),
    }
}

#[test]
fn advanced_query_rows_are_typed() {
    let (mut spi, receiver, _) = harness(8);
    spi.exec_order_response(None, Some(info(0, "")), 70, true);
    spi.parked_order_actions(None, Some(info(0, "")), 71, true);
    spi.for_quote_query_response(None, Some(info(0, "")), 72, true);
    for expected_id in [70, 71, 72] {
        match receiver.try_recv() {
            Ok(TraderInput::Spi(TraderEvent::QueryRow {
                request_id,
                is_last: true,
                row: None,
                ..
            })) if request_id == expected_id => {}
            _ => panic!("期望 request_id={expected_id} 的空载荷查询末行"),
        }
    }
}

// ---- 批次五：银期转账 ----

#[test]
fn bank_to_future_response_carries_echo_and_payload() {
    let (mut spi, receiver, _) = harness(8);
    let raw = sys::CThostFtdcReqTransferField {
        TradeAmount: 1000.0,
        ..Default::default()
    };
    let transfer = ReqTransfer::from_sys(&raw).unwrap();
    spi.from_bank_to_future_by_future_response(Some(transfer), Some(info(0, "")), 81, true);
    match next_report(&receiver) {
        TraderSessionEvent::BankToFutureResponse {
            request_id,
            transfer,
            error_id,
            ..
        } => {
            assert_eq!(request_id, 81);
            assert_eq!(error_id, 0);
            assert_eq!(transfer.unwrap().trade_amount, 1000.0);
        }
        event => panic!("期望入金应答，实际 {event:?}"),
    }
}

#[test]
fn repeal_error_maps_to_event() {
    let (mut spi, receiver, _) = harness(8);
    let raw = sys::CThostFtdcReqRepealField::default();
    let repeal = ReqRepeal::from_sys(&raw).unwrap();
    spi.repeal_bank_to_future_by_future_manual_error(Some(repeal), Some(info(70, "冲正被拒绝")));
    match next_report(&receiver) {
        TraderSessionEvent::RepealBankToFutureError {
            repeal,
            error_id,
            message,
        } => {
            assert!(repeal.is_some());
            assert_eq!(error_id, 70);
            assert_eq!(message, "冲正被拒绝");
        }
        event => panic!("期望冲正错误回报，实际 {event:?}"),
    }
}

#[test]
fn transfer_reports_pass_payload_and_skip_null() {
    let (mut spi, receiver, _) = harness(8);
    spi.from_bank_to_future_by_future_report(None);
    spi.repeal_from_bank_to_future_by_bank_report(None);
    spi.open_account_by_bank_report(None);
    assert!(receiver.try_recv().is_err(), "空回报不应产生事件");

    let raw = sys::CThostFtdcRspTransferField {
        TradeAmount: 500.0,
        ..Default::default()
    };
    spi.from_bank_to_future_by_future_report(Some(RspTransfer::from_sys(&raw).unwrap()));
    match next_report(&receiver) {
        TraderSessionEvent::BankToFutureByFutureReport(transfer) => {
            assert_eq!(transfer.trade_amount, 500.0);
        }
        event => panic!("期望期商入金处理回报，实际 {event:?}"),
    }

    let raw = sys::CThostFtdcNotifyQueryAccountField {
        BankUseAmount: 8888.0,
        ..Default::default()
    };
    spi.query_bank_balance_by_future_report(Some(NotifyQueryAccount::from_sys(&raw).unwrap()));
    match next_report(&receiver) {
        TraderSessionEvent::QueryBankBalanceReport(notify) => {
            assert_eq!(notify.bank_use_amount, 8888.0);
        }
        event => panic!("期望银行余额回报，实际 {event:?}"),
    }
}

#[test]
fn transfer_query_rows_are_typed() {
    let (mut spi, receiver, _) = harness(8);
    spi.transfer_bank_response(None, Some(info(0, "")), 90, true);
    spi.transfer_serial_response(None, Some(info(0, "")), 91, true);
    spi.account_register_response(None, Some(info(0, "")), 92, true);
    spi.contract_bank_response(None, Some(info(0, "")), 93, true);
    for expected_id in [90, 91, 92, 93] {
        match receiver.try_recv() {
            Ok(TraderInput::Spi(TraderEvent::QueryRow {
                request_id,
                is_last: true,
                row: None,
                ..
            })) if request_id == expected_id => {}
            _ => panic!("期望 request_id={expected_id} 的空载荷查询末行"),
        }
    }
}

#[test]
fn batch6_query_rows_are_typed() {
    let (mut spi, receiver, _) = harness(1);
    macro_rules! assert_query_row {
        ($callback:ident, $variant:ident, $request_id:expr) => {{
            spi.$callback(Some(from_default()), Some(info(0, "")), $request_id, true);
            match receiver.try_recv() {
                Ok(TraderInput::Spi(TraderEvent::QueryRow {
                    request_id,
                    is_last: true,
                    row: Some(QueryRow::$variant(_)),
                    ..
                })) => assert_eq!(request_id, $request_id),
                _ => panic!(
                    "期望 request_id={} 的 {} 查询行",
                    $request_id,
                    stringify!($variant)
                ),
            }
        }};
    }
    assert_query_row!(max_order_volume_response, MaxOrderVolume, 101);
    assert_query_row!(instrument_margin_rate_response, InstrumentMarginRate, 102);
    assert_query_row!(
        instrument_commission_rate_response,
        InstrumentCommissionRate,
        103
    );
    assert_query_row!(
        mm_instrument_commission_rate_response,
        MMInstrumentCommissionRate,
        104
    );
    assert_query_row!(
        mm_option_instrument_commission_rate_response,
        MMOptionInstrCommRate,
        105
    );
    assert_query_row!(
        instrument_order_commission_rate_response,
        InstrumentOrderCommRate,
        106
    );
    assert_query_row!(
        option_instrument_trade_cost_response,
        OptionInstrTradeCost,
        107
    );
    assert_query_row!(
        option_instrument_commission_rate_response,
        OptionInstrCommRate,
        108
    );
    assert_query_row!(exchange_margin_rate_response, ExchangeMarginRate, 109);
    assert_query_row!(
        exchange_margin_rate_adjust_response,
        ExchangeMarginRateAdjust,
        110
    );
    assert_query_row!(exchange_rate_response, ExchangeRate, 111);
    assert_query_row!(product_exchange_rate_response, ProductExchRate, 112);
    assert_query_row!(product_group_response, ProductGroup, 113);
    assert_query_row!(
        investor_product_group_margin_response,
        InvestorProductGroupMargin,
        114
    );
    assert_query_row!(comb_instrument_guard_response, CombInstrumentGuard, 115);
    assert_query_row!(
        max_order_volume_with_price_query_response,
        MaxOrderVolumeWithPrice,
        116
    );
    assert_query_row!(spbm_future_parameter_response, SPBMFutureParameter, 117);
    assert_query_row!(spbm_option_parameter_response, SPBMOptionParameter, 118);
    assert_query_row!(spbm_intra_parameter_response, SPBMIntraParameter, 119);
    assert_query_row!(spbm_inter_parameter_response, SPBMInterParameter, 120);
    assert_query_row!(spbm_portfolio_definition_response, SPBMPortfDefinition, 121);
    assert_query_row!(
        spbm_investor_portfolio_definition_response,
        SPBMInvestorPortfDef,
        122
    );
    assert_query_row!(
        spbm_add_on_inter_parameter_response,
        SPBMAddOnInterParameter,
        123
    );
    assert_query_row!(
        investor_portfolio_margin_ratio_response,
        InvestorPortfMarginRatio,
        124
    );
    assert_query_row!(
        investor_product_spbm_detail_response,
        InvestorProdSPBMDetail,
        125
    );
    assert_query_row!(spmm_instrument_parameter_response, SPMMInstParam, 126);
    assert_query_row!(spmm_product_parameter_response, SPMMProductParam, 127);
    assert_query_row!(
        investor_commodity_spmm_margin_response,
        InvestorCommoditySPMMMargin,
        128
    );
    assert_query_row!(
        investor_commodity_group_spmm_margin_response,
        InvestorCommodityGroupSPMMMargin,
        129
    );
    assert_query_row!(rcams_comb_product_info_response, RCAMSCombProductInfo, 130);
    assert_query_row!(
        rcams_instrument_parameter_response,
        RCAMSInstrParameter,
        131
    );
    assert_query_row!(rcams_intra_parameter_response, RCAMSIntraParameter, 132);
    assert_query_row!(rcams_inter_parameter_response, RCAMSInterParameter, 133);
    assert_query_row!(
        rcams_short_opt_adjust_parameter_response,
        RCAMSShortOptAdjustParam,
        134
    );
    assert_query_row!(
        rcams_investor_comb_position_response,
        RCAMSInvestorCombPosition,
        135
    );
    assert_query_row!(
        investor_product_rcams_margin_response,
        InvestorProdRCAMSMargin,
        136
    );
    assert_query_row!(rule_instrument_parameter_response, RULEInstrParameter, 137);
    assert_query_row!(rule_intra_parameter_response, RULEIntraParameter, 138);
    assert_query_row!(rule_inter_parameter_response, RULEInterParameter, 139);
    assert_query_row!(
        investor_product_rule_margin_response,
        InvestorProdRULEMargin,
        140
    );
    assert_query_row!(
        investor_portfolio_setting_response,
        InvestorPortfSetting,
        141
    );
    assert_query_row!(
        risk_settle_investor_position_response,
        RiskSettleInvestPosition,
        142
    );
    assert_query_row!(
        risk_settle_product_status_response,
        RiskSettleProductStatus,
        143
    );
    assert_query_row!(
        investor_info_commission_record_response,
        InvestorInfoCommRec,
        144
    );
}

#[test]
fn batch7_events_and_query_rows_are_typed() {
    let (mut spi, receiver, _) = harness(1);
    macro_rules! assert_response {
        ($callback:ident, $variant:ident, $request_id:expr) => {{
            spi.$callback(Some(from_default()), Some(info(0, "")), $request_id, true);
            match next_report(&receiver) {
                TraderSessionEvent::$variant {
                    request_id,
                    error_id,
                    ..
                } => {
                    assert_eq!(request_id, $request_id);
                    assert_eq!(error_id, 0);
                }
                event => panic!("期望 {} 回执，实际 {event:?}", stringify!($variant)),
            }
        }};
    }
    macro_rules! assert_error {
        ($callback:ident, $variant:ident) => {{
            spi.$callback(Some(from_default()), Some(info(70, "失败")));
            match next_report(&receiver) {
                TraderSessionEvent::$variant {
                    error_id, message, ..
                } => {
                    assert_eq!(error_id, 70);
                    assert_eq!(message, "失败");
                }
                event => panic!("期望 {} 错误，实际 {event:?}", stringify!($variant)),
            }
        }};
    }
    macro_rules! assert_report {
        ($callback:ident, $variant:ident) => {{
            spi.$callback(Some(from_default()));
            assert!(matches!(
                next_report(&receiver),
                TraderSessionEvent::$variant(_)
            ));
        }};
    }
    macro_rules! assert_query {
        ($callback:ident, $variant:ident, $request_id:expr) => {{
            spi.$callback(Some(from_default()), Some(info(0, "")), $request_id, true);
            match receiver.try_recv() {
                Ok(TraderInput::Spi(TraderEvent::QueryRow {
                    request_id,
                    is_last: true,
                    row: Some(QueryRow::$variant(_)),
                    ..
                })) => assert_eq!(request_id, $request_id),
                _ => panic!("期望 {} 查询行", stringify!($variant)),
            }
        }};
    }
    assert_response!(set_offset_response, SetOffsetResponse, 201);
    assert_response!(
        cancel_offset_setting_response,
        CancelOffsetSettingResponse,
        202
    );
    assert_report!(offset_setting_report, OffsetSettingReport);
    assert_error!(offset_setting_error, OffsetSettingError);
    assert_error!(cancel_offset_setting_error, CancelOffsetSettingError);
    assert_query!(offset_settings_response, OffsetSetting, 203);

    assert_response!(spd_apply_response, SpdApplyResponse, 204);
    assert_response!(spd_apply_action_response, SpdApplyActionResponse, 205);
    assert_query!(spd_apply_query_response, SpdApply, 206);
    assert_report!(spd_apply_report, SpdApplyReport);
    assert_error!(spd_apply_error, SpdApplyError);
    assert_error!(spd_apply_action_error, SpdApplyActionError);

    assert_response!(hedge_cfm_response, HedgeCfmResponse, 207);
    assert_response!(hedge_cfm_action_response, HedgeCfmActionResponse, 208);
    assert_query!(hedge_cfm_query_response, HedgeCfm, 209);
    assert_report!(hedge_cfm_report, HedgeCfmReport);
    assert_error!(hedge_cfm_error, HedgeCfmError);
    assert_error!(hedge_cfm_action_error, HedgeCfmActionError);

    assert_query!(instrument_trade_limit_query_response, InstrTradeLimit, 210);
    assert_query!(classified_instrument_response, ClassifiedInstrument, 211);
    assert_query!(comb_promotion_parameter_response, CombPromotionParam, 212);
}
