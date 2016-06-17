在前面例子中关于 `unwrap()` 的 panic 给我们留下的是没用的消息。为了避免这样，我们需要更具体的返回类型。在那个例子中，记得该常规元素为 `i32` 类型。为了确定 `Err` 的类型，我们可以借助 `parse()`。我们使用 [`FromStr trait`][from_str] 来针对 [`i32`][i32] 实现了 `parse()`。结果是，`Err` 类型被指定为 [`ParseIntError`][parse_int_error]。

在下面例子中要注意，使用简单的 `match` 语句会导致更加繁琐的代码。事实证明，用到 `Option` 的 `map` 方法也对 `Result` 进行了实现。

{result.play}

和 `Option` 很像，`Result` 实现了除 `map` 外还有更多的组合算子（combinator），比如 `and_then` 和 `unwrap_or`。甚至包括那些特定的句柄错误（handle error），如 `map_err`。

### 参见：

[`i32`][i32], [`FromStr`][from_str], 和 [`ParseIntErr`][parse_int_error]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse_int_error]: http://doc.rust-lang.org/std/num/struct.ParseIntError.html
[from_str]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: http://doc.rust-lang.org/std/primitive.i32.html
