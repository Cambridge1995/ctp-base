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
    for index in 0..4 {
        let status = context.submit_wait(context.order(
            Direction::Buy,
            OffsetFlag::Open,
            quote.upper_limit_price,
            1,
        ))?;
        if status != OrderStatus::AllTraded {
            return Err(format!("第 {} 笔重复业务意图未成交：{status:?}", index + 1).into());
        }
        if index > 0 {
            context.stats.repeat_intent += 1;
        }
    }
    context.print_stats();
    println!(
        "Session 为每笔报单分配新的 OrderRef；这里统计的是重复业务意图，不是相同 OrderRef 的协议重复报单"
    );
    context.exit();
    Ok(())
}
