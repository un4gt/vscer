# Vscer

简体中文 | [English](README.md)

**Vscer** 是一个用 Rust 编写的命令行工具，用于查询 Visual Studio Marketplace 上的扩展信息并下载对应的 VSIX 扩展包。该工具提供了交互式选择功能，可以让你选择扩展的版本和目标平台，并通过加载动画提升用户体验。


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

可以从 [Releases](https://github.com/un4gt/vscer/releases) 进行下载。

*感谢 [dist](https://opensource.axo.dev/cargo-dist/book/introduction.html)*

或者在本地进行构建：

```bash
git clone https://github.com/un4gt/vscer.git
cd vscer
cargo build --release
```

