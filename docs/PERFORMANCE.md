# Performance

根包无 runtime 依赖不是速度或内存测量。比较中立的原生 A 与 Kit B 场景，增量为 B−A；
完整 Gallery 不能用作单按钮基准。实现/资产/配置变化后，旧测量只作为历史证据。

## Commands and scope

| 命令 | 内容 |
|---|---|
| `python scripts/run_perf.py --generate-only` | 生成隔离的原生/Kit 消费者 |
| `python scripts/run_perf.py --native-probe` | 编译公开原生能力探针 |
| `python scripts/run_perf.py --profile release --samples 30` | 21 配对场景、交替 A/B 顺序，共 1,260 个进程 |
| `python scripts/evaluate_perf.py <result.json> --output <gate.json>` | 完整有限样本与各预算；collection 与 gate 分开 |
| `python scripts/inspect_perf_artifacts.py <result.json>` | 保存二进制 hash、PE 区段、SVG 与额外原生窗口几何 |
| `python scripts/run_interaction_perf.py --pairs 3` | 六场景、每进程 200 个控件操作及软件帧 |
| `python scripts/run_idle_perf.py` | 五个匹配场景，在无活动 driver 时各采 >=60s idle |
| `python scripts/run_gallery_lifecycle.py` | 三次 100 往返和 idle；独立第四个页 Timer 探针 |
| `python scripts/run_motion_bench.py --label current --counts 0 1 20` | 100 开关周期、200 帧及 >=60s idle |

具体选项用各脚本 `--help`。报告、生成项目、命令、lock/解析图和原始样本留在 target。
`--scenes` 可选择有明确风险的子集；只测子集不得宣称全矩阵通过。

## Matching and measurement

- 匹配 release 工具链、Slint/features、后端/renderer、窗口、字体、DPI、主题、内容和资源。
  默认微基准为 Fluent/winit-software、820x440、Light、100%、请求系统 Segoe UI Variable Text。
- 新进程使用已有编译/系统缓存，不能称为受控冷启动。构造、show 返回、render callback、
  软件缓冲完成和真实屏幕 present 分别记录；不支持的 hook 留空，不补零。
- `first_software_frame_ms` 从 Rust main 到非透明 `take_snapshot()` 完成，排除 loader，
  也不代表屏幕呈现。它只能接受对应的命名指标预算。
- 普通配对至少 30 组，交替顺序，保留全部样本及 p50/p95/n/min/max。禁止删离群点、
  修剪 working set、关闭辅助技术或改 features 制造优势。噪声过大则改善采样或标不可判定。
- Private Bytes 与 Working Set 分开；1s/2s 固定观察不证明稳态。Idle 使用真实 CPU
  时间差和 >=60s 无 driver 区间，CPU 为单核百分比。GPU/精确重绘数未采到则 NOT_RUN。
- 1,000 控件场景仅有 100 个在固定视口可见；ListView 100/1,000/10,000 行保留原生
  直接 repeater 虚拟化。累计创建次数不是同时驻留数量，也不等于模型数据零内存。
- 交互同时检查 dispatch 和 dispatch+snapshot；快截图不能掩盖慢输入。Unicode 不是 IME。
- Navigation 检查 16/64/256/257；保留每模型 256 上限及非法 fail-closed，不把大列表预算套给导航。
- 浮层和页面至少 100 周期；检查计时器、活跃实例、焦点与内存趋势。独立插入页 Timer 的
  验证副本不能与未插桩的时序/内存混算。关闭后内存未归还不自动等于泄漏。

## Unchanged budgets

以下工程阈值未因文档精简而放宽。缺少相应采样或稳定性证据的门禁仍未关闭。

| 指标 | B−A 上限 / 规则 |
|---|---|
| Startup p50 | max(10ms, 10% of A) |
| Startup p95 | max(20ms, 15% of A) |
| Steady Private Bytes | max(2MiB, 5% of A)，另查随数量增长斜率 |
| Interaction frame p95 | max(1ms, 10% of A)；A 能稳定 60Hz 时 B 不应破坏它 |
| Idle CPU | 增量 <=0.2 个百分点；无非必要持续刷新 |
| Simple binary size | max(512KiB, 5% of A)，匹配并说明必需代码/资源 |
| Navigation validation | 256 项目标 <=16.67ms；不扩大模型上限掩盖超时 |
| Hidden work | 关闭周期结束后无非必要 Timer 或持续装饰动画 |

隐藏 Toast 对空窗口是非等价比较，原始超标仍 BLOCKED。匹配代码/资源的另一个隐藏
参考只比较该状态成本，不证明可见 Toast 等价或原生存在 Toast API。

P7/P8 通过的命名软件缓冲/固定时点内存门禁，不自动关闭实际 present、长期内存、
GPU 或跨平台接受。关键数字及原始结果在 [HISTORY](HISTORY.md#p7p8-主要证据)，
当前未完成项在 [STATUS](STATUS.md)。
