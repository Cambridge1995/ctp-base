#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::TestContext;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = TestContext::connect()?;
    println!(
        "穿透式连通成功：FrontID={}，SessionID={}，交易日={}，合约={}",
        context.session.front_id(),
        context.session.session_id(),
        context.session.trading_day(),
        context.instrument.instrument_id.as_str()
    );
    context.print_snapshot()?;
    context.exit();
    Ok(())
}
