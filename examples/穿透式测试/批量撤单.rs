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

    let reference = context.submit(context.order(
        Direction::Buy,
        OffsetFlag::Open,
        quote.lower_limit_price,
        1,
    ))?;
    context.wait_queueing(&reference)?;
    for attempt in 0..3 {
        match context.cancel(&reference) {
            Ok(OrderStatus::Canceled) if attempt == 0 => println!("首次撤单成功"),
            Ok(status) => return Err(format!("重复撤单未被拒绝，状态={status:?}").into()),
            Err(error) if attempt > 0 => {
                if error.to_string().contains("超时") {
                    return Err(format!("重复撤单未收到明确拒绝：{error}").into());
                }
                context.stats.repeat_intent += 1;
                println!("EXPECTED_REJECT 重复撤单 {}：{error}", attempt + 1);
            }
            Err(error) => return Err(error),
        }
    }

    let mut references = Vec::new();
    for _ in 0..5 {
        let reference = context.submit(context.order(
            Direction::Buy,
            OffsetFlag::Open,
            quote.lower_limit_price,
            1,
        ))?;
        context.wait_queueing(&reference)?;
        references.push(reference);
    }
    context.batch_cancel(&references)?;
    println!("批量撤单最终状态均为 Canceled；CTP 批量请求作用于当前会话，不按单个合约过滤");
    context.print_stats();
    context.exit();
    Ok(())
}
