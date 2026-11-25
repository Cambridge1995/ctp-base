use super::enums::{LOGIN_TIMEOUT, TraderCommand, TraderInput};
use super::trader::*;
use crate::sys::convert::FromSys;
use crate::{
    InputBatchOrderAction, InputOrder, InputOrderAction, Order, QryOrder, QryTradingAccount,
    ReqQueryAccount, ReqTransfer, sys,
};
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

fn config() -> TraderConfig {
    TraderConfig {
        broker_id: "9999".to_owned(),
        user_id: "1001".to_owned(),
        ..Default::default()
    }
}

fn blank_order() -> InputOrder {
    InputOrder::from_sys(&sys::CThostFtdcInputOrderField::default()).unwrap()
}

fn from_default<T>() -> T
where
    T: FromSys,
    T::Raw: Default,
{
    T::from_sys(&T::Raw::default()).unwrap()
}

trait Sample {
    fn sample() -> Self;
}

fn sample<T: Sample>() -> T {
    T::sample()
}

macro_rules! impl_sample {
        ($type:ident { $($field:ident: $value:expr),* $(,)? }) => {
            impl Sample for $type {
                fn sample() -> Self {
                    Self { $($field: $value),* }
                }
            }
        };
    }

impl_sample!(QryMaxOrderVolume {
    broker_id: Default::default(),
    investor_id: Default::default(),
    direction: crate::Direction::Buy,
    offset_flag: crate::OffsetFlag::Open,
    hedge_flag: crate::HedgeFlag::Speculation,
    max_volume: Default::default(),
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryInstrumentMarginRate {
    broker_id: Default::default(),
    investor_id: Default::default(),
    hedge_flag: crate::HedgeFlag::Speculation,
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryInstrumentCommissionRate {
    broker_id: Default::default(),
    investor_id: Default::default(),
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryMMInstrumentCommissionRate {
    broker_id: Default::default(),
    investor_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryMMOptionInstrCommRate {
    broker_id: Default::default(),
    investor_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryInstrumentOrderCommRate {
    broker_id: Default::default(),
    investor_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryOptionInstrTradeCost {
    broker_id: Default::default(),
    investor_id: Default::default(),
    hedge_flag: crate::HedgeFlag::Speculation,
    input_price: Default::default(),
    underlying_price: Default::default(),
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryOptionInstrCommRate {
    broker_id: Default::default(),
    investor_id: Default::default(),
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryExchangeMarginRate {
    broker_id: Default::default(),
    hedge_flag: crate::HedgeFlag::Speculation,
    exchange_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryExchangeMarginRateAdjust {
    broker_id: Default::default(),
    hedge_flag: crate::HedgeFlag::Speculation,
    instrument_id: Default::default(),
});
impl_sample!(QryExchangeRate {
    broker_id: Default::default(),
    from_currency_id: Default::default(),
    to_currency_id: Default::default(),
});
impl_sample!(QryProductExchRate {
    exchange_id: Default::default(),
    product_id: Default::default(),
});
impl_sample!(QryProductGroup {
    exchange_id: Default::default(),
    product_id: Default::default(),
});
impl_sample!(QryInvestorProductGroupMargin {
    broker_id: Default::default(),
    investor_id: Default::default(),
    hedge_flag: crate::HedgeFlag::Speculation,
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    product_group_id: Default::default(),
});
impl_sample!(QryCombInstrumentGuard {
    broker_id: Default::default(),
    exchange_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryMaxOrderVolumeWithPrice {
    broker_id: Default::default(),
    investor_id: Default::default(),
    direction: crate::Direction::Buy,
    offset_flag: crate::OffsetFlag::Open,
    hedge_flag: crate::HedgeFlag::Speculation,
    max_volume: Default::default(),
    price: Default::default(),
    exchange_id: Default::default(),
    invest_unit_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QrySPBMFutureParameter {
    exchange_id: Default::default(),
    instrument_id: Default::default(),
    product_family_code: Default::default(),
});
impl_sample!(QrySPBMOptionParameter {
    exchange_id: Default::default(),
    instrument_id: Default::default(),
    product_family_code: Default::default(),
});
impl_sample!(QrySPBMIntraParameter {
    exchange_id: Default::default(),
    product_family_code: Default::default(),
});
impl_sample!(QrySPBMInterParameter {
    exchange_id: Default::default(),
    leg1_product_family_code: Default::default(),
    leg2_product_family_code: Default::default(),
});
impl_sample!(QrySPBMPortfDefinition {
    exchange_id: Default::default(),
    portfolio_definition_id: Default::default(),
    product_family_code: Default::default(),
});
impl_sample!(QrySPBMInvestorPortfDef {
    exchange_id: Default::default(),
    broker_id: Default::default(),
    investor_id: Default::default(),
});
impl_sample!(QrySPBMAddOnInterParameter {
    exchange_id: Default::default(),
    leg1_product_family_code: Default::default(),
    leg2_product_family_code: Default::default(),
});
impl_sample!(QryInvestorPortfMarginRatio {
    broker_id: Default::default(),
    investor_id: Default::default(),
    exchange_id: Default::default(),
    product_group_id: Default::default(),
});
impl_sample!(QryInvestorProdSPBMDetail {
    exchange_id: Default::default(),
    broker_id: Default::default(),
    investor_id: Default::default(),
    product_family_code: Default::default(),
});
impl_sample!(QrySPMMInstParam {
    instrument_id: Default::default(),
});
impl_sample!(QrySPMMProductParam {
    product_id: Default::default(),
});
impl_sample!(QryInvestorCommoditySPMMMargin {
    broker_id: Default::default(),
    investor_id: Default::default(),
    commodity_id: Default::default(),
});
impl_sample!(QryInvestorCommodityGroupSPMMMargin {
    broker_id: Default::default(),
    investor_id: Default::default(),
    commodity_group_id: Default::default(),
});
impl_sample!(QryRCAMSCombProductInfo {
    product_id: Default::default(),
    comb_product_id: Default::default(),
    product_group_id: Default::default(),
});
impl_sample!(QryRCAMSInstrParameter {
    product_id: Default::default(),
});
impl_sample!(QryRCAMSIntraParameter {
    comb_product_id: Default::default(),
});
impl_sample!(QryRCAMSInterParameter {
    product_group_id: Default::default(),
    comb_product1: Default::default(),
    comb_product2: Default::default(),
});
impl_sample!(QryRCAMSShortOptAdjustParam {
    comb_product_id: Default::default(),
});
impl_sample!(QryRCAMSInvestorCombPosition {
    broker_id: Default::default(),
    investor_id: Default::default(),
    instrument_id: Default::default(),
    comb_instrument_id: Default::default(),
});
impl_sample!(QryInvestorProdRCAMSMargin {
    broker_id: Default::default(),
    investor_id: Default::default(),
    comb_product_id: Default::default(),
    product_group_id: Default::default(),
});
impl_sample!(QryRULEInstrParameter {
    exchange_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryRULEIntraParameter {
    exchange_id: Default::default(),
    product_family_code: Default::default(),
});
impl_sample!(QryRULEInterParameter {
    exchange_id: Default::default(),
    leg1_product_family_code: Default::default(),
    leg2_product_family_code: Default::default(),
    commodity_group_id: Default::default(),
});
impl_sample!(QryInvestorProdRULEMargin {
    exchange_id: Default::default(),
    broker_id: Default::default(),
    investor_id: Default::default(),
    product_family_code: Default::default(),
    commodity_group_id: Default::default(),
});
impl_sample!(QryInvestorPortfSetting {
    exchange_id: Default::default(),
    broker_id: Default::default(),
    investor_id: Default::default(),
});
impl_sample!(QryRiskSettleInvestPosition {
    broker_id: Default::default(),
    investor_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryRiskSettleProductStatus {
    product_id: Default::default(),
});
impl_sample!(QryInvestorInfoCommRec {
    investor_id: Default::default(),
    instrument_id: Default::default(),
    broker_id: Default::default(),
});
impl_sample!(QryOffsetSetting {
    broker_id: Default::default(),
    investor_id: Default::default(),
    product_id: Default::default(),
    offset_type: crate::OffsetType::OptionOffset,
});
impl_sample!(QrySpdApply {
    broker_id: Default::default(),
    investor_id: Default::default(),
    exchange_id: Default::default(),
    order_system_id: Default::default(),
    first_leg_instrument_id: Default::default(),
    second_leg_instrument_id: Default::default(),
});
impl_sample!(QryHedgeCfm {
    broker_id: Default::default(),
    investor_id: Default::default(),
    exchange_id: Default::default(),
    order_system_id: Default::default(),
    instrument_id: Default::default(),
});
impl_sample!(QryInstrTradeLimit {
    instrument_id: Default::default(),
    exchange_id: Default::default(),
});
impl_sample!(QryClassifiedInstrument {
    instrument_id: Default::default(),
    exchange_id: Default::default(),
    exchange_instrument_id: Default::default(),
    product_id: Default::default(),
    trading_type: crate::TradingType::TRADE,
    class_type: crate::ClassType::COMB,
});
impl_sample!(QryCombPromotionParam {
    exchange_id: Default::default(),
    instrument_id: Default::default(),
});

#[test]
fn insert_order_assigns_increasing_refs_and_identity() {
    let config = config();
    let mut next_ref = 1;
    let mut order = blank_order();
    let first = prepare_insert_order(&config, &mut next_ref, &mut order).unwrap();
    assert_eq!(first, "1");
    assert_eq!(order.order_ref.as_str(), "1");
    assert_eq!(order.broker_id.as_str(), "9999");
    assert_eq!(order.investor_id.as_str(), "1001");
    assert_eq!(order.user_id.as_str(), "1001");

    let second = prepare_insert_order(&config, &mut next_ref, &mut blank_order()).unwrap();
    assert_eq!(second, "2", "OrderRef 必须严格递增（CTP 拒单 ErrorID 22）");
}

#[test]
fn cancel_order_fills_session_and_increasing_action_ref() {
    let config = config();
    let mut next_ref = 1;
    let mut action =
        InputOrderAction::from_sys(&sys::CThostFtdcInputOrderActionField::default()).unwrap();
    let assigned = prepare_cancel_order(&config, (7, 9), &mut next_ref, &mut action);
    assert_eq!(assigned, 1);
    assert_eq!(action.order_action_ref, 1);
    assert_eq!(action.front_id.get(), 7);
    assert_eq!(action.session_id.get(), 9);
    assert_eq!(action.broker_id.as_str(), "9999");
    assert_eq!(action.investor_id.as_str(), "1001");

    let mut second =
        InputOrderAction::from_sys(&sys::CThostFtdcInputOrderActionField::default()).unwrap();
    assert_eq!(
        prepare_cancel_order(&config, (7, 9), &mut next_ref, &mut second),
        2
    );
}

#[test]
fn batch_action_shares_action_ref_sequence() {
    let config = config();
    let mut next_ref = 5;
    let mut action =
        InputBatchOrderAction::from_sys(&sys::CThostFtdcInputBatchOrderActionField::default())
            .unwrap();
    let assigned = prepare_batch_action(&config, (7, 9), &mut next_ref, &mut action);
    assert_eq!(
        assigned, 5,
        "批量撤单与单笔撤单共用同一 OrderActionRef 递增序列"
    );
    assert_eq!(action.front_id.get(), 7);
    assert_eq!(action.session_id.get(), 9);
}

/// 连不上任何前置时：LOGIN_TIMEOUT 后返回 Timeout，且 loop 线程已被 Shutdown+join 回收
/// （函数能干净返回即证明线程没有泄漏悬挂）。
#[test]
#[ignore = "真实等待 15 秒超时，需手动 cargo test -- --ignored 运行"]
fn unreachable_front_times_out_and_reclaims_worker() {
    let config = TraderConfig {
        fronts: vec!["tcp://127.0.0.1:1".to_owned()],
        ..Default::default()
    };
    let start = std::time::Instant::now();
    let result = TraderSession::new(&config);
    assert!(matches!(result, Err(TraderSessionError::Timeout)));
    assert!(start.elapsed() >= LOGIN_TIMEOUT, "必须等到就绪超时才返回");
}

use crate::session::enums::{QueryKind, QueryRow, TraderSessionEvent};

fn kind_order() -> QueryKind {
    QueryKind::Order(QryOrder {
        broker_id: Default::default(),
        investor_id: Default::default(),
        exchange_id: Default::default(),
        order_system_id: Default::default(),
        insert_time_start: Default::default(),
        insert_time_end: Default::default(),
        invest_unit_id: Default::default(),
        instrument_id: Default::default(),
    })
}

fn sample_order() -> Order {
    Order::from_sys(&sys::CThostFtdcOrderField::default()).unwrap()
}

#[test]
fn query_row_is_exposed_immediately_without_waiting_for_last() {
    let event = into_session_query_row(
        7,
        false,
        0,
        String::new(),
        Some(QueryRow::Order(Box::new(sample_order()))),
    );
    match event {
        TraderSessionEvent::QueryRow {
            request_id,
            is_last,
            row: Some(QueryRow::Order(_)),
            ..
        } => {
            assert_eq!(request_id, 7);
            assert!(!is_last, "中间行必须立即公开，不能等 is_last");
        }
        other => panic!("期望公开 QueryRow：{other:?}"),
    }
}

#[test]
fn query_error_preserves_request_id_for_caller_cleanup() {
    let event = into_session_query_error(7, -3, "查询流控".to_owned());
    match event {
        TraderSessionEvent::QueryError {
            request_id,
            error_id,
            message,
        } => {
            assert_eq!(request_id, 7);
            assert_eq!(error_id, -3);
            assert_eq!(message, "查询流控");
        }
        other => panic!("期望 QueryError：{other:?}"),
    }
}

#[test]
fn empty_last_query_row_is_still_exposed() {
    let event = into_session_query_row(8, true, 0, String::new(), None);
    match event {
        TraderSessionEvent::QueryRow {
            request_id,
            is_last,
            row: None,
            ..
        } => {
            assert_eq!(request_id, 8);
            assert!(is_last);
        }
        other => panic!("空载荷末行也必须公开：{other:?}"),
    }
}

#[test]
fn query_identity_is_filled_from_config() {
    let config = config();
    let mut kind = kind_order();
    fill_query_identity(&config, &mut kind);
    match &kind {
        QueryKind::Order(request) => {
            assert_eq!(request.broker_id.as_str(), "9999");
            assert_eq!(request.investor_id.as_str(), "1001");
        }
        _ => panic!("期望报单查询"),
    }
    let mut account = QueryKind::TradingAccount(QryTradingAccount {
        broker_id: Default::default(),
        investor_id: Default::default(),
        currency_id: String::new(),
        biz_type: crate::BizType::Future,
        account_id: Default::default(),
    });
    fill_query_identity(&config, &mut account);
    match &account {
        QueryKind::TradingAccount(request) => {
            assert_eq!(request.broker_id.as_str(), "9999");
            assert_eq!(request.investor_id.as_str(), "1001");
            assert_eq!(request.account_id.as_str(), "1001");
            assert!(request.currency_id.is_empty(), "币种不由会话代填");
        }
        _ => panic!("期望资金查询"),
    }
}

/// 批次五：转账请求的业务功能码与身份填充。
#[test]
fn transfer_prepare_fills_trade_code_and_identity() {
    let config = config();
    let mut transfer = ReqTransfer::from_sys(&sys::CThostFtdcReqTransferField::default()).unwrap();
    transfer.password = "资金密码".to_owned();
    prepare_transfer(&config, TransferDirection::ToFuture, &mut transfer).unwrap();
    assert_eq!(transfer.trade_code, "202001", "入金业务功能码");
    assert_eq!(transfer.broker_id.as_str(), "9999");
    assert_eq!(transfer.account_id.as_str(), "1001");
    assert_eq!(transfer.user_id.as_str(), "1001");
    assert_eq!(transfer.password, "资金密码", "资金密码不被覆盖");

    prepare_transfer(&config, TransferDirection::ToBank, &mut transfer).unwrap();
    assert_eq!(transfer.trade_code, "202002", "出金业务功能码");

    let mut query =
        ReqQueryAccount::from_sys(&sys::CThostFtdcReqQueryAccountField::default()).unwrap();
    query.password = "资金密码".to_owned();
    prepare_query_account(&config, &mut query).unwrap();
    assert_eq!(query.broker_id.as_str(), "9999");
    assert_eq!(query.account_id.as_str(), "1001");
}

/// 资金密码必须手动传入：空密码在本地直接拒绝，不流向柜台。
#[test]
fn transfer_prepare_rejects_empty_fund_password() {
    let config = config();
    let mut transfer = ReqTransfer::from_sys(&sys::CThostFtdcReqTransferField::default()).unwrap();
    assert!(
        prepare_transfer(&config, TransferDirection::ToFuture, &mut transfer).is_err(),
        "转账缺资金密码必须本地拒绝"
    );
    let mut query =
        ReqQueryAccount::from_sys(&sys::CThostFtdcReqQueryAccountField::default()).unwrap();
    assert!(
        prepare_query_account(&config, &mut query).is_err(),
        "余额查询缺资金密码必须本地拒绝"
    );
}

#[test]
fn batch6_query_identity_is_filled_from_config() {
    macro_rules! assert_both {
        ($variant:ident) => {{
            let mut kind = QueryKind::$variant(sample());
            fill_query_identity(&config(), &mut kind);
            match kind {
                QueryKind::$variant(request) => {
                    assert_eq!(request.broker_id.as_str(), "9999");
                    assert_eq!(request.investor_id.as_str(), "1001");
                }
                _ => unreachable!(),
            }
        }};
    }
    macro_rules! assert_broker {
        ($variant:ident) => {{
            let mut kind = QueryKind::$variant(sample());
            fill_query_identity(&config(), &mut kind);
            match kind {
                QueryKind::$variant(request) => assert_eq!(request.broker_id.as_str(), "9999"),
                _ => unreachable!(),
            }
        }};
    }
    assert_both!(MaxOrderVolume);
    assert_both!(InstrumentMarginRate);
    assert_both!(InstrumentCommissionRate);
    assert_both!(MMInstrumentCommissionRate);
    assert_both!(MMOptionInstrCommRate);
    assert_both!(InstrumentOrderCommRate);
    assert_both!(OptionInstrTradeCost);
    assert_both!(OptionInstrCommRate);
    assert_broker!(ExchangeMarginRate);
    assert_broker!(ExchangeMarginRateAdjust);
    assert_broker!(ExchangeRate);
    assert_both!(InvestorProductGroupMargin);
    assert_broker!(CombInstrumentGuard);
    assert_both!(MaxOrderVolumeWithPrice);
    assert_both!(SPBMInvestorPortfDef);
    assert_both!(InvestorPortfMarginRatio);
    assert_both!(InvestorProdSPBMDetail);
    assert_both!(InvestorCommoditySPMMMargin);
    assert_both!(InvestorCommodityGroupSPMMMargin);
    assert_both!(RCAMSInvestorCombPosition);
    assert_both!(InvestorProdRCAMSMargin);
    assert_both!(InvestorProdRULEMargin);
    assert_both!(InvestorPortfSetting);
    assert_both!(RiskSettleInvestPosition);
    assert_both!(InvestorInfoCommRec);
}

fn query_session() -> (TraderSession, Receiver<TraderInput>) {
    let (inputs, input_receiver) = std::sync::mpsc::sync_channel(1);
    let (_event_sender, events) = std::sync::mpsc::sync_channel(1);
    let session = TraderSession {
        inputs,
        events,
        dropped: Arc::new(AtomicU64::new(0)),
        identity: Arc::new(Mutex::new(Identity {
            front_id: 1,
            session_id: 2,
            trading_day: "20261124".to_owned(),
        })),
        worker: Mutex::new(None),
    };
    (session, input_receiver)
}

#[test]
fn call_returns_command_result() {
    let (session, inputs) = query_session();
    let responder = std::thread::spawn(move || match inputs.recv().unwrap() {
        TraderInput::Command(TraderCommand::Query { reply, .. }) => {
            reply.send(Ok(73)).unwrap();
        }
        _ => panic!("期望查询命令"),
    });
    let result = session.call(|reply| TraderCommand::Query {
        kind: Box::new(kind_order()),
        reply,
    });
    assert_eq!(result, Ok(73));
    responder.join().unwrap();
}

#[test]
fn send_checked_allocates_request_id() {
    let mut next_request_id = 7;
    let result = TraderLoop::send_checked(&mut next_request_id, "报单", |request_id| {
        assert_eq!(request_id, 7);
        Ok(0)
    });
    assert_eq!(result, Ok(7));
    assert_eq!(next_request_id, 8);
}

#[test]
fn send_checked_rejects_nonzero_code() {
    let mut next_request_id = 9;
    let result = TraderLoop::send_checked(&mut next_request_id, "撤单", |_| Ok(-3));
    assert!(matches!(
        result,
        Err(TraderSessionError::Request(message)) if message == "撤单发送失败，CTP 返回码 -3"
    ));
    assert_eq!(next_request_id, 10);
}

#[test]
fn batch6_public_queries_map_to_their_request_kind() {
    macro_rules! assert_session_query {
        ($method:ident, $variant:ident, $request_id:expr) => {{
            let (session, inputs) = query_session();
            let responder = std::thread::spawn(move || match inputs.recv().unwrap() {
                TraderInput::Command(TraderCommand::Query { kind, reply }) => {
                    assert!(matches!(*kind, QueryKind::$variant(_)));
                    reply.send(Ok($request_id)).unwrap();
                }
                _ => panic!("期望查询命令"),
            });
            assert_eq!(session.$method(sample()).unwrap(), $request_id);
            responder.join().unwrap();
        }};
    }
    assert_session_query!(query_max_order_volume, MaxOrderVolume, 101);
    assert_session_query!(query_instrument_margin_rate, InstrumentMarginRate, 102);
    assert_session_query!(
        query_instrument_commission_rate,
        InstrumentCommissionRate,
        103
    );
    assert_session_query!(
        query_mm_instrument_commission_rate,
        MMInstrumentCommissionRate,
        104
    );
    assert_session_query!(query_mm_option_instr_comm_rate, MMOptionInstrCommRate, 105);
    assert_session_query!(
        query_instrument_order_comm_rate,
        InstrumentOrderCommRate,
        106
    );
    assert_session_query!(query_option_instr_trade_cost, OptionInstrTradeCost, 107);
    assert_session_query!(query_option_instr_comm_rate, OptionInstrCommRate, 108);
    assert_session_query!(query_exchange_margin_rate, ExchangeMarginRate, 109);
    assert_session_query!(
        query_exchange_margin_rate_adjust,
        ExchangeMarginRateAdjust,
        110
    );
    assert_session_query!(query_exchange_rate, ExchangeRate, 111);
    assert_session_query!(query_product_exch_rate, ProductExchRate, 112);
    assert_session_query!(query_product_group, ProductGroup, 113);
    assert_session_query!(
        query_investor_product_group_margin,
        InvestorProductGroupMargin,
        114
    );
    assert_session_query!(query_comb_instrument_guard, CombInstrumentGuard, 115);
    assert_session_query!(
        query_max_order_volume_with_price,
        MaxOrderVolumeWithPrice,
        116
    );
    assert_session_query!(query_spbm_future_parameter, SPBMFutureParameter, 117);
    assert_session_query!(query_spbm_option_parameter, SPBMOptionParameter, 118);
    assert_session_query!(query_spbm_intra_parameter, SPBMIntraParameter, 119);
    assert_session_query!(query_spbm_inter_parameter, SPBMInterParameter, 120);
    assert_session_query!(query_spbm_portf_definition, SPBMPortfDefinition, 121);
    assert_session_query!(query_spbm_investor_portf_def, SPBMInvestorPortfDef, 122);
    assert_session_query!(
        query_spbm_add_on_inter_parameter,
        SPBMAddOnInterParameter,
        123
    );
    assert_session_query!(
        query_investor_portf_margin_ratio,
        InvestorPortfMarginRatio,
        124
    );
    assert_session_query!(query_investor_prod_spbm_detail, InvestorProdSPBMDetail, 125);
    assert_session_query!(query_spmm_inst_param, SPMMInstParam, 126);
    assert_session_query!(query_spmm_product_param, SPMMProductParam, 127);
    assert_session_query!(
        query_investor_commodity_spmm_margin,
        InvestorCommoditySPMMMargin,
        128
    );
    assert_session_query!(
        query_investor_commodity_group_spmm_margin,
        InvestorCommodityGroupSPMMMargin,
        129
    );
    assert_session_query!(query_rcams_comb_product_info, RCAMSCombProductInfo, 130);
    assert_session_query!(query_rcams_instr_parameter, RCAMSInstrParameter, 131);
    assert_session_query!(query_rcams_intra_parameter, RCAMSIntraParameter, 132);
    assert_session_query!(query_rcams_inter_parameter, RCAMSInterParameter, 133);
    assert_session_query!(
        query_rcams_short_opt_adjust_param,
        RCAMSShortOptAdjustParam,
        134
    );
    assert_session_query!(
        query_rcams_investor_comb_position,
        RCAMSInvestorCombPosition,
        135
    );
    assert_session_query!(
        query_investor_prod_rcams_margin,
        InvestorProdRCAMSMargin,
        136
    );
    assert_session_query!(query_rule_instr_parameter, RULEInstrParameter, 137);
    assert_session_query!(query_rule_intra_parameter, RULEIntraParameter, 138);
    assert_session_query!(query_rule_inter_parameter, RULEInterParameter, 139);
    assert_session_query!(query_investor_prod_rule_margin, InvestorProdRULEMargin, 140);
    assert_session_query!(query_investor_portf_setting, InvestorPortfSetting, 141);
    assert_session_query!(
        query_risk_settle_invest_position,
        RiskSettleInvestPosition,
        142
    );
    assert_session_query!(
        query_risk_settle_product_status,
        RiskSettleProductStatus,
        143
    );
    assert_session_query!(query_investor_info_comm_rec, InvestorInfoCommRec, 144);
}

#[test]
fn batch7_prepare_preserves_business_fields_and_refs() {
    let config = config();
    let mut offset: InputOffsetSetting = from_default();
    offset.request_id = crate::RequestId::new(77);
    offset.exchange_id = crate::ExchangeId::new("DCE").unwrap();
    prepare_offset_setting(&config, &mut offset);
    assert_eq!(offset.broker_id.as_str(), "9999");
    assert_eq!(offset.investor_id.as_str(), "1001");
    assert_eq!(offset.user_id.as_str(), "1001");
    assert_eq!(offset.request_id.get(), 77);
    assert_eq!(offset.exchange_id.as_str(), "DCE");

    let mut next_ref = 5;
    let mut spd: InputSpdApply = from_default();
    spd.request_id = crate::RequestId::new(88);
    assert_eq!(
        prepare_spd_apply(&config, &mut next_ref, &mut spd).unwrap(),
        "5"
    );
    assert_eq!(spd.order_ref.as_str(), "5");
    assert_eq!(spd.request_id.get(), 88);
    let mut hedge: InputHedgeCfm = from_default();
    assert_eq!(
        prepare_hedge_cfm(&config, &mut next_ref, &mut hedge).unwrap(),
        "6"
    );
    assert_eq!(hedge.broker_id.as_str(), "9999");
    assert_eq!(hedge.user_id.as_str(), "1001");

    let mut spd_action: InputSpdApplyAction = from_default();
    spd_action.order_ref = OrderRef::new("5").unwrap();
    spd_action.order_system_id = crate::OrderSysId::new("99").unwrap();
    prepare_spd_apply_action(&config, (7, 9), &mut spd_action);
    assert_eq!(spd_action.order_ref.as_str(), "5");
    assert_eq!(spd_action.order_system_id.as_str(), "99");
    assert_eq!(spd_action.front_id.get(), 7);
    assert_eq!(spd_action.session_id.get(), 9);

    let mut hedge_action: InputHedgeCfmAction = from_default();
    hedge_action.order_ref = OrderRef::new("6").unwrap();
    prepare_hedge_cfm_action(&config, (8, 10), &mut hedge_action);
    assert_eq!(hedge_action.order_ref.as_str(), "6");
    assert_eq!(hedge_action.front_id.get(), 8);
    assert_eq!(hedge_action.session_id.get(), 10);
}

#[test]
fn batch7_query_identity_is_filled_from_config() {
    macro_rules! assert_both {
        ($variant:ident) => {{
            let mut kind = QueryKind::$variant(sample());
            fill_query_identity(&config(), &mut kind);
            match kind {
                QueryKind::$variant(request) => {
                    assert_eq!(request.broker_id.as_str(), "9999");
                    assert_eq!(request.investor_id.as_str(), "1001");
                }
                _ => unreachable!(),
            }
        }};
    }
    assert_both!(OffsetSetting);
    assert_both!(SpdApply);
    assert_both!(HedgeCfm);
}

#[test]
fn batch7_public_methods_map_to_commands_and_queries() {
    macro_rules! assert_id_command {
        ($method:ident, $variant:ident, $request_id:expr) => {{
            let (session, inputs) = query_session();
            let responder = std::thread::spawn(move || match inputs.recv().unwrap() {
                TraderInput::Command(TraderCommand::$variant { reply, .. }) => {
                    reply.send(Ok($request_id)).unwrap();
                }
                _ => panic!("期望批次七命令"),
            });
            assert_eq!(session.$method(from_default()).unwrap(), $request_id);
            responder.join().unwrap();
        }};
    }
    macro_rules! assert_ref_command {
        ($method:ident, $variant:ident, $reference:expr) => {{
            let (session, inputs) = query_session();
            let expected = $reference.to_owned();
            let responder = std::thread::spawn(move || match inputs.recv().unwrap() {
                TraderInput::Command(TraderCommand::$variant { reply, .. }) => {
                    reply.send(Ok(expected)).unwrap();
                }
                _ => panic!("期望批次七申请命令"),
            });
            assert_eq!(session.$method(from_default()).unwrap(), $reference);
            responder.join().unwrap();
        }};
    }
    assert_id_command!(set_offset, SetOffset, 201);
    assert_id_command!(cancel_offset_setting, CancelOffsetSetting, 202);
    assert_ref_command!(spd_apply, SpdApply, "203");
    assert_id_command!(spd_apply_action, SpdApplyAction, 204);
    assert_ref_command!(hedge_cfm, HedgeCfm, "205");
    assert_id_command!(hedge_cfm_action, HedgeCfmAction, 206);

    macro_rules! assert_session_query {
        ($method:ident, $variant:ident, $request_id:expr) => {{
            let (session, inputs) = query_session();
            let responder = std::thread::spawn(move || match inputs.recv().unwrap() {
                TraderInput::Command(TraderCommand::Query { kind, reply }) => {
                    assert!(matches!(*kind, QueryKind::$variant(_)));
                    reply.send(Ok($request_id)).unwrap();
                }
                _ => panic!("期望批次七查询命令"),
            });
            assert_eq!(session.$method(sample()).unwrap(), $request_id);
            responder.join().unwrap();
        }};
    }
    assert_session_query!(query_offset_settings, OffsetSetting, 211);
    assert_session_query!(query_spd_apply, SpdApply, 212);
    assert_session_query!(query_hedge_cfm, HedgeCfm, 213);
    assert_session_query!(query_instr_trade_limit, InstrTradeLimit, 214);
    assert_session_query!(query_classified_instrument, ClassifiedInstrument, 215);
    assert_session_query!(query_comb_promotion_param, CombPromotionParam, 216);
}
