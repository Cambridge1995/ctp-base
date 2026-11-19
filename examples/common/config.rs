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
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("config/ctp.toml");
        let content = fs::read_to_string(&path)?;
        Self::from_simple_toml(&content).map_err(Into::into)
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
