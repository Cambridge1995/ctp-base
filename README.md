# ctp-base

`ctp-base` 是 CTP 行情与交易 API 的安全 Rust 绑定。它保留 CTP 的请求响应、回调、错误码和 `request_id` 语义，同时将原始指针、GBK 文本、定长字段和对象生命周期封装为 Rust 类型。

## 支持平台

| 平台 | 架构 | 状态 |
|---|---|---|
| Windows 10/11 | x86_64 | 支持 |
| Linux | x86_64 | 支持 |
| macOS | x86_64 / aarch64（Apple Silicon） | 支持 |
| Windows 32 位 | x86 / i686 | 不支持 |

当前 crate 只携带 64 位 CTP SDK，不提供 32 位 Windows 库。

## 版本关系

| ctp-base | CTP SDK | 状态 |
|---|---|---|
| 0.1.3 | v6.7.13_20260225 | 历史版本 |
| 1.0.0 | v6.7.13_20260225 | 历史版本 |
| 1.0.1 | v6.7.13_20260225 | 历史版本 |
| 1.1.0 | v6.7.13_P1_20261103 | 历史版本 |
| 1.1.1 | v6.7.13_P1_20261103 | 历史版本 |
| 1.1.2 | v6.7.13_P1_20261103 | 当前版本（新增 raw/session 分层、逐行查询事件与请求默认值） |

本项目使用的 CTP 版本均为非商密版本。若券商要求使用商密（SM）版本，crates.io 包无法直接替换，
请从 GitHub 克隆源码后按「二次开发与商密（SM）版本」一节操作。

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

### macOS

安装 Xcode 命令行工具，并通过 Homebrew 安装 LLVM（提供 libclang）：

```bash
xcode-select --install
brew install llvm
```

如 bindgen 未自动找到 libclang，请设置：

```bash
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
```

macOS 版 CTP SDK 为 fat dylib，x86_64 与 Apple Silicon（aarch64）使用同一份库。

## 使用流程

1. 复制或拉取 `ctp-base` 目录；
    
    **方法1：**

    若不希望冗余文件，使用时，只需要`sdk/`、`cpp/` 和 `src/`、`build.rs`、`Cargo.lock`、`Cargo.toml`打包到个人项目下的空`ctp-base`中即可。

    **方法2：**

    若不介意冗余文件，直接使用以下命令，克隆到自己项目就行。
    ```bash
    git clone git@github.com:Cambridge1995/ctp-base.git
    # 或者使用
    git clone https://github.com/Cambridge1995/ctp-base.git
    ```

   **方法3：**

   直接通过 crates.io 引入（推荐，无需手动拷贝 SDK 与桥接代码）：

   `Cargo.toml`：

   ```toml
   [dependencies]
   ctp-base = "1.1.2"
   ```

   或使用 cargo 命令：

   ```bash
   cargo add ctp-base@1.1.2
   ```

   引入后按下方「配置文件」准备 `config/ctp.toml` 即可使用；crate 内含三平台
   CTP 库（构建时自动解压），本机仍需 Rust 工具链、C++ 编译器和 libclang（见「工具链要求」）。
   


2. 快速验证：

    复制 `config/ctp-template.toml` 为 `config/ctp.toml`，填写柜台、用户、密码、AppID、授权码和前置地址；
运行测试或示例。

    ```bash
    cargo test
    cargo run --example md
    cargo run --example trader
    ```
    示例会在当前目录创建 `./flow/` 流量目录。首次连接真实或仿真柜台前，请确认配置正确。

## 二次开发与商密（SM）版本

通过 crates.io（`cargo add ctp-base`）引入的是**只读的发布包**，其中
`sdk/binaries.tar.xz` 仅包含非商密（`_se`）三平台库，无法直接替换。

如果需要进行二次开发，或改用商密（SM）版 CTP 接口，请**不要使用 crates.io 方式**，
改为从 GitHub 克隆源码：

```bash
git clone https://github.com/Cambridge1995/ctp-base.git
```

源码方式下，只需把对应平台的库文件放入 `sdk/<平台>/`（`windows`、`linux`、`macos`），
构建脚本会自动识别，无需修改代码：

- 目录中只有非商密库时使用 `_se`；没有 `_se` 时自动降级使用 `_sm`
  （匹配 `thostmduserapi_*` / `thosttraderapi_*`）；
- 也可用你自己的库重新打包 `sdk/binaries.tar.xz`（目录结构
  `windows/`、`linux/`、`macos/`），首次构建会解压到 `target/ctp-sdk-libs/<平台>/`；
- 替换库文件后删除 `target/ctp-sdk-libs/<平台>/`，下次构建会重新解压或直接使用新文件。

crates.io 上的历史版本内容不可修改，上述定制请以 GitHub 源码为准。

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
use ctp_base::{MdConfig, MdSession};

let session = MdSession::new(&MdConfig {
    broker_id: "9999".to_owned(),
    user_id: "投资者代码".to_owned(),
    fronts: vec!["tcp://行情前置:端口".to_owned()],
    flow_dir: "./flow/".to_owned(),
    subscribe: vec!["ag2612".to_owned()],
    production_mode: false,
    ..Default::default()
})?;
println!("交易日：{}", session.trading_day());
session.exit();
```

### 查询响应与调用方视图

`TraderSessionEvent::QueryRow` 在每个查询响应到达时立即发送，包含 `request_id`、`is_last`、错误信息和类型化载荷。Session 不再等待 `is_last` 后发送整批结果；空载荷末行也会发送。`OnRspError` 映射为携带 `request_id` 的 `QueryError`，供调用方结束对应批次。

调用方逐行更新自己的视图；需要完整批次时，自行创建 `QueryRows` 并逐行追加，直到 `is_last`。`OnRspError`、`Disconnected`、超时和完整性清理由调用方负责。`examples/common/query.rs` 展示了不依赖底座聚合器的手动聚合方式。

更多原始接口请查看 [raw层接口.md](docs/raw层接口.md)。开发、仿真与生产环境的常见问题和故障处置请查阅 [CTP 开发、测试与生产知识库](docs/CTP开发知识库.md)；Session 接口的开发、测试与生产审核状态见 [Session层接口.md](docs/Session层接口.md)；PeopleQuant 本地校验与后续增强参考见 [功能增强.md](docs/功能增强.md)。

## 在 GitHub Actions 中运行示例

示例支持两种配置来源：环境变量优先，缺失时读取 `config/ctp.toml`。

推送仓库后，在 `Settings → Secrets and variables → Actions` 配置
`CTP_BROKER_ID`、`CTP_USER_ID`、`CTP_PASSWORD`、`CTP_APP_ID`、`CTP_AUTH_CODE`、
`CTP_MARKET_FRONT`、`CTP_TRADE_FRONT` 七个 Repository secrets，
然后到 `Actions → examples → Run workflow` 手动触发，即可在
Windows / Linux / macOS 三个平台并行运行真实连接示例。

## 生命周期说明

- `close()` 只释放本地 CTP API，不发送柜台登出请求；
- 需要正常业务登出时先调用 `logout()`，再释放 API；
- 回调参数已经 owned 化，可直接保存或转移；
- 慢 IO 不应在 CTP 原生回调线程中执行，应转移到调用方自己的线程或队列。

## 许可证

本 crate 的 Rust 代码采用 **MIT OR Apache-2.0** 双许可，详见 `LICENSE-MIT` 与 `LICENSE-APACHE`。

`sdk/binaries.tar.xz` 中的 CTP 动态库版权归上海期货信息技术有限公司（上期技术）所有，
仅随本 crate 作为官方接口的绑定依赖原样再分发，适用其官方使用协议，详见 `NOTICE`。

## 目录结构

```text
ctp-base/
├── build.rs             # Cargo 构建脚本（无需手动运行）：生成 FFI 绑定、
│                        # 编译 C++ 桥接、按平台链接 CTP 库
├── config/ctp-template.toml  # 配置模板，本地复制为 ctp.toml 使用
├── cpp/                 # C++ SPI 与 API 桥接
├── docs/raw层接口.md     # 原始 API/SPI 对应关系
├── docs/Session层接口.md # Session 三栏审核状态与证据
├── examples/            # 行情、交易与 SimNow 审核示例
├── sdk/                 # CTP 头文件与 xz 压缩的三平台库（首次构建解压到 target/ctp-sdk-libs，之后复用）
├── src/                 # Rust 公共 API、SPI 与内部桥接
└── tests/
```

`ctp-base` 构建时只使用本目录内的 `sdk/`、`cpp/` 和 `src/`、`build.rs`、`Cargo.lock`、`Cargo.toml`。