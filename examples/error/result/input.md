[`Result`][result] 是 [`Option`][option] 类型的更丰富的版本，描述的是可能的**错误**而不是可能的**不存在**。

也就是说，`Result<T，E>` 可以有两个结果的其中一个：

* `Ok<T>`：找到 `T` 元素
* `Err<E>`：发现错误，使用元素 `E` 表示（An error was found with element `E`）

按照约定，预期结果是 “Ok”，而意外结果是 “Err”。

和 `Option` 类似，`Result` 也有很多相关联的方法。例如 `unwrap（）`，能够产生元素 `T` 或 `panic`。 对于事件的处理，`Result` 和 `Option` 两者间有很多组合算子重叠。

使用 Rust 过程中，你可能会遇到返回 `Result` 类型的方法，例如 [`parse()`][parse] 方法。 它在某些情况下可能不能将一个字符串解析为另一种类型，所以 `parse()` 返回一个 `Result` 表示可能的失败。

我们来看看当 `parse()` 字符串成功和失败时会发生什么：

{result.play}

在失败的情况下，`parse()` 留给我们一个错误，让 `unwrap()` 产生 `panic`（原文：`parse()` leaves us with an error for `unwrap()` to `panic` on）。另外，`panic` 会退出我们的程序，并提供一个不愉快的错误消息。

为了改善错误消息的质量，我们应该更具体地了解返回类型并考虑显式地处理错误。

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
