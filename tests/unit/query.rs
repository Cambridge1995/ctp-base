#[path = "../../examples/common/query.rs"]
mod query_example;

use ctp_base::sys::convert::FromSys;
use ctp_base::{Order, QueryRow, QueryRows, sys};
use query_example::append_query_row;

fn order() -> Order {
    Order::from_sys(&sys::CThostFtdcOrderField::default()).unwrap()
}

#[test]
fn caller_appends_row_to_matching_view() {
    let mut rows = QueryRows::Orders(Vec::new());
    append_query_row(&mut rows, QueryRow::Order(Box::new(order()))).unwrap();
    match rows {
        QueryRows::Orders(rows) => assert_eq!(rows.len(), 1),
        _ => panic!("视图类型被意外修改"),
    }
}

#[test]
fn caller_rejects_row_from_different_interface() {
    let mut rows = QueryRows::Trades(Vec::new());
    let error = append_query_row(&mut rows, QueryRow::Order(Box::new(order())))
        .expect_err("报单行不能写入成交视图");
    assert!(error.contains("类型不符"), "{error}");
}
