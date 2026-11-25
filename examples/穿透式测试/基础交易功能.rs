#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::{TestContext, market_price};
use ctp_base::{Direction, OffsetFlag, OrderStatus};
use std::error::Error;

fn expect_traded(status: OrderStatus, name: &str) -> Result<(), Box<dyn Error>> {
    if status == OrderStatus::AllTraded {
        println!("PASS {name}：AllTraded");
        Ok(())
    } else {
        Err(format!("{name} 未全部成交：{status:?}").into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = TestContext::connect()?;
    context.print_snapshot()?;
    let quote = market_price(&context.ctp)?;

    let status = context.submit_wait(context.order(
        Direction::Buy,
        OffsetFlag::Open,
        quote.upper_limit_price,
        1,
    ))?;
    expect_traded(status, "多头开仓")?;
    context.print_positions()?;
    let status = context.submit_wait(context.order(
        Direction::Sell,
        OffsetFlag::Open,
        quote.lower_limit_price,
        1,
    ))?;
    expect_traded(status, "空头开仓")?;
    context.print_positions()?;
    let status = context.submit_wait(context.order(
        Direction::Sell,
        context.close_offset(),
        quote.lower_limit_price,
        1,
    ))?;
    expect_traded(status, "多头平仓")?;
    context.print_positions()?;
    let status = context.submit_wait(context.order(
        Direction::Buy,
        context.close_offset(),
        quote.upper_limit_price,
        1,
    ))?;
    expect_traded(status, "空头平仓")?;
    context.print_positions()?;

    context.submit_and_cancel(Direction::Buy, OffsetFlag::Open, quote.lower_limit_price, 1)?;
    println!("PASS 报单后撤单：Canceled");
    context.print_stats();
    context.exit();
    Ok(())
}
