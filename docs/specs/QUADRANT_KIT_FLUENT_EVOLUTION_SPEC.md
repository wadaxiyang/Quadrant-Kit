# Quadrant Kit Fluent evolution SPEC

v1.1 文档整理版：保留原目标、版本政策与验收边界，移除已落地的模板和重复阶段日志。
原文以 `737d0aae0975232f520cc1e82640d99e8eb49467` 审计；该 SHA 不是当前实现基线。
完整旧文可按 [历史追溯](../HISTORY.md#找回完整报告)读取。

## 阅读顺序

1. 根/UI/Gallery AGENTS.md：长期开发、分层与授权规则。
2. 本页：任务目标与边界。
3. [ROADMAP](ROADMAP.md)：原阶段范围；[ACCEPTANCE](ACCEPTANCE.md)：验收标准。
4. [STATUS](../STATUS.md)：已经交付什么、仍缺什么。要求不是已通过的结果。

## 目标

建设以 Slint 公开原生控件为行为基础、接近 WinUI 3 桌面视觉、可通过统一入口复用、
按版本独立演进且能测量其开销的源码组件库。

| 需求 | 维护位置 |
|---|---|
| 模块化与调用/实现分离 | [Architecture](../ARCHITECTURE.md)、[README 架构图](../../README.md#架构与组件接入)、静态 facade |
| 当前接口可以增删调整 | [Public API](../PUBLIC_API.md)、当前 snapshot/probe；不保留旧实现/兼容层 |
| 原生控件优先、有限例外 | [Native reuse](../NATIVE_REUSE.md) 和精确 manifest |
| 常用控件与组合补齐 | [Component status](../COMPONENT_STATUS.md)、Gallery 可达示例 |
| Fluent 视觉、清晰交互与状态 | [Design system](../DESIGN_SYSTEM.md)、组件行为合约 |
| 克制且可取消的动效 | [Motion](../MOTION.md) |
| 轻量、可验证与可分发 | [Performance](../PERFORMANCE.md)、[Validation](../VALIDATION.md)、[Consumer guide](../CONSUMER_GUIDE.md) |

## 边界

- 根包仅为构建提供源码路径，无正常/runtime 依赖、窗口、事件循环、业务模型或平台服务。
  不嵌入 Microsoft.UI.Xaml，不替换 Slint runtime，不修改 Tasks 或其依赖 revision。
- Slint/slint-build 固定 1.17.1，Rust MSRV 1.92；不夹带后端、工具链升级或另一轮窗口壳重写。
- 消费者只通过 `@quadrant-kit`；新增组件由所属层实现、facade 静态导出、当前示例验证。
  无运行时注册、万能字典接口、强制 host 服务或重复接口/实现层。
- 一个版本只有当前实现。声明、类型、方向、默认值和基类可以有理由地改变；同步当前
  文档/调用/测试/快照，不创建旧消费者兼容矩阵，也不删除仍有效的行为断言。
- 优先复用公开 native/builtin 的真实输入所有者。不导入或复制内部 widget，不隐藏原生
  代理。确有缺失能力的例外必须给出窄范围、理由和验证，不能用例外掩盖普通控件回归。
- 主题、系统状态、字体和 Motion 按窗口初始化；业务状态、路由/历史、OS 窗口动作属于宿主。
- 不承诺一般 TreeView、任意内容 ContentDialog、完整 DataGrid、富文本、CalendarView、
  可拖动多窗口 TabView、WebView/媒体/地图、通知中心、Mica/Acrylic 或 Lottie/粒子引擎。

只执行被要求的阶段/修复。普通泛化实现请求仍按 P0 处理；明确后续指令优先。
已完成的 P0–P8 不因阅读 SPEC 自动重做；阶段或当前待办不能成为发布授权。
