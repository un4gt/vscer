# Vscer

English | [简体中文](README_zh.md)

**Vscer** is a command-line tool written in [Rust](https://www.rust-lang.org/) for downloading vscode extension.

*[Can't download VSIX extensions from the web marketplace anymore ? : r/vscode](https://www.reddit.com/r/vscode/comments/1i6k7gf/cant_download_vsix_extensions_from_the_web/)*

[![Demo](https://markdown-videos-api.jorgenkh.no/url?url=https%3A%2F%2Fyoutu.be%2FUWDHJKj24ls)](https://youtu.be/UWDHJKj24ls)

## Tech Stack
- **Rust**  
  A stable, high-performance, cross-platform system programming language.
- **Key Dependencies**
    - [clap](https://crates.io/crates/clap): Command-line argument parsing.
    - [reqwest](https://crates.io/crates/reqwest): A synchronous HTTP client for sending network requests.
    - [serde_json](https://crates.io/crates/serde_json): Parsing JSON data.
    - [indicatif](https://crates.io/crates/indicatif): Progress bars and loading animations.
    - [inquire](https://crates.io/crates/inquire): Interactive command-line prompts.
    - [indexmap](https://crates.io/crates/indexmap): An ordered Map data structure that maintains insertion order.


## Download

You can download from [Releases](https://github.com/un4gt/vscer/releases).

*Thanks for [dist](https://opensource.axo.dev/cargo-dist/book/introduction.html)*

Or build it locally:

```bash
git clone https://github.com/un4gt/vscer.git
cd vscer
cargo build --release
```

## Usage

Type `vscer --help` to get help information.

```plaintext
Usage: vscer.exe [OPTIONS] --ext-id <EXT_ID>

Options:
  -i, --ext-id <EXT_ID>
          extension id, eg: `charliermarsh.ruff`
  -n, --n-latest <N_LATEST>
          number of latest versions to display, default is 10
  -s, --spec-version <SPEC_VERSION>
          specified version to download
  -h, --help
          Print help
  -V, --version
          Print version
```

### Examples

By default, the latest 10 versions of the extension will be displayed.

- List the latest 20 versions of the extension:

```bash
vscer -i charliermarsh.ruff -n 20
```

- Download the specified version of the extension:

```bash
vscer -i charliermarsh.ruff -s x.x.x
```