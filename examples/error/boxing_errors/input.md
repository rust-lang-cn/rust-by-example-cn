我们已经看到，通过对错误类型实现 `Display` 和 `From`，我们能够利用上绝大部分标准库错误处理工具。那就是说，我们漏掉了一个功能：轻松 `box` 我们错误类型的能力。

也就是说，任意通过 `Form` 实现 `Error` trait 到 trait 对象 `Box<Error>` 的类型，都能被标准库自动转换（原文：Namely, the std library will automatically convert from any type which implements the `Error` trait into the trait object `Box<Error>` via `From`.）。对于一个库用户，下面可以很容易做到：

```rust
// 任意错误类型自动转换成 `Box<Error>` 都可以在这里使用。
// （原文：Any error type automatically convertible to `Box<Error>` may be used here.）
fn foo(...) -> Result<T, Box<Error>> { ... }
```

例如，用户可以使用一系列库，其中每个都提供各自错误类型。为了定义一个有效的 `Result<T, E>` 类型，用户有几个选择：

* 定义一个新的限定在外部库错误类型的包装（wrapper）错误类型（原文：define a new wrapper error type around the external libraries error types）
* 将它转换成 `String` 或者其他合适的选择
* 通过类型擦除（type erasure）将它装包（box）成 `Box<Error>`

将内容装包是一个常见的选择。唯一的代价是潜在的错误类型只能在运行时知道，且不能[静态确定][dynamic_dispatch]（statically determined）。要做到这点所有要做的事情就是实现 `Error` trait：

```rust
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

通过实现这点，当错误类型为 `Box<Error>` 时，我们前面的例子也将变成有效的了，就像前面用到的 `DoubleError` 那样。（原文：By implementing this, our previous example would be just as valid when the error type is `Box<Error>` as it was before with `DoubleError`.）

{rethink.play}

### 参见：

[Dynamic dispatch][dynamic_dispatch] 和 [`Error` trait][error]

[dynamic_dispatch]: http://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error]: http://doc.rust-lang.org/std/error/trait.Error.html
