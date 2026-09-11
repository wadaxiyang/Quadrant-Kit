# 历史证据与追溯

本页保留交付节点、重要失败与原始证据入口。它不是当前工作树的全量测试报告。
逐阶段叙述已从当前 docs 删除，完整原文仍在 Git 中，未改写提交或标签。
根目录与正式 SPEC 完全重复的副本也已删除，当前唯一入口在 `docs/specs/`。
本地 `target/` 路径是原始记录位置，可能不随新克隆存在；不可据此假称重新执行成功。

## v0.1.0

- 发布时间：2026-09-11；保留引用：`refs/tags/v0.1.0`。
- 范围：当前 59 个公开名称、42 个视觉组件、Gallery、API/边界/原生复用检查及
  源码包；根包仍无正常/runtime 依赖。
- 发布前在标签源码上执行 core、Gallery build、distribution/package 和 Rust 1.92
  构建；精确结果以本次发布任务输出和 tag 对应 Actions 为准。
- 按发布授权省略全新目录中的匿名 Git+完整 SHA 消费者获取/运行，状态为 NOT_RUN；
  不据此宣称外部首次获取已经实际验证。
- 完整 WinUI 对照、真实 present/GPU、长期内存归因、完整读屏/IME/跨屏 DPI 与本轮
  macOS 原生运行仍不属于 PASS，详见 [STATUS](STATUS.md)。

## 找回完整报告

文档整理前的完整快照为 `823c87216bf1843058b41a3cb30bdfc985d196b3`：

```console
git ls-tree -r --name-only 823c87216bf1843058b41a3cb30bdfc985d196b3 docs
git show 823c87216bf1843058b41a3cb30bdfc985d196b3:docs/implementation/kit-fluent-v1/P7.md
git show 823c87216bf1843058b41a3cb30bdfc985d196b3:docs/specs/QUADRANT_KIT_FLUENT_EVOLUTION_SPEC.md
```

同一方式可读取原 `NAVIGATION_REBUILD_PHASE0.md` 至 `PHASE8.md`、两份 Gallery chrome
报告、Fluent P0–P8/子阶段、STATE、AUDIT 和五份后续导航报告。历史原文里的“未提交”或
“下一阶段”描述当时状态，不能作为当前授权或当前结果。

## Published extraction

历史提取候选（API 与当前 Fluent 版不同）：

- 源码：`838ecfbead2d0a1966907ddd742cb6f34516d3f6`。
- 保留引用：`refs/tags/candidate/extraction-838ecfbead2d`；tag 对象
  `aa736b6873652d0c8dd8ea55df6d16bb5cec9f39`。Cargo 使用前者的源码 SHA。
- 当时记录的 [candidate CI](https://github.com/wadaxiyang/Quadrant-Kit/actions/runs/34003620362)、
  [tag CI](https://github.com/wadaxiyang/Quadrant-Kit/actions/runs/34004051391) 和
  [main CI](https://github.com/wadaxiyang/Quadrant-Kit/actions/runs/34004053852) 均通过。
- 当时匿名获取、独立 Git+SHA 消费者及 Light/Dark 运行通过；本地原始记录在
  `target/phase3/`。这些历史结果不认证当前 Fluent 源码。

## Fluent 交付节点

| 节点 | 源码/记录 | 当时的结论 |
|---|---|---|
| P0–P2 | `f47933832376566d69f66fa430ff0e76e120253d` | 边界/探针与 FluentButton 垂直切片；初次脏源码打包失败，提交后打包通过 |
| P3 | `1f288c93e7fd97fadf47cb84ef44b5f1f47a851a` | 基础控件迁移、49 运行断言、140 捕获和配对性能；WinUI 运行参考未完成 |
| P4A–D | 逐批实现选择、数值、容器、表格/日期时间 | 每批独立提交与当前消费者验证；公开合约以当前 API 为准 |
| P5A–E / P6 | 固定确认、导航、弹出/行内和有限动效 | 子阶段运行验证通过；未宣称任意 ContentDialog 或读屏完整支持 |
| P7 | `8bd293d4647be70489a08f812a532d6cccd4b73e` | Windows 配对测量与集成，完整接受仍 PARTIAL |
| P8 | `5e3bff0bd20528816f085cf9e4930ef8867f579c` | 本地候选收尾、平台与包记录；未发布当前 Fluent 候选 |
| 导航/Settings 后续修正 | 纳入 `823c87216bf1843058b41a3cb30bdfc985d196b3` | flat 行、Settings、原生标题栏对齐、公共 Back 和收起态修复；下列测试在提交前工作树执行 |

### P7/P8 主要证据

- P7 21 场景共 1,260 进程：19 个原始匹配场景及新增匹配隐藏 Toast 参考通过相应
  软件缓冲、固定 2s 内存与二进制预算。原始空窗口/Toast 比较 BLOCKED，二进制
  增量 2,570,752 bytes，未调整预算；匹配内容参考增量 108,544 bytes（106 KiB）。
  原始 `target/p7-measurement-source/target/perf-harness/20260910T012407920482Z/result.json`，
  匹配 Toast `target/perf-harness/20260910T022233317973Z/result.json`。
- 7,200 交互软件帧：snapshot 与 dispatch+snapshot p95 增量门禁均通过；
  `target/interaction-perf/20260910T022523142072Z/result.json`。这不是屏幕 present 或通用 60 fps 声明。
- 3 次 Gallery 100 往返及独立页 Timer 探针证明活跃页有界；内存斜率约
  1,794 / 3,030 / 9,597 bytes/往返，未完成长期增长归因。
  `target/gallery-lifecycle/20260910T023606419430Z/result.json`。
- 动效 0/1/20 实例、100 周期和 >=60s idle：
  `target/motion-bench/20260910T025417629479Z/result.json`；代表性配对 idle
  `target/idle-perf/20260910T025739929122Z/result.json`。一组长 idle 不等于 30 组长期置信度。
- Windows 核心、88 Python 测试和 128 Gallery 捕获见 `target/verify-p8/`。
  原生 UIA 见 `target/p7-uia-final/result.json`，不是读屏会话。
- 干净 P7 包/实际 archive 消费者、Windows Rust 1.92 与精确 token/SVG 恢复通过，
  `target/p7-postcommit/`、`target/package-consumer/20260910T031029372291Z/result.json`；
  后续 P8 包结果 `target/p8-postcommit/`。不替代远程消费者。
- Linux 构建/WSLg 英文控件烟测通过；88 Python 测试中 1 项因无 pwsh 跳过。
  中文缺字 FAIL；原始 `target/p7-linux/`。本轮 Mac 原生验收和远程发布未执行。

### 最新导航证据

对应最后导航修正的独立 facade 消费者：

| 验证 | 结果与原始目录 |
|---|---|
| Navigation | 169 断言 PASS，包含像素居中、整行点击、键盘展开与搜索保留；`target/button-checks/20260910T084930326353Z/` |
| Foundation | 30 断言 PASS；`target/button-checks/20260910T084752246533Z/` |
| 实际 toolbar/Settings | 18 断言 PASS；`target/button-checks/20260910T084552839021Z/` |
| fmt / 边界 / 88 Python 测试 | PASS；`target/verify-navigation-compact/` |
| Gallery 构建 | 首次被旧 exe 占用而 FAIL；正常关闭旧窗口后重试 PASS，`build-retry.log` |
| Windows 整窗/UIA | PASS；`target/navigation-compact-window/result.json` 与 PNG |

该 Gallery 二进制 SHA256：`f9b829bcf51d4f625b7b5f2d7064993b806514b9666e145e9b6d70e96160bff5`。
普通/最大化 Back 为 47x30 / 47x29 物理像素，与原生最小化矩形相同，三系统按钮
纵向中心偏差为 0（OS 100% / Slint scale 1）。系统按钮自身宽度并不完全相同。
此前完整 Rust/Clippy 检查在 `target/verify-navigation-header/`；compact 修正未重跑
该两项，只重新编译 Gallery 并执行上述专项。完整限制继续在 [STATUS](STATUS.md) 保留。
