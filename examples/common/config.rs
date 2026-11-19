use std::{error::Error, fs, path::Path};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtpConfig {
    pub broker_id: String,
    pub user_id: String,
    pub password: String,
    pub app_id: String,
    pub auth_code: String,
    pub market_front: String,
    pub trade_front: String,
    pub instrument: String,
}

impl CtpConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // 优先读取环境变量（用于 GitHub Actions 等无配置文件的环境），
        // 未设置的项回退到本地 config/ctp.toml。
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config/ctp.toml");
        match fs::read_to_string(&path) {
            Ok(content) => {
                let mut config = Self::from_simple_toml(&content)?;
                config.apply_environment();
                Ok(config)
            }
            Err(_) => {
                let config = Self::from_environment().ok_or_else(
                    || "缺少配置文件 config/ctp.toml，且未通过 CTP_* 环境变量提供连接参数",
                )?;
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
            self.market_front = value;
        }
        if let Some(value) = Self::env("CTP_TRADE_FRONT") {
            self.trade_front = value;
        }
        if let Some(value) = Self::env("CTP_INSTRUMENT") {
            self.instrument = value;
        }
    }

    fn env(name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.is_empty())
    }

    fn from_environment() -> Option<Self> {
        Some(Self {
            broker_id: Self::env("CTP_BROKER_ID")?,
            user_id: Self::env("CTP_USER_ID")?,
            password: Self::env("CTP_PASSWORD")?,
            app_id: Self::env("CTP_APP_ID")?,
            auth_code: Self::env("CTP_AUTH_CODE")?,
            market_front: Self::env("CTP_MARKET_FRONT")?,
            trade_front: Self::env("CTP_TRADE_FRONT")?,
            instrument: Self::env("CTP_INSTRUMENT").unwrap_or_else(|| "ag2612".to_string()),
        })
    }

    fn from_simple_toml(content: &str) -> Result<Self, String> {
        let mut broker_id = None;
        let mut user_id = None;
        let mut password = None;
        let mut app_id = None;
        let mut auth_code = None;
        let mut market_front = None;
        let mut trade_front = None;
        let mut instrument = None;
        for (index, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(format!("第 {} 行缺少 =：{line}", index + 1));
            };
            let value = parse_toml_string(value.trim(), index + 1)?;
            match key.trim() {
                "broker_id" => broker_id = Some(value),
                "user_id" => user_id = Some(value),
                "password" => password = Some(value),
                "app_id" => app_id = Some(value),
                "auth_code" => auth_code = Some(value),
                "market_front" => market_front = Some(value),
                "trade_front" => trade_front = Some(value),
                "instrument" => instrument = Some(value),
                other => return Err(format!("第 {} 行存在未知配置项：{other}", index + 1)),
            }
        }
        Ok(Self {
            broker_id: required(broker_id, "broker_id")?,
            user_id: required(user_id, "user_id")?,
            password: required(password, "password")?,
            app_id: required(app_id, "app_id")?,
            auth_code: required(auth_code, "auth_code")?,
            market_front: required(market_front, "market_front")?,
            trade_front: required(trade_front, "trade_front")?,
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

fn required(value: Option<String>, name: &str) -> Result<String, String> {
    value.ok_or_else(|| format!("缺少配置项：{name}"))
}
