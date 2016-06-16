在前面，我们用 `Option` 类型来标注不存在是一种可能的情形（原文：absence is a possibility）。此类不存在时常以错误的形式出现，比如当 `None` 没有解包（unwrap）。当多种失败情况可能发生时，可以使用更通用的 `Result` 类型来替代 `Option`。`Result<T, E>` 有这形变量：

* `Ok<T>`: An element `T` was found
* `Err<E>`: 
* `Ok<T>`：找到 `T` 元素
* `Err<E>`：发现错误，使用元素 `E` 表示（An error was found with element `E`）

和 `Option` 类似，`Result` 也包含产生 `T` 元素的 `unwrap()` 方法或调用 `panic!()`。截至目前，这似乎应该和 `Option` 很像（So far, this should seem similar to `Option`）：

{result.play}

显然，对于 `Err` 的 panic 留下没有帮助的错误消息。对我们来说幸运的是，即将讲到的组合可以帮助我们处理错误。


### 参见：

[`Result`][result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
