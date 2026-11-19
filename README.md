# ctp-base

`ctp-base` 是 CTP 行情与交易 API 的安全 Rust 绑定。它保留 CTP 的请求响应、回调、错误码和 `request_id` 语义，同时将原始指针、GBK 文本、定长字段和对象生命周期封装为 Rust 类型。

## 支持平台

| 平台 | 架构 | 状态 |
|---|---|---|
| Windows 10/11 | x86_64 | 支持 |
| Linux | x86_64 | 支持 |
| Windows 32 位 | x86 / i686 | 不支持 |
| macOS | x86_64 / Apple Silicon | 暂未支持 |

当前 crate 只携带 64 位 CTP SDK，不提供 32 位 Windows 库。

## 版本关系

| ctp-base | CTP SDK | 状态 |
|---|---|---|
| 0.1.0 | v6.7.11_20250617 | 历史版本 |
| 0.1.1 | v6.7.13_20260225 | 当前已验证版本 |

生成器会优先使用非商密 CTP SDK 压缩包；普通包不存在时才尝试商密包。SDK 结构体、枚举、字段、主动 API 或 SPI 与审核表不一致时会阻止生成，避免生成不可用的绑定。

## 工具链要求

### Rust

需要 Rust 1.85 或更高版本，当前项目使用 Rust 2024 edition。

安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### Windows

安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，勾选“使用 C++ 的桌面开发”。bindgen 还需要 libclang；如未自动找到，请设置：

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

### Linux

Ubuntu / Debian：

```bash
sudo apt-get update
sudo apt-get install -y build-essential libclang-dev
```

其他发行版需安装等价的 g++、C 标准库开发文件和 libclang。

## 使用流程

1. 复制或发布 `ctp-base` 目录；
2. 复制 `config/ctp-template.toml` 为 `config/ctp.toml`，填写柜台、用户、密码、AppID、授权码和前置地址；
3. 运行测试或示例。

```bash
cargo test
cargo run --example md
cargo run --example trader
```

示例会在当前目录创建 `./flow/` 流量目录。首次连接真实或仿真柜台前，请确认配置正确。

## 配置文件

配置位于：

```text
config/ctp.toml
```

仓库只携带模板 `config/ctp-template.toml`；本地使用时复制为 `config/ctp.toml` 后填写：

```toml
broker_id = "9999"
user_id = "你的用户"
password = "你的密码"
app_id = "你的AppID"
auth_code = "你的授权码"
market_front = "tcp://行情前置:端口"
trade_front = "tcp://交易前置:端口"
instrument = "ag2612"
```

`config/ctp.toml` 已加入 `.gitignore`，不会被提交。

## 基本用法

```rust
use ctp_base::{MdApi, MdConfig, MdSpi};

struct Spi;

impl MdSpi for Spi {}

let api = MdApi::new(
    MdConfig::new("./flow/"),
    Box::new(Spi),
)?;
api.register_front("tcp://行情前置:端口")?;
api.init();
api.close();
```

更多接口请查看 [docs/接口.md](docs/接口.md)。

## 生命周期说明

- `close()` 只释放本地 CTP API，不发送柜台登出请求；
- 需要正常业务登出时先调用 `logout()`，再释放 API；
- 回调参数已经 owned 化，可直接保存或转移；
- 慢 IO 不应在 CTP 原生回调线程中执行，应转移到调用方自己的线程或队列。

## 目录结构

```text
ctp-base/
├── build.rs
├── config/ctp-template.toml  # 配置模板，本地复制为 ctp.toml 使用
├── cpp/                 # C++ SPI 与 API 桥接
├── docs/接口.md
├── examples/            # 行情与交易示例
├── sdk/                 # CTP 头文件与 64 位库
├── src/                 # Rust 公共 API、SPI 与内部桥接
└── tests/
```

`ctp-base` 构建时只使用本目录内的 `sdk/`、`cpp/` 和 `src/`，不需要 `ctp-generator`。
