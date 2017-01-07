文档注释对于需要文档的大型项目来说非常重要。当运行 [Rustdoc][1]，这些注释就会编译成文档。它们使用 `///` 标记，并支持 [`Markdown`][2]。

{doc.play}

要运行测试，首先将代码构建为库，然后告诉 `rustdoc` 在哪里找到库，以便它可以将代码链接成各个文档测试程序：

```rust
rustc doc.rs --crate-type lib
rustdoc --test --extern doc="libdoc.rs"
```

（当你在库 crate 上运行 `cargo test` 时，`Cargo` 将自动生成并运行正确的 `rustc` 和 `rustdoc` 命令。）

[1]: http://doc.rust-lang.org/book/documentation.html
[2]: https://en.wikipedia.org/wiki/Markdown
