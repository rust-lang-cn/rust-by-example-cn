# 构建脚本

有时使用 `cargo` 正常构建还是不够的。也许你的 crate 在 cargo 成功编译之前需要一些先决条件，比如代码生成或者需要编译的一些本地代码。为了解决这个问题，我们构建了 cargo 可以运行的脚本。

要向包中添加构建脚本，可以在 `Cargo.toml` 中指定它，如下所示：

```toml
[package]
...
build = "build.rs"
```

跟默认情况不同，这里 cargo 将在项目目录中优先查找 `build.rs` 文件。（本句采用意译，英文原文为：Otherwise Cargo will look for a `build.rs` file in the project directory by default.）

## 怎么使用构建脚本

构建脚本只是另一个 Rust 文件，此文件将在编译包中的任何其他内容之前，优先进行编译和调用。 因此，此文件可实现满足 crate 的先决条件。

cargo 通过[此处指定][specified
here]的可以使用的环境变量为脚本提供输入。（英文原文：Cargo provides the script with inputs via environment variables [specified
here] that can be used.）

此脚本通过 stdout （标准输出）提供输出。打印的所有行都写入到 `target/debug/build/<pkg>/output`。另外，以 `cargo:` 为前缀的行将由 cargo 直接解析，因此可用于定义包编译的参数。

有关进一步的说明和示例，请阅读 [cargo 规定说明文档][cargo specification]。

[specified here]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

[cargo specification]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
