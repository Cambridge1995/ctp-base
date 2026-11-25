// 由 ctp-generator 根据审核表生成，用于类型与原生桥接。
// 请勿手工编辑；正式映射以当前审核表为准。

use core::fmt;
use std::os::raw::c_char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextError {
    TooLong { maximum: usize },
    NonAscii { index: usize },
    InteriorNul { index: usize },
    InvalidEncoding,
    UnknownEnumValue,
}

impl fmt::Display for TextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong { maximum } => write!(formatter, "文本长度超过上限 {maximum}"),
            Self::NonAscii { index } => write!(formatter, "字节 {index} 不是 ASCII"),
            Self::InteriorNul { index } => write!(formatter, "字节 {index} 是内嵌 NUL"),
            Self::InvalidEncoding => formatter.write_str("文本不是有效的 GBK 编码"),
            Self::UnknownEnumValue => formatter.write_str("请求包含未知枚举值"),
        }
    }
}

impl std::error::Error for TextError {}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedText<const N: usize> {
    bytes: [u8; N],
    length: usize,
}

impl<const N: usize> FixedText<N> {
    pub fn new(value: &str) -> Result<Self, TextError> {
        let source = value.as_bytes();
        if source.len() >= N {
            return Err(TextError::TooLong { maximum: N - 1 });
        }
        if let Some(index) = source.iter().position(|byte| *byte == 0) {
            return Err(TextError::InteriorNul { index });
        }
        if let Some(index) = source.iter().position(|byte| !byte.is_ascii()) {
            return Err(TextError::NonAscii { index });
        }
        let mut bytes = [0; N];
        bytes[..source.len()].copy_from_slice(source);
        Ok(Self {
            bytes,
            length: source.len(),
        })
    }

    pub(crate) fn from_wire(bytes: &[c_char; N]) -> Result<Self, TextError> {
        let length = bytes.iter().position(|byte| *byte == 0).unwrap_or(N);
        if let Some(index) = bytes[..length]
            .iter()
            .position(|byte| !(*byte as u8).is_ascii())
        {
            return Err(TextError::NonAscii { index });
        }
        let mut output = [0; N];
        for (index, byte) in bytes.iter().enumerate() {
            output[index] = *byte as u8;
        }
        Ok(Self {
            bytes: output,
            length,
        })
    }

    pub(crate) fn to_wire(self) -> [c_char; N] {
        let mut output = [0; N];
        for (index, byte) in self.bytes.iter().enumerate() {
            output[index] = *byte as c_char;
        }
        output
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.length]).expect("FixedText 只包含 ASCII")
    }
}

impl<const N: usize> Default for FixedText<N> {
    fn default() -> Self {
        Self {
            bytes: [0; N],
            length: 0,
        }
    }
}

impl<const N: usize> fmt::Debug for FixedText<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<const N: usize> fmt::Display for FixedText<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

macro_rules! text_type {
    ($name:ident, $capacity:expr, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name(FixedText<$capacity>);

        impl $name {
            pub fn new(value: &str) -> Result<Self, TextError> {
                FixedText::new(value).map(Self)
            }

            #[allow(dead_code)]
            pub(crate) fn from_wire(bytes: &[c_char; $capacity]) -> Result<Self, TextError> {
                FixedText::from_wire(bytes).map(Self)
            }

            #[allow(dead_code)]
            pub(crate) fn to_wire(self) -> [c_char; $capacity] {
                self.0.to_wire()
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! numeric_type {
    ($name:ident, $inner:ty, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name($inner);

        impl $name {
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            pub const fn get(self) -> $inner {
                self.0
            }
        }
    };
}

text_type!(AccountId, 13, "CTP 类型：`TThostFtdcAccountIDType`。");
pub type ActionDay = DateText;
text_type!(BrokerId, 11, "CTP 类型：`TThostFtdcBrokerIDType`。");
text_type!(BusinessUnit, 21, "CTP 类型：`TThostFtdcBusinessUnitType`。");
text_type!(DateText, 9, "CTP 类型：`TThostFtdcDateType`。");
text_type!(ExchangeId, 9, "CTP 类型：`TThostFtdcExchangeIDType`。");
text_type!(
    ExchangeInstrumentId,
    81,
    "CTP 类型：`TThostFtdcExchangeInstIDType`。"
);
numeric_type!(FrontId, i32, "CTP 类型：`TThostFtdcFrontIDType`。");
text_type!(InstrumentId, 81, "CTP 类型：`TThostFtdcInstrumentIDType`。");
text_type!(InvestUnitId, 17, "CTP 类型：`TThostFtdcInvestUnitIDType`。");
text_type!(InvestorId, 13, "CTP 类型：`TThostFtdcInvestorIDType`。");
text_type!(OrderLocalId, 13, "CTP 类型：`TThostFtdcOrderLocalIDType`。");
text_type!(OrderRef, 13, "CTP 类型：`TThostFtdcOrderRefType`。");
text_type!(OrderSysId, 21, "CTP 类型：`TThostFtdcOrderSysIDType`。");
text_type!(ProductId, 41, "CTP 类型：`TThostFtdcProductIDType`。");
numeric_type!(RequestId, i32, "CTP 类型：`TThostFtdcRequestIDType`。");
numeric_type!(SessionId, i32, "CTP 类型：`TThostFtdcSessionIDType`。");
text_type!(TimeText, 9, "CTP 类型：`TThostFtdcTimeType`。");
text_type!(TradeId, 21, "CTP 类型：`TThostFtdcTradeIDType`。");
pub type TradingDay = DateText;
pub type UnderlyingInstrumentId = InstrumentId;
pub type UpdateTime = TimeText;
text_type!(UserId, 16, "CTP 类型：`TThostFtdcUserIDType`。");

const fn assemble_comb(first: Option<u8>, tail: [u8; 4]) -> Option<[u8; 5]> {
    match first {
        Some(first) => Some([first, tail[0], tail[1], tail[2], tail[3]]),
        None => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CombOffsetFlag {
    first: crate::OffsetFlag,
    tail: [u8; 4],
}

impl CombOffsetFlag {
    pub const fn new(first: crate::OffsetFlag) -> Self {
        Self {
            first,
            tail: [0; 4],
        }
    }

    pub const fn from_wire(bytes: [u8; 5]) -> Self {
        Self {
            first: crate::OffsetFlag::from_code(bytes[0]),
            tail: [bytes[1], bytes[2], bytes[3], bytes[4]],
        }
    }

    pub const fn first(&self) -> crate::OffsetFlag {
        self.first
    }

    pub const fn tail(&self) -> &[u8; 4] {
        &self.tail
    }

    pub const fn to_wire(self) -> Option<[u8; 5]> {
        assemble_comb(self.first.code(), self.tail)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CombHedgeFlag {
    first: crate::HedgeFlag,
    tail: [u8; 4],
}

impl CombHedgeFlag {
    pub const fn new(first: crate::HedgeFlag) -> Self {
        Self {
            first,
            tail: [0; 4],
        }
    }

    pub const fn from_wire(bytes: [u8; 5]) -> Self {
        Self {
            first: crate::HedgeFlag::from_code(bytes[0]),
            tail: [bytes[1], bytes[2], bytes[3], bytes[4]],
        }
    }

    pub const fn first(&self) -> crate::HedgeFlag {
        self.first
    }

    pub const fn tail(&self) -> &[u8; 4] {
        &self.tail
    }

    pub const fn to_wire(self) -> Option<[u8; 5]> {
        assemble_comb(self.first.code(), self.tail)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NewsUrgency(u8);

impl NewsUrgency {
    pub const fn from_code(code: u8) -> Self {
        Self(code)
    }

    pub const fn code(self) -> u8 {
        self.0
    }
}

pub type FBTTradeCodeEnum = FixedText<7>;
pub type VirementTradeCode = FixedText<7>;

pub mod codes {
    pub const FBT_BANK_LAUNCH_BANK_TO_BROKER: &str = "102001";
    pub const FBT_BROKER_LAUNCH_BANK_TO_BROKER: &str = "202001";
    pub const FBT_BANK_LAUNCH_BROKER_TO_BANK: &str = "102002";
    pub const FBT_BROKER_LAUNCH_BROKER_TO_BANK: &str = "202002";
    pub const VIREMENT_BANK_BANK_TO_FUTURE: &str = "102001";
    pub const VIREMENT_BANK_FUTURE_TO_BANK: &str = "102002";
    pub const VIREMENT_FUTURE_BANK_TO_FUTURE: &str = "202001";
    pub const VIREMENT_FUTURE_FUTURE_TO_BANK: &str = "202002";
}
