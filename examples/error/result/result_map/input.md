前面关于 panic 例子，提供给我们的是一个无用的错误消息。为了避免这样，我们需要更具体地指定返回类型。在那个例子中，该常规元素为 `i32` 类型。

为了确定 `Err` 的类型，我们可以借助 `parse()`，它使用 [`FromStr`][from_str] trait 来针对 [`i32`][i32] 实现。结果是，`Err` 类型被指定为 [`ParseIntError`][parse_int_error]。

在下面例子中要注意，使用简单的 `match` 语句会导致更加繁琐的代码。事实证明，用到 `Option` 的 `map` 方法也对 `Result` 进行了实现。

幸运的是，`Option` 的 `map` 方法是对 `Result` 进行了实现的许多组合算子之一。 [`enum.Result`][result] 包含一个完整的列表。

{result_map.play}

[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[from_str]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: http://doc.rust-lang.org/std/primitive.i32.html
[parse_int_error]: http://doc.rust-lang.org/std/num/struct.ParseIntError.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
