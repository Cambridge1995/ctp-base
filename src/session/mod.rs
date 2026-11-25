//! 托管会话层：在原始 API 之上提供流程自动化。
//!
//! 内建协议级自动化（连接、登录、断线重连、重订阅），业务策略由调用方决定。
//! 本层只处理 ctp-base 自有类型，不依赖任何外部 crate。
//!
//! - [`md`]：行情托管会话；
//! - [`trader`]：交易托管会话（会话链、交易指令、核心与高级查询、银期转账、
//!   保证金/费率、风险监控、对冲设置与 SPD/套保确认；查询响应逐行公开，调用方按需聚合）。

pub mod enums;
pub mod md;
mod md_spi;
mod request;
pub mod trader;
mod trader_spi;

pub use enums::{MdSessionError, MdSessionEvent};
pub use enums::{QueryKind, QueryRow, QueryRows, TraderSessionEvent};
pub use md::{MdConfig, MdSession};
pub use trader::{TraderConfig, TraderSession, TraderSessionError};
