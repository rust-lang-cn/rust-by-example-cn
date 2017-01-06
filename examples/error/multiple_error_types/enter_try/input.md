有时我们只是想要 `unwrap` 的简单，而又不会产生 `panic`。截至目前，`unwrap` 迫使我们嵌套了一层又一层，而我们想要的只不过是将相应的变量取出来。正因为这样，我们引入了 `try!`。

在发现错误（`Err`）时，有两个有效的操作：

1. `panic!`，但我们已经尽可能回避这种情况
2. `return`，因为 `Err` 意味着它不能被处理

`try!` **几乎完全**[^1]等同于一个这样的 `unwrap`——对待错误（`Err`）采用返回的方式而不是 `panic。我们来看看如何简化之前使用组合器的示例：

{try.play}

注意到目前为止，我们一直使用 `String` 作为错误类型。但它们作为错误类型是有一定限制的。在下一节中，我们将学习如何通过自定义类型来创建更具结构化和更多信息量的错误。

[^1]: 参考 [re-enter try!][re_enter_try]，查看更多详细内容。

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[re_enter_try]: ../../error/reenter_try.html
