# RMD - Fast Native Windows Markdown Editor

一款快速、原生 Windows 的 Markdown 编辑器，支持实时预览。使用 Rust 和 GPU-UI (egui) 构建。

## 特性

- **实时预览** - 编辑时即时预览渲染结果
- **GPU 加速** - 使用 egui 构建的原生 GPU 渲染 UI
- **多布局模式** - 仅编辑器、仅预览、分屏视图
- **语法高亮** - Markdown 语法实时高亮
- **代码块高亮** - 使用 syntect 支持多种编程语言
- **主题支持** - 内置浅色/深色主题
- **文件拖放** - 支持拖放文件打开
- **自动保存** - 可配置的自动保存功能
- **快捷键支持** - 完整的键盘快捷键支持
- **导出功能** - 支持导出 HTML/PDF

## 截图

[待添加]

## 系统要求

- Windows 10/11 (64-bit)
- GPU 支持 DirectX 11/12 或 Vulkan
- 至少 4GB RAM

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/rmd.git
cd rmd

# 构建 Release 版本
cargo build --release

# 运行
./target/release/rmd.exe
```

### 下载预编译版本

从 [Releases](https://github.com/yourusername/rmd/releases) 页面下载最新版本。

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl + N` | 新建文件 |
| `Ctrl + O` | 打开文件 |
| `Ctrl + S` | 保存文件 |
| `Ctrl + Shift + S` | 另存为 |
| `Ctrl + Z` | 撤销 |
| `Ctrl + Y` | 重做 |
| `Ctrl + X` | 剪切 |
| `Ctrl + C` | 复制 |
| `Ctrl + V` | 粘贴 |
| `Ctrl + F` | 查找 |
| `Ctrl + H` | 替换 |
| `Ctrl + +` | 放大 |
| `Ctrl + -` | 缩小 |
| `Ctrl + 0` | 重置缩放 |
| `Ctrl + 1` | 仅编辑器模式 |
| `Ctrl + 2` | 仅预览模式 |
| `Ctrl + 3` | 分屏模式 |
| `F11` | 全屏切换 |
| `Alt + F4` | 退出 |

## 配置

配置文件位于 `%APPDATA%/rmd/config.toml`：

```toml
# 主题设置
theme_mode = "system"  # "light", "dark", "system"

# 布局设置
layout_mode = "split"  # "editor_only", "preview_only", "split"
split_direction = "horizontal"  # "horizontal", "vertical"

# 字体设置
[font]
editor_font = "Fira Code"
editor_font_size = 14.0
ui_font = "Segoe UI"
ui_font_size = 14.0

# 编辑器设置
[editor]
word_wrap = true
show_line_numbers = true
highlight_current_line = true
auto_indent = true
tab_size = 4

# 自动保存
auto_save = false
auto_save_interval_seconds = 30
```

## 开发

### 项目结构

```
rmd/
├── Cargo.toml          # 项目配置
├── src/
│   ├── main.rs         # 程序入口
│   ├── app.rs          # 主应用逻辑
│   ├── config.rs       # 配置管理
│   ├── editor/         # 编辑器模块
│   │   ├── mod.rs
│   │   ├── text_buffer.rs
│   │   └── highlighter.rs
│   ├── markdown/       # Markdown 渲染
│   │   └── mod.rs
│   ├── preview/        # 预览模块
│   │   └── mod.rs
│   ├── theme/          # 主题系统
│   │   └── mod.rs
│   ├── ui/             # UI 组件
│   │   ├── mod.rs
│   │   ├── layouts.rs
│   │   └── widgets/
│   └── utils/          # 工具函数
│       └── mod.rs
├── assets/             # 资源文件
│   └── fonts/
└── README.md
```

### 构建

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release

# 运行测试
cargo test

# 检查代码
cargo clippy

# 格式化代码
cargo fmt
```

## 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建你的功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- [egui](https://github.com/emilk/egui) - 即时模式 GUI 库
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown 解析器
- [syntect](https://github.com/trishume/syntect) - 语法高亮

---

Made with ❤️ and Rust
