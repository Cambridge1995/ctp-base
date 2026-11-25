#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::{TestContext, market_price};
use ctp_base::{Direction, OffsetFlag, OrderStatus};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rounds = std::env::var("CTP_TEST_ROUNDS")
        .unwrap_or_else(|_| "1".to_owned())
        .parse::<usize>()?;
    let max_orders = std::env::var("CTP_TEST_MAX_ORDERS")
        .unwrap_or_else(|_| "4".to_owned())
        .parse::<usize>()?;
    let max_cancels = std::env::var("CTP_TEST_MAX_CANCELS")
        .unwrap_or_else(|_| "2".to_owned())
        .parse::<usize>()?;
    let max_repeat = std::env::var("CTP_TEST_MAX_REPEAT")
        .unwrap_or_else(|_| "2".to_owned())
        .parse::<usize>()?;
    if rounds == 0 {
        return Err("CTP_TEST_ROUNDS 必须大于 0".into());
    }

    let mut context = TestContext::connect()?;
    let quote = market_price(&context.ctp)?;
    for round in 0..rounds {
        if round > 0 {
            context.stats.repeat_intent += 1;
        }
        let status = context.submit_wait(context.order(
            Direction::Buy,
            OffsetFlag::Open,
            quote.upper_limit_price,
            1,
        ))?;
        if status != OrderStatus::AllTraded {
            return Err(format!("第 {} 轮开仓未成交", round + 1).into());
        }
        context.submit_and_cancel(Direction::Buy, OffsetFlag::Open, quote.lower_limit_price, 1)?;
        context.print_stats();
        if context.stats.order_count >= max_orders {
            println!(
                "警告：报单笔数达到阈值 {} >= {max_orders}",
                context.stats.order_count
            );
        }
        if context.stats.cancel_count >= max_cancels {
            println!(
                "警告：撤单笔数达到阈值 {} >= {max_cancels}",
                context.stats.cancel_count
            );
        }
        if context.stats.repeat_intent >= max_repeat {
            println!(
                "警告：重复业务意图达到阈值 {} >= {max_repeat}",
                context.stats.repeat_intent
            );
        }
    }
    println!("阈值预警使用本进程本地计数，不冒充柜台风控结果");
    context.exit();
    Ok(())
}
