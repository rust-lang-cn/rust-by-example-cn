通过对错误类型实现 `Display` 和 `From`，我们能够利用上绝大部分标准库错误处理工具。然而，我们遗漏了一个功能：轻松 `Box` 我们错误类型的能力。

标准库会自动通过 `Form` 将任意实现了 `Error` trait 的类型转换成 trait 对象 `Box<Error>` 的类型（原文：The `std` library automatically converts any type that implements the `Error` trait into the trait object `Box<Error>`, via `From`. ）。对于一个库用户，下面可以很容易做到：

```rust
fn foo(...) -> Result<T, Box<Error>> { ... }
```

用户可以使用一系列外部库，其中每个都提供各自错误类型。为了定义一个有效的 `Result<T, E>` 类型，用户有几个选择：

* 定义一个新的限定在外部库错误类型的包装（wrapper）错误类型（原文：define a new wrapper error type around the libraries error types）
* 将错误类型转换成 `String` 或者其他合适的选择
* 通过类型擦除（type erasure）将错误类型装包（`Box`）成 `Box<Error>`

将内容“装包”（"Boxing"）是一个常见的选择。缺点是潜在的错误类型只能在运行时知道，且不能[静态确定][dynamic_dispatch]（statically determined）。正如刚才提到的，要做到这点所有要做的事情就是实现 `Error` trait：

```rust
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

有了这个实现后，我们再来回顾前面学过的最近例子。注意到它所带的错误类型 `Box<Error>` 也变成有效的了，就像前面用到的 `DoubleError` 那样（原文：With this implementation, let's look at our most recent example. Note that it is just as valid with the error type of `Box<Error>` as it was before with `DoubleError`）：

{boxing_errors.play}

### 参见：

[Dynamic dispatch][dynamic_dispatch] 和 [`Error` trait][error]

[dynamic_dispatch]: http://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error]: http://doc.rust-lang.org/std/error/trait.Error.html
