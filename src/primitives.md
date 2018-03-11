# 原生类型

Rust 提供了多种原生类型(primitive)，包括：

## 标量类型（scalar type）

* 有符号整型（signed integers）：`i8`、`i16`、`i32`、`i64` 和 `isize`（指针宽度）
* 无符号整型（unsigned integers）： `u8`、`u16`、`u32`、`u64` 和 `usize`（指针宽
度）
* 浮点类型（floating point）： `f32`、`f64`
* `char`（字符）：单个 Unicode 字符，如 `'a'`，`'α'` 和 `'∞'`（每个都是 4 字节）
* `bool`（布尔型）：只能是 `true` 或 `false`
* 单元类型（unit type）：`()`。其唯一可能的值就是 `()` 这个空元组

尽管单元类型的值是个元组，它却并不被认为是复合类型，因为并不包含多个值。

## 复合类型（compound type）

* 数组（array）：如 `[1, 2, 3]`
* 元组（tuple）：如 `(1, true)`

变量都能够显式地给出**类型说明**（type annotation）。数字还可以通过**后缀**
（suffix）或**默认方式**来声明类型。整型默认为 `i32` 类型，浮点型默认为 `f64`
 类型。注意 Rust 还可以根据上下文来推断（infer）类型（译注：比如一个未声明类型整
数和 `i64` 的整数相加，则该整数会自动推断为 `i64` 类型。仅当根据环境无法推断时
，才按默认方式取整型数值为 `i32`，浮点数值为 `f64`）。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;
    
    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // 报错！变量的类型并不能改变。
    mutable = true;
    
    // 但可以用掩蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}
```

### 参见：

[`std` 库][std]、[`mut`][mut]、[类型推断][inference] 和[变量掩蔽](shadowing)。

[std]: http://doc.rust-lang.org/std/
[mut]: https://rustbyexample.com/variable_bindings/mut.html
[inference]: https://rustbyexample.com/types/inference.html
[shadowing]: https://rustbyexample.com/variable_bindings/scope.html

