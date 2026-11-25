//! 主动重连验证：`TraderSession::reconnect` 的完整事件链。
//!
//! 用法：`cargo run --example reconnect`
//!
//! 预期：reconnect 后依次收到 Logout → Disconnected → Recovered，且 Recovered 携带的
//! SessionID 与首登不同（官方开发指南记载的"登出=断开重连、SessionID 重置"行为）。
//! 若 Recovered 在超时内未到达，说明当前柜台版本登出后不重连，命中FAQ
//! 第 2 条标注的版本风险——reconnect 的实现需改为销毁实例重建。

#[path = "common/config.rs"]
mod config;

use ctp_base::{TraderConfig, TraderSession, TraderSessionEvent};
use std::error::Error;
use std::time::{Duration, Instant};

const RECOVER_TIMEOUT: Duration = Duration::from_secs(30);

fn main() -> Result<(), Box<dyn Error>> {
    let ctp = config::CtpConfig::load()?;
    std::fs::create_dir_all(&ctp.flow)?;
    let session = TraderSession::new(&TraderConfig {
        broker_id: ctp.broker_id,
        user_id: ctp.user_id,
        password: ctp.password,
        app_id: ctp.app_id,
        auth_code: ctp.auth_code,
        fronts: ctp.trade_fronts,
        flow_dir: ctp.flow,
        ..Default::default()
    })?;
    let first = (
        session.front_id(),
        session.session_id(),
        session.trading_day(),
    );
    println!(
        "首登就绪：FrontID={} SessionID={} 交易日={}",
        first.0, first.1, first.2
    );

    println!("发起主动重连……");
    session.reconnect()?;

    let deadline = Instant::now() + RECOVER_TIMEOUT;
    let mut saw_logout = false;
    let mut saw_disconnected = false;
    loop {
        let Some(event) = session.try_event() else {
            if Instant::now() >= deadline {
                return Err(format!(
                    "{} 秒内未收到 Recovered：当前柜台版本登出后可能不重连，\
                     命中FAQ第 2 条的版本风险",
                    RECOVER_TIMEOUT.as_secs()
                )
                .into());
            }
            std::thread::sleep(Duration::from_millis(50));
            continue;
        };
        match event {
            TraderSessionEvent::Logout => {
                saw_logout = true;
                println!("[事件] Logout：登出回执");
            }
            TraderSessionEvent::Disconnected { reason } => {
                saw_disconnected = true;
                println!("[事件] Disconnected：{reason}");
            }
            TraderSessionEvent::Recovered {
                front_id,
                session_id,
                trading_day,
            } => {
                println!(
                    "[事件] Recovered：FrontID={front_id} SessionID={session_id} 交易日={trading_day}"
                );
                if !saw_logout {
                    println!("警告：未收到 Logout 就恢复，事件链与官方记载不符");
                }
                if !saw_disconnected {
                    println!("警告：未收到 Disconnected 就恢复，事件链与官方记载不符");
                }
                if session_id == first.1 {
                    return Err(format!(
                        "重连后 SessionID 未变（{session_id}），与官方记载的 SessionID 重置不符"
                    )
                    .into());
                }
                let current = (
                    session.front_id(),
                    session.session_id(),
                    session.trading_day(),
                );
                if current != (front_id, session_id, trading_day.clone()) {
                    return Err(format!(
                        "会话身份 getter 未同步：事件={front_id}/{session_id}，getter={current:?}"
                    )
                    .into());
                }
                println!(
                    "验证通过：SessionID {} → {session_id} 已重置，getter 与事件一致",
                    first.1
                );
                break;
            }
            event => println!("[事件] {event:?}"),
        }
    }

    println!("丢弃事件计数：{}", session.dropped_count());
    session.exit();
    println!("会话已关闭（销毁实例）");
    Ok(())
}
