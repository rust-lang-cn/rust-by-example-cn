# 格式化输出

打印操作由 [`std::fmt`][fmt] 里面所定义的一系列[宏][macros]来处理，包括：

- `format!`：将格式化文本写到[字符串][string]。
- `print!`：与 `format!` 类似，但将文本输出到控制台（io::stdout）。
- `println!`: 与 `print!` 类似，但输出结果追加一个换行符。
- `eprint!`：与 `print!` 类似，但将文本输出到标准错误（io::stderr）。
- `eprintln!`：与 `eprint!` 类似，但输出结果追加一个换行符。

这些宏都以相同的做法解析文本。有个额外优点是格式化的正确性会在编译时检查。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 可以使用位置参数。
    // 在 `{}` 中指定一个整数确定将替换哪个附加参数。
    // 参数从紧接在格式字符串之后的 0 开始。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 通过在 `:` 后面指定格式字符，可以调用不同的格式。
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // 可以用指定的宽度对文本进行右对齐。
    // 这将输出“    1”。(四个空格和一个“1”，总宽度为5)
    println!("{number:>5}", number=1);

    // 你可以在数字左边补 0，
    println!("{number:0>5}", number=1); // 00001
    // 在数字右边补零，下面语句输出 "000001"。
    println!("{number:0<5}", number=1); // 10000

    // 可以通过添加 `$` 在格式说明符中使用命名参数。
    println!("{number:0>width$}", number=1, width=5);

    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ 补上漏掉的参数："James"

    // 只有实现了fmt::Display的类型才能使用'{}'进行格式化。
    // 默认情况下，用户定义的类型不实现fmt::Display。
    #[allow(dead_code)]  // 禁用对未使用模块发出警告的' dead_code '
    struct Structure(i32);

    // 这将不会编译，因为`Structure`没有实现
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ 注释掉此行。

    // 对于 Rust 1.58 及更高版本，您可以直接从周围变量捕获参数。
    // 就像上面一样，这将输出“    1”，4 个空格和 1 个“1”。
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
```

[`std::fmt`][fmt] 包含多种 [`traits`][traits]（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：

- `fmt::Debug`：使用 `{:?}` 标记。格式化文本以供调试使用。
- `fmt::Display`：使用 `{}` 标记。以更优雅和友好的风格来格式化文本。

上例使用了 `fmt::Display`，因为标准库提供了那些类型的实现。若要打印自定义类型的文本，需要更多的步骤。

实现 `fmt::Display` 特性会自动实现 [`ToString`] 特征，它允许我们将类型 [convert] 为 [`String`][string]。

在*第 43 行*中，`#[allow(dead_code)]` 是一个 [attribute]，只适用于它后面的模块。

### 动手试一试

- 改正上面代码中的错误（见 FIXME），使它可以没有错误地运行。
- 尝试取消注释，输出结构体内容（见 TODO）。
- 再用一个 `println!` 宏，通过控制显示的小数位数来打印：`Pi is roughly 3.142`（Pi 约等于 3.142）。为了达到练习目的，使用 `let pi = 3.141592` 作为 Pi 的近似值（提示：设置小数位的显示格式可以参考文档 [`std::fmt`][fmt]）。

### 参见：

[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs], [`traits`][traits] 和 [`dead_code`][dead_code]

[fmt]: https://rustwiki.org/zh-CN/std/fmt/
[macros]: ../macros.md
[string]: ../std/str.md
[structs]: ../custom_types/structs.md
[traits]: ../trait.md
[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[convert]: ../conversion/string.md
[attribute]: ../attribute.md
[dead_code]: ../attribute/unused.md
