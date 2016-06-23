从前面例子可以看到，一个解决这个带有 `unwrap` 的问题的方法就是将 `unwrap` 移除。为达成这点，我们必须由隐式的错误处理转examples/error/option_with_result/input.md
	modified:   examples/error/option_with_result/option_result.rs成显式的错误处理。既然运行中的类型只有 `Option` 和 `Result`，我们可以考虑将两者转换成带有相同 `Err` 类型的 `Result`。作为问题解决的第一次尝试，让我们试试使用字符串（`String`）来应对错误（原文：For our first attempt at this solution, 
let's try using a `String` for our error）：

{result_string.play}

情况不算特别糟糕，但相比原始代码很难说足够优雅（当然也可以改得更优雅，但这不是我们要关心的重点）。悲剧的是，这种方法通过枯燥地增加更多的 `Result` 来达到目的，这种情况在下个例子中也会看到（原文：Unfortunately, this approach scales poorly with increasing numbers of `Result`s, as will be seen in the next example.）。
