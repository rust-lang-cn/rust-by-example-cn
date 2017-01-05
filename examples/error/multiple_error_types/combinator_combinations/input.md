要是有多个不同的 `Result` 需要相互作用怎么办？处理起来还会方便吗？事实表明并非如此。

{result_try.play}

思路是这样，这个方法尝试处理数据，而不用移除包裹在它上面的 `Ok`（原文：What is happening is this approach tries to work with the data without ever removing the `Ok` wrapper on it.）。有时这是一个好办法，但对于这个情况却很糟糕。要是不用包含 `panic` 来解包（`unwrap`）会怎样呢？这正是我们接下来要了解的内容。

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
