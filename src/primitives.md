Rust 提供了多种原生类型，包括：

* 有符号整型（signed integers）：`i8`， `i16`， `i32`， `i64` 和 `isize`（指针 size）
* 无符号整型（unsigned integers）： `u8`， `u16`， `u64` 和 `usize`（指针 size）
* 浮点类型（floating point）： `f32`， `f64`
* `char`（字符）：单独的 Unicode 字符，如 `'a'`，`'α'` 和 `'∞'`（大小都是4个字节）
* `bool`（布尔型）：只能是 `true` 或 `false`
* 单元类型(unit type，空元组)： 只有 `()` 这个唯一值
* 数组：如 `[1, 2, 3]`
* 元组： 如 `(1, true)`

变量都能够显式地给出**类型声明**。数字可以通过**加后缀**或**默认方式**来额外地声明。整型默认为
`i32` 类型，浮点型默认为 `f64` 类型（译注：此说法不明确，[Rust语言参考][reference]指出：
未声明类型数值的具体类型由实际使用情况推断，比如一个未声明类型整数和 `i64` 的整数相加，则该
整数会自动推断为 `i64` 类型，仅当使用环境无法推断时，整型数值时才断定为 `i32`，浮点数值才
断定为 `f64`）。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // 变量可以声明类型。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规声明
    let an_integer   = 5i32; // 后缀声明

    // 否则自动推断类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // 可变类型 `i32`。

    // 报错！变量的类型不可改变。
    mutable = true;
}
```

### 参见：

[`std` 库][std]

[std]: http://doc.rust-lang.org/std/
[reference]: http://doc.rust-lang.org/reference.html#number-literals
