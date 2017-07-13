打印操作由[`std::fmt`][fmt]里面所定义的一系列[`宏`][macros]来处理，其中包括：

* `format!`：将格式化文本写到[`字符串`][string](String)。(译注: `字符串`是返回值不是参数。)
* `print!`：与 `format!`类似，但将文本输出到控制台。
* `println!`: 与 `print!`类似，但输出结果追加一个换行符。

所有的解析文本都以相同的方式进行。另外一点是格式化的正确性在编译时检查。

{print.play}

[`std::fmt`][fmt]包含多种[`traits`][traits]（traits翻译成中文有“特征，特性”等意思）来控制文字显示。这里面有两个重要的基本格式类型如下：

* `fmt::Debug`：使用 `{:?}` 作标记。格式化文本以便调试。
* `fmt::Display`：使用 `{}` 作标记。以优雅和友好的方式来格式文本。

在本书中我们使用`fmt::Display`，因为标准库提供了这些类型的实现。若要打印自定义类型的文本，需要更多的步骤。

### 动手试一试

 * 改正上面代码中的两个错误（见 改正），使得运行不会报错。

 * 添加一个 `println!` 宏来打印：`Pi is roughly 3.142`（Pi约等于3.142），通过控制有效数字得到显示的结果。为了达到练习目的，使用 `let pi = 3.141592` 作为 Pi 的近似值（提示：设置小数位的显示格式可以参考文档[`std::fmt`][fmt]）。

### 参见：

[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs]
和 [`traits`][traits]

[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: ../macros.html
[string]: ../std/str.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
