在接下来几个小节里，我们将看到如何把返回 `Option` 和 `Result` 的操作符组合成一个最有意义的单一操作符。（原文：In the following sections, we will see how to combine separate operations returning `Option` and `Result` into a single operation that returns whichever one makes the most sense.）

前面出现的例子确实很方便；一个 `Result` 与另一个 `Result` 相互作用，一个 `Option` 与另一个 `Option` 相互作用。不幸地是，情况并非一直都如此简单。一个 `Option` 可能必须和 `Result` 相互作用，一个 `Result<T, Error1>` 可能必须与一个 `Result<T, Error2>` 相互作用。

为从头说起，下面的例子使用了 `Vec::first`，以及使用 `unwrap` 生成错误消息的 `parse::<i32>` 。`Vec::first` 返回一个 `Option`，同时 `parse::<i32>` 返回一个 `Result<i32, ParseIntError>`。

注意到这段代码“确实正常工作了”，但这仅仅是为了展示**不恰当**的错误此理：

{option_result.play}
