前面我们一直使用字符串（`String`）作为错误消息。实际上，字符串作为错误类型是存在一些局限的。下面是友好的错误类型标准。字符串（`String`）很好地实现了前两点，但无法做到后两点：
Rust 允许自定义错误类型。一般而言，一个“良好”的错误类型：

* 使用相同类型来表达不同的错误
* 给用户提供友好的错误信息
* 方便和其他类型比较
    - Good: `Err(EmptyVec)`
	- Bad: `Err("Please use a vector with at least one element".to_owned())`
* 能够保存错误的信息（原文：Can hold information about the error.）：
    - Good: `Err(BadChar(c, position))`
	- Bad: `Err("+ cannot be used here".to_owned())`

可以看到字符串（`String`）（前面我一们一值在用）可以地满足前两点标准，但后两条无法满足。这使得 `String` 错误既难以创建，也难以达到要求。仅仅为了优雅地显示，实在不应该使用 `String` 格式化方式污染大量的逻辑代码（原文：It should not be necessary to pollute logic heavy code with `String` formatting simply to display nicely.）。

{define_error_type.play}

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: ../error/option_with_result/result_string_errors.html
