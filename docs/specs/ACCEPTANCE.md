# Acceptance

本页是要求；执行入口见 [VALIDATION](../VALIDATION.md)，预算见 [PERFORMANCE](../PERFORMANCE.md)，
实际结果见 [STATUS](../STATUS.md)。未执行项写 NOT_RUN，环境/授权阻碍写 BLOCKED；失败
保留原因与原始结果。没有真实数据就不宣称完整 Fluent、读屏、性能或跨平台验收。

## Source and contracts

- 当前 facade、声明/默认值、snapshot、probe、Gallery/catalog、组件状态及原生清单一致。
  所有导出有真实类型/组件使用；新增组件无 runtime 注册，删除旧声明同步当前调用。
- 根正常依赖为空、层级无环、实现不导入 facade、Gallery 不导入私有 Kit、Product 源码
  边界不变。公开语法、静态资源、许可证与实际 archive 字节必须核验。
- 公共属性保持单一状态所有者，程序赋值和用户回调语义明确。事件不因动画或重复按键
  意外重发；disabled/read-only、空/非法模型、越界和 slot 输入有明确合约。
- 编译、真实输入和辅助技术独立验证。继承成员和编译器特殊 child 语法需要实际生成
  Rust 构建，不能只运行 build script 或依赖词法扫描。

## Interaction and lifecycle

| 范围 | 验收场景 |
|---|---|
| 普通动作 | 点击、Space/Return、Tab/Shift+Tab、禁用、焦点、重复/取消；一个真实输入所有者 |
| 编辑/选择 | Empty/Value/Error/Read-only、模型和程序更新、中文/IME、选区/剪贴板、禁用 |
| Tooltip/popup/menu/picker | 四边、滚动容器、窄窗、高 DPI、宿主移动/缩放、快速开关、失焦及关闭 |
| 导航 | 主/页脚及 0–2 深度、16/64/256/257、行变更和整模型替换、展开/收起、键盘与焦点恢复 |
| Toast/InfoBar | 每个真实 shown 周期最多一次关闭请求；程序隐藏不伪装用户操作；消息变化不重置周期 |
| Modal | 就绪后焦点到正确动作；次按钮 Return 不被全局默认动作覆盖；Tab/Escape 与一次性请求；关闭取消旧聚焦并请求 host 恢复 |
| 任意 slot | 宿主显式绑定 enabled/active；Expander 用条件构造卸载 children；不能靠 opacity 隐藏输入 |
| 动效 | 中途反向、reduced/disabled、零时长、隐藏和再开；退出有截止期，随后没有非必要 Timer |

Modal 只验收固定确认动作集合，不借 scrim 宣称任意内容焦点隔离、嵌套栈或完整读屏。
弹出层不能靠 z 值冒充跨 clip/跨原生窗口服务。Toast 不抢焦点；恢复目标由 host 提供。
保留隐藏祖先时必须关闭或卸载子浮层，不能假设祖先不可见会自动停止所有工作。

## Visual and accessibility

代表控件检查 Normal/Hover/Pressed/Disabled/Keyboard focus，加适用的选中/输入状态；
Light/Dark、中文/英文、短/长文本、正常/窄宽度与 100/200/225% scale。其他组件按风险
扩展矩阵，避免没有判读价值的全排列。几何、文本、图标、颜色、边框和焦点分别审查。

截图记录 SHA/dirty/内容和二进制 hash、场景、逻辑尺寸、scale、主题、字体、后端和
renderer；标明静态 reference、真实触发状态及人工判读范围。字体/抗锯齿差异单独解释，
不能用宽松像素阈值掩盖问题。真实 WinUI 参考记录实际版本、同机环境和来源。

可访问标签/role 不等于完整可访问性。单独检查动作、状态、Tab 路径、读屏名称/值/禁用、
动态宣告和焦点恢复。UIA、截图、模拟 scale 不替代实际 reader/OS IME/物理显示器证据。

## Performance and distribution

按 [PERFORMANCE](../PERFORMANCE.md) 匹配 A/B、保留原始样本与既定阈值，覆盖空窗口/
仅导入、1/100/1000 控件、100/1000/10000 列表、导航规模、100 次页面/浮层周期与 >=60s idle。
分别报告二进制、构造/软件缓冲/真实 present、Private Bytes/Working Set/GPU、交互、
对象增长和隐藏工作。不能删样本、修剪工作集、牺牲输入/可访问性或放宽预算制造 PASS。

包/消费者从实际预期源码验证，deep token/SVG 必须正确触发重编译并恢复原字节；原生
构建资源、字体和平台运行另验。远程发布需要明确授权、同 SHA CI、保留引用及独立匿名
Git+SHA 消费者；本地构建或历史发布不能替代。不得因为包检查要求 clean 而擅自提交。
