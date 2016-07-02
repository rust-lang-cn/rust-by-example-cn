前面我们一直使用字符串（`String`）作为错误消息。实际上，字符串作为错误类型是存在一些局限的。下面是友好的错误类型标准。字符串（`String`）很好地实现了前两点，但无法做到后两点：

* 使用相同类型来表达不同的错误
* 给用户提供友好的错误信息
* 支持良好的错误比较。考虑比较以下两种类型：
    - `Err("Please use a vector with at least one element".to_owned())`
    - `Err(EmptyVec)`
* 能够保存错误的信息（原文：Can hold information about the error.）。比较：
    - `Err("+ cannot be used here".to_owned())`
    - `Err(BadChar(c, position))`

这使得 `String` 错误既难以达到要求，也难以创建。实际上，一个优雅的错误消息应该和类型组织的形式是无关的。这只不过是 `Display` 对类型实现的结果。对优雅的错误消息来说，实在不应该使用 `String` 格式化方式，因为这会严重污染代码的逻辑（原文：It should not be necessary to pollute logic heavy code with `String` formatting simply for nice error messages.）。

{rethink.play}

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: ../error/option_with_result/result_string_errors.html
