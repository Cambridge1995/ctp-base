use std::{error::Error, fs, path::Path};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtpConfig {
    pub broker_id: String,
    pub user_id: String,
    pub password: String,
    pub app_id: String,
    pub auth_code: String,
    /// 首个行情前置（兼容字段；完整列表见 `market_fronts`）。
    pub market_front: String,
    /// 首个交易前置（兼容字段；完整列表见 `trade_fronts`）。
    pub trade_front: String,
    pub market_fronts: Vec<String>,
    pub trade_fronts: Vec<String>,
    /// CTP 流量文件目录。
    pub flow: String,
    pub instrument: String,
}

impl CtpConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config/ctp.toml");
        // CI 与全新检出没有 ctp.toml，使用纯离线默认值，保证单元测试可运行。
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => return Ok(Self::offline_defaults()),
        };
        let profile = std::env::var("CTP_PROFILE").unwrap_or_else(|_| "simnow".to_owned());
        Self::from_profile_toml(&content, &profile).map_err(Into::into)
    }

    fn offline_defaults() -> Self {
        Self {
            broker_id: "9999".into(),
            user_id: "test_user".into(),
            password: "test_password".into(),
            app_id: "simnow_client_test".into(),
            auth_code: "0000000000000000".into(),
            market_front: String::new(),
            trade_front: String::new(),
            market_fronts: Vec::new(),
            trade_fronts: Vec::new(),
            flow: "./flow/".into(),
            instrument: "ag2612".into(),
        }
    }

    /// 解析 `[ctp.profiles.<profile>]` 段的扁平键值与字符串数组；段外内容（含 version 行）忽略。
    fn from_profile_toml(content: &str, profile: &str) -> Result<Self, String> {
        let target = format!("[ctp.profiles.{profile}]");
        let mut in_section = false;
        let mut section_seen = false;
        let mut broker_id = None;
        let mut user_id = None;
        let mut password = None;
        let mut app_id = None;
        let mut auth_code = None;
        let mut market_fronts = None;
        let mut trade_fronts = None;
        let mut flow = None;
        let mut instrument = None;
        for (index, raw) in content.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') {
                in_section = line == target;
                section_seen |= in_section;
                continue;
            }
            if !in_section {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(format!("第 {} 行缺少 =：{line}", index + 1));
            };
            let value = value.trim();
            match key.trim() {
                "broker_id" => broker_id = Some(parse_toml_string(value, index + 1)?),
                "user_id" => user_id = Some(parse_toml_string(value, index + 1)?),
                "password" => password = Some(parse_toml_string(value, index + 1)?),
                "app_id" => app_id = Some(parse_toml_string(value, index + 1)?),
                "auth_code" => auth_code = Some(parse_toml_string(value, index + 1)?),
                "market_fronts" => market_fronts = Some(parse_toml_array(value, index + 1)?),
                "trade_fronts" => trade_fronts = Some(parse_toml_array(value, index + 1)?),
                "flow" => flow = Some(parse_toml_string(value, index + 1)?),
                "instrument" => instrument = Some(parse_toml_string(value, index + 1)?),
                other => return Err(format!("第 {} 行存在未知配置项：{other}", index + 1)),
            }
        }
        if !section_seen {
            return Err(format!("配置文件中不存在档案段：{target}"));
        }
        let market_fronts = required_fronts(market_fronts, "market_fronts", profile)?;
        let trade_fronts = required_fronts(trade_fronts, "trade_fronts", profile)?;
        Ok(Self {
            broker_id: required(broker_id, "broker_id")?,
            user_id: required(user_id, "user_id")?,
            password: required(password, "password")?,
            app_id: required(app_id, "app_id")?,
            auth_code: required(auth_code, "auth_code")?,
            market_front: market_fronts[0].clone(),
            trade_front: trade_fronts[0].clone(),
            market_fronts,
            trade_fronts,
            flow: flow.unwrap_or_else(|| "./flow/".to_owned()),
            instrument: required(instrument, "instrument")?,
        })
    }
}

fn parse_toml_string(value: &str, line: usize) -> Result<String, String> {
    let Some(inner) = value
        .strip_prefix('"')
        .and_then(|text| text.strip_suffix('"'))
    else {
        return Err(format!("第 {line} 行必须是双引号字符串：{value}"));
    };
    let mut output = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        match chars.next() {
            Some('"') => output.push('"'),
            Some('\\') => output.push('\\'),
            Some('n') => output.push('\n'),
            Some(other) => return Err(format!("第 {line} 行包含不支持转义：\\{other}")),
            None => return Err(format!("第 {line} 行以未完成的转义结尾")),
        }
    }
    Ok(output)
}

/// 解析单行字符串数组（`["a", "b"]`；元素间逗号分隔，不允许跨行）。
fn parse_toml_array(value: &str, line: usize) -> Result<Vec<String>, String> {
    let Some(inner) = value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) else {
        return Err(format!("第 {line} 行必须是字符串数组：{value}"));
    };
    let mut items = Vec::new();
    for item in inner.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        items.push(parse_toml_string(item, line)?);
    }
    Ok(items)
}

fn required(value: Option<String>, name: &str) -> Result<String, String> {
    value.ok_or_else(|| format!("缺少配置项：{name}"))
}

fn required_fronts(
    value: Option<Vec<String>>,
    name: &str,
    profile: &str,
) -> Result<Vec<String>, String> {
    let fronts = value.ok_or_else(|| format!("缺少配置项：{name}"))?;
    if fronts.is_empty() {
        return Err(format!("档案 {profile} 的 {name} 不能为空数组"));
    }
    Ok(fronts)
}
