# 开发 FAQ（问题 / 原因 / 解法）

> 记录开发过程中真实遇到的问题、陷阱与 bug 及解决方法，供复查与新人对齐。
> **适用范围**：本文以本地开发和 SimNow 实测为主，只作为开发环境的问题参考；
> 单次仿真结果不能直接推广为期货公司仿真或生产柜台的通用行为。
> 测试环境、生产环境、故障恢复、对账与上线清单请查阅 [CTP 开发、测试与生产知识库](CTP开发知识库.md)。
> 新接口开发前必须检索本文和知识库；Session 接口的开发、测试与生产审核状态及证据见 [Session层接口.md](Session层接口.md)，强制流程见根目录 `AGENTS.md`。
> 本文已按 ctp-base 当前实现（托管层批次一~七）核对修订，过期条目已删除或改写；
> 本文包含经当前代码和资料核对后的项目协议细节；跨环境规则以知识库列出的当前资料和目标期货公司要求为准。

## 一、Cargo 与工程结构

### Q：IDE 里测试看起来"卡住"，输出停在 `Running tests\...` 不动？

**现象**：RustRover 运行测试，输出停在启动行没有下文。
**原因**：多数情况只是 IDE 输出窗口的显示时机问题——命令行用同一条命令复现是正常的。
**解法**：先用命令行复现（`cargo test --test <name>`）排除真实挂死；
确认代码无恙后重启 IDE 运行即可。

### Q：第二个 cargo 命令长时间不产出，疑似卡死？

**原因**：上一个 cargo 进程（如 IDE 后台任务）残留，持有 target 目录的文件锁，
后来的 cargo 无限等待，而且默认不打印任何提示。
**解法**：`tasklist //FI "IMAGENAME eq cargo.exe"` 找到残留进程，`taskkill //PID <pid> //F` 清理。

### Q：`toml = "1.0.3+spec-1.1.0"` 刷 semver 元数据警告？

**原因**：cargo 忽略版本号中 `+` 后的构建元数据并告警。
**解法**：只写 `"1.0.3"`，想表达的规范版本写进旁边注释。

### Q：ctp-base 的 SDK 包（sdk/binaries.tar.xz）为什么那么小？

**查证**：tar.xz（LZMA2），34.5MB → 6.8MB，约 5:1。
gzip/deflate 天花板低；zstd 级别 19 仍逊一筹；xz preset 9 最高。
**关键技巧是 tar 固体压缩**：多文件打包后跨文件去重（三平台同一源码的库有大量相同机器码）。

## 二、Rust 语言与并发

### Q：API 对象（MdApi/TraderApi）不能 move 进线程，`is not Send`？

**原因**：内含原生指针，编译器无法证明跨线程安全。
**解法**：API 在 loop 线程**内部**创建并使用（`MdLoop::new` / `TraderLoop::new` 在线程闭包里调用），
外部命令经通道传入。这也是"句柄 + loop 状态"两个 struct 分开的根本原因：
`MdSession`/`TraderSession`（用户线程的句柄：发送端/接收端/线程句柄）与
`MdLoop`/`TraderLoop`（loop 线程独占的状态）字段零重叠，actor 模式的标准形态，不是冗余。

### Q：loop 要同时收 SPI 事件和用户命令，怎么阻塞等待？

**原因**：Rust 标准库通道不能同时阻塞等两个接收端（没有 select），
最初用"双 try_recv + sleep 5ms"轮询是权宜。
**解法**：两类输入合并进**同一条队列**（`enum Input { Spi(..), Command(..) }`），
loop 用阻塞 `recv()`——有事件才醒，零空转。
（老 session 的 5ms poll 是因为有定时任务：查询限流、重连计时；当前 loop 没有定时任务，纯阻塞即可。）

### Q：Drop 时线程 join 死锁？

**原因**：用 `try_send` 发 Shutdown，队列满时命令丢失，worker 排空后永远阻塞在 `recv()`，
而 drop 期间发送端仍存活，永远等不到 Disconnected。
**解法**：Shutdown 用阻塞 `send()`——worker 存活时总能排出一个槽位。
当前 `MdSession::shutdown` / `TraderSession::shutdown` 即按此实现。

### Q：`self.api.xxx(&request, self.next_request_id())` 编译报 E0502 借用冲突？

**原因**：同一表达式里 `self.api`（不可变借用）与 `self.next_request_id()`（可变借用）冲突。
**解法**：先 `let request_id = self.next_request_id();` 取值，再调用。
当前代码全部按此模式书写。

### Q：文档注释报错 / clippy 警告？

- 函数**参数**上不能用 `///`（documentation comments cannot be applied to function parameters）——
  写到方法的文档注释里；
- doc 注释后紧跟空行会触发 `empty_line_after_doc_comments`——
  自由说明文字用普通 `//` 注释，不要 `///`。

### Q：clippy 报 `module_inception`（session::session）？

**解法**：内层文件按内容改名（session/session.rs → session/md.rs），
为对称的 session/trader.rs 留位。已成历史。

### Q：`ambiguous glob re-exports` / 内部类型被意外公开？

**真实事故**（2026-11）：`src/session/mod.rs` 与 `src/lib.rs` 被改成 glob 导出
（`pub use spi::*` 等），导致三个问题——内部适配器 `spi`/`trader_spi` 被公开化、
`session::md` 与 `spi::md` 在公开 API 撞名（`ctp_base::md` 歧义）、5 个 clippy 警告。
**解法**：托管层模块坚持**显式导出**（只暴露 Session/Config/Error/Event/QueryRows），
内部模块保持私有。glob 导出在多层嵌套 crate 里是公开 API 事故的高发区。

### Q：clippy 报 `large size difference between variants`？

**原因**：事件枚举里 `Order`/`Trade` 等结构体超 1KB，内联在 enum 里使每个值都按最大变体分配。
**解法**：大载荷一律 `Box<T>`（`OrderUpdate(Box<Order>)`、`QueryRow::Order(Box<Order>)` 等）。
本项目的约定：超过几十字节的载荷在事件与命令枚举中全部装箱。

## 三、异步与队列

### Q：队列写死 4096 合适吗？能不能动态扩容？

**结论**：容量做成可配（`queue_capacity`，0 = 默认 4096），但**不做动态扩容**：
无界队列把"丢数据"升级成 OOM（交易网关优化的公开复盘里，无界队列是雪崩原因之一），
且堆积意味着用陈旧行情做决策（新鲜度 > 完整性）。
tick 可再生（丢了下一秒有新的），成交回报不可再生——所以 md 通道敢丢、trade 通道要告警。
真正会顶破容量的场景是**脉冲式到达**：CTP 每 500ms 切片一次，全市场订阅时几千条 tick
在同一瞬间涌来。
**观测出口**：`dropped_count()`（SPI 与 loop 双侧共享计数），队列满丢弃必须计数——
这是"坏消息不静默"的量化形式。

## 四、配置

### Q：ctp.toml 平铺写字段为什么报错？现在的格式是什么？

**原因**：配置文件是档案式结构（`[ctp.profiles.<名字>]` 段 + 前置数组），
平铺字段不在任何段内会被解析器拒绝或忽略。
**当前格式**（`config/ctp.toml`）：

```toml
version = 1

[ctp.profiles.simnow]
broker_id = "9999"
user_id = "..."
password = "..."
app_id = "simnow_client_test"
auth_code = "..."
market_fronts = ["tcp://...", "tcp://..."]
trade_fronts = ["tcp://...", "tcp://..."]
flow = "./flow/"
instrument = "ag2612"
```

- 档案由 `CTP_PROFILE` 环境变量选择（默认 `simnow`）；`CTP_*` 环境变量可逐项覆盖；
- 解析器是 `examples/common/config.rs` 与 `tests/common/config.rs` 里的手写实现
  （零依赖，只认档案段内已知键，段外内容忽略）。
**教训**：配置文件结构演进时**两份解析器必须同步**——曾发生配置重构后
`tests/unit/raw.rs` 两个用例连带失败（它们经 config 加载构造请求）。

## 五、CTP 协议行为（实测/查证）

### Q：错误密码登录 simnow 行情端，居然成功了？

**实测**（2026-11-23）：错误 broker_id + user_id + password 三重全错也登录成功。
**官方口径**：行情端登录默认无必填项；6.6.2 起柜台可开启"行情身份校验"
（届时 broker_id/user_id 必须正确且与交易端一致）。**凭据校验在交易端**
（BrokerID/UserID/Password 必填，穿透式环境通常还需 AppID/AuthCode 共 5 个配置项）。
详细字段和适用边界见本文第八节。

### Q：登录响应里 FrontID/SessionID 是 0？

行情端恒为 0——会话标识是交易端概念。交易端登录才返回真实值。
（衍生设计：行情端握手只回传交易日；交易端 `TraderSession::front_id()/session_id()`
随重连刷新。）

### Q：logout 为什么收不到回执？现在 `exit()` 还发登出请求吗？

**这是本文被修订最多的条目，以当前实现为准：**

- **行情端**：前置不支持登出（返回错误 77「CTP:无此功能」），回执永远不会到达；
- **交易端**：官方《CTP 客户端开发指南》记载"登出会先将现有连接断开，再重新建立一个新连接，
  重新登录后 SessionID 会重置"——**登出不是关闭，是换会话的重连**；
- **当前实现**：两侧 Session 的 `exit()`/`Drop` **不再发送 `ReqUserLogout`**，
  关闭 = 通知 loop 退出 + join 线程 + 原生 API 析构（`Release`）——销毁实例才是 CTP 真正的关闭语义
  （与 vnpy 等框架的 close 路径一致）。柜台在 TCP 断开后自行释放会话。
  交易端的"登出=断开重连"行为被正面利用为 `TraderSession::reconnect()`（网络不稳定时主动重连）。
- **版本边界**：该行为出自 2015-2016 年指南，现行 API 参考未再承诺；若目标版本改为优雅登出，`reconnect` 必须改为销毁重建。
- **raw 层取舍**：`MdApi::logout` / `TraderApi::logout` 仍保留为 CTP 原始镜像；Session 不发送行情登出，交易侧只把 `Logout` 作为会话事件，不把登出当关闭。
- 实测（simnow 2026-11-24）：`exit()` 15.4 毫秒完成 join + Release，原地可立即重建会话。

### Q：断线了谁负责重连？重连后要做什么？

CTP 自动重连**只恢复传输层**；重连地址从已注册的前置里自动选
（`RegisterFront` 可注册多个，随机选择）。**重登录、重订阅、（交易端）重新认证是客户端的职责**——
托管层的 `Connected → 认证链 → 就绪` 自动化就干这个，完成后发 `Recovered` 事件。
**重连后必须做的对账**：私有流 Quick 模式不重放历史，断线期间的成交/撤单/转账状态变化会丢——
业界标准对策是恢复后以柜台查询为准重查（挂单 `query_order`、持仓 `query_investor_position`、
资金 `query_trading_account`、转账 `query_transfer_serial`），`Recovered` 事件就是触发钩子。
另注意：重连是新会话（SessionID 变），OrderRef/OrderActionRef 归零重计
。

### Q：request_id 要自增吗？为什么回报里的对不上？

- 回声标签，协议**不强制**递增；强制递增的是 `OrderRef`/`OrderActionRef`（违反报 ErrorID 22）。
- `OnRsp*`（响应）回声 request_id；`OnRtn*`/`OnErrRtn*`（回报）**不带**，
  报单对账走 FrontID+SessionID+OrderRef。
- 查询多行响应以 request_id + is_last 标识；Session 逐行转发，调用方按需聚合并管理完整性。
- 托管层统一自增分配，不混用固定值。

### Q：报单被拒有哪几条路径？为什么有时候 OnRspOrderInsert 没来，拒单却到了？

三条路径，一条都不能漏：

1. **CTP 前置拒单（应答路径）**：`OnRspOrderInsert` 带 request_id 和错误码；
2. **CTP 拒单（回报路径）**：`OnErrRtnOrderInsert`，**不带 request_id**，按 OrderRef 对账；
3. **交易所拒单（被动撤单）**：`OnRtnOrder` 报单状态变 `Canceled` 且 `StatusMsg` 带原因。

托管层分别映射为 `OrderInsertResponse` / `OrderInsertError` / `OrderUpdate` 三类事件。
撤单同理（`OnRspOrderAction` + `OnErrRtnOrderAction`）。

### Q：`Req*` 的返回码非 0 是什么？需要处理吗？

`ReqOrderInsert` 等函数的**返回值**（不是报文字段）：0 发送成功（仅表示报文发出）、
-1 网络连接失败、-2 未处理请求超过许可数、-3 每秒请求数超过许可数（流控）。
交易指令和查询不能忽略它——流控拒绝时请求根本没发出去，傻等应答就会超时。托管层 `check_send_code` 对指令/查询的非 0 返回直接报错给调用方（`TraderSessionError::Request`），重发策略由策略层决定。

**已修复（2026-11-25）**：认证、登录和结算确认统一检查 `Ok(非 0)`，首登立即返回 `TraderSessionError::Request`，重连发送 `Error(error_id=-1)`；行情登录、订阅、退订及询价订退也检查转换错误与非零返回码并明确上报。`Ok(0)` 仍只表示报文发出，不代表柜台或交易所已接受业务。

### Q：查询批次为什么可能等不到结束（is_last）？

三个真实踩过的坑，当前职责划分如下：

1. **空载荷行被跳过**：某些行载荷为 None，若 SPI 跳过不发事件，调用方就等不到结束边界；
   当前 SPI 对空载荷行**照发**，Session 也立即公开 `QueryRow`，不在内部囤积到末行；
2. **`OnRspError` 异步拒绝**：请求返回 0 不代表受理，查询可能随后被 `OnRspError` 拒绝，
   此时 `OnRsp*Qry*` 不会到达——Session 发送携带 request_id 的 `QueryError`，调用方结束受影响的在途批次；
3. **断线重置**：查询流不被柜台维护状态，断线后在途批次注定等不到——
   Session 原样发送 `Disconnected`，调用方清理自己的批次。

调用方直接逐行消费并自行聚合；完整视图的关联、超时、失败清理和发布均由调用方负责。`examples/common/query.rs` 提供手动聚合示例，底座不提供聚合器。

### Q：Logout 和 Disconnected 哪个先到？可以按顺序写状态机吗？

**实测两次顺序相反**（simnow 2026-11-24）：一次 Disconnected 先于 Logout，
一次 Logout 先于 Disconnected，且重建过程可能出现多次 Disconnected。
**结论**：消费事件链不得假设顺序或次数，只对完整链路（最终到达 `Recovered`）断言。
该结论已纳入本文重连规则。

### Q：银期转账明明确认受理了，钱怎么没动？

**受理≠成功**：期商发起转账是两段式——
`OnRsp*ByFuture`（应答，带 request_id）只表示前置**受理**，
资金是否到账以 `OnRtn*ByFuture`（处理结果回报）为准；
`OnErrRtn*` 是第三条失败路径。策略记账必须等结果回报。
断线时在途应答可能丢失（对话流重置），资金状态一律以 `query_transfer_serial`
流水查询对账（`Recovered` 后重查）——流水是柜台侧的事实来源。

### Q：为什么报"资金密码必须手动传入"？

`ReqTransfer`/`ReqQueryAccount` 的 `password` 是**银行端资金密码**，与登录密码不同，
会话不代填且**校验非空**——调用方留空时本地直接报错（`TraderSessionError::Request`），
不让空密码流向柜台才被拒。业务功能码（入金 202001/出金 202002）则由 loop 按方向填充。
`future_serial`（期货流水号）也不代填：CTP 要求客户端维护递增流水，部分柜台校验唯一性，
按"业务字段调用方决定"约定交给调用方（vnpy 等实现填 0 可过 simnow）。

### Q：预埋单回报里的 ParkedOrderID 为什么不能填回请求里？

`ParkedOrderID`/`ParkedOrderActionID` 由 CTP 分配。调用方复用回报结构体再申报时，
残留的旧 ID 会被发回柜台导致拒绝或行为未定义——loop 发送前**清空**这两个字段。
删除预埋单/预埋撤单（`RemoveParkedOrder`/`RemoveParkedOrderAction`）才按 ID 定位，
此时 ID 由调用方从回报中取出填入。

### Q：OrderRef、ExecOrderRef、QuoteRef……这么多引用怎么保证递增不撞号？

**单序列裁决**：报单/预埋单/行权/期权自对冲/组合/报价/询价共用
`next_order_ref` 一条递增序列——各家族子序列天然各自递增（满足柜台递增校验），
且从构造上**杜绝跨家族撞号**（不依赖"柜台按业务流查重"的假设）。
各操作引用（OrderActionRef/ExecOrderActionRef 等）共用另一条序列 `next_action_ref`。
断线重连（新会话）后两条序列归零重计。

### Q：为什么请求里的 broker_id/investor_id/user_id 被会话覆盖了？

托管层一致性约定：指令与含身份字段的查询，身份一律按会话配置填充——
一个会话本来就在一个经纪商一个投资者下，让调用方手填只会引入静默错配
（漏填/填错得到的是前置错误或空结果）。**例外**：`currency_id`、`biz_type`、
`future_serial`、资金密码等业务字段由调用方决定。

### Q：simnow 前置连不上？

三组前置（30011/30012/30013）收盘后离线；24h 前置（40011/40001）可用但交易日滞后一天。
夜盘（21:00 后）交易日归属**下一自然日**。配置多组前置做冗余（CTP 随机选一个连）。

### Q：heartbeat / 组播合约查询 / 报价询价，为什么没有对应接口？

- **heartbeat**（`OnHeartBeatWarning`）：CTP C++ 端未启用，trait 保留默认空实现，
  断线检测已由 `disconnected` 覆盖；
- **组播**（`QryMulticastInstrument`）：组播行情是托管在期货商服务器的高频分发模式，
  标准 CTP + simnow 走 TCP 用不到，未实现（高频场景需评估 CTP mini 等专用方案）；
- **报价/询价方向别搞混**：`for_quote_insert` 是**请求**对手方报价，`quote_insert` 是**应价**报价；
  行情侧的询价订阅（`subscribe_quote`）与交易侧的询价申报是两个方向。

### Q：组合申请为什么没有"撤销组合"回调？

解除组合与申请组合共用 `CombActionInsert`（`comb_direction` 区分申请/解除），
因此 comb 家族只有 insert 系列回调，没有 action 系列。

### Q：保证金率/手续费率查询为什么为空，或者返回的 InstrumentID 变成了品种？

**查证**（2026-11-24，CTP 6.7.11 API 文档）：这两类查询不是“空条件查全市场”。

- `query_instrument_margin_rate`：空 InstrumentID 只返回持仓相关合约；BrokerID、InvestorID、HedgeFlag 要按业务正确填写，漏填可能得到空结果；
- `query_instrument_commission_rate`：空 InstrumentID 同样只返回持仓相关合约；柜台未设置具体合约费率时，会返回品种级费率，响应 InstrumentID 可以是品种代码；
- **解法**：明确要查的合约与投机套保标志，逐合约查询并遵守流控；保留响应原始 InstrumentID，不把它强制改写回请求值。空结果先检查查询范围、权限与柜台配置，不得解释为“零费率”。

来源：[保证金率文档](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQQRYINSTRUMENTMARGINRATE.html)、[手续费率文档](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQQRYINSTRUMENTCOMMISSIONRATE.html)、[实践说明](https://www.vnpy.com/forum/topic/7684-ctpjie-kou-zhi-bao-zheng-jin-lu-shou-xu-fei-lu-cha-xun)。生产口径见知识库 9.3。

### Q：期权成本、组合保证金或申报费查询为什么不能直接给出一个“本地最终费用”？

**原因**：`query_option_instr_trade_cost` 使用调用方 InputPrice 等参数估算，和资金查询中的实际保证金价格口径可能不同；SPBM/SPMM/RCAMS/RULE 又分交易所参数和投资者参数，部分请求不含账户身份，部分必须含 BrokerID/InvestorID。InvestorPortfSetting 可按交易编码返回多行，InvestorInfoCommRec 也可对期货和系列期权各返一行。

**解法**：Session 只覆盖真实身份字段并逐行透传完整响应，不在绑定层聚合视图、猜测算法或去重；业务层按目标交易所、柜台、交易日和批次完整性验证计算。详见知识库 9.3 及各接口开发前检索记录。

### Q：对冲设置为什么撤销成功条件不明确，或者同一笔设置覆盖了前一笔？

**查证**（2026-11-24）：ReqOffsetSetting/ReqCancelOffsetSetting 仅适用大商所，设置维度由 InstrumentID、UnderlyingInstrID、ProductID、OffsetType 等共同决定，撤销要按原报入维度填写。同一“投资者+对冲粒度+对冲类型”后台只保留一笔；结构体内 RequestID 只回显，可用于区分请求，不负责去重。

**解法**：保存原始设置维度，撤销复用这些字段；不要只按合约代码定位，也不要让 Session 覆盖 RequestID。合约级 IsOffset=true 的手数规则不能直接套到客户/品种级。详见知识库 5.6 和官方 [设置](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQOFFSETSETTING.html)/[撤销](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQCANCELOFFSETSETTING.html) 文档。

### Q：SPD/套保申请收到 OnRsp 后，为什么还要等 OnRtn？撤销时 OrderRef 怎么处理？

**原因**：申请和撤销都有直接应答、错误回报与状态通知路径。OnRsp 不是完整业务终态；OnErrRtn 不带外层 request_id，需要用原始申请标识关联。撤销结构体中的 OrderRef 是被撤销申请的引用，不是新分配的操作引用。

**解法**：申请由 Session 从共享序列分配 OrderRef 并返回；撤销保留调用方提供的原 OrderRef/OrderSysID，只覆盖会话身份及当前 FrontID/SessionID。重连后旧会话 OrderRef 不得直接改写，跨会话优先按 OrderSysID 定位。网络未找到这些新接口的可靠专项 bug 时，不得据此省略任何一条错误路径。

## 六、托管层设计决策（为什么这么做）

### Q：为什么 `exit(self)` 要消费所有权？

让"销毁后再调用"成为**编译期错误**而不是运行时失败——`exit(mut self)` 移动后
调用方手里不再持有会话，误用在编译时就被拦住。
（`ChannelClosed` 错误只剩 loop 线程异常退出这一条运行时路径。）

### Q：为什么查询逐行经 loop 发送，而由调用层聚合？

SPI 与 loop 的串行通道保留 `Disconnected → Recovered` 与查询响应的 FIFO 顺序，
loop 收到每一行立即公开，不再持有在途批次；调用层按自己的视图需要聚合、超时和清理。
代价：控制事件与数据回报共享丢弃预算——洪流下 `Disconnected` 理论上可能被丢弃
（有 `dropped_count` 可观测，重连自愈）；聚合后的完整视图还必须处理缺行和超时，不能把末行当成完整性证明。

### Q：`front_id()/session_id()/trading_day()` 重连后会读到旧值吗？

**曾经会**（批次二终审发现）：三个 getter 只缓存首登值，重连后对账静默失配。
**修复**：会话身份改为 loop 与句柄共享的 `Arc<Mutex<Identity>>`，就绪（含 Recovered）时刷新，
getter 读取即当前值。已在 simnow 实测验证（Recovered 后 SessionID 更新为新值）。

### Q：曾经踩过的资源泄漏/挂死还有哪些？都怎么修的？

- **`TraderSession::new` 超时路径泄漏 worker**：超时报错时 loop 线程与原生 API 存活至进程结束。
  修复：超时路径与"被拒绝"路径同一套收尾（Shutdown + join）。
  有 15 秒实测测试（`unreachable_front_times_out_and_reclaims_worker`，默认 `#[ignore]`）；
- **握手 (Some, None) 防御死角**：`SettlementConfirmed` 先于登录回报到达时握手通道被静默丢弃，
  `new()` 干等到超时。修复：该分支以 `Rejected(-1)` 立即作答；
- **登出窗口期指令被接受**：收到 `Logout` 到 `Disconnected` 之间 session 仍标记就绪。
  修复：`Logout` 分支即刻 `session = None`（登出语义是断开重连，Disconnected 必随后到达）。

### Q：配置文件重构为什么会导致单元测试失败？

`tests/unit/raw.rs` 的部分用例经 `tests/common/config.rs` 加载配置构造请求。
配置格式演进（平铺 → `[ctp.profiles.*]` 档案式）后旧解析器报错，测试连带失败。
**纪律**：配置结构变更必须同步更新 `examples/` 与 `tests/` 两份解析器，并跑全套测试验证。

### Q：exit() 销毁实例真的干净吗？怎么验证的？

simnow 实测（`examples/lifecycle.rs`）：`exit()` 15.4 毫秒完成 loop join + 原生 `Release`，
原地立即重建新会话成功（流量句柄/前置连接完全释放），`Drop` 隐式回收进程干净退出。
重建验证是"资源释放干净"最强的外部证据。

## 七、流程纪律


### Q：不确定接口语义，怎么避免拍脑袋？

**规则**：以生成器审核表（`reviews/event-semantics.csv`）的官方语义为准，
不凭英文名猜测——CTP 命名有误导：`InvestorInfoCommRec` 实为**申报费阶梯收取记录**
（活跃策略的真实成本），不是佣金流水。本 FAQ 中标注"实测/查证"的条目均附日期与出处，
出处存疑的语义在写入文档前必须先查证。

### Q：用户边改配置边跑测试，出现一闪而过的报错？

**现象**：例如此前"环境变量未设置"报错，重跑消失。
**原因**：编辑器保存的中间状态被运行撞上。先重跑确认，再排查代码。

### Q：工作树被并行改动（glob 化、配置重构）造成冲突怎么办？

**真实事故**：开发期间 `mod.rs`/`lib.rs` 被并行改成 glob 导出、`ctp.toml` 被并行重构，
导致 clippy 警告、公开 API 歧义与测试失败。
**解法**：以编译器与 clippy 为裁判逐项对齐——恢复显式导出消除歧义；
解析器随新格式重写并同步两处。教训：**结构级改动（导出方式、配置格式、共享常量位置）
必须在台账/文档中留痕**，让并行工作方能察觉。

### Q：SimNow 报撤单成功时一定会收到成功的 OnRsp 应答吗？

不一定。2026-11-24 在 SimNow 7×24 前置实测 `TraderSession::insert_order` 时，未观察到成功的 `OrderInsertResponse`，但收到 `Unknown → NoTradeQueueing` 的 `OrderUpdate`，随后撤单收到最终 `Canceled`。因此测试和策略不能无限等待一个成功 OnRsp 才认定业务成功；应同时消费直接应答与状态回报，并以实际业务状态、错误回报和后续对账判断。该观察仅适用于本次 SimNow 环境，不能推广为所有柜台的固定回调顺序。复现入口：`examples/session_audit.rs`。

## 八、合并后的协议细节补充

### Q：行情和交易登录分别需要哪些字段？

- 行情端官方默认没有必填项，不要求 AppID/AuthCode；CTP 6.6.2 起柜台可开启行情身份校验，届时 `BrokerID`、`UserID` 必须正确且与交易端一致，当前 IP 当天还须已成功登录交易系统。`MdConfig` 保留这两个字段正是为了覆盖该例外，不保留行情密码和授权码。
- 交易端 `ReqUserLogin` 必填 `BrokerID`、`UserID`、`Password`；强制终端认证时先调用 `ReqAuthenticate`，填写 `BrokerID`、`UserID`、`AppID`、`AuthCode`。启用动态口令时还需 `OneTimePassword`。前置地址是连接参数，不属于登录请求字段。

来源：[行情 ReqUserLogin](https://ctp.madongdong.com/6.7.11_API/HQJK/CTHOSTFTDCMDAPI/REQUSERLOGIN.html)、[交易 ReqUserLogin](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQUSERLOGIN.html)、[ReqAuthenticate](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQAUTHENTICATE.html)。实际权限仍以目标柜台为准。

### Q：为什么 Session 就绪前必须确认结算？Quick 流订阅又解决什么？

多数柜台未确认结算时不能下单，因此交易 Session 将就绪链定义为“连接→认证→登录→结算确认”，全部完成才返回 Ready。私有流和公共流在 `init` 前订阅为 `Quick`，避免默认恢复语义重放历史消息；代价是断线期间的变化可能丢失，所以 `Recovered` 后必须查询挂单、成交、持仓、资金和转账流水进行对账。

### Q：MdSpi 为什么是 13 个 C++ 回调外加 `conversion_error`？

`CThostFtdcMdSpi` 的 13 个回调在 FFI 中一一对应；`conversion_error` 是 Rust 侧新增的 owned 转换失败出口，不是 C++ 回调。转换失败意味着该数据被丢弃，必须计入 `dropped_count` 并上报，不能静默。SPI 回调只做必要转换与非阻塞投递，不同步等待 loop 结果，避免拖住或死锁 CTP 回调线程。

### Q：查询除 `is_last` 外还要处理哪些限制？

- CTP 查询存在频率与在途限制，常见约每秒一笔；本地 `-2/-3` 和异步 `OnRspError` 都是真实失败路径，调用方应限速，不能盲目并发重发。
- 同一 `request_id` 的响应行必须与查询类型一致；调用方构造视图时必须拒绝类型不符的行，不允许把其他接口的数据拼进当前结果。
- 空载荷末行也逐行公开；断线和 `OnRspError` 原样转发，由调用方清理批次并设置超时，不能把“尚未断线”误解为“一定会结束”。

### Q：大商所全成交时的报单/成交顺序有什么特殊性？

旧指南与项目记录指出，大商所全成交时交易所可能直发成交回报，CTP 随后补全部成交报单回报。策略不得假设所有交易所都严格先发报单、再发成交，应按 OrderRef、OrderSysID 和成交标识对账，而不是依赖固定到达顺序。该规则不能替代对目标交易所和当前柜台的验证。

### Q：批次六、七有哪些字段不能由 Session 擅自补写或过滤？

- 保证金、费率与风险监控查询按真实结构体处理身份：21 类覆盖 BrokerID+InvestorID，4 类只覆盖 BrokerID，19 类不含会话身份字段；不得给 SPBM/SPMM/RCAMS/RULE 的交易所参数机械添加账户身份。
- 对冲设置仅适用大商所，结构体内 RequestID 是回显字段，不能与外层 `request_id` 混同或自动覆盖。
- 分类合约查询只按文档确认的 TradingType/ClassType 过滤；CombPromotionParam 的 ExchangeID/InstrumentID 不是文档确认的过滤条件。Session 保留完整原始行，不做额外本地筛选或破坏性去重。

### Q：银期转账的 TradeCode 和流水号怎样分工？

期商主动入金使用 202001、出金使用 202002，由 loop 按方向覆盖；银行端发起的 102001/102002 不属于客户端主动填充方向。`future_serial` 仍由调用方按目标柜台规则维护，因为部分柜台校验递增或唯一性。资金密码、流水号与业务功能码的职责不能混为一谈，断线后以转账流水和资金查询对账，不盲目重发。

### Q：穿透式示例出现 `Decrypt handshake data failed`，为什么调整 production_mode？

2026-11-24 在同一账户、同一 SimNow 7×24 前置做了对照：穿透式公共示例使用 `production_mode: false` 时握手失败并超时；已有 `reconnect` 示例使用 `true` 时认证、登录、恢复成功。仅把公共示例改为 `true` 后，连通性、合约、资金与持仓查询全部成功。因此该参数不能简单按“仿真=false、生产=true”猜，必须匹配当前 SDK、加密认证组件和前置要求。若仍失败，继续核对 AppID/AuthCode、SDK/动态库一致性和前置类型，不要把解密失败当成报单业务错误。
