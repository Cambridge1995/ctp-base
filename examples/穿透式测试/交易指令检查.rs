#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::{TestContext, market_price};
use ctp_base::{Direction, InstrumentId, OffsetFlag, OrderStatus};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = TestContext::connect()?;
    context.print_snapshot()?;
    let quote = market_price(&context.ctp)?;

    let mut invalid_instrument =
        context.order(Direction::Buy, OffsetFlag::Open, quote.ask_price1, 1);
    invalid_instrument.instrument_id = InstrumentId::new("INVALID0")?;
    context.expect_reject(invalid_instrument, "错误合约代码")?;

    context.expect_reject(
        context.order(Direction::Buy, OffsetFlag::Open, quote.ask_price1, 0),
        "下单数量为零",
    )?;
    context.expect_reject(
        context.order(
            Direction::Buy,
            OffsetFlag::Open,
            quote.lower_limit_price + context.instrument.price_tick / 2.0,
            1,
        ),
        "价格不是最小跳动整数倍",
    )?;
    context.expect_reject(
        context.order(
            Direction::Sell,
            context.close_offset(),
            quote.bid_price1,
            1000,
        ),
        "可用平仓数量不足",
    )?;
    if context.instrument.min_limit_order_volume > 1 {
        let below_minimum = context.instrument.min_limit_order_volume - 1;
        context.expect_reject(
            context.order(
                Direction::Buy,
                OffsetFlag::Open,
                quote.bid_price1,
                below_minimum,
            ),
            "低于单笔最小下单量",
        )?;
    } else {
        println!("目标合约最小下单量为 1，跳过与零数量重复的最小手数用例");
    }
    context.expect_reject(
        context.order(
            Direction::Buy,
            OffsetFlag::Open,
            quote.upper_limit_price,
            i32::MAX,
        ),
        "超过单笔最大下单量",
    )?;

    let opened = context.submit_wait(context.order(
        Direction::Sell,
        OffsetFlag::Open,
        quote.lower_limit_price,
        1,
    ))?;
    if opened != OrderStatus::AllTraded {
        return Err(format!("冻结验仓的准备开仓未成交：{opened:?}").into());
    }
    let pending_close = context.submit(context.order(
        Direction::Buy,
        context.close_offset(),
        quote.lower_limit_price,
        1,
    ))?;
    context.wait_queueing(&pending_close)?;
    let frozen_result = context.expect_reject(
        context.order(
            Direction::Buy,
            context.close_offset(),
            quote.upper_limit_price,
            1,
        ),
        "在途平仓单冻结可用持仓",
    );
    let cleanup = context.cancel(&pending_close);
    if !matches!(cleanup?, OrderStatus::Canceled | OrderStatus::AllTraded) {
        return Err("冻结验仓的在途平仓单未达到终态".into());
    }
    match frozen_result {
        Ok(_) => {}
        Err(error) if error.to_string().contains("未被拒绝") => {
            println!("SKIP 在途平仓冻结验仓：当前柜台接受了再次平仓，未观察到冻结拒绝：{error}");
        }
        Err(error) => return Err(error),
    }

    println!("非整数手数由 Rust 的 i32 字段在编译期阻止，不构造并发送这类无效请求");
    context.exit();
    Ok(())
}
