# 惯例

在上一节中，我们看到过这样的目录树：

```txt
foo
├── Cargo.toml
└── src
    └── main.rs
```

假设咱们想让一个工程生成两个可执行文件，怎么办呢？

`cargo` 是支持这样做的。默认的可执行文件是从 `main.rs` 生成的，不过你可以在
 `bin/` 目录下添加其他的可执行文件来源：

```txt
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

为了让 `cargo` 编译和运行 `my_other_bin.rs`，我们需要使用 `cargo` 的
 `--bin my_other_bin` 选项，其中 `my_other_bin` 是要处理的可执行文件名。

除去额外的可执行文件，`cargo` 还支持性能评估（benchmark）、测试（test）和
示例程序（example）。完整的功能介绍
请看[这里](http://doc.crates.io/book/guide/project-layout.html)。

在下一节，咱们会详细了解测试。
