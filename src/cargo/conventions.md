# 约定规范

在上一小节中，我们看到了以下目录层次结构：

```txt
foo
├── Cargo.toml
└── src
    └── main.rs
```

假设我们要在同一个项目中有两个二进制可执行文件。 那要怎样做呢？

很显然，`cargo` 支持这一点。正如我们之前看到的，默认二进制名称是 `main`，但可以通过将文件放在 `bin/` 目录中来添加其他二进制可执行文件：

```txt
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

为了使得 `cargo` 只编译或运行这个二进制可执行文件，我们只需给 `cargo` 增加一个参数 `--bin my_other_bin`，其中 `my_other_bin` 是我们想要使用的二进制可执行文件的名称。

除了可添加其他二进制可执行文件外，`cargo` 还支持[更多功能][more features]，如基准测试，测试和示例。

在下一节中，我们将更仔细地学习测试的内容。

[more features]: https://doc.rust-lang.org/cargo/guide/project-layout.html
