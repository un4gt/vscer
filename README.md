# Vscer

**Vscer** 是一个用 Rust 编写的命令行工具，用于查询 Visual Studio Marketplace 上的扩展信息并下载对应的 VSIX 扩展包。该工具提供了交互式选择功能，可以让你选择扩展的版本和目标平台，并通过加载动画提升用户体验。

## 功能

- **查询扩展信息**  
  根据指定的扩展 ID，从 Visual Studio Marketplace 获取扩展详细信息（包括版本、目标平台、VSIX 文件地址等）。

- **解析与分组**  
  解析返回的 JSON 数据，将扩展版本按版本号和目标平台进行分组（使用有序的 [IndexMap](https://docs.rs/indexmap/latest/indexmap/) 保留插入顺序）。

- **交互式选择**  
  使用 [inquire](https://crates.io/crates/inquire) 提供交互式命令行界面，让用户选择所需的版本以及目标平台。

- **下载 VSIX 文件**  
  下载所选版本的 VSIX 扩展包，并保存至用户指定的目录中。

- **加载动画**  
  使用 [indicatif](https://crates.io/crates/indicatif) 展示加载进度动画（spinner），改善用户体验。

## 技术栈

- **Rust**  
  稳定、高性能、跨平台的系统编程语言。

- **主要依赖**
    - [clap](https://crates.io/crates/clap)：命令行参数解析。
    - [reqwest](https://crates.io/crates/reqwest)：同步 HTTP 客户端，用于发送网络请求。
    - [serde_json](https://crates.io/crates/serde_json)：解析 JSON 数据。
    - [indicatif](https://crates.io/crates/indicatif)：进度条和加载动画。
    - [inquire](https://crates.io/crates/inquire)：交互式命令行提示。
    - [indexmap](https://crates.io/crates/indexmap)：保持插入顺序的有序 Map 数据结构。

## 安装

### 前提条件

- 安装 [Rust](https://rustup.rs)

### 克隆与构建

```bash
git clone https://github.com/yourusername/vscer.git
cd vscer
cargo build --release
