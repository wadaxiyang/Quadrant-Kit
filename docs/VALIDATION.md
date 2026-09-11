# Validation

要求 Python >=3.11、固定 Rust/Slint 工具链及平台原生构建依赖。命令从 Kit 根目录执行。
选择与改动相关的检查；记录实际命令、源码 SHA/dirty/内容哈希、环境、结果和原始报告。
静态、编译、输入、截图、读屏、性能分别报告 PASS / FAIL / NOT_RUN / BLOCKED。

## Core checks

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/check_ui_boundaries.py
python scripts/check_native_reuse.py
python -m unittest discover -s scripts/tests -p "test_*.py"
cargo build --locked -p quadrant-kit-gallery
```

边界检查包括分层/循环、Gallery 公共入口、当前 API/默认值、原生复用、Cargo 解析图、
本地资源路径、hash 与许可。根包正常依赖为空；Gallery/测试路径例外不授权 Product 覆盖。
未知语法显式失败。词法分析不验证表达式类型、继承成员、实际输入或辅助技术；
锁定编译器、`gallery/ui/api_probe.slint` 与运行检查补足这些类别。

## API review

```console
python scripts/check_ui_boundaries.py --write-baseline target/kit_api_candidate.json
git diff --no-index scripts/kit_api_v1.json target/kit_api_candidate.json
```

第一条只生成候选，不表示通过。审核新增/删除、成员、类型和默认表达式，同步 API 文档、
probe、调用和行为断言后才采用快照。CI 不自动刷新。函数体等行为不由声明快照证明。
公共 API 文档声明、组件状态/manifest/catalog 和 README 架构图另有一致性测试；
文档内的本地链接与章节锚点也由 Python 测试检查。

## Runtime suites

```console
python scripts/run_button_checks.py --suite navigation --profile release
```

同一入口支持下列 suite；`--build-only` 不宣称输入通过，长生命周期可设置有界
`--timeout-seconds`（1..300）。生成源码、锁文件、解析图、日志、图像和结果保存在 target。

| Suite | 重点 |
|---|---|
| button / foundation | 按钮、焦点、禁用、文本、基础组合 |
| text-field | 默认普通输入、Light/Dark 原生遮蔽对照、实际值编辑/提交、程序赋值静默、选择/清空、禁用和焦点；仅虚构凭据 |
| selection | 原生 checked、RadioGroup 静态 child 语法、模型/程序更新 |
| numeric | Slider/SpinBox 边界、只读与 progress 停止/隐藏 |
| containers | 原生滚动、列表选择、虚拟化、GroupBox/TabWidget |
| pickers | 表格状态/排序请求、Date/Time 原生类型、取消与焦点 |
| toast / modal | 一次性请求、关闭、有限焦点与恢复 |
| navigation | 层级、非法模型、受控状态、键盘、居中收起态与搜索保留 |
| popup / inline | 原生 popup/menu、关闭、焦点和 host 条件子内容 |
| motion | 快速反转、reduced-motion、退出输入/Timer 与 100 次生命周期 |
| gallery-settings | 实际 toolbar/公共 Back、主题/预览及页面重建 |

WindowEvent 输入是控件运行证据；Unicode 编辑不等于 OS IME。Native Menu 的系统输入
只在验证进程拥有前台窗口时发送。UIA 也不等于完整读屏验证。

## Gallery and native window

```console
python scripts/capture_gallery_baseline.py --mode Smoke --destination navigation-view
python scripts/capture_gallery_baseline.py --mode Catalog
```

场景、参数和练习见 [GALLERY](GALLERY.md#snapshot-interface)。成功写出图像与人工视觉
判读分开；模拟 scale 不证明真实显示器切换，软件图像不包含 DWM 合成区域。

Windows 整窗与 UIA 检查：先把构建好的 exe 复制为
`target/gallery-settings-uia/gallery.exe`，再执行：

```console
powershell -NoProfile -File scripts/probe_gallery_settings.ps1 -Executable target/gallery-settings-uia/gallery.exe -OutputDirectory target/gallery-settings-uia
```

该脚本只操作自己的 Gallery，记录 PrintWindow、普通/最大化几何、Back/Settings、
收起态唯一 Toggle、无搜索/箭头、主/页脚居中和展开恢复。构建时不要运行输出目录里的
exe，以免 Windows 链接器无法替换它。

## Documentation website

GitHub Pages 从 `docs/` 的同一份 Markdown 构建，不另存 API 副本。根目录 `mkdocs.yml`
维护导航/主题，`docs-site/hooks.py` 将站点外源码链接转为对应 Git SHA 的 GitHub 链接。

```console
python -m venv target/docs-venv
# Windows: target/docs-venv/Scripts/python.exe；Linux/macOS: target/docs-venv/bin/python
target/docs-venv/Scripts/python.exe -m pip install -r docs-site/requirements.txt
target/docs-venv/Scripts/python.exe -m mkdocs build --strict
target/docs-venv/Scripts/python.exe -m mkdocs serve -a 127.0.0.1:8000
```

严格构建检查失效链接/锚点；浏览器另验 API 章节、搜索、主题与窄屏。`.github/workflows/docs.yml`
在文档 PR 中只构建；合并/推送 main 后部署到 GitHub Pages，使用 `github-pages` environment。
站点产物位于忽略的 `target/docs-site/`，依赖只用于文档工具，不进入 Kit runtime。

## Distribution

从干净、已提交且包含当前预期源码的 checkout 执行：

```console
python scripts/verify_distribution.py --package
cargo package --locked -p quadrant-kit --list
cargo package --locked -p quadrant-kit
python scripts/verify_distribution.py --package --archive target/package/quadrant-kit-0.1.0.crate
python scripts/verify_package_consumer.py --help
cargo +1.92.0 build --locked -p quadrant-kit -p quadrant-kit-gallery --target-dir target/msrv-1.92
```

先看实际 archive consumer 脚本参数再选输入。包检查核对 Git-tracked 源码/静态资源闭包
及实际 archive 字节；不为清洁状态擅自提交文件。发行与外部接入见 [CONSUMER_GUIDE](CONSUMER_GUIDE.md)。

明确授权远程验证后，用 `verify_distribution.py --remote`，指定 `--kit-url`、已验证
40 位 `--rev` 和 `--retained-ref`；支持 `--run-gui`、`--result`。它匿名获取/核对保留
引用，隔离 Cargo/Git 源码与凭证覆盖，生成全新 Git+SHA 消费者。Build-only 的运行状态
是 NOT_RUN；本地包消费者不等于新远程消费者。

## Exclusive and performance checks

`python scripts/verify_incremental.py` 需要独占 checkout/build：它修改再精确恢复深层
token 与 SVG，验证 build-script 重跑及二进制变化，记录源码恢复状态。不要并发编辑。
性能场景、采样和预算见 [PERFORMANCE](PERFORMANCE.md)；不得改变预算掩盖失败。

CI 配置在 [.github/workflows/ci.yml](../.github/workflows/ci.yml)。工作流存在不等于该
SHA 已通过；平台/发布现状见 [STATUS](STATUS.md)，历史执行见 [HISTORY](HISTORY.md)。
