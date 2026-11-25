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
        // 优先读取环境变量（用于 GitHub Actions 等无配置文件的环境），
        // 未设置的项回退到本地 config/ctp.toml 的 CTP_PROFILE 指定档案（默认 simnow）。
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config/ctp.toml");
        match fs::read_to_string(&path) {
            Ok(content) => {
                let profile = Self::env("CTP_PROFILE").unwrap_or_else(|| "simnow".to_owned());
                let mut config = Self::from_profile_toml(&content, &profile)?;
                config.apply_environment();
                Ok(config)
            }
            Err(_) => {
                let config = Self::from_environment()
                    .ok_or("缺少配置文件 config/ctp.toml，且未通过 CTP_* 环境变量提供连接参数")?;
                Ok(config)
            }
        }
    }

    fn apply_environment(&mut self) {
        if let Some(value) = Self::env("CTP_BROKER_ID") {
            self.broker_id = value;
        }
        if let Some(value) = Self::env("CTP_USER_ID") {
            self.user_id = value;
        }
        if let Some(value) = Self::env("CTP_PASSWORD") {
            self.password = value;
        }
        if let Some(value) = Self::env("CTP_APP_ID") {
            self.app_id = value;
        }
        if let Some(value) = Self::env("CTP_AUTH_CODE") {
            self.auth_code = value;
        }
        if let Some(value) = Self::env("CTP_MARKET_FRONT") {
            self.market_fronts = vec![value];
        }
        if let Some(value) = Self::env("CTP_TRADE_FRONT") {
            self.trade_fronts = vec![value];
        }
        if let Some(value) = Self::env("CTP_INSTRUMENT") {
            self.instrument = value;
        }
    }

    fn env(name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.is_empty())
    }

    fn from_environment() -> Option<Self> {
        let market_fronts = vec![Self::env("CTP_MARKET_FRONT")?];
        let trade_fronts = vec![Self::env("CTP_TRADE_FRONT")?];
        Some(Self {
            broker_id: Self::env("CTP_BROKER_ID")?,
            user_id: Self::env("CTP_USER_ID")?,
            password: Self::env("CTP_PASSWORD")?,
            app_id: Self::env("CTP_APP_ID")?,
            auth_code: Self::env("CTP_AUTH_CODE")?,
            market_front: market_fronts[0].clone(),
            trade_front: trade_fronts[0].clone(),
            market_fronts,
            trade_fronts,
            flow: "./flow/".to_owned(),
            instrument: Self::env("CTP_INSTRUMENT").unwrap_or_else(|| "ag2612".to_string()),
        })
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
