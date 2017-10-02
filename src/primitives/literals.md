整型 `1`，浮点型 `1.2`，字符 `'a'`，字符串 `"abc"`，布尔型 `true` 和 单元类型 `()` 可以
用数字、文字或符号的字面意义表示出来。

数字可以加上前缀 `0x`、`0o`、`0b` 分别表示十六进制数、八进制数、二进制数。

为了改善数字的可读性，可以在数字类型之间加上下划线(_)，比如： `1_000` 等同于 `1000`，
`0.000_001` 等同于 `0.000001`。

我们需要告诉计算机使用到的数据类型。如前面学过的，我们使用 `u32`后缀来表明该数据是一个 32 位
存储的无符号整数，`i32` 后缀表明数据是一个 32 位存储的带符号整数。

[Rust][rust op-prec] 提供了一系列的运算符号，它们的优先级和[类C语言][op-prec]的类似。
（译注：类C语言包括 C/C++，Java，PHP 等语言。）

```rust,editable
fn main() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要

    // 短路的布尔类型的逻辑运算
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算符
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
}
```

[rust op-prec]: http://doc.rust-lang.org/reference.html#operator-precedence
[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
