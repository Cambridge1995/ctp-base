#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::{TestContext, market_price};
use ctp_base::{Direction, OffsetFlag};
use std::error::Error;

fn env_number<T: std::str::FromStr>(name: &str, default: T) -> Result<T, Box<dyn Error>>
where
    T::Err: Error + 'static,
{
    Ok(std::env::var(name).map_or(Ok(default), |value| value.parse())?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = TestContext::connect()?;
    let min_available = env_number("CTP_TEST_MIN_AVAILABLE", 0.0)?;
    let max_orders = env_number("CTP_TEST_MAX_ORDERS", 1)?;
    let max_cancels = env_number("CTP_TEST_MAX_CANCELS", 1)?;
    let available = context.account()?.available;

    if available < min_available
        || context.stats.order_count >= max_orders
        || context.stats.cancel_count >= max_cancels
    {
        println!(
            "已暂停新开仓：可用资金={available:.2}，报单={}，撤单={}；只允许后续人工核对与平仓",
            context.stats.order_count, context.stats.cancel_count
        );
        context.exit();
        return Ok(());
    }

    let quote = market_price(&context.ctp)?;
    context.submit_and_cancel(Direction::Buy, OffsetFlag::Open, quote.lower_limit_price, 1)?;
    println!("阈值内测试完成；生产策略应在每次新交易意图前重新计算权益、风险度和报撤单计数");
    context.print_stats();
    context.exit();
    Ok(())
}
