use ctp_base::{
    FensUserInfo, InputOrder, QryInstrument, QryTradingAccount, ReqTransfer, TraderApi, TraderSpi,
};

struct Spi;

impl TraderSpi for Spi {}

#[test]
fn api_requests_support_struct_update_defaults() {
    let order = InputOrder {
        volume_total_original: 1,
        ..Default::default()
    };
    assert_eq!(order.volume_total_original, 1);
    assert!(order.instrument_id.as_str().is_empty());

    let account = QryTradingAccount::default();
    assert!(account.currency_id.is_empty());
    assert!(QryInstrument::default().instrument_id.as_str().is_empty());
    assert!(ReqTransfer::default().bank_id.is_empty());
    assert!(FensUserInfo::default().broker_id.as_str().is_empty());
}

#[test]
fn default_order_rejects_unselected_enum_before_sending() {
    let api = TraderApi::new(Box::new(Spi), "./flow/", false).unwrap();
    let error = api.insert_order(&InputOrder::default(), 1).unwrap_err();
    assert_eq!(error.field(), "OrderPriceType");
}
