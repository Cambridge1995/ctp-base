use crate::ConversionError;

pub(crate) fn error_message(result: Result<i32, ConversionError>, action: &str) -> Option<String> {
    match result {
        Ok(0) => None,
        Ok(code) => Some(format!("{action}发送失败，CTP 返回码 {code}")),
        Err(error) => Some(format!("{action}请求转换失败：{error}")),
    }
}

pub(crate) fn emit_request_error(
    result: Result<i32, ConversionError>,
    action: &str,
    emit: impl FnOnce(String),
) {
    if let Some(message) = error_message(result, action) {
        emit(message);
    }
}
