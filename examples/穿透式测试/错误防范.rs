#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::{TestContext, market_price};
use ctp_base::{Direction, OffsetFlag, OrderStatus};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = TestContext::connect()?;
    let quote = market_price(&context.ctp)?;

    let status = context.submit_wait(context.order(
        Direction::Buy,
        OffsetFlag::Open,
        quote.ask_price1,
        1,
    ))?;
    if status != OrderStatus::AllTraded {
        return Err(format!("基础开仓未成交：{status:?}").into());
    }
    context.expect_reject(
        context.order(Direction::Buy, OffsetFlag::Open, quote.ask_price1, i32::MAX),
        "资金或持仓限额不足",
    )?;

    println!("错误测试只发送一次并记录结果；资金状态不明时先查询对账，不盲目重复报单");
    context.print_stats();
    context.exit();
    Ok(())
}
