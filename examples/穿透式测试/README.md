# 穿透式测试示例

本目录对应 `穿透式认证/` 下 pyCTP demo 的十项测试，使用本项目当前的 `MdSession`、`TraderSession` 和强类型请求结构体。

## 运行约束

- 仅使用目标期货公司明确批准的穿透式测试账户和柜台；不得直接用于生产。
- `production_mode` 必须匹配 SDK 与前置的加密认证要求。2026-11-24 的 SimNow 7×24 对照测试中，`false` 出现解密握手失败，`true` 成功；其他柜台不得直接照搬该结果。
- 多前置由 CTP 随机选择；若离线前置导致超时，可用 `CTP_TRADE_FRONT`、`CTP_MARKET_FRONT` 显式指定当前可用前置。
- 合约使用 `config/ctp.toml` 的当前档案或 `CTP_INSTRUMENT` 覆盖值，不沿用 demo 中可能已过期的合约。
- 示例会真实报单、成交或撤单。运行前确认账户权限、品种、手数、交易时段和回退方案。
- 本地报撤单统计不是柜台官方风控指标；计数在每次进程启动时清零，最终以 CTP 回报和查询对账为准。
- 依赖实时 Tick 的报单示例需要在有效交易/测试时段运行；原 demo 的休市报单场景须在目标柜台休市窗口另行验证。
- 应急示例演示可用资金和本地计数暂停，不完整实现账户风险度等柜台风控算法。
- 上期所/能源中心的当日平仓使用 CloseToday；其他交易所使用 Close。冻结验仓若被柜台接受或成交，会明确输出 SKIP，不冒充拒绝通过。

## 对照表

| 原 demo | 本项目 example | 内容 |
|---|---|---|
| 穿透认证_连通性.py | `penetration_connectivity` | 连接、认证、登录、结算确认、账户/合约/持仓查询 |
| 穿透认证_基础交易功能.py | `penetration_basic_trading` | 多空开平仓及报单后撤单 |
| 穿透认证_交易指令检查.py | `penetration_order_checks` | 错误合约、零数量、价格跳动、平仓不足、最小/最大手数 |
| 穿透认证_批量撤单.py | `penetration_batch_cancel` | 重复撤单与当前会话批量撤单 |
| 穿透认证_重复报单监测.py | `penetration_repeat_orders` | 重复业务意图及本地计数；OrderRef 由 Session 更新 |
| 穿透认证_报撤单笔数信息量等监测.py | `penetration_order_metrics` | 开平仓、撤单、成交和消息本地统计 |
| 穿透认证_阈值设置及预警.py | `penetration_risk_thresholds` | 报单、撤单、重复意图阈值预警 |
| 穿透认证_应急处置暂停交易.py | `penetration_emergency_stop` | 资金或计数超限时暂停新开仓 |
| 穿透认证_错误防范.py | `penetration_error_handling` | 基础开仓、限额拒绝及不盲目重发 |
| 穿透认证_系统连接状态异常监测.py | `penetration_connection_recovery` | 受控重连、Recovered 与 SessionID 更新 |

## 命令示例

```bash
cargo run --example penetration_connectivity
```

报撤单类示例：

```bash
cargo run --example penetration_basic_trading
```

阈值示例默认只执行一轮；对照原 demo 的五轮测试可设置：

```bash
CTP_TEST_ROUNDS=5 cargo run --example penetration_risk_thresholds
```

非整数手数等错误由 Rust 类型系统在编译期阻止，不构造无效 CTP 请求。成功报单以实际 `OrderUpdate`/`Execution` 为准，错误测试必须收到明确拒绝才算符合预期。
