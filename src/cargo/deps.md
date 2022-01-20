# 依赖

大多数程序都会依赖于某些库。如果你曾经手动管理过库依赖，那么你就知道这会带来的极大的痛苦。幸运的是，Rust 的生态链标配 `cargo` 工具！`cargo` 可以管理项目的依赖关系。

下面创建一个新的 Rust 项目：

```sh
# 二进制可执行文件
cargo new foo

# 或者库
cargo new --lib foo
```

对于本章的其余部分，我们选定创建的都是二进制可执行文件而不是库，但所有的概念都是相同的。

完成上述命令后，将看到如下内容：

```txt
foo
├── Cargo.toml
└── src
    └── main.rs
```

`main.rs` 是新项目的入口源文件——这里没什么新东西。 `Cargo.toml` 是本项目（`foo`）的 `cargo` 的配置文件。 浏览 `Cargo.toml` 文件，将看到类似以下的的内容：

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```

`package` 下面的 `name` 字段表明项目的名称。 如果您发布 crate（后面将做更多介绍），那么 `crates.io` 将使用此字段标明的名称。 这也是编译时输出的二进制可执行文件的名称。

`version` 字段是使用[语义版本控制](https://semver.org/)（Semantic
Versioning）的 crate 版本号。

`authors` 字段表明发布 crate 时的作者列表。

`dependencies` 这部分可以让你为项目添加依赖。

举个例子，假设我们希望程序有一个很棒的命令行界面（command-line interface，CLI））。 你可以在 [crates.io](https://crates.io)（官方的 Rust 包注册服务）上找到很多很棒的 Rust 包。其中一个受欢迎的包是 [clap](https://crates.io/crates/clap)（译注：一个命令行参数的解析器）。在撰写本文时，[clap] 最新发布的版本为 `2.27.1`。要在程序中添加依赖，我们可以很简单地在 `Cargo.toml` 文件中的 `dependencies` 项后面将以下内容添加进来 ：`clap = "2.27.1"`。当然，在 `main.rs` 文件中写上 `extern crate clap`，就和平常一样。 就是这样！你就可以在程序中开始使用 `clap` 了。

`cargo` 还支持[其他类型的依赖][dependencies]。 下面是一个简单的示例：

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # 来自 crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自网上的仓库
bar = { path = "../bar" } # 来自本地文件系统的路径
```

`cargo` 不仅仅是一个包依赖管理器。`Cargo.toml` 的所有可用配置选项都列在 [格式规范][manifest]中。

要构建我们的项目，我们可以在项目目录中的任何位置（包括子目录！）执行 `cargo build`。我们也可以执行 `cargo run` 来构建和运行。请注意，这些命令将处理所有依赖，在需要时下载 crate，并构建所有内容，包括 crate。（请注意，它只重新构建尚未构建的内容，这和 `make` 类似）。

瞧！这里的所有都和 `cargo` 有关！


[manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
