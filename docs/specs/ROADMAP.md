# Fluent evolution roadmap

这是原阶段的范围索引，不是新的待执行任务。已交付与缺口见 [STATUS](../STATUS.md)，
历史证据见 [HISTORY](../HISTORY.md)，共用门禁见 [ACCEPTANCE](ACCEPTANCE.md)。
各阶段应核对实际 HEAD、已有改动和当前实现，而不是回退到原审计提交。

| 阶段 | 范围 | 必须交代的交付 |
|---|---|---|
| P0 | 当前源码、API、原生能力与视觉/性能基线 | 清单、探针、基线和缺口；不开始生产 UI 迁移 |
| P1 | 模块化、版本政策、长期规则与 fail-closed scanner | 静态导出、分层、当前声明/默认值与原生清单检查 |
| P2 | FluentButton 垂直切片 | 真正原生输入、视觉/焦点/禁用、受控状态、匹配测量；再推广 |
| P3 | 现有基础组件与视觉 | 一一交代迁移/保留/删除，字段/slot/命名和当前调用同步 |
| P4 | 原生封装组，按 A→D | 每批先定义公开能力与语法限制，再实现、导出、示例和验收 |
| P5 | 浮层/导航/行内组合，按 A→E | 生命周期、焦点、关闭/取消和状态所有权；不制造统一运行服务 |
| P6 | 仅 Kit 所有的有限动效 | 原生动效不重复；中断、隐藏、reduced-motion 和关闭清理验证 |
| P7 | 轻量化和当前版本集成 | 配对测量、规模增长、交互/空闲/100 周期、当前消费者与包 |
| P8 | 当前文档、合约与候选收尾 | 一致的 API/许可/示例、剩余验收缺口、候选分发条件 |

## P4 batches

| 批次 | 组件 | 重点 |
|---|---|---|
| A | CheckBox、Switch、RadioGroup、ComboBox | checked/selection、模型替换、禁用与实际键盘语法 |
| B | Slider、SpinBox、ProgressBar、ProgressRing | 边界/步长/read-only/int、停止/隐藏状态 |
| C | ScrollView、ListView、StandardListView、GroupBox、TabWidget | 滚动、虚拟化、selection、原生 children 与 slot 生命周期 |
| D | StandardTableView、DatePicker、TimePicker | 原生类型、排序请求、有效值/取消、位置和焦点 |

## P5 batches

| 批次 | 范围 | 重点 |
|---|---|---|
| A | Tooltip / Toast | 服务与呈现分离、一次性关闭、hover/自动关闭与隐藏 |
| B | ModalManager | 固定动作初始焦点、Tab/Return/Escape、关闭及 host 恢复 |
| C | NavigationView | 16/64/256/257 校验、行通知、选中与结构分离、层级和焦点恢复 |
| D | Flyout、DropDownButton、SplitButton、native menu 示例 | 锚点、原生关闭/焦点、窗口边缘与快速反转 |
| E | Expander、InfoBar | host 条件子内容、程序收起焦点、一次性 dismiss |

P4/P5 未指定更细范围时只执行首个未完成子阶段。每一批确认当前调用/API、运行行为和
实测限制后结束；不足项如实记录，不能用编译或截图代替输入/性能。
后续修复可以修改已完成组件；其结果归入当前状态与证据摘要，不保留平行旧实现。
