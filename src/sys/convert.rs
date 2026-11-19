// 由 ctp-generator 根据审核表生成，用于类型与原生桥接。
// 请勿手工编辑；正式映射以当前审核表为准。

use crate::sys;
use crate::types::{FixedText, TextError};
use std::{error::Error, fmt, os::raw::c_char};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionError {
    field: &'static str,
    source: TextError,
}

impl ConversionError {
    pub const fn new(field: &'static str, source: TextError) -> Self {
        Self { field, source }
    }

    pub const fn field(self) -> &'static str {
        self.field
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "字段 {} 转换失败：{}", self.field, self.source)
    }
}

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

pub trait FromSys: Sized {
    type Raw;
    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError>;
}

pub(crate) unsafe fn owned<T: FromSys>(data: *const u8) -> Result<Option<T>, ConversionError> {
    if data.is_null() {
        return Ok(None);
    }
    // SAFETY: 调用方保证指针指向对应的官方结构，且仅在当前回调内借用。
    let raw = unsafe { &*data.cast::<T::Raw>() };
    T::from_sys(raw).map(Some)
}

macro_rules! owned_params {
    ($sink:expr, $($name:ident : $type:ty),+ $(,)?) => {
        $(
            let $name: Option<$type> = match unsafe {
                // SAFETY: 指针由对应的 C++ SPI 回调提供，仅在本回调内借用并立即 owned 化。
                $crate::sys::convert::owned::<$type>($name)
            } {
                Ok(value) => value,
                Err(error) => {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        $sink.spi.conversion_error(error)
                    }));
                    return;
                }
            };
        )+
    };
}

pub(crate) use owned_params;

pub(crate) fn string_from_wire<const N: usize>(
    bytes: &[c_char; N],
    field: &'static str,
) -> Result<String, ConversionError> {
    let bytes: [u8; N] = std::array::from_fn(|index| bytes[index] as u8);
    let length = bytes.iter().position(|byte| *byte == 0).unwrap_or(N);
    let decoded =
        encoding_rs::GBK.decode_without_bom_handling_and_without_replacement(&bytes[..length]);
    let text = decoded
        .ok_or_else(|| ConversionError::new(field, TextError::InvalidEncoding))?
        .into_owned();
    Ok(text)
}

fn string_field<const N: usize>(
    value: &str,
    field: &'static str,
) -> Result<[c_char; N], ConversionError> {
    let text = FixedText::<N>::new(value).map_err(|source| ConversionError::new(field, source))?;
    Ok(text.to_wire())
}

pub(crate) fn ensure_no_nul(value: &str, field: &'static str) -> Result<(), ConversionError> {
    if let Some(index) = value.as_bytes().iter().position(|byte| *byte == 0) {
        return Err(ConversionError::new(
            field,
            TextError::InteriorNul { index },
        ));
    }
    Ok(())
}

fn comb_to_wire(
    bytes: Option<[u8; 5]>,
    field: &'static str,
) -> Result<[c_char; 5], ConversionError> {
    let bytes = bytes.ok_or_else(|| ConversionError::new(field, TextError::UnknownEnumValue))?;
    Ok(bytes.map(|byte| byte as c_char))
}

impl FromSys for crate::AccountRegister {
    type Raw = sys::CThostFtdcAccountregisterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_day: string_from_wire(&raw.TradeDay, "TradeDay")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            open_or_destroy: crate::OpenOrDestroy::from_code(raw.OpenOrDestroy as u8),
            reg_date: string_from_wire(&raw.RegDate, "RegDate")?,
            out_date: string_from_wire(&raw.OutDate, "OutDate")?,
            tid: raw.TID,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::BatchOrderAction {
    type Raw = sys::CThostFtdcBatchOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_action_ref: raw.OrderActionRef,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::BrokerTradingAlgos {
    type Raw = sys::CThostFtdcBrokerTradingAlgosField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            handle_position_algo_id: crate::HandlePositionAlgoID::from_code(
                raw.HandlePositionAlgoID as u8,
            ),
            find_margin_rate_algo_id: crate::FindMarginRateAlgoID::from_code(
                raw.FindMarginRateAlgoID as u8,
            ),
            handle_trading_account_algo_id: crate::HandleTradingAccountAlgoID::from_code(
                raw.HandleTradingAccountAlgoID as u8,
            ),
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::BrokerTradingParams {
    type Raw = sys::CThostFtdcBrokerTradingParamsField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            margin_price_type: crate::MarginPriceType::from_code(raw.MarginPriceType as u8),
            algorithm: crate::Algorithm::from_code(raw.Algorithm as u8),
            avail_include_close_profit: crate::IncludeCloseProfit::from_code(
                raw.AvailIncludeCloseProfit as u8,
            ),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            option_royalty_price_type: crate::OptionRoyaltyPriceType::from_code(
                raw.OptionRoyaltyPriceType as u8,
            ),
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
        })
    }
}

impl FromSys for crate::Bulletin {
    type Raw = sys::CThostFtdcBulletinField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            bulletin_id: raw.BulletinID,
            sequence_no: raw.SequenceNo,
            news_type: string_from_wire(&raw.NewsType, "NewsType")?,
            news_urgency: crate::NewsUrgency::from_code(raw.NewsUrgency as u8),
            send_time: crate::TimeText::from_wire(&raw.SendTime)
                .map_err(|source| ConversionError::new("SendTime", source))?,
            summary: string_from_wire(&raw.Abstract, "Abstract")?,
            source: string_from_wire(&raw.ComeFrom, "ComeFrom")?,
            content: string_from_wire(&raw.Content, "Content")?,
            url_link: string_from_wire(&raw.URLLink, "URLLink")?,
            market_id: string_from_wire(&raw.MarketID, "MarketID")?,
        })
    }
}

impl FromSys for crate::CFMMCTradingAccountKey {
    type Raw = sys::CThostFtdcCFMMCTradingAccountKeyField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            key_id: raw.KeyID,
            current_key: string_from_wire(&raw.CurrentKey, "CurrentKey")?,
        })
    }
}

impl FromSys for crate::CFMMCTradingAccountToken {
    type Raw = sys::CThostFtdcCFMMCTradingAccountTokenField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            key_id: raw.KeyID,
            token: string_from_wire(&raw.Token, "Token")?,
        })
    }
}

impl FromSys for crate::CancelAccount {
    type Raw = sys::CThostFtdcCancelAccountField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            gender: crate::Gender::from_code(raw.Gender as u8),
            country_code: string_from_wire(&raw.CountryCode, "CountryCode")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            address: string_from_wire(&raw.Address, "Address")?,
            zip_code: string_from_wire(&raw.ZipCode, "ZipCode")?,
            telephone: string_from_wire(&raw.Telephone, "Telephone")?,
            mobile_phone: string_from_wire(&raw.MobilePhone, "MobilePhone")?,
            fax: string_from_wire(&raw.Fax, "Fax")?,
            email: string_from_wire(&raw.EMail, "EMail")?,
            money_account_status: crate::MoneyAccountStatus::from_code(
                raw.MoneyAccountStatus as u8,
            ),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            cash_exchange_code: crate::CashExchangeCode::from_code(raw.CashExchangeCode as u8),
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            tid: raw.TID,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::CancelOffsetSetting {
    type Raw = sys::CThostFtdcCancelOffsetSettingField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            underlying_instrument_id: crate::InstrumentId::from_wire(&raw.UnderlyingInstrID)
                .map_err(|source| ConversionError::new("UnderlyingInstrID", source))?,
            product_id: crate::ProductId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            offset_type: crate::OffsetType::from_code(raw.OffsetType as u8),
            volume: raw.Volume,
            is_offset: raw.IsOffset != 0,
            request_id: crate::RequestId::new(raw.RequestID),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            exchange_serial_no: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeSerialNo)
                .map_err(|source| ConversionError::new("ExchangeSerialNo", source))?,
            exchange_product_id: crate::ProductId::from_wire(&raw.ExchangeProductID)
                .map_err(|source| ConversionError::new("ExchangeProductID", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
        })
    }
}

impl FromSys for crate::ChangeAccount {
    type Raw = sys::CThostFtdcChangeAccountField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            gender: crate::Gender::from_code(raw.Gender as u8),
            country_code: string_from_wire(&raw.CountryCode, "CountryCode")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            address: string_from_wire(&raw.Address, "Address")?,
            zip_code: string_from_wire(&raw.ZipCode, "ZipCode")?,
            telephone: string_from_wire(&raw.Telephone, "Telephone")?,
            mobile_phone: string_from_wire(&raw.MobilePhone, "MobilePhone")?,
            fax: string_from_wire(&raw.Fax, "Fax")?,
            email: string_from_wire(&raw.EMail, "EMail")?,
            money_account_status: crate::MoneyAccountStatus::from_code(
                raw.MoneyAccountStatus as u8,
            ),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            new_bank_account: string_from_wire(&raw.NewBankAccount, "NewBankAccount")?,
            new_bank_pass_word: string_from_wire(&raw.NewBankPassWord, "NewBankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            install_id: raw.InstallID,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            tid: raw.TID,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::CombAction {
    type Raw = sys::CThostFtdcCombActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            comb_action_ref: crate::OrderRef::from_wire(&raw.CombActionRef)
                .map_err(|source| ConversionError::new("CombActionRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            direction: crate::Direction::from_code(raw.Direction as u8),
            volume: raw.Volume,
            comb_direction: crate::CombDirection::from_code(raw.CombDirection as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            action_status: crate::OrderActionStatus::from_code(raw.ActionStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            sequence_no: raw.SequenceNo,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            com_trade_id: crate::TradeId::from_wire(&raw.ComTradeID)
                .map_err(|source| ConversionError::new("ComTradeID", source))?,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::CombInstrumentGuard {
    type Raw = sys::CThostFtdcCombInstrumentGuardField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            guaranty_ratio: raw.GuarantRatio,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::CombLeg {
    type Raw = sys::CThostFtdcCombLegField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            comb_instrument_id: crate::InstrumentId::from_wire(&raw.CombInstrumentID)
                .map_err(|source| ConversionError::new("CombInstrumentID", source))?,
            leg_id: raw.LegID,
            leg_instrument_id: crate::InstrumentId::from_wire(&raw.LegInstrumentID)
                .map_err(|source| ConversionError::new("LegInstrumentID", source))?,
            direction: crate::Direction::from_code(raw.Direction as u8),
            leg_multiple: raw.LegMultiple,
            imply_level: raw.ImplyLevel,
        })
    }
}

impl FromSys for crate::CombPromotionParam {
    type Raw = sys::CThostFtdcCombPromotionParamField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            comb_hedge_flag: crate::CombHedgeFlag::from_wire(std::array::from_fn(|index| {
                raw.CombHedgeFlag[index] as u8
            })),
            xparameter: raw.Xparameter,
        })
    }
}

impl FromSys for crate::ContractBank {
    type Raw = sys::CThostFtdcContractBankField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBrchID, "BankBrchID")?,
            bank_name: string_from_wire(&raw.BankName, "BankName")?,
            csrc_bank_id: string_from_wire(&raw.csrcBankID, "csrcBankID")?,
        })
    }
}

impl FromSys for crate::DepthMarketData {
    type Raw = sys::CThostFtdcDepthMarketDataField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            last_price: raw.LastPrice,
            pre_settlement_price: raw.PreSettlementPrice,
            pre_close_price: raw.PreClosePrice,
            pre_open_interest: raw.PreOpenInterest,
            open_price: raw.OpenPrice,
            highest_price: raw.HighestPrice,
            lowest_price: raw.LowestPrice,
            volume: raw.Volume,
            turnover: raw.Turnover,
            open_interest: raw.OpenInterest,
            close_price: raw.ClosePrice,
            settlement_price: raw.SettlementPrice,
            upper_limit_price: raw.UpperLimitPrice,
            lower_limit_price: raw.LowerLimitPrice,
            pre_delta: raw.PreDelta,
            current_delta: raw.CurrDelta,
            update_time: crate::TimeText::from_wire(&raw.UpdateTime)
                .map_err(|source| ConversionError::new("UpdateTime", source))?,
            update_millis: raw.UpdateMillisec,
            bid_price1: raw.BidPrice1,
            bid_volume1: raw.BidVolume1,
            ask_price1: raw.AskPrice1,
            ask_volume1: raw.AskVolume1,
            bid_price2: raw.BidPrice2,
            bid_volume2: raw.BidVolume2,
            ask_price2: raw.AskPrice2,
            ask_volume2: raw.AskVolume2,
            bid_price3: raw.BidPrice3,
            bid_volume3: raw.BidVolume3,
            ask_price3: raw.AskPrice3,
            ask_volume3: raw.AskVolume3,
            bid_price4: raw.BidPrice4,
            bid_volume4: raw.BidVolume4,
            ask_price4: raw.AskPrice4,
            ask_volume4: raw.AskVolume4,
            bid_price5: raw.BidPrice5,
            bid_volume5: raw.BidVolume5,
            ask_price5: raw.AskPrice5,
            ask_volume5: raw.AskVolume5,
            average_price: raw.AveragePrice,
            action_day: crate::DateText::from_wire(&raw.ActionDay)
                .map_err(|source| ConversionError::new("ActionDay", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            banding_upper_price: raw.BandingUpperPrice,
            banding_lower_price: raw.BandingLowerPrice,
        })
    }
}

impl FromSys for crate::EWarrantOffset {
    type Raw = sys::CThostFtdcEWarrantOffsetField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            direction: crate::Direction::from_code(raw.Direction as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            volume: raw.Volume,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::ErrorConditionalOrder {
    type Raw = sys::CThostFtdcErrorConditionalOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            order_price_type: crate::OrderPriceType::from_code(raw.OrderPriceType as u8),
            direction: crate::Direction::from_code(raw.Direction as u8),
            comb_offset_flag: crate::CombOffsetFlag::from_wire(std::array::from_fn(|index| {
                raw.CombOffsetFlag[index] as u8
            })),
            comb_hedge_flag: crate::CombHedgeFlag::from_wire(std::array::from_fn(|index| {
                raw.CombHedgeFlag[index] as u8
            })),
            limit_price: raw.LimitPrice,
            volume_total_original: raw.VolumeTotalOriginal,
            time_condition: crate::TimeCondition::from_code(raw.TimeCondition as u8),
            gtd_date: crate::DateText::from_wire(&raw.GTDDate)
                .map_err(|source| ConversionError::new("GTDDate", source))?,
            volume_condition: crate::VolumeCondition::from_code(raw.VolumeCondition as u8),
            min_volume: raw.MinVolume,
            contingent_condition: crate::ContingentCondition::from_code(
                raw.ContingentCondition as u8,
            ),
            stop_price: raw.StopPrice,
            force_close_reason: crate::ForceCloseReason::from_code(raw.ForceCloseReason as u8),
            is_auto_suspend: raw.IsAutoSuspend != 0,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            order_source: crate::OrderSource::from_code(raw.OrderSource as u8),
            order_status: crate::OrderStatus::from_code(raw.OrderStatus as u8),
            order_type: crate::OrderType::from_code(raw.OrderType as u8),
            volume_traded: raw.VolumeTraded,
            volume_total: raw.VolumeTotal,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            active_time: crate::TimeText::from_wire(&raw.ActiveTime)
                .map_err(|source| ConversionError::new("ActiveTime", source))?,
            suspend_time: crate::TimeText::from_wire(&raw.SuspendTime)
                .map_err(|source| ConversionError::new("SuspendTime", source))?,
            update_time: crate::TimeText::from_wire(&raw.UpdateTime)
                .map_err(|source| ConversionError::new("UpdateTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            active_trader_id: string_from_wire(&raw.ActiveTraderID, "ActiveTraderID")?,
            clearing_part_id: string_from_wire(&raw.ClearingPartID, "ClearingPartID")?,
            sequence_no: raw.SequenceNo,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            user_force_close: raw.UserForceClose != 0,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_order_seq: raw.BrokerOrderSeq,
            relative_order_system_id: crate::OrderSysId::from_wire(&raw.RelativeOrderSysID)
                .map_err(|source| ConversionError::new("RelativeOrderSysID", source))?,
            zce_total_traded_volume: raw.ZCETotalTradedVolume,
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            is_swap_order: raw.IsSwapOrder != 0,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::Exchange {
    type Raw = sys::CThostFtdcExchangeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            exchange_name: string_from_wire(&raw.ExchangeName, "ExchangeName")?,
            exchange_property: crate::ExchangeProperty::from_code(raw.ExchangeProperty as u8),
        })
    }
}

impl FromSys for crate::ExchangeMarginRate {
    type Raw = sys::CThostFtdcExchangeMarginRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            long_margin_ratio_by_money: raw.LongMarginRatioByMoney,
            long_margin_ratio_by_volume: raw.LongMarginRatioByVolume,
            short_margin_ratio_by_money: raw.ShortMarginRatioByMoney,
            short_margin_ratio_by_volume: raw.ShortMarginRatioByVolume,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::ExchangeMarginRateAdjust {
    type Raw = sys::CThostFtdcExchangeMarginRateAdjustField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            long_margin_ratio_by_money: raw.LongMarginRatioByMoney,
            long_margin_ratio_by_volume: raw.LongMarginRatioByVolume,
            short_margin_ratio_by_money: raw.ShortMarginRatioByMoney,
            short_margin_ratio_by_volume: raw.ShortMarginRatioByVolume,
            exchange_long_margin_ratio_by_money: raw.ExchLongMarginRatioByMoney,
            exchange_long_margin_ratio_by_volume: raw.ExchLongMarginRatioByVolume,
            exchange_short_margin_ratio_by_money: raw.ExchShortMarginRatioByMoney,
            exchange_short_margin_ratio_by_volume: raw.ExchShortMarginRatioByVolume,
            no_long_margin_ratio_by_money: raw.NoLongMarginRatioByMoney,
            no_long_margin_ratio_by_volume: raw.NoLongMarginRatioByVolume,
            no_short_margin_ratio_by_money: raw.NoShortMarginRatioByMoney,
            no_short_margin_ratio_by_volume: raw.NoShortMarginRatioByVolume,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::ExchangeRate {
    type Raw = sys::CThostFtdcExchangeRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            from_currency_id: string_from_wire(&raw.FromCurrencyID, "FromCurrencyID")?,
            from_currency_unit: raw.FromCurrencyUnit,
            to_currency_id: string_from_wire(&raw.ToCurrencyID, "ToCurrencyID")?,
            exchange_rate: raw.ExchangeRate,
        })
    }
}

impl FromSys for crate::ExecOrder {
    type Raw = sys::CThostFtdcExecOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exec_order_ref: crate::OrderRef::from_wire(&raw.ExecOrderRef)
                .map_err(|source| ConversionError::new("ExecOrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            volume: raw.Volume,
            request_id: crate::RequestId::new(raw.RequestID),
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            offset_flag: crate::OffsetFlag::from_code(raw.OffsetFlag as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            action_type: crate::ActionType::from_code(raw.ActionType as u8),
            position_direction: crate::PosiDirection::from_code(raw.PosiDirection as u8),
            reserve_position_flag: crate::ExecOrderPositionFlag::from_code(
                raw.ReservePositionFlag as u8,
            ),
            close_flag: crate::ExecOrderCloseFlag::from_code(raw.CloseFlag as u8),
            exec_order_local_id: crate::OrderLocalId::from_wire(&raw.ExecOrderLocalID)
                .map_err(|source| ConversionError::new("ExecOrderLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            exec_order_system_id: string_from_wire(&raw.ExecOrderSysID, "ExecOrderSysID")?,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            exec_result: crate::ExecResult::from_code(raw.ExecResult as u8),
            clearing_part_id: string_from_wire(&raw.ClearingPartID, "ClearingPartID")?,
            sequence_no: raw.SequenceNo,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_exec_order_seq: raw.BrokerExecOrderSeq,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::ExecOrderAction {
    type Raw = sys::CThostFtdcExecOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exec_order_action_ref: raw.ExecOrderActionRef,
            exec_order_ref: crate::OrderRef::from_wire(&raw.ExecOrderRef)
                .map_err(|source| ConversionError::new("ExecOrderRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            exec_order_system_id: string_from_wire(&raw.ExecOrderSysID, "ExecOrderSysID")?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            exec_order_local_id: crate::OrderLocalId::from_wire(&raw.ExecOrderLocalID)
                .map_err(|source| ConversionError::new("ExecOrderLocalID", source))?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            action_type: crate::ActionType::from_code(raw.ActionType as u8),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::ForQuote {
    type Raw = sys::CThostFtdcForQuoteField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            for_quote_ref: crate::OrderRef::from_wire(&raw.ForQuoteRef)
                .map_err(|source| ConversionError::new("ForQuoteRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            for_quote_local_id: crate::OrderLocalId::from_wire(&raw.ForQuoteLocalID)
                .map_err(|source| ConversionError::new("ForQuoteLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            for_quote_status: crate::ForQuoteStatus::from_code(raw.ForQuoteStatus as u8),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_for_quote_seq: raw.BrokerForQutoSeq,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::ForQuoteRsp {
    type Raw = sys::CThostFtdcForQuoteRspField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            for_quote_system_id: crate::OrderSysId::from_wire(&raw.ForQuoteSysID)
                .map_err(|source| ConversionError::new("ForQuoteSysID", source))?,
            for_quote_time: crate::TimeText::from_wire(&raw.ForQuoteTime)
                .map_err(|source| ConversionError::new("ForQuoteTime", source))?,
            action_day: crate::DateText::from_wire(&raw.ActionDay)
                .map_err(|source| ConversionError::new("ActionDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::HedgeCfm {
    type Raw = sys::CThostFtdcHedgeCfmField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            volume: raw.Volume,
            direction: crate::Direction::from_code(raw.Direction as u8),
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_order_seq: raw.BrokerOrderSeq,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            apply_status: crate::ApplyStatus::from_code(raw.ApplyStatus as u8),
            sequence_no: raw.SequenceNo,
            deal_volume: raw.DealVolume,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            req_date: crate::DateText::from_wire(&raw.ReqDate)
                .map_err(|source| ConversionError::new("ReqDate", source))?,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::HedgeCfmAction {
    type Raw = sys::CThostFtdcHedgeCfmActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::InputBatchOrderAction {
    type Raw = sys::CThostFtdcInputBatchOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_action_ref: raw.OrderActionRef,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputCombAction {
    type Raw = sys::CThostFtdcInputCombActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            comb_action_ref: crate::OrderRef::from_wire(&raw.CombActionRef)
                .map_err(|source| ConversionError::new("CombActionRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            direction: crate::Direction::from_code(raw.Direction as u8),
            volume: raw.Volume,
            comb_direction: crate::CombDirection::from_code(raw.CombDirection as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputExecOrder {
    type Raw = sys::CThostFtdcInputExecOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exec_order_ref: crate::OrderRef::from_wire(&raw.ExecOrderRef)
                .map_err(|source| ConversionError::new("ExecOrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            volume: raw.Volume,
            request_id: crate::RequestId::new(raw.RequestID),
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            offset_flag: crate::OffsetFlag::from_code(raw.OffsetFlag as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            action_type: crate::ActionType::from_code(raw.ActionType as u8),
            position_direction: crate::PosiDirection::from_code(raw.PosiDirection as u8),
            reserve_position_flag: crate::ExecOrderPositionFlag::from_code(
                raw.ReservePositionFlag as u8,
            ),
            close_flag: crate::ExecOrderCloseFlag::from_code(raw.CloseFlag as u8),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputExecOrderAction {
    type Raw = sys::CThostFtdcInputExecOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exec_order_action_ref: raw.ExecOrderActionRef,
            exec_order_ref: crate::OrderRef::from_wire(&raw.ExecOrderRef)
                .map_err(|source| ConversionError::new("ExecOrderRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            exec_order_system_id: string_from_wire(&raw.ExecOrderSysID, "ExecOrderSysID")?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputForQuote {
    type Raw = sys::CThostFtdcInputForQuoteField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            for_quote_ref: crate::OrderRef::from_wire(&raw.ForQuoteRef)
                .map_err(|source| ConversionError::new("ForQuoteRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputHedgeCfm {
    type Raw = sys::CThostFtdcInputHedgeCfmField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            volume: raw.Volume,
            direction: crate::Direction::from_code(raw.Direction as u8),
            request_id: crate::RequestId::new(raw.RequestID),
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::InputHedgeCfmAction {
    type Raw = sys::CThostFtdcInputHedgeCfmActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            request_id: crate::RequestId::new(raw.RequestID),
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::InputOffsetSetting {
    type Raw = sys::CThostFtdcInputOffsetSettingField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            underlying_instrument_id: crate::InstrumentId::from_wire(&raw.UnderlyingInstrID)
                .map_err(|source| ConversionError::new("UnderlyingInstrID", source))?,
            product_id: crate::ProductId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            offset_type: crate::OffsetType::from_code(raw.OffsetType as u8),
            volume: raw.Volume,
            is_offset: raw.IsOffset != 0,
            request_id: crate::RequestId::new(raw.RequestID),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::InputOptionSelfClose {
    type Raw = sys::CThostFtdcInputOptionSelfCloseField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            option_self_close_ref: crate::OrderRef::from_wire(&raw.OptionSelfCloseRef)
                .map_err(|source| ConversionError::new("OptionSelfCloseRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            volume: raw.Volume,
            request_id: crate::RequestId::new(raw.RequestID),
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            opt_self_close_flag: crate::OptSelfCloseFlag::from_code(raw.OptSelfCloseFlag as u8),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputOptionSelfCloseAction {
    type Raw = sys::CThostFtdcInputOptionSelfCloseActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            option_self_close_action_ref: raw.OptionSelfCloseActionRef,
            option_self_close_ref: crate::OrderRef::from_wire(&raw.OptionSelfCloseRef)
                .map_err(|source| ConversionError::new("OptionSelfCloseRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            option_self_close_system_id: crate::OrderSysId::from_wire(&raw.OptionSelfCloseSysID)
                .map_err(|source| ConversionError::new("OptionSelfCloseSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::InputOrder {
    type Raw = sys::CThostFtdcInputOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            order_price_type: crate::OrderPriceType::from_code(raw.OrderPriceType as u8),
            direction: crate::Direction::from_code(raw.Direction as u8),
            comb_offset_flag: crate::CombOffsetFlag::from_wire(std::array::from_fn(|index| {
                raw.CombOffsetFlag[index] as u8
            })),
            comb_hedge_flag: crate::CombHedgeFlag::from_wire(std::array::from_fn(|index| {
                raw.CombHedgeFlag[index] as u8
            })),
            limit_price: raw.LimitPrice,
            volume_total_original: raw.VolumeTotalOriginal,
            time_condition: crate::TimeCondition::from_code(raw.TimeCondition as u8),
            gtd_date: crate::DateText::from_wire(&raw.GTDDate)
                .map_err(|source| ConversionError::new("GTDDate", source))?,
            volume_condition: crate::VolumeCondition::from_code(raw.VolumeCondition as u8),
            min_volume: raw.MinVolume,
            contingent_condition: crate::ContingentCondition::from_code(
                raw.ContingentCondition as u8,
            ),
            stop_price: raw.StopPrice,
            force_close_reason: crate::ForceCloseReason::from_code(raw.ForceCloseReason as u8),
            is_auto_suspend: raw.IsAutoSuspend != 0,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            user_force_close: raw.UserForceClose != 0,
            is_swap_order: raw.IsSwapOrder != 0,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::InputOrderAction {
    type Raw = sys::CThostFtdcInputOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_action_ref: raw.OrderActionRef,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            limit_price: raw.LimitPrice,
            volume_change: raw.VolumeChange,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::InputQuote {
    type Raw = sys::CThostFtdcInputQuoteField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            quote_ref: crate::OrderRef::from_wire(&raw.QuoteRef)
                .map_err(|source| ConversionError::new("QuoteRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            ask_price: raw.AskPrice,
            bid_price: raw.BidPrice,
            ask_volume: raw.AskVolume,
            bid_volume: raw.BidVolume,
            request_id: crate::RequestId::new(raw.RequestID),
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            ask_offset_flag: crate::OffsetFlag::from_code(raw.AskOffsetFlag as u8),
            bid_offset_flag: crate::OffsetFlag::from_code(raw.BidOffsetFlag as u8),
            ask_hedge_flag: crate::HedgeFlag::from_code(raw.AskHedgeFlag as u8),
            bid_hedge_flag: crate::HedgeFlag::from_code(raw.BidHedgeFlag as u8),
            ask_order_ref: crate::OrderRef::from_wire(&raw.AskOrderRef)
                .map_err(|source| ConversionError::new("AskOrderRef", source))?,
            bid_order_ref: crate::OrderRef::from_wire(&raw.BidOrderRef)
                .map_err(|source| ConversionError::new("BidOrderRef", source))?,
            for_quote_system_id: crate::OrderSysId::from_wire(&raw.ForQuoteSysID)
                .map_err(|source| ConversionError::new("ForQuoteSysID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            replace_system_id: crate::OrderSysId::from_wire(&raw.ReplaceSysID)
                .map_err(|source| ConversionError::new("ReplaceSysID", source))?,
            time_condition: crate::TimeCondition::from_code(raw.TimeCondition as u8),
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::InputQuoteAction {
    type Raw = sys::CThostFtdcInputQuoteActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            quote_action_ref: raw.QuoteActionRef,
            quote_ref: crate::OrderRef::from_wire(&raw.QuoteRef)
                .map_err(|source| ConversionError::new("QuoteRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            quote_system_id: crate::OrderSysId::from_wire(&raw.QuoteSysID)
                .map_err(|source| ConversionError::new("QuoteSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::InputSpdApply {
    type Raw = sys::CThostFtdcInputSpdApplyField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            first_leg_instrument_id: crate::InstrumentId::from_wire(&raw.FirstLegInstrumentID)
                .map_err(|source| ConversionError::new("FirstLegInstrumentID", source))?,
            second_leg_instrument_id: crate::InstrumentId::from_wire(&raw.SecondLegInstrumentID)
                .map_err(|source| ConversionError::new("SecondLegInstrumentID", source))?,
            volume: raw.Volume,
            direction: crate::Direction::from_code(raw.Direction as u8),
            cmb_type: crate::CmbType::from_code(raw.CmbType as u8),
            request_id: crate::RequestId::new(raw.RequestID),
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::InputSpdApplyAction {
    type Raw = sys::CThostFtdcInputSpdApplyActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            request_id: crate::RequestId::new(raw.RequestID),
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::Instrument {
    type Raw = sys::CThostFtdcInstrumentField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_name: string_from_wire(&raw.InstrumentName, "InstrumentName")?,
            product_class: crate::ProductClass::from_code(raw.ProductClass as u8),
            delivery_year: raw.DeliveryYear,
            delivery_month: raw.DeliveryMonth,
            max_market_order_volume: raw.MaxMarketOrderVolume,
            min_market_order_volume: raw.MinMarketOrderVolume,
            max_limit_order_volume: raw.MaxLimitOrderVolume,
            min_limit_order_volume: raw.MinLimitOrderVolume,
            volume_multiple: raw.VolumeMultiple,
            price_tick: raw.PriceTick,
            create_date: crate::DateText::from_wire(&raw.CreateDate)
                .map_err(|source| ConversionError::new("CreateDate", source))?,
            open_date: crate::DateText::from_wire(&raw.OpenDate)
                .map_err(|source| ConversionError::new("OpenDate", source))?,
            expire_date: crate::DateText::from_wire(&raw.ExpireDate)
                .map_err(|source| ConversionError::new("ExpireDate", source))?,
            start_delivery_date: crate::DateText::from_wire(&raw.StartDelivDate)
                .map_err(|source| ConversionError::new("StartDelivDate", source))?,
            end_delivery_date: crate::DateText::from_wire(&raw.EndDelivDate)
                .map_err(|source| ConversionError::new("EndDelivDate", source))?,
            instrument_life_phase: crate::InstLifePhase::from_code(raw.InstLifePhase as u8),
            is_trading: raw.IsTrading != 0,
            position_type: crate::PositionType::from_code(raw.PositionType as u8),
            position_date_type: crate::PositionDateType::from_code(raw.PositionDateType as u8),
            long_margin_ratio: raw.LongMarginRatio,
            short_margin_ratio: raw.ShortMarginRatio,
            max_margin_side_algorithm: crate::MaxMarginSideAlgorithm::from_code(
                raw.MaxMarginSideAlgorithm as u8,
            ),
            strike_price: raw.StrikePrice,
            options_type: crate::OptionsType::from_code(raw.OptionsType as u8),
            underlying_multiple: raw.UnderlyingMultiple,
            combination_type: crate::CombinationType::from_code(raw.CombinationType as u8),
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            product_id: crate::InstrumentId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            underlying_instrument_id: crate::InstrumentId::from_wire(&raw.UnderlyingInstrID)
                .map_err(|source| ConversionError::new("UnderlyingInstrID", source))?,
        })
    }
}

impl FromSys for crate::InstrumentCommissionRate {
    type Raw = sys::CThostFtdcInstrumentCommissionRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            open_ratio_by_money: raw.OpenRatioByMoney,
            open_ratio_by_volume: raw.OpenRatioByVolume,
            close_ratio_by_money: raw.CloseRatioByMoney,
            close_ratio_by_volume: raw.CloseRatioByVolume,
            close_today_ratio_by_money: raw.CloseTodayRatioByMoney,
            close_today_ratio_by_volume: raw.CloseTodayRatioByVolume,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            biz_type: crate::BizType::from_code(raw.BizType as u8),
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::InstrumentMarginRate {
    type Raw = sys::CThostFtdcInstrumentMarginRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            long_margin_ratio_by_money: raw.LongMarginRatioByMoney,
            long_margin_ratio_by_volume: raw.LongMarginRatioByVolume,
            short_margin_ratio_by_money: raw.ShortMarginRatioByMoney,
            short_margin_ratio_by_volume: raw.ShortMarginRatioByVolume,
            is_relative: raw.IsRelative != 0,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::InstrumentOrderCommRate {
    type Raw = sys::CThostFtdcInstrumentOrderCommRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            order_commission_by_volume: raw.OrderCommByVolume,
            order_action_commission_by_volume: raw.OrderActionCommByVolume,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            order_commission_by_trade: raw.OrderCommByTrade,
            order_action_commission_by_trade: raw.OrderActionCommByTrade,
        })
    }
}

impl FromSys for crate::InstrumentStatus {
    type Raw = sys::CThostFtdcInstrumentStatusField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            settlement_group_id: string_from_wire(&raw.SettlementGroupID, "SettlementGroupID")?,
            instrument_status: crate::InstrumentStatusType::from_code(raw.InstrumentStatus as u8),
            trading_segment_sn: raw.TradingSegmentSN,
            enter_time: crate::TimeText::from_wire(&raw.EnterTime)
                .map_err(|source| ConversionError::new("EnterTime", source))?,
            enter_reason: crate::InstStatusEnterReason::from_code(raw.EnterReason as u8),
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::InvestUnit {
    type Raw = sys::CThostFtdcInvestUnitField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            investor_unit_name: string_from_wire(&raw.InvestorUnitName, "InvestorUnitName")?,
            investor_group_id: crate::InvestorId::from_wire(&raw.InvestorGroupID)
                .map_err(|source| ConversionError::new("InvestorGroupID", source))?,
            commission_model_id: crate::InvestorId::from_wire(&raw.CommModelID)
                .map_err(|source| ConversionError::new("CommModelID", source))?,
            margin_model_id: crate::InvestorId::from_wire(&raw.MarginModelID)
                .map_err(|source| ConversionError::new("MarginModelID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
        })
    }
}

impl FromSys for crate::Investor {
    type Raw = sys::CThostFtdcInvestorField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_group_id: crate::InvestorId::from_wire(&raw.InvestorGroupID)
                .map_err(|source| ConversionError::new("InvestorGroupID", source))?,
            investor_name: string_from_wire(&raw.InvestorName, "InvestorName")?,
            identified_card_type: crate::IdCardType::from_code(raw.IdentifiedCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            is_active: raw.IsActive != 0,
            telephone: string_from_wire(&raw.Telephone, "Telephone")?,
            address: string_from_wire(&raw.Address, "Address")?,
            open_date: crate::DateText::from_wire(&raw.OpenDate)
                .map_err(|source| ConversionError::new("OpenDate", source))?,
            mobile: string_from_wire(&raw.Mobile, "Mobile")?,
            commission_model_id: crate::InvestorId::from_wire(&raw.CommModelID)
                .map_err(|source| ConversionError::new("CommModelID", source))?,
            margin_model_id: crate::InvestorId::from_wire(&raw.MarginModelID)
                .map_err(|source| ConversionError::new("MarginModelID", source))?,
            is_order_frequency: crate::EnumBool::from_code(raw.IsOrderFreq as u8),
            is_open_vol_limit: crate::EnumBool::from_code(raw.IsOpenVolLimit as u8),
        })
    }
}

impl FromSys for crate::InvestorCommodityGroupSPMMMargin {
    type Raw = sys::CThostFtdcInvestorCommodityGroupSPMMMarginField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            commodity_group_id: string_from_wire(&raw.CommodityGroupID, "CommodityGroupID")?,
            margin_before_discount: raw.MarginBeforeDiscount,
            margin_no_discount: raw.MarginNoDiscount,
            long_risk: raw.LongRisk,
            short_risk: raw.ShortRisk,
            close_frozen_margin: raw.CloseFrozenMargin,
            inter_commodity_rate: raw.InterCommodityRate,
            mini_margin_ratio: raw.MiniMarginRatio,
            adjust_ratio: raw.AdjustRatio,
            intra_commodity_discount: raw.IntraCommodityDiscount,
            inter_commodity_discount: raw.InterCommodityDiscount,
            exchange_margin: raw.ExchMargin,
            investor_margin: raw.InvestorMargin,
            frozen_commission: raw.FrozenCommission,
            commission: raw.Commission,
            frozen_cash: raw.FrozenCash,
            cash_in: raw.CashIn,
            strike_frozen_margin: raw.StrikeFrozenMargin,
        })
    }
}

impl FromSys for crate::InvestorCommoditySPMMMargin {
    type Raw = sys::CThostFtdcInvestorCommoditySPMMMarginField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            commodity_id: string_from_wire(&raw.CommodityID, "CommodityID")?,
            margin_before_discount: raw.MarginBeforeDiscount,
            margin_no_discount: raw.MarginNoDiscount,
            long_pos_risk: raw.LongPosRisk,
            long_open_frozen_risk: raw.LongOpenFrozenRisk,
            long_close_frozen_risk: raw.LongCloseFrozenRisk,
            short_pos_risk: raw.ShortPosRisk,
            short_open_frozen_risk: raw.ShortOpenFrozenRisk,
            short_close_frozen_risk: raw.ShortCloseFrozenRisk,
            intra_commodity_rate: raw.IntraCommodityRate,
            option_discount_rate: raw.OptionDiscountRate,
            pos_discount: raw.PosDiscount,
            open_frozen_discount: raw.OpenFrozenDiscount,
            net_risk: raw.NetRisk,
            close_frozen_margin: raw.CloseFrozenMargin,
            frozen_commission: raw.FrozenCommission,
            commission: raw.Commission,
            frozen_cash: raw.FrozenCash,
            cash_in: raw.CashIn,
            strike_frozen_margin: raw.StrikeFrozenMargin,
        })
    }
}

impl FromSys for crate::InvestorInfoCommRec {
    type Raw = sys::CThostFtdcInvestorInfoCommRecField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            order_count: raw.OrderCount,
            order_action_count: raw.OrderActionCount,
            for_quote_count: raw.ForQuoteCnt,
            info_commission: raw.InfoComm,
            is_opt_series: raw.IsOptSeries != 0,
            product_id: crate::ProductId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            info_count: raw.InfoCnt,
        })
    }
}

impl FromSys for crate::InvestorPortfMarginRatio {
    type Raw = sys::CThostFtdcInvestorPortfMarginRatioField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            margin_ratio: raw.MarginRatio,
            product_group_id: crate::ProductId::from_wire(&raw.ProductGroupID)
                .map_err(|source| ConversionError::new("ProductGroupID", source))?,
        })
    }
}

impl FromSys for crate::InvestorPortfSetting {
    type Raw = sys::CThostFtdcInvestorPortfSettingField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            use_portfolio: raw.UsePortf != 0,
        })
    }
}

impl FromSys for crate::InvestorPosition {
    type Raw = sys::CThostFtdcInvestorPositionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            position_direction: crate::PosiDirection::from_code(raw.PosiDirection as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            position_date: crate::PositionDate::from_code(raw.PositionDate as u8),
            yd_position: raw.YdPosition,
            position: raw.Position,
            long_frozen: raw.LongFrozen,
            short_frozen: raw.ShortFrozen,
            long_frozen_amount: raw.LongFrozenAmount,
            short_frozen_amount: raw.ShortFrozenAmount,
            open_volume: raw.OpenVolume,
            close_volume: raw.CloseVolume,
            open_amount: raw.OpenAmount,
            close_amount: raw.CloseAmount,
            position_cost: raw.PositionCost,
            pre_margin: raw.PreMargin,
            use_margin: raw.UseMargin,
            frozen_margin: raw.FrozenMargin,
            frozen_cash: raw.FrozenCash,
            frozen_commission: raw.FrozenCommission,
            cash_in: raw.CashIn,
            commission: raw.Commission,
            close_profit: raw.CloseProfit,
            position_profit: raw.PositionProfit,
            pre_settlement_price: raw.PreSettlementPrice,
            settlement_price: raw.SettlementPrice,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            open_cost: raw.OpenCost,
            exchange_margin: raw.ExchangeMargin,
            comb_position: raw.CombPosition,
            comb_long_frozen: raw.CombLongFrozen,
            comb_short_frozen: raw.CombShortFrozen,
            close_profit_by_date: raw.CloseProfitByDate,
            close_profit_by_trade: raw.CloseProfitByTrade,
            today_position: raw.TodayPosition,
            margin_rate_by_money: raw.MarginRateByMoney,
            margin_rate_by_volume: raw.MarginRateByVolume,
            strike_frozen: raw.StrikeFrozen,
            strike_frozen_amount: raw.StrikeFrozenAmount,
            abandon_frozen: raw.AbandonFrozen,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            yd_strike_frozen: raw.YdStrikeFrozen,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            position_cost_offset: raw.PositionCostOffset,
            tas_position: raw.TasPosition,
            tas_position_cost: raw.TasPositionCost,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            option_value: raw.OptionValue,
        })
    }
}

impl FromSys for crate::InvestorPositionCombineDetail {
    type Raw = sys::CThostFtdcInvestorPositionCombineDetailField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            open_date: crate::DateText::from_wire(&raw.OpenDate)
                .map_err(|source| ConversionError::new("OpenDate", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            settlement_id: raw.SettlementID,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            com_trade_id: crate::TradeId::from_wire(&raw.ComTradeID)
                .map_err(|source| ConversionError::new("ComTradeID", source))?,
            trade_id: crate::TradeId::from_wire(&raw.TradeID)
                .map_err(|source| ConversionError::new("TradeID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            direction: crate::Direction::from_code(raw.Direction as u8),
            total_amount: raw.TotalAmt,
            margin: raw.Margin,
            exchange_margin: raw.ExchMargin,
            margin_rate_by_money: raw.MarginRateByMoney,
            margin_rate_by_volume: raw.MarginRateByVolume,
            leg_id: raw.LegID,
            leg_multiple: raw.LegMultiple,
            trade_group_id: raw.TradeGroupID,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            comb_instrument_id: crate::InstrumentId::from_wire(&raw.CombInstrumentID)
                .map_err(|source| ConversionError::new("CombInstrumentID", source))?,
        })
    }
}

impl FromSys for crate::InvestorPositionDetail {
    type Raw = sys::CThostFtdcInvestorPositionDetailField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            direction: crate::Direction::from_code(raw.Direction as u8),
            open_date: crate::DateText::from_wire(&raw.OpenDate)
                .map_err(|source| ConversionError::new("OpenDate", source))?,
            trade_id: crate::TradeId::from_wire(&raw.TradeID)
                .map_err(|source| ConversionError::new("TradeID", source))?,
            volume: raw.Volume,
            open_price: raw.OpenPrice,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            trade_type: crate::TradeType::from_code(raw.TradeType as u8),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            close_profit_by_date: raw.CloseProfitByDate,
            close_profit_by_trade: raw.CloseProfitByTrade,
            position_profit_by_date: raw.PositionProfitByDate,
            position_profit_by_trade: raw.PositionProfitByTrade,
            margin: raw.Margin,
            exchange_margin: raw.ExchMargin,
            margin_rate_by_money: raw.MarginRateByMoney,
            margin_rate_by_volume: raw.MarginRateByVolume,
            last_settlement_price: raw.LastSettlementPrice,
            settlement_price: raw.SettlementPrice,
            close_volume: raw.CloseVolume,
            close_amount: raw.CloseAmount,
            time_first_volume: raw.TimeFirstVolume,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            spec_position_type: crate::SpecPosiType::from_code(raw.SpecPosiType as u8),
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            comb_instrument_id: crate::InstrumentId::from_wire(&raw.CombInstrumentID)
                .map_err(|source| ConversionError::new("CombInstrumentID", source))?,
        })
    }
}

impl FromSys for crate::InvestorProdRCAMSMargin {
    type Raw = sys::CThostFtdcInvestorProdRCAMSMarginField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            comb_product_id: crate::ProductId::from_wire(&raw.CombProductID)
                .map_err(|source| ConversionError::new("CombProductID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            product_group_id: crate::ProductId::from_wire(&raw.ProductGroupID)
                .map_err(|source| ConversionError::new("ProductGroupID", source))?,
            risk_before_discount: raw.RiskBeforeDiscount,
            intra_instrument_risk: raw.IntraInstrRisk,
            b_pos_risk: raw.BPosRisk,
            spos_risk: raw.SPosRisk,
            intra_product_risk: raw.IntraProdRisk,
            net_risk: raw.NetRisk,
            inter_product_risk: raw.InterProdRisk,
            short_opt_risk_adj: raw.ShortOptRiskAdj,
            option_royalty: raw.OptionRoyalty,
            mmsa_close_frozen_margin: raw.MMSACloseFrozenMargin,
            close_comb_frozen_margin: raw.CloseCombFrozenMargin,
            close_frozen_margin: raw.CloseFrozenMargin,
            mmsa_open_frozen_margin: raw.MMSAOpenFrozenMargin,
            delivery_open_frozen_margin: raw.DeliveryOpenFrozenMargin,
            open_frozen_margin: raw.OpenFrozenMargin,
            use_frozen_margin: raw.UseFrozenMargin,
            mmsa_exchange_margin: raw.MMSAExchMargin,
            delivery_exchange_margin: raw.DeliveryExchMargin,
            comb_exchange_margin: raw.CombExchMargin,
            exchange_margin: raw.ExchMargin,
            use_margin: raw.UseMargin,
        })
    }
}

impl FromSys for crate::InvestorProdRULEMargin {
    type Raw = sys::CThostFtdcInvestorProdRULEMarginField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            instrument_class: crate::InstrumentClass::from_code(raw.InstrumentClass as u8),
            commodity_group_id: raw.CommodityGroupID,
            b_std_position: raw.BStdPosition,
            s_std_position: raw.SStdPosition,
            b_std_open_frozen: raw.BStdOpenFrozen,
            s_std_open_frozen: raw.SStdOpenFrozen,
            b_std_close_frozen: raw.BStdCloseFrozen,
            s_std_close_frozen: raw.SStdCloseFrozen,
            intra_product_std_position: raw.IntraProdStdPosition,
            net_std_position: raw.NetStdPosition,
            inter_product_std_position: raw.InterProdStdPosition,
            single_std_position: raw.SingleStdPosition,
            intra_product_margin: raw.IntraProdMargin,
            inter_product_margin: raw.InterProdMargin,
            single_margin: raw.SingleMargin,
            non_comb_margin: raw.NonCombMargin,
            add_on_margin: raw.AddOnMargin,
            exchange_margin: raw.ExchMargin,
            add_on_frozen_margin: raw.AddOnFrozenMargin,
            open_frozen_margin: raw.OpenFrozenMargin,
            close_frozen_margin: raw.CloseFrozenMargin,
            margin: raw.Margin,
            frozen_margin: raw.FrozenMargin,
        })
    }
}

impl FromSys for crate::InvestorProdSPBMDetail {
    type Raw = sys::CThostFtdcInvestorProdSPBMDetailField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            intra_instrument_margin: raw.IntraInstrMargin,
            b_collecting_margin: raw.BCollectingMargin,
            s_collecting_margin: raw.SCollectingMargin,
            intra_product_margin: raw.IntraProdMargin,
            net_margin: raw.NetMargin,
            inter_product_margin: raw.InterProdMargin,
            single_margin: raw.SingleMargin,
            add_on_margin: raw.AddOnMargin,
            delivery_margin: raw.DeliveryMargin,
            call_option_min_risk: raw.CallOptionMinRisk,
            put_option_min_risk: raw.PutOptionMinRisk,
            option_min_risk: raw.OptionMinRisk,
            option_value_offset: raw.OptionValueOffset,
            option_royalty: raw.OptionRoyalty,
            real_option_value_offset: raw.RealOptionValueOffset,
            margin: raw.Margin,
            exchange_margin: raw.ExchMargin,
        })
    }
}

impl FromSys for crate::InvestorProductGroupMargin {
    type Raw = sys::CThostFtdcInvestorProductGroupMarginField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            frozen_margin: raw.FrozenMargin,
            long_frozen_margin: raw.LongFrozenMargin,
            short_frozen_margin: raw.ShortFrozenMargin,
            use_margin: raw.UseMargin,
            long_use_margin: raw.LongUseMargin,
            short_use_margin: raw.ShortUseMargin,
            exchange_margin: raw.ExchMargin,
            long_exchange_margin: raw.LongExchMargin,
            short_exchange_margin: raw.ShortExchMargin,
            close_profit: raw.CloseProfit,
            frozen_commission: raw.FrozenCommission,
            commission: raw.Commission,
            frozen_cash: raw.FrozenCash,
            cash_in: raw.CashIn,
            position_profit: raw.PositionProfit,
            offset_amount: raw.OffsetAmount,
            long_offset_amount: raw.LongOffsetAmount,
            short_offset_amount: raw.ShortOffsetAmount,
            exchange_offset_amount: raw.ExchOffsetAmount,
            long_exchange_offset_amount: raw.LongExchOffsetAmount,
            short_exchange_offset_amount: raw.ShortExchOffsetAmount,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            product_group_id: crate::InstrumentId::from_wire(&raw.ProductGroupID)
                .map_err(|source| ConversionError::new("ProductGroupID", source))?,
        })
    }
}

impl FromSys for crate::MMInstrumentCommissionRate {
    type Raw = sys::CThostFtdcMMInstrumentCommissionRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            open_ratio_by_money: raw.OpenRatioByMoney,
            open_ratio_by_volume: raw.OpenRatioByVolume,
            close_ratio_by_money: raw.CloseRatioByMoney,
            close_ratio_by_volume: raw.CloseRatioByVolume,
            close_today_ratio_by_money: raw.CloseTodayRatioByMoney,
            close_today_ratio_by_volume: raw.CloseTodayRatioByVolume,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::MMOptionInstrCommRate {
    type Raw = sys::CThostFtdcMMOptionInstrCommRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            open_ratio_by_money: raw.OpenRatioByMoney,
            open_ratio_by_volume: raw.OpenRatioByVolume,
            close_ratio_by_money: raw.CloseRatioByMoney,
            close_ratio_by_volume: raw.CloseRatioByVolume,
            close_today_ratio_by_money: raw.CloseTodayRatioByMoney,
            close_today_ratio_by_volume: raw.CloseTodayRatioByVolume,
            strike_ratio_by_money: raw.StrikeRatioByMoney,
            strike_ratio_by_volume: raw.StrikeRatioByVolume,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::MulticastInstrument {
    type Raw = sys::CThostFtdcMulticastInstrumentField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            topic_id: raw.TopicID,
            instrument_no: raw.InstrumentNo,
            code_price: raw.CodePrice,
            volume_multiple: raw.VolumeMultiple,
            price_tick: raw.PriceTick,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::Notice {
    type Raw = sys::CThostFtdcNoticeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            content: string_from_wire(&raw.Content, "Content")?,
            sequence_label: string_from_wire(&raw.SequenceLabel, "SequenceLabel")?,
        })
    }
}

impl FromSys for crate::NotifyQueryAccount {
    type Raw = sys::CThostFtdcNotifyQueryAccountField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            future_serial: raw.FutureSerial,
            install_id: raw.InstallID,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            request_id: crate::RequestId::new(raw.RequestID),
            tid: raw.TID,
            bank_use_amount: raw.BankUseAmount,
            bank_fetch_amount: raw.BankFetchAmount,
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::OffsetSetting {
    type Raw = sys::CThostFtdcOffsetSettingField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            underlying_instrument_id: crate::InstrumentId::from_wire(&raw.UnderlyingInstrID)
                .map_err(|source| ConversionError::new("UnderlyingInstrID", source))?,
            product_id: crate::ProductId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            offset_type: crate::OffsetType::from_code(raw.OffsetType as u8),
            volume: raw.Volume,
            is_offset: raw.IsOffset != 0,
            request_id: crate::RequestId::new(raw.RequestID),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            exchange_serial_no: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeSerialNo)
                .map_err(|source| ConversionError::new("ExchangeSerialNo", source))?,
            exchange_product_id: crate::ProductId::from_wire(&raw.ExchangeProductID)
                .map_err(|source| ConversionError::new("ExchangeProductID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            exec_result: crate::ExecResult::from_code(raw.ExecResult as u8),
            sequence_no: raw.SequenceNo,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_offset_setting_seq: raw.BrokerOffsetSettingSeq,
            apply_src: crate::ApplySrc::from_code(raw.ApplySrc as u8),
        })
    }
}

impl FromSys for crate::OpenAccount {
    type Raw = sys::CThostFtdcOpenAccountField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            gender: crate::Gender::from_code(raw.Gender as u8),
            country_code: string_from_wire(&raw.CountryCode, "CountryCode")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            address: string_from_wire(&raw.Address, "Address")?,
            zip_code: string_from_wire(&raw.ZipCode, "ZipCode")?,
            telephone: string_from_wire(&raw.Telephone, "Telephone")?,
            mobile_phone: string_from_wire(&raw.MobilePhone, "MobilePhone")?,
            fax: string_from_wire(&raw.Fax, "Fax")?,
            email: string_from_wire(&raw.EMail, "EMail")?,
            money_account_status: crate::MoneyAccountStatus::from_code(
                raw.MoneyAccountStatus as u8,
            ),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            cash_exchange_code: crate::CashExchangeCode::from_code(raw.CashExchangeCode as u8),
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            tid: raw.TID,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::OptionInstrCommRate {
    type Raw = sys::CThostFtdcOptionInstrCommRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            open_ratio_by_money: raw.OpenRatioByMoney,
            open_ratio_by_volume: raw.OpenRatioByVolume,
            close_ratio_by_money: raw.CloseRatioByMoney,
            close_ratio_by_volume: raw.CloseRatioByVolume,
            close_today_ratio_by_money: raw.CloseTodayRatioByMoney,
            close_today_ratio_by_volume: raw.CloseTodayRatioByVolume,
            strike_ratio_by_money: raw.StrikeRatioByMoney,
            strike_ratio_by_volume: raw.StrikeRatioByVolume,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::OptionInstrTradeCost {
    type Raw = sys::CThostFtdcOptionInstrTradeCostField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            fixed_margin: raw.FixedMargin,
            mini_margin: raw.MiniMargin,
            royalty: raw.Royalty,
            exchange_fixed_margin: raw.ExchFixedMargin,
            exchange_mini_margin: raw.ExchMiniMargin,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::OptionSelfClose {
    type Raw = sys::CThostFtdcOptionSelfCloseField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            option_self_close_ref: crate::OrderRef::from_wire(&raw.OptionSelfCloseRef)
                .map_err(|source| ConversionError::new("OptionSelfCloseRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            volume: raw.Volume,
            request_id: crate::RequestId::new(raw.RequestID),
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            opt_self_close_flag: crate::OptSelfCloseFlag::from_code(raw.OptSelfCloseFlag as u8),
            option_self_close_local_id: crate::OrderLocalId::from_wire(&raw.OptionSelfCloseLocalID)
                .map_err(|source| ConversionError::new("OptionSelfCloseLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            option_self_close_system_id: crate::OrderSysId::from_wire(&raw.OptionSelfCloseSysID)
                .map_err(|source| ConversionError::new("OptionSelfCloseSysID", source))?,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            exec_result: crate::ExecResult::from_code(raw.ExecResult as u8),
            clearing_part_id: string_from_wire(&raw.ClearingPartID, "ClearingPartID")?,
            sequence_no: raw.SequenceNo,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_option_self_close_seq: raw.BrokerOptionSelfCloseSeq,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::OptionSelfCloseAction {
    type Raw = sys::CThostFtdcOptionSelfCloseActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            option_self_close_action_ref: raw.OptionSelfCloseActionRef,
            option_self_close_ref: crate::OrderRef::from_wire(&raw.OptionSelfCloseRef)
                .map_err(|source| ConversionError::new("OptionSelfCloseRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            option_self_close_system_id: crate::OrderSysId::from_wire(&raw.OptionSelfCloseSysID)
                .map_err(|source| ConversionError::new("OptionSelfCloseSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            option_self_close_local_id: crate::OrderLocalId::from_wire(&raw.OptionSelfCloseLocalID)
                .map_err(|source| ConversionError::new("OptionSelfCloseLocalID", source))?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::Order {
    type Raw = sys::CThostFtdcOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            order_price_type: crate::OrderPriceType::from_code(raw.OrderPriceType as u8),
            direction: crate::Direction::from_code(raw.Direction as u8),
            comb_offset_flag: crate::CombOffsetFlag::from_wire(std::array::from_fn(|index| {
                raw.CombOffsetFlag[index] as u8
            })),
            comb_hedge_flag: crate::CombHedgeFlag::from_wire(std::array::from_fn(|index| {
                raw.CombHedgeFlag[index] as u8
            })),
            limit_price: raw.LimitPrice,
            volume_total_original: raw.VolumeTotalOriginal,
            time_condition: crate::TimeCondition::from_code(raw.TimeCondition as u8),
            gtd_date: crate::DateText::from_wire(&raw.GTDDate)
                .map_err(|source| ConversionError::new("GTDDate", source))?,
            volume_condition: crate::VolumeCondition::from_code(raw.VolumeCondition as u8),
            min_volume: raw.MinVolume,
            contingent_condition: crate::ContingentCondition::from_code(
                raw.ContingentCondition as u8,
            ),
            stop_price: raw.StopPrice,
            force_close_reason: crate::ForceCloseReason::from_code(raw.ForceCloseReason as u8),
            is_auto_suspend: raw.IsAutoSuspend != 0,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            order_source: crate::OrderSource::from_code(raw.OrderSource as u8),
            order_status: crate::OrderStatus::from_code(raw.OrderStatus as u8),
            order_type: crate::OrderType::from_code(raw.OrderType as u8),
            volume_traded: raw.VolumeTraded,
            volume_total: raw.VolumeTotal,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            active_time: crate::TimeText::from_wire(&raw.ActiveTime)
                .map_err(|source| ConversionError::new("ActiveTime", source))?,
            suspend_time: crate::TimeText::from_wire(&raw.SuspendTime)
                .map_err(|source| ConversionError::new("SuspendTime", source))?,
            update_time: crate::TimeText::from_wire(&raw.UpdateTime)
                .map_err(|source| ConversionError::new("UpdateTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            active_trader_id: string_from_wire(&raw.ActiveTraderID, "ActiveTraderID")?,
            clearing_part_id: string_from_wire(&raw.ClearingPartID, "ClearingPartID")?,
            sequence_no: raw.SequenceNo,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            user_force_close: raw.UserForceClose != 0,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_order_seq: raw.BrokerOrderSeq,
            relative_order_system_id: crate::OrderSysId::from_wire(&raw.RelativeOrderSysID)
                .map_err(|source| ConversionError::new("RelativeOrderSysID", source))?,
            zce_total_traded_volume: raw.ZCETotalTradedVolume,
            is_swap_order: raw.IsSwapOrder != 0,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::OrderAction {
    type Raw = sys::CThostFtdcOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_action_ref: raw.OrderActionRef,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            limit_price: raw.LimitPrice,
            volume_change: raw.VolumeChange,
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::ParkedOrder {
    type Raw = sys::CThostFtdcParkedOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            order_price_type: crate::OrderPriceType::from_code(raw.OrderPriceType as u8),
            direction: crate::Direction::from_code(raw.Direction as u8),
            comb_offset_flag: crate::CombOffsetFlag::from_wire(std::array::from_fn(|index| {
                raw.CombOffsetFlag[index] as u8
            })),
            comb_hedge_flag: crate::CombHedgeFlag::from_wire(std::array::from_fn(|index| {
                raw.CombHedgeFlag[index] as u8
            })),
            limit_price: raw.LimitPrice,
            volume_total_original: raw.VolumeTotalOriginal,
            time_condition: crate::TimeCondition::from_code(raw.TimeCondition as u8),
            gtd_date: crate::DateText::from_wire(&raw.GTDDate)
                .map_err(|source| ConversionError::new("GTDDate", source))?,
            volume_condition: crate::VolumeCondition::from_code(raw.VolumeCondition as u8),
            min_volume: raw.MinVolume,
            contingent_condition: crate::ContingentCondition::from_code(
                raw.ContingentCondition as u8,
            ),
            stop_price: raw.StopPrice,
            force_close_reason: crate::ForceCloseReason::from_code(raw.ForceCloseReason as u8),
            is_auto_suspend: raw.IsAutoSuspend != 0,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            user_force_close: raw.UserForceClose != 0,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            parked_order_id: string_from_wire(&raw.ParkedOrderID, "ParkedOrderID")?,
            user_type: crate::UserType::from_code(raw.UserType as u8),
            status: crate::ParkedOrderStatus::from_code(raw.Status as u8),
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            is_swap_order: raw.IsSwapOrder != 0,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::ParkedOrderAction {
    type Raw = sys::CThostFtdcParkedOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_action_ref: raw.OrderActionRef,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            limit_price: raw.LimitPrice,
            volume_change: raw.VolumeChange,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            parked_order_action_id: string_from_wire(
                &raw.ParkedOrderActionID,
                "ParkedOrderActionID",
            )?,
            user_type: crate::UserType::from_code(raw.UserType as u8),
            status: crate::ParkedOrderStatus::from_code(raw.Status as u8),
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl FromSys for crate::Product {
    type Raw = sys::CThostFtdcProductField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            product_name: string_from_wire(&raw.ProductName, "ProductName")?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_class: crate::ProductClass::from_code(raw.ProductClass as u8),
            volume_multiple: raw.VolumeMultiple,
            price_tick: raw.PriceTick,
            max_market_order_volume: raw.MaxMarketOrderVolume,
            min_market_order_volume: raw.MinMarketOrderVolume,
            max_limit_order_volume: raw.MaxLimitOrderVolume,
            min_limit_order_volume: raw.MinLimitOrderVolume,
            position_type: crate::PositionType::from_code(raw.PositionType as u8),
            position_date_type: crate::PositionDateType::from_code(raw.PositionDateType as u8),
            close_deal_type: crate::CloseDealType::from_code(raw.CloseDealType as u8),
            trade_currency_id: string_from_wire(&raw.TradeCurrencyID, "TradeCurrencyID")?,
            mortgage_fund_use_range: crate::MortgageFundUseRange::from_code(
                raw.MortgageFundUseRange as u8,
            ),
            underlying_multiple: raw.UnderlyingMultiple,
            product_id: crate::InstrumentId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            exchange_product_id: crate::InstrumentId::from_wire(&raw.ExchangeProductID)
                .map_err(|source| ConversionError::new("ExchangeProductID", source))?,
            open_limit_control_level: crate::OpenLimitControlLevel::from_code(
                raw.OpenLimitControlLevel as u8,
            ),
            order_frequency_control_level: crate::OrderFreqControlLevel::from_code(
                raw.OrderFreqControlLevel as u8,
            ),
        })
    }
}

impl FromSys for crate::ProductExchRate {
    type Raw = sys::CThostFtdcProductExchRateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            quote_currency_id: string_from_wire(&raw.QuoteCurrencyID, "QuoteCurrencyID")?,
            exchange_rate: raw.ExchangeRate,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_id: crate::InstrumentId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
        })
    }
}

impl FromSys for crate::ProductGroup {
    type Raw = sys::CThostFtdcProductGroupField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_id: crate::InstrumentId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            product_group_id: crate::InstrumentId::from_wire(&raw.ProductGroupID)
                .map_err(|source| ConversionError::new("ProductGroupID", source))?,
        })
    }
}

impl FromSys for crate::QryMaxOrderVolume {
    type Raw = sys::CThostFtdcQryMaxOrderVolumeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            direction: crate::Direction::from_code(raw.Direction as u8),
            offset_flag: crate::OffsetFlag::from_code(raw.OffsetFlag as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            max_volume: raw.MaxVolume,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::QueryCFMMCTradingAccountToken {
    type Raw = sys::CThostFtdcQueryCFMMCTradingAccountTokenField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
        })
    }
}

impl FromSys for crate::Quote {
    type Raw = sys::CThostFtdcQuoteField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            quote_ref: crate::OrderRef::from_wire(&raw.QuoteRef)
                .map_err(|source| ConversionError::new("QuoteRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            ask_price: raw.AskPrice,
            bid_price: raw.BidPrice,
            ask_volume: raw.AskVolume,
            bid_volume: raw.BidVolume,
            request_id: crate::RequestId::new(raw.RequestID),
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            ask_offset_flag: crate::OffsetFlag::from_code(raw.AskOffsetFlag as u8),
            bid_offset_flag: crate::OffsetFlag::from_code(raw.BidOffsetFlag as u8),
            ask_hedge_flag: crate::HedgeFlag::from_code(raw.AskHedgeFlag as u8),
            bid_hedge_flag: crate::HedgeFlag::from_code(raw.BidHedgeFlag as u8),
            quote_local_id: crate::OrderLocalId::from_wire(&raw.QuoteLocalID)
                .map_err(|source| ConversionError::new("QuoteLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            notify_sequence: raw.NotifySequence,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            quote_system_id: crate::OrderSysId::from_wire(&raw.QuoteSysID)
                .map_err(|source| ConversionError::new("QuoteSysID", source))?,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            quote_status: crate::OrderStatus::from_code(raw.QuoteStatus as u8),
            clearing_part_id: string_from_wire(&raw.ClearingPartID, "ClearingPartID")?,
            sequence_no: raw.SequenceNo,
            ask_order_system_id: crate::OrderSysId::from_wire(&raw.AskOrderSysID)
                .map_err(|source| ConversionError::new("AskOrderSysID", source))?,
            bid_order_system_id: crate::OrderSysId::from_wire(&raw.BidOrderSysID)
                .map_err(|source| ConversionError::new("BidOrderSysID", source))?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_quote_seq: raw.BrokerQuoteSeq,
            ask_order_ref: crate::OrderRef::from_wire(&raw.AskOrderRef)
                .map_err(|source| ConversionError::new("AskOrderRef", source))?,
            bid_order_ref: crate::OrderRef::from_wire(&raw.BidOrderRef)
                .map_err(|source| ConversionError::new("BidOrderRef", source))?,
            for_quote_system_id: crate::OrderSysId::from_wire(&raw.ForQuoteSysID)
                .map_err(|source| ConversionError::new("ForQuoteSysID", source))?,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            replace_system_id: crate::OrderSysId::from_wire(&raw.ReplaceSysID)
                .map_err(|source| ConversionError::new("ReplaceSysID", source))?,
            time_condition: crate::TimeCondition::from_code(raw.TimeCondition as u8),
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::QuoteAction {
    type Raw = sys::CThostFtdcQuoteActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            quote_action_ref: raw.QuoteActionRef,
            quote_ref: crate::OrderRef::from_wire(&raw.QuoteRef)
                .map_err(|source| ConversionError::new("QuoteRef", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            quote_system_id: crate::OrderSysId::from_wire(&raw.QuoteSysID)
                .map_err(|source| ConversionError::new("QuoteSysID", source))?,
            action_flag: crate::ActionFlag::from_code(raw.ActionFlag as u8),
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            quote_local_id: crate::OrderLocalId::from_wire(&raw.QuoteLocalID)
                .map_err(|source| ConversionError::new("QuoteLocalID", source))?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            order_memo: string_from_wire(&raw.OrderMemo, "OrderMemo")?,
            session_req_seq: raw.SessionReqSeq,
        })
    }
}

impl FromSys for crate::RCAMSCombProductInfo {
    type Raw = sys::CThostFtdcRCAMSCombProductInfoField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_id: crate::ProductId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            comb_product_id: crate::ProductId::from_wire(&raw.CombProductID)
                .map_err(|source| ConversionError::new("CombProductID", source))?,
            product_group_id: crate::ProductId::from_wire(&raw.ProductGroupID)
                .map_err(|source| ConversionError::new("ProductGroupID", source))?,
        })
    }
}

impl FromSys for crate::RCAMSInstrParameter {
    type Raw = sys::CThostFtdcRCAMSInstrParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_id: crate::ProductId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            hedge_rate: raw.HedgeRate,
        })
    }
}

impl FromSys for crate::RCAMSInterParameter {
    type Raw = sys::CThostFtdcRCAMSInterParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_group_id: crate::ProductId::from_wire(&raw.ProductGroupID)
                .map_err(|source| ConversionError::new("ProductGroupID", source))?,
            priority: raw.Priority,
            credit_rate: raw.CreditRate,
            comb_product1: crate::ProductId::from_wire(&raw.CombProduct1)
                .map_err(|source| ConversionError::new("CombProduct1", source))?,
            comb_product2: crate::ProductId::from_wire(&raw.CombProduct2)
                .map_err(|source| ConversionError::new("CombProduct2", source))?,
        })
    }
}

impl FromSys for crate::RCAMSIntraParameter {
    type Raw = sys::CThostFtdcRCAMSIntraParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            comb_product_id: crate::ProductId::from_wire(&raw.CombProductID)
                .map_err(|source| ConversionError::new("CombProductID", source))?,
            hedge_rate: raw.HedgeRate,
        })
    }
}

impl FromSys for crate::RCAMSInvestorCombPosition {
    type Raw = sys::CThostFtdcRCAMSInvestorCombPositionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            position_direction: crate::PosiDirection::from_code(raw.PosiDirection as u8),
            comb_instrument_id: crate::InstrumentId::from_wire(&raw.CombInstrumentID)
                .map_err(|source| ConversionError::new("CombInstrumentID", source))?,
            leg_id: raw.LegID,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            total_amount: raw.TotalAmt,
            exchange_margin: raw.ExchMargin,
            margin: raw.Margin,
        })
    }
}

impl FromSys for crate::RCAMSShortOptAdjustParam {
    type Raw = sys::CThostFtdcRCAMSShortOptAdjustParamField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            comb_product_id: crate::ProductId::from_wire(&raw.CombProductID)
                .map_err(|source| ConversionError::new("CombProductID", source))?,
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            adjust_value: raw.AdjustValue,
        })
    }
}

impl FromSys for crate::RULEInstrParameter {
    type Raw = sys::CThostFtdcRULEInstrParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            instrument_class: crate::InstrumentClass::from_code(raw.InstrumentClass as u8),
            std_instrument_id: crate::InstrumentId::from_wire(&raw.StdInstrumentID)
                .map_err(|source| ConversionError::new("StdInstrumentID", source))?,
            b_spec_ratio: raw.BSpecRatio,
            s_spec_ratio: raw.SSpecRatio,
            b_hedge_ratio: raw.BHedgeRatio,
            s_hedge_ratio: raw.SHedgeRatio,
            b_add_on_margin: raw.BAddOnMargin,
            s_add_on_margin: raw.SAddOnMargin,
            commodity_group_id: raw.CommodityGroupID,
        })
    }
}

impl FromSys for crate::RULEInterParameter {
    type Raw = sys::CThostFtdcRULEInterParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            spread_id: raw.SpreadId,
            inter_rate: raw.InterRate,
            leg1_product_family_code: crate::InstrumentId::from_wire(&raw.Leg1ProdFamilyCode)
                .map_err(|source| ConversionError::new("Leg1ProdFamilyCode", source))?,
            leg2_product_family_code: crate::InstrumentId::from_wire(&raw.Leg2ProdFamilyCode)
                .map_err(|source| ConversionError::new("Leg2ProdFamilyCode", source))?,
            leg1_prop_factor: raw.Leg1PropFactor,
            leg2_prop_factor: raw.Leg2PropFactor,
            commodity_group_id: raw.CommodityGroupID,
            commodity_group_name: string_from_wire(&raw.CommodityGroupName, "CommodityGroupName")?,
        })
    }
}

impl FromSys for crate::RULEIntraParameter {
    type Raw = sys::CThostFtdcRULEIntraParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            std_instrument_id: crate::InstrumentId::from_wire(&raw.StdInstrumentID)
                .map_err(|source| ConversionError::new("StdInstrumentID", source))?,
            std_instrument_margin: raw.StdInstrMargin,
            usual_intra_rate: raw.UsualIntraRate,
            delivery_intra_rate: raw.DeliveryIntraRate,
        })
    }
}

impl FromSys for crate::RemoveParkedOrder {
    type Raw = sys::CThostFtdcRemoveParkedOrderField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            parked_order_id: string_from_wire(&raw.ParkedOrderID, "ParkedOrderID")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
        })
    }
}

impl FromSys for crate::RemoveParkedOrderAction {
    type Raw = sys::CThostFtdcRemoveParkedOrderActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            parked_order_action_id: string_from_wire(
                &raw.ParkedOrderActionID,
                "ParkedOrderActionID",
            )?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
        })
    }
}

impl FromSys for crate::ReqQueryAccount {
    type Raw = sys::CThostFtdcReqQueryAccountField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            future_serial: raw.FutureSerial,
            install_id: raw.InstallID,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            request_id: crate::RequestId::new(raw.RequestID),
            tid: raw.TID,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::ReqRepeal {
    type Raw = sys::CThostFtdcReqRepealField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            repeal_time_interval: raw.RepealTimeInterval,
            repealed_times: raw.RepealedTimes,
            bank_repeal_flag: crate::BankRepealFlag::from_code(raw.BankRepealFlag as u8),
            broker_repeal_flag: crate::BrokerRepealFlag::from_code(raw.BrokerRepealFlag as u8),
            plate_repeal_serial: raw.PlateRepealSerial,
            bank_repeal_serial: string_from_wire(&raw.BankRepealSerial, "BankRepealSerial")?,
            future_repeal_serial: raw.FutureRepealSerial,
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            future_serial: raw.FutureSerial,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            trade_amount: raw.TradeAmount,
            future_fetch_amount: raw.FutureFetchAmount,
            fee_pay_flag: crate::FeePayFlag::from_code(raw.FeePayFlag as u8),
            cust_fee: raw.CustFee,
            broker_fee: raw.BrokerFee,
            message: string_from_wire(&raw.Message, "Message")?,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            request_id: crate::RequestId::new(raw.RequestID),
            tid: raw.TID,
            transfer_status: crate::TransferStatus::from_code(raw.TransferStatus as u8),
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::ReqTransfer {
    type Raw = sys::CThostFtdcReqTransferField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            future_serial: raw.FutureSerial,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            trade_amount: raw.TradeAmount,
            future_fetch_amount: raw.FutureFetchAmount,
            fee_pay_flag: crate::FeePayFlag::from_code(raw.FeePayFlag as u8),
            cust_fee: raw.CustFee,
            broker_fee: raw.BrokerFee,
            message: string_from_wire(&raw.Message, "Message")?,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            request_id: crate::RequestId::new(raw.RequestID),
            tid: raw.TID,
            transfer_status: crate::TransferStatus::from_code(raw.TransferStatus as u8),
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::RiskSettleInvestPosition {
    type Raw = sys::CThostFtdcRiskSettleInvstPositionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            position_direction: crate::PosiDirection::from_code(raw.PosiDirection as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            position_date: crate::PositionDate::from_code(raw.PositionDate as u8),
            yd_position: raw.YdPosition,
            position: raw.Position,
            long_frozen: raw.LongFrozen,
            short_frozen: raw.ShortFrozen,
            long_frozen_amount: raw.LongFrozenAmount,
            short_frozen_amount: raw.ShortFrozenAmount,
            open_volume: raw.OpenVolume,
            close_volume: raw.CloseVolume,
            open_amount: raw.OpenAmount,
            close_amount: raw.CloseAmount,
            position_cost: raw.PositionCost,
            pre_margin: raw.PreMargin,
            use_margin: raw.UseMargin,
            frozen_margin: raw.FrozenMargin,
            frozen_cash: raw.FrozenCash,
            frozen_commission: raw.FrozenCommission,
            cash_in: raw.CashIn,
            commission: raw.Commission,
            close_profit: raw.CloseProfit,
            position_profit: raw.PositionProfit,
            pre_settlement_price: raw.PreSettlementPrice,
            settlement_price: raw.SettlementPrice,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            open_cost: raw.OpenCost,
            exchange_margin: raw.ExchangeMargin,
            comb_position: raw.CombPosition,
            comb_long_frozen: raw.CombLongFrozen,
            comb_short_frozen: raw.CombShortFrozen,
            close_profit_by_date: raw.CloseProfitByDate,
            close_profit_by_trade: raw.CloseProfitByTrade,
            today_position: raw.TodayPosition,
            margin_rate_by_money: raw.MarginRateByMoney,
            margin_rate_by_volume: raw.MarginRateByVolume,
            strike_frozen: raw.StrikeFrozen,
            strike_frozen_amount: raw.StrikeFrozenAmount,
            abandon_frozen: raw.AbandonFrozen,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            yd_strike_frozen: raw.YdStrikeFrozen,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            position_cost_offset: raw.PositionCostOffset,
            tas_position: raw.TasPosition,
            tas_position_cost: raw.TasPositionCost,
        })
    }
}

impl FromSys for crate::RiskSettleProductStatus {
    type Raw = sys::CThostFtdcRiskSettleProductStatusField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_id: crate::InstrumentId::from_wire(&raw.ProductID)
                .map_err(|source| ConversionError::new("ProductID", source))?,
            product_status: crate::ProductStatus::from_code(raw.ProductStatus as u8),
        })
    }
}

impl FromSys for crate::RspAuthenticate {
    type Raw = sys::CThostFtdcRspAuthenticateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            app_id: string_from_wire(&raw.AppID, "AppID")?,
            app_type: crate::AppType::from_code(raw.AppType as u8),
        })
    }
}

impl FromSys for crate::RspGenSMSCode {
    type Raw = sys::CThostFtdcRspGenSMSCodeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            gen_time: crate::TimeText::from_wire(&raw.GenTime)
                .map_err(|source| ConversionError::new("GenTime", source))?,
        })
    }
}

impl FromSys for crate::RspGenUserCaptcha {
    type Raw = sys::CThostFtdcRspGenUserCaptchaField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            captcha_info_len: raw.CaptchaInfoLen,
            captcha_info: string_from_wire(&raw.CaptchaInfo, "CaptchaInfo")?,
        })
    }
}

impl FromSys for crate::RspGenUserText {
    type Raw = sys::CThostFtdcRspGenUserTextField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            user_text_seq: raw.UserTextSeq,
        })
    }
}

impl FromSys for crate::RspInfo {
    type Raw = sys::CThostFtdcRspInfoField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
        })
    }
}

impl FromSys for crate::RspRepeal {
    type Raw = sys::CThostFtdcRspRepealField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            repeal_time_interval: raw.RepealTimeInterval,
            repealed_times: raw.RepealedTimes,
            bank_repeal_flag: crate::BankRepealFlag::from_code(raw.BankRepealFlag as u8),
            broker_repeal_flag: crate::BrokerRepealFlag::from_code(raw.BrokerRepealFlag as u8),
            plate_repeal_serial: raw.PlateRepealSerial,
            bank_repeal_serial: string_from_wire(&raw.BankRepealSerial, "BankRepealSerial")?,
            future_repeal_serial: raw.FutureRepealSerial,
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            future_serial: raw.FutureSerial,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            trade_amount: raw.TradeAmount,
            future_fetch_amount: raw.FutureFetchAmount,
            fee_pay_flag: crate::FeePayFlag::from_code(raw.FeePayFlag as u8),
            cust_fee: raw.CustFee,
            broker_fee: raw.BrokerFee,
            message: string_from_wire(&raw.Message, "Message")?,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            request_id: crate::RequestId::new(raw.RequestID),
            tid: raw.TID,
            transfer_status: crate::TransferStatus::from_code(raw.TransferStatus as u8),
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::RspTransfer {
    type Raw = sys::CThostFtdcRspTransferField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            trading_day: string_from_wire(&raw.TradingDay, "TradingDay")?,
            plate_serial: raw.PlateSerial,
            last_fragment: crate::LastFragment::from_code(raw.LastFragment as u8),
            session_id: crate::SessionId::new(raw.SessionID),
            customer_name: string_from_wire(&raw.CustomerName, "CustomerName")?,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            cust_type: crate::CustType::from_code(raw.CustType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_pass_word: string_from_wire(&raw.BankPassWord, "BankPassWord")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            future_serial: raw.FutureSerial,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            verify_cert_no_flag: crate::YesNoIndicator::from_code(raw.VerifyCertNoFlag as u8),
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            trade_amount: raw.TradeAmount,
            future_fetch_amount: raw.FutureFetchAmount,
            fee_pay_flag: crate::FeePayFlag::from_code(raw.FeePayFlag as u8),
            cust_fee: raw.CustFee,
            broker_fee: raw.BrokerFee,
            message: string_from_wire(&raw.Message, "Message")?,
            digest: string_from_wire(&raw.Digest, "Digest")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            device_id: string_from_wire(&raw.DeviceID, "DeviceID")?,
            bank_secu_acc_type: crate::BankAccType::from_code(raw.BankSecuAccType as u8),
            broker_id_by_bank: string_from_wire(&raw.BrokerIDByBank, "BrokerIDByBank")?,
            bank_secu_acc: string_from_wire(&raw.BankSecuAcc, "BankSecuAcc")?,
            bank_password_flag: crate::PwdFlag::from_code(raw.BankPwdFlag as u8),
            secu_password_flag: crate::PwdFlag::from_code(raw.SecuPwdFlag as u8),
            oper_no: string_from_wire(&raw.OperNo, "OperNo")?,
            request_id: crate::RequestId::new(raw.RequestID),
            tid: raw.TID,
            transfer_status: crate::TransferStatus::from_code(raw.TransferStatus as u8),
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::RspUserAuthMethod {
    type Raw = sys::CThostFtdcRspUserAuthMethodField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            usable_auth_method: raw.UsableAuthMethod,
        })
    }
}

impl FromSys for crate::RspUserLogin {
    type Raw = sys::CThostFtdcRspUserLoginField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            login_time: crate::TimeText::from_wire(&raw.LoginTime)
                .map_err(|source| ConversionError::new("LoginTime", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            system_name: string_from_wire(&raw.SystemName, "SystemName")?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            max_order_ref: crate::OrderRef::from_wire(&raw.MaxOrderRef)
                .map_err(|source| ConversionError::new("MaxOrderRef", source))?,
            shfe_time: crate::TimeText::from_wire(&raw.SHFETime)
                .map_err(|source| ConversionError::new("SHFETime", source))?,
            dce_time: crate::TimeText::from_wire(&raw.DCETime)
                .map_err(|source| ConversionError::new("DCETime", source))?,
            czce_time: crate::TimeText::from_wire(&raw.CZCETime)
                .map_err(|source| ConversionError::new("CZCETime", source))?,
            ffex_time: crate::TimeText::from_wire(&raw.FFEXTime)
                .map_err(|source| ConversionError::new("FFEXTime", source))?,
            ine_time: crate::TimeText::from_wire(&raw.INETime)
                .map_err(|source| ConversionError::new("INETime", source))?,
            system_version: string_from_wire(&raw.SysVersion, "SysVersion")?,
            gfex_time: crate::TimeText::from_wire(&raw.GFEXTime)
                .map_err(|source| ConversionError::new("GFEXTime", source))?,
            login_dr_identity_id: raw.LoginDRIdentityID,
            user_dr_identity_id: raw.UserDRIdentityID,
            last_login_time: string_from_wire(&raw.LastLoginTime, "LastLoginTime")?,
            reserve_info: string_from_wire(&raw.ReserveInfo, "ReserveInfo")?,
        })
    }
}

impl FromSys for crate::SPBMAddOnInterParameter {
    type Raw = sys::CThostFtdcSPBMAddOnInterParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            spread_id: raw.SpreadId,
            add_on_inter_rate_z2: raw.AddOnInterRateZ2,
            leg1_product_family_code: crate::InstrumentId::from_wire(&raw.Leg1ProdFamilyCode)
                .map_err(|source| ConversionError::new("Leg1ProdFamilyCode", source))?,
            leg2_product_family_code: crate::InstrumentId::from_wire(&raw.Leg2ProdFamilyCode)
                .map_err(|source| ConversionError::new("Leg2ProdFamilyCode", source))?,
        })
    }
}

impl FromSys for crate::SPBMFutureParameter {
    type Raw = sys::CThostFtdcSPBMFutureParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            cvf: raw.Cvf,
            time_range: crate::TimeRange::from_code(raw.TimeRange as u8),
            margin_rate: raw.MarginRate,
            lock_rate_x: raw.LockRateX,
            add_on_rate: raw.AddOnRate,
            pre_settlement_price: raw.PreSettlementPrice,
            add_on_lock_rate_x2: raw.AddOnLockRateX2,
        })
    }
}

impl FromSys for crate::SPBMInterParameter {
    type Raw = sys::CThostFtdcSPBMInterParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            spread_id: raw.SpreadId,
            inter_rate_z: raw.InterRateZ,
            leg1_product_family_code: crate::InstrumentId::from_wire(&raw.Leg1ProdFamilyCode)
                .map_err(|source| ConversionError::new("Leg1ProdFamilyCode", source))?,
            leg2_product_family_code: crate::InstrumentId::from_wire(&raw.Leg2ProdFamilyCode)
                .map_err(|source| ConversionError::new("Leg2ProdFamilyCode", source))?,
        })
    }
}

impl FromSys for crate::SPBMIntraParameter {
    type Raw = sys::CThostFtdcSPBMIntraParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            intra_rate_y: raw.IntraRateY,
            add_on_intra_rate_y2: raw.AddOnIntraRateY2,
        })
    }
}

impl FromSys for crate::SPBMInvestorPortfDef {
    type Raw = sys::CThostFtdcSPBMInvestorPortfDefField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            portfolio_definition_id: raw.PortfolioDefID,
        })
    }
}

impl FromSys for crate::SPBMOptionParameter {
    type Raw = sys::CThostFtdcSPBMOptionParameterField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            cvf: raw.Cvf,
            down_price: raw.DownPrice,
            delta: raw.Delta,
            slim_delta: raw.SlimiDelta,
            pre_settlement_price: raw.PreSettlementPrice,
        })
    }
}

impl FromSys for crate::SPBMPortfDefinition {
    type Raw = sys::CThostFtdcSPBMPortfDefinitionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            portfolio_definition_id: raw.PortfolioDefID,
            product_family_code: crate::InstrumentId::from_wire(&raw.ProdFamilyCode)
                .map_err(|source| ConversionError::new("ProdFamilyCode", source))?,
            is_spbm: raw.IsSPBM != 0,
        })
    }
}

impl FromSys for crate::SPMMInstParam {
    type Raw = sys::CThostFtdcSPMMInstParamField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            instrument_margin_cal_id: crate::InstMarginCalID::from_code(raw.InstMarginCalID as u8),
            commodity_id: string_from_wire(&raw.CommodityID, "CommodityID")?,
            commodity_group_id: string_from_wire(&raw.CommodityGroupID, "CommodityGroupID")?,
        })
    }
}

impl FromSys for crate::SPMMProductParam {
    type Raw = sys::CThostFtdcSPMMProductParamField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            product_id: string_from_wire(&raw.ProductID, "ProductID")?,
            commodity_id: string_from_wire(&raw.CommodityID, "CommodityID")?,
            commodity_group_id: string_from_wire(&raw.CommodityGroupID, "CommodityGroupID")?,
        })
    }
}

impl FromSys for crate::SecAgentACIDMap {
    type Raw = sys::CThostFtdcSecAgentACIDMapField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            broker_sec_agent_id: crate::AccountId::from_wire(&raw.BrokerSecAgentID)
                .map_err(|source| ConversionError::new("BrokerSecAgentID", source))?,
        })
    }
}

impl FromSys for crate::SecAgentCheckMode {
    type Raw = sys::CThostFtdcSecAgentCheckModeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            broker_sec_agent_id: crate::AccountId::from_wire(&raw.BrokerSecAgentID)
                .map_err(|source| ConversionError::new("BrokerSecAgentID", source))?,
            check_self_account: raw.CheckSelfAccount != 0,
        })
    }
}

impl FromSys for crate::SecAgentTradeInfo {
    type Raw = sys::CThostFtdcSecAgentTradeInfoField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_sec_agent_id: crate::AccountId::from_wire(&raw.BrokerSecAgentID)
                .map_err(|source| ConversionError::new("BrokerSecAgentID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            long_customer_name: string_from_wire(&raw.LongCustomerName, "LongCustomerName")?,
        })
    }
}

impl FromSys for crate::SettlementInfo {
    type Raw = sys::CThostFtdcSettlementInfoField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            sequence_no: raw.SequenceNo,
            content: string_from_wire(&raw.Content, "Content")?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
        })
    }
}

impl FromSys for crate::SettlementInfoConfirm {
    type Raw = sys::CThostFtdcSettlementInfoConfirmField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            confirm_date: crate::DateText::from_wire(&raw.ConfirmDate)
                .map_err(|source| ConversionError::new("ConfirmDate", source))?,
            confirm_time: crate::TimeText::from_wire(&raw.ConfirmTime)
                .map_err(|source| ConversionError::new("ConfirmTime", source))?,
            settlement_id: raw.SettlementID,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
        })
    }
}

impl FromSys for crate::SpdApply {
    type Raw = sys::CThostFtdcSpdApplyField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            first_leg_instrument_id: crate::InstrumentId::from_wire(&raw.FirstLegInstrumentID)
                .map_err(|source| ConversionError::new("FirstLegInstrumentID", source))?,
            second_leg_instrument_id: crate::InstrumentId::from_wire(&raw.SecondLegInstrumentID)
                .map_err(|source| ConversionError::new("SecondLegInstrumentID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            volume: raw.Volume,
            direction: crate::Direction::from_code(raw.Direction as u8),
            request_id: crate::RequestId::new(raw.RequestID),
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            active_user_id: crate::UserId::from_wire(&raw.ActiveUserID)
                .map_err(|source| ConversionError::new("ActiveUserID", source))?,
            broker_order_seq: raw.BrokerOrderSeq,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            apply_status: crate::ApplyStatus::from_code(raw.ApplyStatus as u8),
            sequence_no: raw.SequenceNo,
            insert_date: crate::DateText::from_wire(&raw.InsertDate)
                .map_err(|source| ConversionError::new("InsertDate", source))?,
            insert_time: crate::TimeText::from_wire(&raw.InsertTime)
                .map_err(|source| ConversionError::new("InsertTime", source))?,
            cancel_time: crate::TimeText::from_wire(&raw.CancelTime)
                .map_err(|source| ConversionError::new("CancelTime", source))?,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_submit_status: crate::OrderSubmitStatus::from_code(raw.OrderSubmitStatus as u8),
            notify_sequence: raw.NotifySequence,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            cmb_type: crate::CmbType::from_code(raw.CmbType as u8),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
        })
    }
}

impl FromSys for crate::SpdApplyAction {
    type Raw = sys::CThostFtdcSpdApplyActionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            action_date: crate::DateText::from_wire(&raw.ActionDate)
                .map_err(|source| ConversionError::new("ActionDate", source))?,
            action_time: crate::TimeText::from_wire(&raw.ActionTime)
                .map_err(|source| ConversionError::new("ActionTime", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            install_id: raw.InstallID,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            action_local_id: crate::OrderLocalId::from_wire(&raw.ActionLocalID)
                .map_err(|source| ConversionError::new("ActionLocalID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            order_action_status: crate::OrderActionStatus::from_code(raw.OrderActionStatus as u8),
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            request_id: crate::RequestId::new(raw.RequestID),
            status_msg: string_from_wire(&raw.StatusMsg, "StatusMsg")?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
        })
    }
}

impl FromSys for crate::SpecificInstrument {
    type Raw = sys::CThostFtdcSpecificInstrumentField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
        })
    }
}

impl FromSys for crate::Trade {
    type Raw = sys::CThostFtdcTradeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            order_ref: crate::OrderRef::from_wire(&raw.OrderRef)
                .map_err(|source| ConversionError::new("OrderRef", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            trade_id: crate::TradeId::from_wire(&raw.TradeID)
                .map_err(|source| ConversionError::new("TradeID", source))?,
            direction: crate::Direction::from_code(raw.Direction as u8),
            order_system_id: crate::OrderSysId::from_wire(&raw.OrderSysID)
                .map_err(|source| ConversionError::new("OrderSysID", source))?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            trading_role: crate::TradingRole::from_code(raw.TradingRole as u8),
            offset_flag: crate::OffsetFlag::from_code(raw.OffsetFlag as u8),
            hedge_flag: crate::HedgeFlag::from_code(raw.HedgeFlag as u8),
            price: raw.Price,
            volume: raw.Volume,
            trade_date: crate::DateText::from_wire(&raw.TradeDate)
                .map_err(|source| ConversionError::new("TradeDate", source))?,
            trade_time: crate::TimeText::from_wire(&raw.TradeTime)
                .map_err(|source| ConversionError::new("TradeTime", source))?,
            trade_type: crate::TradeType::from_code(raw.TradeType as u8),
            price_source: crate::PriceSource::from_code(raw.PriceSource as u8),
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            clearing_part_id: string_from_wire(&raw.ClearingPartID, "ClearingPartID")?,
            business_unit: crate::BusinessUnit::from_wire(&raw.BusinessUnit)
                .map_err(|source| ConversionError::new("BusinessUnit", source))?,
            sequence_no: raw.SequenceNo,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            broker_order_seq: raw.BrokerOrderSeq,
            trade_source: crate::TradeSource::from_code(raw.TradeSource as u8),
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
            instrument_id: crate::InstrumentId::from_wire(&raw.InstrumentID)
                .map_err(|source| ConversionError::new("InstrumentID", source))?,
            exchange_instrument_id: crate::ExchangeInstrumentId::from_wire(&raw.ExchangeInstID)
                .map_err(|source| ConversionError::new("ExchangeInstID", source))?,
        })
    }
}

impl FromSys for crate::TraderOffer {
    type Raw = sys::CThostFtdcTraderOfferField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            trader_id: string_from_wire(&raw.TraderID, "TraderID")?,
            participant_id: string_from_wire(&raw.ParticipantID, "ParticipantID")?,
            password: string_from_wire(&raw.Password, "Password")?,
            install_id: raw.InstallID,
            order_local_id: crate::OrderLocalId::from_wire(&raw.OrderLocalID)
                .map_err(|source| ConversionError::new("OrderLocalID", source))?,
            trader_connect_status: crate::TraderConnectStatus::from_code(
                raw.TraderConnectStatus as u8,
            ),
            connect_request_date: crate::DateText::from_wire(&raw.ConnectRequestDate)
                .map_err(|source| ConversionError::new("ConnectRequestDate", source))?,
            connect_request_time: crate::TimeText::from_wire(&raw.ConnectRequestTime)
                .map_err(|source| ConversionError::new("ConnectRequestTime", source))?,
            last_report_date: crate::DateText::from_wire(&raw.LastReportDate)
                .map_err(|source| ConversionError::new("LastReportDate", source))?,
            last_report_time: crate::TimeText::from_wire(&raw.LastReportTime)
                .map_err(|source| ConversionError::new("LastReportTime", source))?,
            connect_date: crate::DateText::from_wire(&raw.ConnectDate)
                .map_err(|source| ConversionError::new("ConnectDate", source))?,
            connect_time: crate::TimeText::from_wire(&raw.ConnectTime)
                .map_err(|source| ConversionError::new("ConnectTime", source))?,
            start_date: crate::DateText::from_wire(&raw.StartDate)
                .map_err(|source| ConversionError::new("StartDate", source))?,
            start_time: crate::TimeText::from_wire(&raw.StartTime)
                .map_err(|source| ConversionError::new("StartTime", source))?,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            max_trade_id: crate::TradeId::from_wire(&raw.MaxTradeID)
                .map_err(|source| ConversionError::new("MaxTradeID", source))?,
            max_order_message_reference: string_from_wire(
                &raw.MaxOrderMessageReference,
                "MaxOrderMessageReference",
            )?,
            order_cancel_alg: crate::OrderCancelAlg::from_code(raw.OrderCancelAlg as u8),
        })
    }
}

impl FromSys for crate::TradingAccount {
    type Raw = sys::CThostFtdcTradingAccountField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            pre_mortgage: raw.PreMortgage,
            pre_credit: raw.PreCredit,
            pre_deposit: raw.PreDeposit,
            pre_balance: raw.PreBalance,
            pre_margin: raw.PreMargin,
            interest_base: raw.InterestBase,
            interest: raw.Interest,
            deposit: raw.Deposit,
            withdraw: raw.Withdraw,
            frozen_margin: raw.FrozenMargin,
            frozen_cash: raw.FrozenCash,
            frozen_commission: raw.FrozenCommission,
            current_margin: raw.CurrMargin,
            cash_in: raw.CashIn,
            commission: raw.Commission,
            close_profit: raw.CloseProfit,
            position_profit: raw.PositionProfit,
            balance: raw.Balance,
            available: raw.Available,
            withdraw_quota: raw.WithdrawQuota,
            reserve: raw.Reserve,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            settlement_id: raw.SettlementID,
            credit: raw.Credit,
            mortgage: raw.Mortgage,
            exchange_margin: raw.ExchangeMargin,
            delivery_margin: raw.DeliveryMargin,
            exchange_delivery_margin: raw.ExchangeDeliveryMargin,
            reserve_balance: raw.ReserveBalance,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            pre_fund_mortgage_in: raw.PreFundMortgageIn,
            pre_fund_mortgage_out: raw.PreFundMortgageOut,
            fund_mortgage_in: raw.FundMortgageIn,
            fund_mortgage_out: raw.FundMortgageOut,
            fund_mortgage_available: raw.FundMortgageAvailable,
            mortgageable_fund: raw.MortgageableFund,
            spec_product_margin: raw.SpecProductMargin,
            spec_product_frozen_margin: raw.SpecProductFrozenMargin,
            spec_product_commission: raw.SpecProductCommission,
            spec_product_frozen_commission: raw.SpecProductFrozenCommission,
            spec_product_position_profit: raw.SpecProductPositionProfit,
            spec_product_close_profit: raw.SpecProductCloseProfit,
            spec_product_position_profit_by_alg: raw.SpecProductPositionProfitByAlg,
            spec_product_exchange_margin: raw.SpecProductExchangeMargin,
            biz_type: crate::BizType::from_code(raw.BizType as u8),
            frozen_swap: raw.FrozenSwap,
            remain_swap: raw.RemainSwap,
            option_value: raw.OptionValue,
        })
    }
}

impl FromSys for crate::TradingAccountPasswordUpdate {
    type Raw = sys::CThostFtdcTradingAccountPasswordUpdateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            old_password: string_from_wire(&raw.OldPassword, "OldPassword")?,
            new_password: string_from_wire(&raw.NewPassword, "NewPassword")?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
        })
    }
}

impl FromSys for crate::TradingCode {
    type Raw = sys::CThostFtdcTradingCodeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            exchange_id: crate::ExchangeId::from_wire(&raw.ExchangeID)
                .map_err(|source| ConversionError::new("ExchangeID", source))?,
            client_id: string_from_wire(&raw.ClientID, "ClientID")?,
            is_active: raw.IsActive != 0,
            client_id_type: crate::ClientIDType::from_code(raw.ClientIDType as u8),
            branch_id: string_from_wire(&raw.BranchID, "BranchID")?,
            biz_type: crate::BizType::from_code(raw.BizType as u8),
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
        })
    }
}

impl FromSys for crate::TradingNotice {
    type Raw = sys::CThostFtdcTradingNoticeField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_range: crate::InvestorRange::from_code(raw.InvestorRange as u8),
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            sequence_series: raw.SequenceSeries,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            send_time: crate::TimeText::from_wire(&raw.SendTime)
                .map_err(|source| ConversionError::new("SendTime", source))?,
            sequence_no: raw.SequenceNo,
            field_content: string_from_wire(&raw.FieldContent, "FieldContent")?,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
        })
    }
}

impl FromSys for crate::TradingNoticeInfo {
    type Raw = sys::CThostFtdcTradingNoticeInfoField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            send_time: crate::TimeText::from_wire(&raw.SendTime)
                .map_err(|source| ConversionError::new("SendTime", source))?,
            field_content: string_from_wire(&raw.FieldContent, "FieldContent")?,
            sequence_series: raw.SequenceSeries,
            sequence_no: raw.SequenceNo,
            invest_unit_id: crate::InvestUnitId::from_wire(&raw.InvestUnitID)
                .map_err(|source| ConversionError::new("InvestUnitID", source))?,
        })
    }
}

impl FromSys for crate::TransferBank {
    type Raw = sys::CThostFtdcTransferBankField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBrchID, "BankBrchID")?,
            bank_name: string_from_wire(&raw.BankName, "BankName")?,
            is_active: raw.IsActive != 0,
        })
    }
}

impl FromSys for crate::TransferSerial {
    type Raw = sys::CThostFtdcTransferSerialField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            plate_serial: raw.PlateSerial,
            trade_date: string_from_wire(&raw.TradeDate, "TradeDate")?,
            trading_day: crate::DateText::from_wire(&raw.TradingDay)
                .map_err(|source| ConversionError::new("TradingDay", source))?,
            trade_time: string_from_wire(&raw.TradeTime, "TradeTime")?,
            trade_code: string_from_wire(&raw.TradeCode, "TradeCode")?,
            session_id: crate::SessionId::new(raw.SessionID),
            bank_id: string_from_wire(&raw.BankID, "BankID")?,
            bank_branch_id: string_from_wire(&raw.BankBranchID, "BankBranchID")?,
            bank_acc_type: crate::BankAccType::from_code(raw.BankAccType as u8),
            bank_account: string_from_wire(&raw.BankAccount, "BankAccount")?,
            bank_serial: string_from_wire(&raw.BankSerial, "BankSerial")?,
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            broker_branch_id: string_from_wire(&raw.BrokerBranchID, "BrokerBranchID")?,
            future_acc_type: crate::FutureAccType::from_code(raw.FutureAccType as u8),
            account_id: crate::AccountId::from_wire(&raw.AccountID)
                .map_err(|source| ConversionError::new("AccountID", source))?,
            investor_id: crate::InvestorId::from_wire(&raw.InvestorID)
                .map_err(|source| ConversionError::new("InvestorID", source))?,
            future_serial: raw.FutureSerial,
            id_card_type: crate::IdCardType::from_code(raw.IdCardType as u8),
            identified_card_no: string_from_wire(&raw.IdentifiedCardNo, "IdentifiedCardNo")?,
            currency_id: string_from_wire(&raw.CurrencyID, "CurrencyID")?,
            trade_amount: raw.TradeAmount,
            cust_fee: raw.CustFee,
            broker_fee: raw.BrokerFee,
            availability_flag: crate::AvailabilityFlag::from_code(raw.AvailabilityFlag as u8),
            operator_code: string_from_wire(&raw.OperatorCode, "OperatorCode")?,
            bank_new_account: string_from_wire(&raw.BankNewAccount, "BankNewAccount")?,
            error_id: raw.ErrorID,
            error_msg: string_from_wire(&raw.ErrorMsg, "ErrorMsg")?,
        })
    }
}

impl FromSys for crate::UserLogout {
    type Raw = sys::CThostFtdcUserLogoutField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
        })
    }
}

impl FromSys for crate::UserPasswordUpdate {
    type Raw = sys::CThostFtdcUserPasswordUpdateField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            old_password: string_from_wire(&raw.OldPassword, "OldPassword")?,
            new_password: string_from_wire(&raw.NewPassword, "NewPassword")?,
        })
    }
}

impl FromSys for crate::UserSession {
    type Raw = sys::CThostFtdcUserSessionField;

    fn from_sys(raw: &Self::Raw) -> Result<Self, ConversionError> {
        Ok(Self {
            front_id: crate::FrontId::new(raw.FrontID),
            session_id: crate::SessionId::new(raw.SessionID),
            broker_id: crate::BrokerId::from_wire(&raw.BrokerID)
                .map_err(|source| ConversionError::new("BrokerID", source))?,
            user_id: crate::UserId::from_wire(&raw.UserID)
                .map_err(|source| ConversionError::new("UserID", source))?,
            login_date: crate::DateText::from_wire(&raw.LoginDate)
                .map_err(|source| ConversionError::new("LoginDate", source))?,
            login_time: crate::TimeText::from_wire(&raw.LoginTime)
                .map_err(|source| ConversionError::new("LoginTime", source))?,
            user_product_info: string_from_wire(&raw.UserProductInfo, "UserProductInfo")?,
            interface_product_info: string_from_wire(
                &raw.InterfaceProductInfo,
                "InterfaceProductInfo",
            )?,
            protocol_info: string_from_wire(&raw.ProtocolInfo, "ProtocolInfo")?,
            mac_address: string_from_wire(&raw.MacAddress, "MacAddress")?,
            login_remark: string_from_wire(&raw.LoginRemark, "LoginRemark")?,
            ip_address: string_from_wire(&raw.IPAddress, "IPAddress")?,
        })
    }
}

impl crate::ReqUserLogin {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcReqUserLoginField, ConversionError> {
        let raw = sys::CThostFtdcReqUserLoginField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            Password: string_field(&self.password, "Password")?,
            UserProductInfo: string_field(&self.user_product_info, "UserProductInfo")?,
            InterfaceProductInfo: string_field(
                &self.interface_product_info,
                "InterfaceProductInfo",
            )?,
            ProtocolInfo: string_field(&self.protocol_info, "ProtocolInfo")?,
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            OneTimePassword: string_field(&self.one_time_password, "OneTimePassword")?,
            LoginRemark: string_field(&self.login_remark, "LoginRemark")?,
            ClientIPPort: self.client_ip_port,
            ClientIPAddress: string_field(&self.client_ip_address, "ClientIPAddress")?,
            SMSCode: string_field(&self.sms_code, "SMSCode")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::FensUserInfo {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcFensUserInfoField, ConversionError> {
        let raw = sys::CThostFtdcFensUserInfoField {
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            LoginMode: self
                .login_mode
                .code()
                .ok_or_else(|| ConversionError::new("LoginMode", TextError::UnknownEnumValue))?
                as c_char,
        };
        Ok(raw)
    }
}

impl crate::UserLogout {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcUserLogoutField, ConversionError> {
        let raw = sys::CThostFtdcUserLogoutField {
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::QryMulticastInstrument {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQryMulticastInstrumentField, ConversionError> {
        let raw = sys::CThostFtdcQryMulticastInstrumentField {
            TopicID: self.topic_id,
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::ReqAuthenticate {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcReqAuthenticateField, ConversionError> {
        let raw = sys::CThostFtdcReqAuthenticateField {
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            UserProductInfo: string_field(&self.user_product_info, "UserProductInfo")?,
            AuthCode: string_field(&self.auth_code, "AuthCode")?,
            AppID: string_field(&self.app_id, "AppID")?,
        };
        Ok(raw)
    }
}

impl crate::SettlementInfoConfirm {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcSettlementInfoConfirmField, ConversionError> {
        let raw = sys::CThostFtdcSettlementInfoConfirmField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ConfirmDate: self.confirm_date.to_wire(),
            ConfirmTime: self.confirm_time.to_wire(),
            SettlementID: self.settlement_id,
            AccountID: self.account_id.to_wire(),
            CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
        };
        Ok(raw)
    }
}

impl crate::QrySettlementInfoConfirm {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQrySettlementInfoConfirmField, ConversionError> {
        let raw = sys::CThostFtdcQrySettlementInfoConfirmField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            AccountID: self.account_id.to_wire(),
            CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
        };
        Ok(raw)
    }
}

impl crate::QrySettlementInfo {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQrySettlementInfoField, ConversionError> {
        let raw = sys::CThostFtdcQrySettlementInfoField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            TradingDay: self.trading_day.to_wire(),
            AccountID: self.account_id.to_wire(),
            CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
        };
        Ok(raw)
    }
}

impl crate::QryInvestorPositionDetail {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQryInvestorPositionDetailField, ConversionError> {
        let raw = sys::CThostFtdcQryInvestorPositionDetailField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ExchangeID: self.exchange_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryProduct {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryProductField, ConversionError> {
        let raw =
            sys::CThostFtdcQryProductField {
                ProductClass: self.product_class.code().ok_or_else(|| {
                    ConversionError::new("ProductClass", TextError::UnknownEnumValue)
                })? as c_char,
                ExchangeID: self.exchange_id.to_wire(),
                ProductID: self.product_id.to_wire(),
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::QryExchange {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryExchangeField, ConversionError> {
        let raw = sys::CThostFtdcQryExchangeField {
            ExchangeID: self.exchange_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::QryTradingCode {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryTradingCodeField, ConversionError> {
        let raw =
            sys::CThostFtdcQryTradingCodeField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                ExchangeID: self.exchange_id.to_wire(),
                ClientID: string_field(&self.client_id, "ClientID")?,
                ClientIDType: self.client_id_type.code().ok_or_else(|| {
                    ConversionError::new("ClientIDType", TextError::UnknownEnumValue)
                })? as c_char,
                InvestUnitID: self.invest_unit_id.to_wire(),
            };
        Ok(raw)
    }
}

impl crate::QryInstrumentMarginRate {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQryInstrumentMarginRateField, ConversionError> {
        let raw = sys::CThostFtdcQryInstrumentMarginRateField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            HedgeFlag: self
                .hedge_flag
                .code()
                .ok_or_else(|| ConversionError::new("HedgeFlag", TextError::UnknownEnumValue))?
                as c_char,
            ExchangeID: self.exchange_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryInstrumentCommissionRate {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQryInstrumentCommissionRateField, ConversionError> {
        let raw = sys::CThostFtdcQryInstrumentCommissionRateField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ExchangeID: self.exchange_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryInstrumentOrderCommRate {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQryInstrumentOrderCommRateField, ConversionError> {
        let raw = sys::CThostFtdcQryInstrumentOrderCommRateField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryMaxOrderVolume {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryMaxOrderVolumeField, ConversionError> {
        let raw =
            sys::CThostFtdcQryMaxOrderVolumeField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                Direction: self
                    .direction
                    .code()
                    .ok_or_else(|| ConversionError::new("Direction", TextError::UnknownEnumValue))?
                    as c_char,
                OffsetFlag: self.offset_flag.code().ok_or_else(|| {
                    ConversionError::new("OffsetFlag", TextError::UnknownEnumValue)
                })? as c_char,
                HedgeFlag: self
                    .hedge_flag
                    .code()
                    .ok_or_else(|| ConversionError::new("HedgeFlag", TextError::UnknownEnumValue))?
                    as c_char,
                MaxVolume: self.max_volume,
                ExchangeID: self.exchange_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                InstrumentID: self.instrument_id.to_wire(),
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::QryDepthMarketData {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryDepthMarketDataField, ConversionError> {
        let raw =
            sys::CThostFtdcQryDepthMarketDataField {
                ExchangeID: self.exchange_id.to_wire(),
                InstrumentID: self.instrument_id.to_wire(),
                ProductClass: self.product_class.code().ok_or_else(|| {
                    ConversionError::new("ProductClass", TextError::UnknownEnumValue)
                })? as c_char,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::QryOrder {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryOrderField, ConversionError> {
        let raw = sys::CThostFtdcQryOrderField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ExchangeID: self.exchange_id.to_wire(),
            OrderSysID: self.order_system_id.to_wire(),
            InsertTimeStart: self.insert_time_start.to_wire(),
            InsertTimeEnd: self.insert_time_end.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryTrade {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryTradeField, ConversionError> {
        let raw = sys::CThostFtdcQryTradeField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ExchangeID: self.exchange_id.to_wire(),
            TradeID: self.trade_id.to_wire(),
            TradeTimeStart: self.trade_time_start.to_wire(),
            TradeTimeEnd: self.trade_time_end.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryInvestorPosition {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcQryInvestorPositionField, ConversionError> {
        let raw = sys::CThostFtdcQryInvestorPositionField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ExchangeID: self.exchange_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::QryTradingAccount {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryTradingAccountField, ConversionError> {
        let raw = sys::CThostFtdcQryTradingAccountField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
            BizType: self
                .biz_type
                .code()
                .ok_or_else(|| ConversionError::new("BizType", TextError::UnknownEnumValue))?
                as c_char,
            AccountID: self.account_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::QryInstrument {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcQryInstrumentField, ConversionError> {
        let raw = sys::CThostFtdcQryInstrumentField {
            ExchangeID: self.exchange_id.to_wire(),
            InstrumentID: self.instrument_id.to_wire(),
            ExchangeInstID: self.exchange_instrument_id.to_wire(),
            ProductID: self.product_id.to_wire(),
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::InputOrder {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputOrderField, ConversionError> {
        let raw =
            sys::CThostFtdcInputOrderField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                OrderRef: self.order_ref.to_wire(),
                UserID: self.user_id.to_wire(),
                OrderPriceType: self.order_price_type.code().ok_or_else(|| {
                    ConversionError::new("OrderPriceType", TextError::UnknownEnumValue)
                })? as c_char,
                Direction: self
                    .direction
                    .code()
                    .ok_or_else(|| ConversionError::new("Direction", TextError::UnknownEnumValue))?
                    as c_char,
                CombOffsetFlag: comb_to_wire(self.comb_offset_flag.to_wire(), "CombOffsetFlag")?,
                CombHedgeFlag: comb_to_wire(self.comb_hedge_flag.to_wire(), "CombHedgeFlag")?,
                LimitPrice: self.limit_price,
                VolumeTotalOriginal: self.volume_total_original,
                TimeCondition: self.time_condition.code().ok_or_else(|| {
                    ConversionError::new("TimeCondition", TextError::UnknownEnumValue)
                })? as c_char,
                GTDDate: self.gtd_date.to_wire(),
                VolumeCondition: self.volume_condition.code().ok_or_else(|| {
                    ConversionError::new("VolumeCondition", TextError::UnknownEnumValue)
                })? as c_char,
                MinVolume: self.min_volume,
                ContingentCondition: self.contingent_condition.code().ok_or_else(|| {
                    ConversionError::new("ContingentCondition", TextError::UnknownEnumValue)
                })? as c_char,
                StopPrice: self.stop_price,
                ForceCloseReason: self.force_close_reason.code().ok_or_else(|| {
                    ConversionError::new("ForceCloseReason", TextError::UnknownEnumValue)
                })? as c_char,
                IsAutoSuspend: self.is_auto_suspend as std::os::raw::c_int,
                BusinessUnit: self.business_unit.to_wire(),
                RequestID: self.request_id.get(),
                UserForceClose: self.user_force_close as std::os::raw::c_int,
                IsSwapOrder: self.is_swap_order as std::os::raw::c_int,
                ExchangeID: self.exchange_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                AccountID: self.account_id.to_wire(),
                CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
                ClientID: string_field(&self.client_id, "ClientID")?,
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                OrderMemo: string_field(&self.order_memo, "OrderMemo")?,
                SessionReqSeq: self.session_req_seq,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::InputOrderAction {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputOrderActionField, ConversionError> {
        let raw =
            sys::CThostFtdcInputOrderActionField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                OrderActionRef: self.order_action_ref,
                OrderRef: self.order_ref.to_wire(),
                RequestID: self.request_id.get(),
                FrontID: self.front_id.get(),
                SessionID: self.session_id.get(),
                ExchangeID: self.exchange_id.to_wire(),
                OrderSysID: self.order_system_id.to_wire(),
                ActionFlag: self.action_flag.code().ok_or_else(|| {
                    ConversionError::new("ActionFlag", TextError::UnknownEnumValue)
                })? as c_char,
                LimitPrice: self.limit_price,
                VolumeChange: self.volume_change,
                UserID: self.user_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                OrderMemo: string_field(&self.order_memo, "OrderMemo")?,
                SessionReqSeq: self.session_req_seq,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::UserSystemInfo {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcUserSystemInfoField, ConversionError> {
        let raw = sys::CThostFtdcUserSystemInfoField {
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            ClientSystemInfoLen: self.client_system_info_len,
            ClientSystemInfo: string_field(&self.client_system_info, "ClientSystemInfo")?,
            ClientIPPort: self.client_ip_port,
            ClientLoginTime: self.client_login_time.to_wire(),
            ClientAppID: string_field(&self.client_app_id, "ClientAppID")?,
            ClientPublicIP: string_field(&self.client_public_ip, "ClientPublicIP")?,
            ClientLoginRemark: string_field(&self.client_login_remark, "ClientLoginRemark")?,
            MAC: string_field(&self.mac, "MAC")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::WechatUserSystemInfo {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcWechatUserSystemInfoField, ConversionError> {
        let raw = sys::CThostFtdcWechatUserSystemInfoField {
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            WechatCltSysInfoLen: self.wechat_clt_system_info_len,
            WechatCltSysInfo: string_field(&self.wechat_clt_system_info, "WechatCltSysInfo")?,
            ClientIPPort: self.client_ip_port,
            ClientLoginTime: self.client_login_time.to_wire(),
            ClientAppID: string_field(&self.client_app_id, "ClientAppID")?,
            ClientPublicIP: string_field(&self.client_public_ip, "ClientPublicIP")?,
            ClientLoginRemark: string_field(&self.client_login_remark, "ClientLoginRemark")?,
        };
        Ok(raw)
    }
}

impl crate::UserPasswordUpdate {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcUserPasswordUpdateField, ConversionError> {
        let raw = sys::CThostFtdcUserPasswordUpdateField {
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            OldPassword: string_field(&self.old_password, "OldPassword")?,
            NewPassword: string_field(&self.new_password, "NewPassword")?,
        };
        Ok(raw)
    }
}

impl crate::TradingAccountPasswordUpdate {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcTradingAccountPasswordUpdateField, ConversionError> {
        let raw = sys::CThostFtdcTradingAccountPasswordUpdateField {
            BrokerID: self.broker_id.to_wire(),
            AccountID: self.account_id.to_wire(),
            OldPassword: string_field(&self.old_password, "OldPassword")?,
            NewPassword: string_field(&self.new_password, "NewPassword")?,
            CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
        };
        Ok(raw)
    }
}

impl crate::ReqUserAuthMethod {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcReqUserAuthMethodField, ConversionError> {
        let raw = sys::CThostFtdcReqUserAuthMethodField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::ReqGenUserCaptcha {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcReqGenUserCaptchaField, ConversionError> {
        let raw = sys::CThostFtdcReqGenUserCaptchaField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::ReqGenUserText {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcReqGenUserTextField, ConversionError> {
        let raw = sys::CThostFtdcReqGenUserTextField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::ReqUserLoginWithCaptcha {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcReqUserLoginWithCaptchaField, ConversionError> {
        let raw = sys::CThostFtdcReqUserLoginWithCaptchaField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            Password: string_field(&self.password, "Password")?,
            UserProductInfo: string_field(&self.user_product_info, "UserProductInfo")?,
            InterfaceProductInfo: string_field(
                &self.interface_product_info,
                "InterfaceProductInfo",
            )?,
            ProtocolInfo: string_field(&self.protocol_info, "ProtocolInfo")?,
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            LoginRemark: string_field(&self.login_remark, "LoginRemark")?,
            Captcha: string_field(&self.captcha, "Captcha")?,
            ClientIPPort: self.client_ip_port,
            ClientIPAddress: string_field(&self.client_ip_address, "ClientIPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::ReqUserLoginWithText {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcReqUserLoginWithTextField, ConversionError> {
        let raw = sys::CThostFtdcReqUserLoginWithTextField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            Password: string_field(&self.password, "Password")?,
            UserProductInfo: string_field(&self.user_product_info, "UserProductInfo")?,
            InterfaceProductInfo: string_field(
                &self.interface_product_info,
                "InterfaceProductInfo",
            )?,
            ProtocolInfo: string_field(&self.protocol_info, "ProtocolInfo")?,
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            LoginRemark: string_field(&self.login_remark, "LoginRemark")?,
            Text: string_field(&self.text, "Text")?,
            ClientIPPort: self.client_ip_port,
            ClientIPAddress: string_field(&self.client_ip_address, "ClientIPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::ReqUserLoginWithOTP {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcReqUserLoginWithOTPField, ConversionError> {
        let raw = sys::CThostFtdcReqUserLoginWithOTPField {
            TradingDay: self.trading_day.to_wire(),
            BrokerID: self.broker_id.to_wire(),
            UserID: self.user_id.to_wire(),
            Password: string_field(&self.password, "Password")?,
            UserProductInfo: string_field(&self.user_product_info, "UserProductInfo")?,
            InterfaceProductInfo: string_field(
                &self.interface_product_info,
                "InterfaceProductInfo",
            )?,
            ProtocolInfo: string_field(&self.protocol_info, "ProtocolInfo")?,
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            LoginRemark: string_field(&self.login_remark, "LoginRemark")?,
            OTPPassword: string_field(&self.otp_password, "OTPPassword")?,
            ClientIPPort: self.client_ip_port,
            ClientIPAddress: string_field(&self.client_ip_address, "ClientIPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::ParkedOrder {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcParkedOrderField, ConversionError> {
        let raw =
            sys::CThostFtdcParkedOrderField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                OrderRef: self.order_ref.to_wire(),
                UserID: self.user_id.to_wire(),
                OrderPriceType: self.order_price_type.code().ok_or_else(|| {
                    ConversionError::new("OrderPriceType", TextError::UnknownEnumValue)
                })? as c_char,
                Direction: self
                    .direction
                    .code()
                    .ok_or_else(|| ConversionError::new("Direction", TextError::UnknownEnumValue))?
                    as c_char,
                CombOffsetFlag: comb_to_wire(self.comb_offset_flag.to_wire(), "CombOffsetFlag")?,
                CombHedgeFlag: comb_to_wire(self.comb_hedge_flag.to_wire(), "CombHedgeFlag")?,
                LimitPrice: self.limit_price,
                VolumeTotalOriginal: self.volume_total_original,
                TimeCondition: self.time_condition.code().ok_or_else(|| {
                    ConversionError::new("TimeCondition", TextError::UnknownEnumValue)
                })? as c_char,
                GTDDate: self.gtd_date.to_wire(),
                VolumeCondition: self.volume_condition.code().ok_or_else(|| {
                    ConversionError::new("VolumeCondition", TextError::UnknownEnumValue)
                })? as c_char,
                MinVolume: self.min_volume,
                ContingentCondition: self.contingent_condition.code().ok_or_else(|| {
                    ConversionError::new("ContingentCondition", TextError::UnknownEnumValue)
                })? as c_char,
                StopPrice: self.stop_price,
                ForceCloseReason: self.force_close_reason.code().ok_or_else(|| {
                    ConversionError::new("ForceCloseReason", TextError::UnknownEnumValue)
                })? as c_char,
                IsAutoSuspend: self.is_auto_suspend as std::os::raw::c_int,
                BusinessUnit: self.business_unit.to_wire(),
                RequestID: self.request_id.get(),
                UserForceClose: self.user_force_close as std::os::raw::c_int,
                ExchangeID: self.exchange_id.to_wire(),
                ParkedOrderID: string_field(&self.parked_order_id, "ParkedOrderID")?,
                UserType: self
                    .user_type
                    .code()
                    .ok_or_else(|| ConversionError::new("UserType", TextError::UnknownEnumValue))?
                    as c_char,
                Status: self
                    .status
                    .code()
                    .ok_or_else(|| ConversionError::new("Status", TextError::UnknownEnumValue))?
                    as c_char,
                ErrorID: self.error_id,
                ErrorMsg: string_field(&self.error_msg, "ErrorMsg")?,
                IsSwapOrder: self.is_swap_order as std::os::raw::c_int,
                AccountID: self.account_id.to_wire(),
                CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
                ClientID: string_field(&self.client_id, "ClientID")?,
                InvestUnitID: self.invest_unit_id.to_wire(),
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::ParkedOrderAction {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcParkedOrderActionField, ConversionError> {
        let raw = sys::CThostFtdcParkedOrderActionField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            OrderActionRef: self.order_action_ref,
            OrderRef: self.order_ref.to_wire(),
            RequestID: self.request_id.get(),
            FrontID: self.front_id.get(),
            SessionID: self.session_id.get(),
            ExchangeID: self.exchange_id.to_wire(),
            OrderSysID: self.order_system_id.to_wire(),
            ActionFlag: self
                .action_flag
                .code()
                .ok_or_else(|| ConversionError::new("ActionFlag", TextError::UnknownEnumValue))?
                as c_char,
            LimitPrice: self.limit_price,
            VolumeChange: self.volume_change,
            UserID: self.user_id.to_wire(),
            ParkedOrderActionID: string_field(&self.parked_order_action_id, "ParkedOrderActionID")?,
            UserType: self
                .user_type
                .code()
                .ok_or_else(|| ConversionError::new("UserType", TextError::UnknownEnumValue))?
                as c_char,
            Status: self
                .status
                .code()
                .ok_or_else(|| ConversionError::new("Status", TextError::UnknownEnumValue))?
                as c_char,
            ErrorID: self.error_id,
            ErrorMsg: string_field(&self.error_msg, "ErrorMsg")?,
            InvestUnitID: self.invest_unit_id.to_wire(),
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            InstrumentID: self.instrument_id.to_wire(),
            IPAddress: string_field(&self.ip_address, "IPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::RemoveParkedOrder {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcRemoveParkedOrderField, ConversionError> {
        let raw = sys::CThostFtdcRemoveParkedOrderField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ParkedOrderID: string_field(&self.parked_order_id, "ParkedOrderID")?,
            InvestUnitID: self.invest_unit_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::RemoveParkedOrderAction {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcRemoveParkedOrderActionField, ConversionError> {
        let raw = sys::CThostFtdcRemoveParkedOrderActionField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ParkedOrderActionID: string_field(&self.parked_order_action_id, "ParkedOrderActionID")?,
            InvestUnitID: self.invest_unit_id.to_wire(),
        };
        Ok(raw)
    }
}

impl crate::InputExecOrder {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputExecOrderField, ConversionError> {
        let raw =
            sys::CThostFtdcInputExecOrderField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                ExecOrderRef: self.exec_order_ref.to_wire(),
                UserID: self.user_id.to_wire(),
                Volume: self.volume,
                RequestID: self.request_id.get(),
                BusinessUnit: self.business_unit.to_wire(),
                OffsetFlag: self.offset_flag.code().ok_or_else(|| {
                    ConversionError::new("OffsetFlag", TextError::UnknownEnumValue)
                })? as c_char,
                HedgeFlag: self
                    .hedge_flag
                    .code()
                    .ok_or_else(|| ConversionError::new("HedgeFlag", TextError::UnknownEnumValue))?
                    as c_char,
                ActionType: self.action_type.code().ok_or_else(|| {
                    ConversionError::new("ActionType", TextError::UnknownEnumValue)
                })? as c_char,
                PosiDirection: self.position_direction.code().ok_or_else(|| {
                    ConversionError::new("PosiDirection", TextError::UnknownEnumValue)
                })? as c_char,
                ReservePositionFlag: self.reserve_position_flag.code().ok_or_else(|| {
                    ConversionError::new("ReservePositionFlag", TextError::UnknownEnumValue)
                })? as c_char,
                CloseFlag: self
                    .close_flag
                    .code()
                    .ok_or_else(|| ConversionError::new("CloseFlag", TextError::UnknownEnumValue))?
                    as c_char,
                ExchangeID: self.exchange_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                AccountID: self.account_id.to_wire(),
                CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
                ClientID: string_field(&self.client_id, "ClientID")?,
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::InputExecOrderAction {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcInputExecOrderActionField, ConversionError> {
        let raw =
            sys::CThostFtdcInputExecOrderActionField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                ExecOrderActionRef: self.exec_order_action_ref,
                ExecOrderRef: self.exec_order_ref.to_wire(),
                RequestID: self.request_id.get(),
                FrontID: self.front_id.get(),
                SessionID: self.session_id.get(),
                ExchangeID: self.exchange_id.to_wire(),
                ExecOrderSysID: string_field(&self.exec_order_system_id, "ExecOrderSysID")?,
                ActionFlag: self.action_flag.code().ok_or_else(|| {
                    ConversionError::new("ActionFlag", TextError::UnknownEnumValue)
                })? as c_char,
                UserID: self.user_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::InputForQuote {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputForQuoteField, ConversionError> {
        let raw = sys::CThostFtdcInputForQuoteField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            ForQuoteRef: self.for_quote_ref.to_wire(),
            UserID: self.user_id.to_wire(),
            ExchangeID: self.exchange_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            InstrumentID: self.instrument_id.to_wire(),
            IPAddress: string_field(&self.ip_address, "IPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::InputQuote {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputQuoteField, ConversionError> {
        let raw =
            sys::CThostFtdcInputQuoteField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                QuoteRef: self.quote_ref.to_wire(),
                UserID: self.user_id.to_wire(),
                AskPrice: self.ask_price,
                BidPrice: self.bid_price,
                AskVolume: self.ask_volume,
                BidVolume: self.bid_volume,
                RequestID: self.request_id.get(),
                BusinessUnit: self.business_unit.to_wire(),
                AskOffsetFlag: self.ask_offset_flag.code().ok_or_else(|| {
                    ConversionError::new("AskOffsetFlag", TextError::UnknownEnumValue)
                })? as c_char,
                BidOffsetFlag: self.bid_offset_flag.code().ok_or_else(|| {
                    ConversionError::new("BidOffsetFlag", TextError::UnknownEnumValue)
                })? as c_char,
                AskHedgeFlag: self.ask_hedge_flag.code().ok_or_else(|| {
                    ConversionError::new("AskHedgeFlag", TextError::UnknownEnumValue)
                })? as c_char,
                BidHedgeFlag: self.bid_hedge_flag.code().ok_or_else(|| {
                    ConversionError::new("BidHedgeFlag", TextError::UnknownEnumValue)
                })? as c_char,
                AskOrderRef: self.ask_order_ref.to_wire(),
                BidOrderRef: self.bid_order_ref.to_wire(),
                ForQuoteSysID: self.for_quote_system_id.to_wire(),
                ExchangeID: self.exchange_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                ClientID: string_field(&self.client_id, "ClientID")?,
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                ReplaceSysID: self.replace_system_id.to_wire(),
                TimeCondition: self.time_condition.code().ok_or_else(|| {
                    ConversionError::new("TimeCondition", TextError::UnknownEnumValue)
                })? as c_char,
                OrderMemo: string_field(&self.order_memo, "OrderMemo")?,
                SessionReqSeq: self.session_req_seq,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::InputQuoteAction {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputQuoteActionField, ConversionError> {
        let raw =
            sys::CThostFtdcInputQuoteActionField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                QuoteActionRef: self.quote_action_ref,
                QuoteRef: self.quote_ref.to_wire(),
                RequestID: self.request_id.get(),
                FrontID: self.front_id.get(),
                SessionID: self.session_id.get(),
                ExchangeID: self.exchange_id.to_wire(),
                QuoteSysID: self.quote_system_id.to_wire(),
                ActionFlag: self.action_flag.code().ok_or_else(|| {
                    ConversionError::new("ActionFlag", TextError::UnknownEnumValue)
                })? as c_char,
                UserID: self.user_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                ClientID: string_field(&self.client_id, "ClientID")?,
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                OrderMemo: string_field(&self.order_memo, "OrderMemo")?,
                SessionReqSeq: self.session_req_seq,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::InputBatchOrderAction {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcInputBatchOrderActionField, ConversionError> {
        let raw = sys::CThostFtdcInputBatchOrderActionField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            OrderActionRef: self.order_action_ref,
            RequestID: self.request_id.get(),
            FrontID: self.front_id.get(),
            SessionID: self.session_id.get(),
            ExchangeID: self.exchange_id.to_wire(),
            UserID: self.user_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            IPAddress: string_field(&self.ip_address, "IPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::InputOptionSelfClose {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcInputOptionSelfCloseField, ConversionError> {
        let raw = sys::CThostFtdcInputOptionSelfCloseField {
            BrokerID: self.broker_id.to_wire(),
            InvestorID: self.investor_id.to_wire(),
            OptionSelfCloseRef: self.option_self_close_ref.to_wire(),
            UserID: self.user_id.to_wire(),
            Volume: self.volume,
            RequestID: self.request_id.get(),
            BusinessUnit: self.business_unit.to_wire(),
            HedgeFlag: self
                .hedge_flag
                .code()
                .ok_or_else(|| ConversionError::new("HedgeFlag", TextError::UnknownEnumValue))?
                as c_char,
            OptSelfCloseFlag: self.opt_self_close_flag.code().ok_or_else(|| {
                ConversionError::new("OptSelfCloseFlag", TextError::UnknownEnumValue)
            })? as c_char,
            ExchangeID: self.exchange_id.to_wire(),
            InvestUnitID: self.invest_unit_id.to_wire(),
            AccountID: self.account_id.to_wire(),
            CurrencyID: string_field(&self.currency_id, "CurrencyID")?,
            ClientID: string_field(&self.client_id, "ClientID")?,
            MacAddress: string_field(&self.mac_address, "MacAddress")?,
            InstrumentID: self.instrument_id.to_wire(),
            IPAddress: string_field(&self.ip_address, "IPAddress")?,
            ..Default::default()
        };
        Ok(raw)
    }
}

impl crate::InputOptionSelfCloseAction {
    pub(crate) fn to_sys(
        &self,
    ) -> Result<sys::CThostFtdcInputOptionSelfCloseActionField, ConversionError> {
        let raw =
            sys::CThostFtdcInputOptionSelfCloseActionField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                OptionSelfCloseActionRef: self.option_self_close_action_ref,
                OptionSelfCloseRef: self.option_self_close_ref.to_wire(),
                RequestID: self.request_id.get(),
                FrontID: self.front_id.get(),
                SessionID: self.session_id.get(),
                ExchangeID: self.exchange_id.to_wire(),
                OptionSelfCloseSysID: self.option_self_close_system_id.to_wire(),
                ActionFlag: self.action_flag.code().ok_or_else(|| {
                    ConversionError::new("ActionFlag", TextError::UnknownEnumValue)
                })? as c_char,
                UserID: self.user_id.to_wire(),
                InvestUnitID: self.invest_unit_id.to_wire(),
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                ..Default::default()
            };
        Ok(raw)
    }
}

impl crate::InputCombAction {
    pub(crate) fn to_sys(&self) -> Result<sys::CThostFtdcInputCombActionField, ConversionError> {
        let raw =
            sys::CThostFtdcInputCombActionField {
                BrokerID: self.broker_id.to_wire(),
                InvestorID: self.investor_id.to_wire(),
                CombActionRef: self.comb_action_ref.to_wire(),
                UserID: self.user_id.to_wire(),
                Direction: self
                    .direction
                    .code()
                    .ok_or_else(|| ConversionError::new("Direction", TextError::UnknownEnumValue))?
                    as c_char,
                Volume: self.volume,
                CombDirection: self.comb_direction.code().ok_or_else(|| {
                    ConversionError::new("CombDirection", TextError::UnknownEnumValue)
                })? as c_char,
                HedgeFlag: self
                    .hedge_flag
                    .code()
                    .ok_or_else(|| ConversionError::new("HedgeFlag", TextError::UnknownEnumValue))?
                    as c_char,
                ExchangeID: self.exchange_id.to_wire(),
                MacAddress: string_field(&self.mac_address, "MacAddress")?,
                InvestUnitID: self.invest_unit_id.to_wire(),
                FrontID: self.front_id.get(),
                SessionID: self.session_id.get(),
                InstrumentID: self.instrument_id.to_wire(),
                IPAddress: string_field(&self.ip_address, "IPAddress")?,
                ..Default::default()
            };
        Ok(raw)
    }
}
