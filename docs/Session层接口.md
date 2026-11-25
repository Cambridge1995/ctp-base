# Session 层接口与审核状态

本文覆盖当前 `MdSession` 与 `TraderSession` 的 117 个公共方法。审核结果只分“待进行”和“已通过”；SimNow 成功仅填写“开发审核”，不代表目标期货公司测试环境或生产环境已经通过。

## 本次 SimNow 证据

- 日期：2026-11-24；环境：SimNow 7×24 前置；交易日：20261123；SDK：项目随包 v6.7.13_P1_20261103。
- 测试合约：`ag2612`；代码：当前工作区（无 Git 元数据）；账户信息不写入文档。
- `MD-1`：`examples/session_audit.rs` 的行情登录、初始/再次订阅、有效 Tick、退订、事件读取及退出均成功。
- `TR-1`：同一示例的交易认证、登录、结算确认、11 个核心查询、1 手低风险限价报单和撤单均成功；资金、投资者、交易所和产品查询另有非空断言，其余允许合法空批次；报单最终进入 `NoTradeQueueing`，撤单最终为 `Canceled`，事件丢弃数为 0。
- `TR-2`：`examples/reconnect.rs` 主动重连成功，`SessionID` 更新，身份 getter 与 `Recovered` 事件一致，退出成功。
- 本轮曾随机选中离线前置导致握手超时；改用 7×24 前置后完成上述验证。只有最终获得成功结果的方法才标为“已通过”。
- **2026-11-25 行为变更**：上述 MD-1、TR-1 属于历史证据。交易认证/登录/结算确认、行情登录与订退的错误处理已修改；全部查询改为 `QueryRow` 逐行公开，Session 不再产生 `QueryBatch`。受影响接口已逐项退回“待进行”，不能以历史整批结果代替当前版本复测。

## MdSession

### 核心接口审核状态

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `MdSession::new` | 创建、连接、登录及初始订阅 | 待进行 | 待进行 | 待进行 | 请求错误路径变更，待复测 |
| `MdSession::trading_day` | 读取登录交易日 | 已通过 | 待进行 | 待进行 | MD-1 |
| `MdSession::subscribe` | 订阅行情 | 待进行 | 待进行 | 待进行 | 请求错误路径变更，待复测 |
| `MdSession::unsubscribe` | 退订行情 | 待进行 | 待进行 | 待进行 | 请求错误路径变更，待复测 |
| `MdSession::try_event` | 非阻塞读取事件 | 已通过 | 待进行 | 待进行 | MD-1 |
| `MdSession::dropped_count` | 读取丢弃事件计数 | 已通过 | 待进行 | 待进行 | MD-1 |
| `MdSession::exit` | 关闭会话并释放资源 | 已通过 | 待进行 | 待进行 | MD-1 |

### 待测试

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `MdSession::subscribe_quote` | 订阅询价 | 待进行 | 待进行 | 待进行 | 请求错误路径变更，待复测 |
| `MdSession::unsubscribe_quote` | 退订询价 | 待进行 | 待进行 | 待进行 | 请求错误路径变更，待复测 |
| `MdSession::next_event` | 阻塞读取事件 | 待进行 | 待进行 | 待进行 | 未执行 |

## TraderSession

### 核心接口审核状态

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `TraderSession::new` | 创建、认证、登录及结算确认 | 待进行 | 待进行 | 待进行 | 请求返回码处理变更，待复测 |
| `TraderSession::front_id` | 读取当前 FrontID | 已通过 | 待进行 | 待进行 | TR-1、TR-2 |
| `TraderSession::session_id` | 读取当前 SessionID | 已通过 | 待进行 | 待进行 | TR-1、TR-2 |
| `TraderSession::trading_day` | 读取当前交易日 | 已通过 | 待进行 | 待进行 | TR-1、TR-2 |
| `TraderSession::try_event` | 非阻塞读取事件 | 已通过 | 待进行 | 待进行 | TR-1、TR-2 |
| `TraderSession::dropped_count` | 读取丢弃事件计数 | 已通过 | 待进行 | 待进行 | TR-1、TR-2 |
| `TraderSession::reconnect` | 主动重连并恢复会话 | 已通过 | 待进行 | 待进行 | TR-2 |
| `TraderSession::exit` | 关闭会话并释放资源 | 已通过 | 待进行 | 待进行 | TR-1、TR-2 |
| `TraderSession::insert_order` | 限价报单 | 已通过 | 待进行 | 待进行 | TR-1 |
| `TraderSession::cancel_order` | 撤销报单 | 已通过 | 待进行 | 待进行 | TR-1 |
| `TraderSession::query_order` | 查询报单 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_trade` | 查询成交 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_investor_position` | 查询持仓 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_investor_position_detail` | 查询持仓明细 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_trading_account` | 查询资金账户 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_instrument` | 查询合约 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_exchange` | 查询交易所 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_product` | 查询产品 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_depth_market_data` | 查询深度行情 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_investor` | 查询投资者 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |
| `TraderSession::query_settlement_info` | 查询结算单 | 待进行 | 待进行 | 待进行 | 查询逐行事件变更，待复测 |

### 会话与交易指令待测试

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `TraderSession::next_event` | 阻塞读取事件 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::batch_order_action` | 批量撤单 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::exec_order_insert` | 行权申报 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::exec_order_action` | 行权操作 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::option_self_close_insert` | 期权自对冲申报 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::option_self_close_action` | 期权自对冲操作 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::comb_action_insert` | 组合申请或解除 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::quote_insert` | 报价申报 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::quote_action` | 报价操作 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::for_quote_insert` | 询价申报 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::parked_order_insert` | 预埋单申报 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::parked_order_action` | 预埋撤单 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::remove_parked_order` | 删除预埋单 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::remove_parked_order_action` | 删除预埋撤单 | 待进行 | 待进行 | 待进行 | 未执行 |

### 早期查询待测试

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `TraderSession::query_investor_position_combine_detail` | 查询组合持仓明细 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_comb_leg` | 查询组合单腿 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_exec_order` | 查询行权 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_option_self_close` | 查询期权自对冲 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_comb_action` | 查询组合申请 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_quote` | 查询报价 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_for_quote` | 查询询价 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_parked_order` | 查询预埋单 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_parked_order_action` | 查询预埋撤单 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_transfer_bank` | 查询转账银行 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_transfer_serial` | 查询转账流水 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_account_register` | 查询银期账户登记 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_contract_bank` | 查询签约银行 | 待进行 | 待进行 | 待进行 | 未执行 |

### 保证金、费率与风险监控查询待测试

| Session 接口 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|
| `TraderSession::query_max_order_volume` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_instrument_margin_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_instrument_commission_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_mm_instrument_commission_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_mm_option_instr_comm_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_instrument_order_comm_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_option_instr_trade_cost` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_option_instr_comm_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_exchange_margin_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_exchange_margin_rate_adjust` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_exchange_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_product_exch_rate` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_product_group` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_product_group_margin` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_comb_instrument_guard` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_max_order_volume_with_price` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_future_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_option_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_intra_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_inter_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_portf_definition` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_investor_portf_def` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spbm_add_on_inter_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_portf_margin_ratio` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_prod_spbm_detail` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spmm_inst_param` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spmm_product_param` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_commodity_spmm_margin` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_commodity_group_spmm_margin` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rcams_comb_product_info` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rcams_instr_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rcams_intra_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rcams_inter_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rcams_short_opt_adjust_param` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rcams_investor_comb_position` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_prod_rcams_margin` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rule_instr_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rule_intra_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_rule_inter_parameter` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_prod_rule_margin` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_portf_setting` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_risk_settle_invest_position` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_risk_settle_product_status` | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_investor_info_comm_rec` | 待进行 | 待进行 | 待进行 | 未执行 |

### 对冲、SPD、套保及其他新增业务待测试

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `TraderSession::set_offset` | 设置对冲 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::cancel_offset_setting` | 撤销对冲设置 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_offset_settings` | 查询对冲设置 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::spd_apply` | SPD 套利确认 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::spd_apply_action` | 撤销 SPD 套利确认 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_spd_apply` | 查询 SPD 申请 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::hedge_cfm` | 套保确认 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::hedge_cfm_action` | 撤销套保确认 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_hedge_cfm` | 查询套保确认 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_instr_trade_limit` | 查询合约交易限制 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_classified_instrument` | 查询分类合约 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_comb_promotion_param` | 查询组合优惠参数 | 待进行 | 待进行 | 待进行 | 未执行 |

### 银期业务待测试

| Session 接口 | 功能 | 开发审核 | 测试审核 | 生产审核 | 证据 |
|---|---|---|---|---|---|
| `TraderSession::transfer_to_future` | 银行转期货 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::transfer_to_bank` | 期货转银行 | 待进行 | 待进行 | 待进行 | 未执行 |
| `TraderSession::query_bank_balance` | 查询银行余额 | 待进行 | 待进行 | 待进行 | 未执行 |

## Session 不对接清单（27 个回调）

“不对接”只表示当前产品取舍，不表示接口已审核或协议不支持；需要接入时仍须执行根目录 `AGENTS.md` 的资料检索、实现与验证流程。

| 分类 | 回调 | 数量 | 当前原因与例外 |
|---|---|---:|---|
| 密码与认证辅助 | `user_password_update_response`、`trading_account_password_update_response`、`user_auth_method_response`、`gen_user_captcha_response`、`gen_user_text_response`、`gen_sms_code_response` | 6 | 穿透式认证使用 AppID/AuthCode，改密和验证码属于人工功能；若目标公司启用动态口令，再接 `gen_sms_code` 与登录的 OneTimePassword 字段 |
| 心跳 | `heartbeat` | 1 | 断线检测由 `disconnected` 覆盖，当前 C++ 桥未启用心跳回调 |
| 私有流序号 | `private_seq_no_report` | 1 | 当前使用 Quick 模式，不重放历史，不需要断点续传序号 |
| 通知公告 | `notice_response`、`notice_history_response`、`notice_received`、`bulletin` | 4 | 人工阅读信息，当前策略不消费 |
| 监控中心 CFMMC | `cfmmc_trading_account_key_response`、`cfmmc_account_token`、`query_cfmmc_trading_account_token_response` | 3 | 监管数据报送相关，属于柜台侧事务 |
| 权证 | `e_warrant_offset_response` | 1 | 历史遗留品种 |
| 二级代理 | `sec_agent_acid_map_response`、`sec_agent_trading_account_response`、`sec_agent_check_mode_response`、`sec_agent_trade_info_response` | 4 | 资管分仓场景 |
| 投资单元 | `invest_unit_response` | 1 | 多投资单元属于资管场景 |
| 管理信息查询 | `user_session_response`、`broker_trading_algos_response`、`broker_trading_parameters_response`、`trading_code_response` | 4 | 账户和经纪商管理信息，当前无策略价值 |
| 交易员要约 | `trader_offer_response` | 1 | 报盘要约场景，当前无策略价值 |
| 结算状态推送 | `settlement_status` | 1 | 会话链已完成结算确认，当前不额外消费该推送 |

复核更正：SPBM/SPMM/RCAMS/RULE 保证金模型、组合保证金参数、风险结算和投资者申报费阶梯记录已从初版“不对接”移出。它们影响真实交易成本、组合保证金优惠或风险处置，不应仅因名称陌生而忽略；当前主动查询方法见上方 TraderSession 表。

## 开发前检索记录

新接口开发前逐项记录实际检索词、官方资料、实践案例和本地结论；没有可靠新增知识时如实记录，不编造。审核状态变化必须在对应接口行更新证据。

| 接口 | 日期 | 检索词与资料 | 本地结论 | 记录人 |
|---|---|---|---|---|
| `ReqAuthenticate` / `ReqUserLogin` / `ReqSettlementInfoConfirm` | 2026-11-25 | 检索“接口名 + 返回值/-1/-2/-3”、“认证登录 + 断线重连/流控”；核对随包头文件、知识库、FAQ 及 CTP 6.7.11 官方接口文档：[认证](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQAUTHENTICATE.html)、[交易登录](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQUSERLOGIN.html)、[结算确认](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQSETTLEMENTINFOCONFIRM.html) | 三者均可能返回 0/-1/-2/-3；`Ok(非0)` 是 CTP 发送失败而非转换错误，现有忽略属于本地缺陷。无新增协议结论 | Kimi |
| 行情 `ReqUserLogin` / `SubscribeMarketData` / 退订与询价订退 | 2026-11-25 | 检索“接口名 + 回调/返回值”、“订阅 + 问题/缓冲区”、“行情 + 断线/流控”；核对知识库、FAQ、当前实现及官方[行情登录](https://ctp.madongdong.com/6.7.11_API/HQJK/CTHOSTFTDCMDAPI/REQUSERLOGIN.html)、[行情订阅](https://ctp.madongdong.com/6.7.11_API/HQJK/CTHOSTFTDCMDAPI/SUBSCRIBEMARKETDATA.html) | 非零返回码必须立即上报；订退失败不能静默忽略。无新增协议结论 | Kimi |
| 全部查询回调（统一事件投递方式，不新增或改变各 CTP 查询语义） | 2026-11-25 | 检索“ReqQryInstrument/OnRspQryInstrument + nRequestID/bIsLast”、“查询 + 空响应/乱序/断线/流控/生产”；核对知识库 7.1-7.3、FAQ、现有 SPI 映射及官方[查询请求](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERAPI/REQQRYINSTRUMENT.html)、[查询响应](https://ctp.madongdong.com/6.7.11_API/JYJK/CTHOSTFTDCTRADERSPI/ONRSPQRYINSTRUMENT.html) | `bIsLast` 是批次边界，不要求客户端缓存到末行才转发；统一改为逐行公开，调用方负责聚合、超时与完整性，所有 QueryRow 类型继续沿用现有逐接口映射。无新增协议结论 | Kimi |
