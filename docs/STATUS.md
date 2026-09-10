# 当前状态与待办

文档整理基线：`823c87216bf1843058b41a3cb30bdfc985d196b3`。
当前 0.1.0 Fluent 演进实现及后续导航修正已提交到本地；**完整发布验收仍为 PARTIAL**。
这份状态不等于远程发布、稳定版声明或 Tasks 依赖更新。历史结果和精确来源见
[HISTORY](HISTORY.md)，组件职责与限制见 [COMPONENT_STATUS](COMPONENT_STATUS.md)。

## 已交付

- 源码-only 根包、统一静态 facade、分层约束及当前声明/资源/原生复用检查。
- 59 个公开名称：42 个视觉组件、6 个 globals、8 个 enums、3 个 structs。
- 原生输入/选择/数值/容器/表格/日期时间控件，以及有限导航、浮层、确认与行内组合。
- Gallery 33 个可达页面，固定 Settings 入口，主题和预览宽度设置，源码示例。
- Kit 负责收起态导航布局及公共无边框 Back；Gallery 只负责宿主路由和原生窗口几何。
- 主机控制的状态和每窗口主题/动效策略；Toast/Modal 有限退出生命周期。

## 验收缺口

| 项目 | 状态 | 尚需证据 |
|---|---|---|
| 当前源码 Windows 编译与组件运行 | 已有 scoped PASS | 历史结果只覆盖对应源码；改动后按风险重跑 |
| 本轮完整 WinUI 运行参考对比 | NOT_RUN | 同机、可识别版本的实际 WinUI 窗口参考 |
| 屏幕实际 present、GPU 内存、真实冷启动、持续重绘计数 | NOT_RUN | 相应采样器及实际平台数据；软件截图不能替代 |
| 长期内存稳定性 | 未关闭 | Gallery 往返内存存在波动/后段增长，需区分缓存、历史和对象生命周期；不宣称无泄漏 |
| 原始 hidden Toast 对空窗口的预算 | BLOCKED | 非等价参考的原始超标保留；另有匹配内容参考 PASS，不能互相覆盖 |
| 完整读屏、OS IME、真实跨显示器 DPI | NOT_RUN / 覆盖不足 | 各平台原生操作；UIA、Unicode 输入和模拟缩放不足以代替 |
| Linux 中文字体 | FAIL（已记录环境） | WSLg 缺中文字体，需正确配置宿主字体后复验；不打包系统字体 |
| 本轮 macOS 原生运行、输入和读屏 | NOT_RUN | 原生宿主验证；历史编译/交叉检查不能代替 |
| 当前 Fluent 版本发布 | NOT_RUN | 明确授权、同 SHA CI、保留引用和全新匿名 Git+SHA 消费者 |

一般 TreeView、任意内容 ContentDialog、完整 DataGrid、富文本和平台服务不属于已交付
范围，见 [SPEC](specs/QUADRANT_KIT_FLUENT_EVOLUTION_SPEC.md)。ModalManager 仅是固定确认层。
Kit 的 reduced-motion 也不控制全部原生动画。

## 后续工作方式

只执行用户指定的阶段或修复。需要继续原演进阶段时，先读
[路线](specs/ROADMAP.md)与[验收要求](specs/ACCEPTANCE.md)。新增证据更新本页和 HISTORY，
操作方法更新对应指南；不再为每次工具重试或短期 UI 调整新建常驻报告。
