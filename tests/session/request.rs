use super::enums::{MdSessionError, MdSessionEvent, TraderSessionEvent};
use super::md::report_md_request_failure;
use super::request::{emit_request_error, error_message};
use super::trader::{TraderSessionError, report_request_failure};
use crate::{ConversionError, TextError};

#[test]
fn successful_request_has_no_error_message() {
    assert_eq!(error_message(Ok(0), "认证"), None);
}

#[test]
fn nonzero_ctp_codes_are_send_failures() {
    for code in [-1, -2, -3] {
        assert_eq!(
            error_message(Ok(code), "认证"),
            Some(format!("认证发送失败，CTP 返回码 {code}"))
        );
    }
}

#[test]
fn conversion_error_names_action_and_field() {
    let error = ConversionError::new("Password", TextError::TooLong { maximum: 40 });
    let message = error_message(Err(error), "登录").expect("转换失败必须返回错误文本");
    assert!(message.starts_with("登录请求转换失败："), "{message}");
    assert!(message.contains("Password"), "{message}");
}

#[test]
fn emit_request_error_only_reports_failures() {
    let mut messages = Vec::new();
    emit_request_error(Ok(-3), "订阅行情", |message| messages.push(message));
    assert_eq!(messages, ["订阅行情发送失败，CTP 返回码 -3"]);

    let mut success_messages = Vec::new();
    emit_request_error(Ok(0), "退订行情", |message| {
        success_messages.push(message)
    });
    assert!(success_messages.is_empty());
}

#[test]
fn first_handshake_failure_returns_request_error() {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let mut handshake = Some(sender);
    let mut events = Vec::new();
    report_request_failure(
        &mut handshake,
        "认证发送失败，CTP 返回码 -3".to_owned(),
        |event| events.push(event),
    );

    match receiver.recv().expect("首登失败必须结束握手") {
        Err(TraderSessionError::Request(message)) => {
            assert_eq!(message, "认证发送失败，CTP 返回码 -3")
        }
        other => panic!("首登错误类型不正确：{other:?}"),
    }
    assert!(handshake.is_none());
    assert!(events.is_empty());
}

#[test]
fn reconnect_failure_emits_error_event() {
    let mut handshake = None;
    let mut events = Vec::new();
    report_request_failure(
        &mut handshake,
        "登录发送失败，CTP 返回码 -1".to_owned(),
        |event| events.push(event),
    );

    assert_eq!(events.len(), 1);
    match &events[0] {
        TraderSessionEvent::Error { error_id, message } => {
            assert_eq!(*error_id, -1);
            assert_eq!(message, "登录发送失败，CTP 返回码 -1");
        }
        other => panic!("重连应发送 Error 事件：{other:?}"),
    }
}

#[test]
fn first_md_login_failure_returns_request_error() {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let mut handshake = Some(sender);
    let mut events = Vec::new();
    report_md_request_failure(
        &mut handshake,
        "行情登录发送失败，CTP 返回码 -3".to_owned(),
        |event| events.push(event),
    );

    match receiver.recv().expect("行情首登失败必须结束握手") {
        Err(MdSessionError::Request(message)) => {
            assert_eq!(message, "行情登录发送失败，CTP 返回码 -3")
        }
        other => panic!("行情首登错误类型不正确：{other:?}"),
    }
    assert!(handshake.is_none());
    assert!(events.is_empty());
}

#[test]
fn reconnect_md_login_failure_emits_error_event() {
    let mut handshake = None;
    let mut events = Vec::new();
    report_md_request_failure(
        &mut handshake,
        "行情登录发送失败，CTP 返回码 -1".to_owned(),
        |event| events.push(event),
    );

    assert_eq!(events.len(), 1);
    match &events[0] {
        MdSessionEvent::Error { error_id, message } => {
            assert_eq!(*error_id, -1);
            assert_eq!(message, "行情登录发送失败，CTP 返回码 -1");
        }
        other => panic!("行情重连应发送 Error 事件：{other:?}"),
    }
}
