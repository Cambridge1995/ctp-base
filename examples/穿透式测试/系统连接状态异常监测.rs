#[path = "common/mod.rs"]
mod common;
#[path = "../common/config.rs"]
mod config;

use common::TestContext;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = TestContext::connect()?;
    let first = context.session.session_id();
    println!("首登成功：SessionID={first}，开始受控重连");
    context.reconnect()?;
    let recovered = context.session.session_id();
    if recovered == first {
        return Err("重连后 SessionID 未变化".into());
    }
    println!("连接恢复成功：SessionID {first} → {recovered}");
    context.exit();
    Ok(())
}
