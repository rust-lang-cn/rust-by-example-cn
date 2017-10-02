在 Rust 中，大部分运算符都可以通过 trait 来重载。也就是说，这些运算符可以根据它们输入的参数来完成不同的任务。为什么这样做是可行的呢，是因为运算符是对方法调用的语法糖。例如，`a + b` 中的 `+` 运算符会调用 `add` 方法（也就是 `a.add(b)`）。这个 `add` 方法是 `Add` trait 的一部分。因此，`+` 运算符可以被 `Add` trait 的实现者（implementor）使用。

[点击这里][ops]查看列举的重载运算符 trait，比如 `Add`。（原文：A list of the traits, such as `Add`, that overload operators are available [here][ops].）

{operator.play}

###参见：

[Add][add], [语法索引][syntax]

[add]: http://doc.rust-lang.org/core/ops/trait.Add.html
[ops]: http://doc.rust-lang.org/core/ops/
[syntax]: https://doc.rust-lang.org/book/syntax-index.html
