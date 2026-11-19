use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisconnectReason {
    NetworkReadFailed,
    NetworkWriteFailed,
    HeartbeatReceiveTimeout,
    HeartbeatSendFailed,
    InvalidMessage,
    Unknown(i32),
}

impl DisconnectReason {
    pub const fn from_code(code: i32) -> Self {
        match code {
            4097 => Self::NetworkReadFailed,
            4098 => Self::NetworkWriteFailed,
            8193 => Self::HeartbeatReceiveTimeout,
            8194 => Self::HeartbeatSendFailed,
            8195 => Self::InvalidMessage,
            value => Self::Unknown(value),
        }
    }

    pub const fn code(self) -> i32 {
        match self {
            Self::NetworkReadFailed => 4097,
            Self::NetworkWriteFailed => 4098,
            Self::HeartbeatReceiveTimeout => 8193,
            Self::HeartbeatSendFailed => 8194,
            Self::InvalidMessage => 8195,
            Self::Unknown(value) => value,
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::NetworkReadFailed => "网络读失败",
            Self::NetworkWriteFailed => "网络写失败",
            Self::HeartbeatReceiveTimeout => "接收心跳超时",
            Self::HeartbeatSendFailed => "发送心跳失败",
            Self::InvalidMessage => "收到错误报文",
            Self::Unknown(_) => "未知断线原因",
        }
    }
}

impl From<i32> for DisconnectReason {
    fn from(code: i32) -> Self {
        Self::from_code(code)
    }
}

impl fmt::Display for DisconnectReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}（{}）", self.description(), self.code())
    }
}
