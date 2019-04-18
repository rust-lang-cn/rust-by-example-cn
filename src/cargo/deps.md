# 依赖

大多数程序都要依赖一些库。如果你手工管理过依赖，你一定知道那是一件多么痛苦的
事情。幸运的是，`cargo` 是 Rust 生态系统的标配！`cargo` 能够管理一个工程的依赖。

使用以下命令可以新建工程：

```sh
# 目标是二进制文件
cargo new --bin foo

# 或者是库
cargo new foo
```

在本章接下来的部分，我会假设我们在编写可执行的应用，而不是库，不过它们所有的概念
都是相同的。

在执行上述命令后，你应该会看到：

```txt
foo
├── Cargo.toml
└── src
    └── main.rs
```

`main.rs` 是你的新工程的根源文件（root source file），这里没什么新奇的。`Cargo.toml`
 是该工程（`foo`）的 `cargo` 配置文件。如果你打开看看，会看到类似这样的内容：

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```

你可以在[这里](http://doc.crates.io/manifest.html)找到所有可用的配置选项。

`package` 下面的 `name` 字段决定了工程的名字。如果你在 `crates.io` 上发布
 `crate`（详见后文），就会使用这个名字。这也是编译产生的二进制文件的名字。

`version` 字段是 crate 的版本号，遵循 [Semantic Versioning](http://semver.org/).

`authors` 字段是发布 crate 时所用的作者列表。

`dependencies` 部分用来添加工程的依赖。

比如说，我想让我的程序拥有一个好看的字符界面（CLI），那我可以在
 [crates.io](https://crates.io) 上找到不少这种库。[clap](https://crates.io/crates/clap)
 是比较流行的一个。在本书写作时，`clap` 最近的版本是 `2.27.1`。要给工程加上这个
依赖，只需在 `Cargo.toml` 的 `dependencies` 下面加上 `clap = "2.27.1"`。当然，还
需要在 `main.rs` 中写上 `extern crate clap`，这和以前一样。这样就完成了！接下来
你可以在程序中使用 `clap` 了。

`cargo` 也支持其他的依赖来源。下面是一些例子，你可以
在[这里](http://doc.crates.io/specifying-dependencies.html)找到更多。


```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # 来自 crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自在线仓库
bar = { path = "../bar" } # 来自本地文件系统的一个路径
```

要构建工程，可以在工程的任何路径（包括子路径！）运行 `cargo build`。也可以运行
 `cargo run` 来构建 + 运行。注意这些命令会解决所有依赖问题，下载 crate（如果
需要的话），然后构建它们和你的 `crate`（注意它只会构建那些还没有构建的东西，和
 `make` 一样）。
 
好了，这节就学到这里！
