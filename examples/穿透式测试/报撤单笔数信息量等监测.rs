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
    for (name, direction, offset, price) in [
        (
            "多头开仓",
            Direction::Buy,
            OffsetFlag::Open,
            quote.upper_limit_price,
        ),
        (
            "空头开仓",
            Direction::Sell,
            OffsetFlag::Open,
            quote.lower_limit_price,
        ),
        (
            "多头平仓",
            Direction::Sell,
            context.close_offset(),
            quote.lower_limit_price,
        ),
        (
            "空头平仓",
            Direction::Buy,
            context.close_offset(),
            quote.upper_limit_price,
        ),
    ] {
        let status = context.submit_wait(context.order(direction, offset, price, 1))?;
        if status != OrderStatus::AllTraded {
            return Err(format!("{name} 未全部成交：{status:?}").into());
        }
    }
    context.submit_and_cancel(Direction::Buy, OffsetFlag::Open, quote.lower_limit_price, 1)?;
    context.print_stats();
    println!("以上为本进程本地统计，不是柜台官方风控计数；成交与报撤单仍须以回报和查询对账");
    context.exit();
    Ok(())
}
