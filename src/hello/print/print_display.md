# 显示（display）

`fmt::Debug` 通常看起来不太简洁，因此自定义输出的外观经常是更可取的。这需要通过
手动实现 [`fmt::Display`][fmt] 来做到。`fmt::Display` 采用 `{}` 标记。实现方式看
起来像这样：

```rust
// （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
// `Structure`，包含一个 `i32` 元素。
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}
```

`fmt::display` 的效果可能比 `fmt::Debug` 简洁，但对于 `std` 库来说，这就有一个问
题。模棱两可的类型该如何显示呢？举个例子，假设标准库对所有的 `Vec<T>` 都实现了同
一种输出样式，那么它应该是哪种样式？下面两种中的一种吗？

* `Vec<path>`：`/:/etc:/home/username:/bin`（使用 `:` 分割）
* `Vec<number>`：`1,2,3`（使用 `,` 分割）

我们没有这样做，因为没有一种合适的样式适用于所有类型，标准库也并不擅自规定一种样
式。对于 `Vec<T>` 或其他任意泛型容器（generic container），`fmt::Display` 都没有
实现。因此在这些泛型的情况下要用 `fmt::Debug`。

这并不是一个问题，因为对于任何**非**泛型的**容器**类型， `fmt::Display` 都能够实
现。

```rust,editable
use std::fmt; // 导入 `fmt`

// 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现 `MinMax` 的 `Display`。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 来表示各个数据。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有具名字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

`fmt::Display` 被实现了，而 `fmt::Binary` 没有，因此 `fmt::Binary` 不能使用。
`std::fmt` 有很多这样的 [`trait`][traits]，它们都要求有各自的实现。这些内容将在
后面的 [`std::fmt`][fmt] 章节中详细介绍。

### 动手试一试

检验上面例子的输出，然后在示例程序中，仿照 `Point2D` 结构体增加一个复数结构体。
使用一样的方式打印，输出结果要求是这个样子：

```txt
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

### 参见：

[`derive`][derive], [`std::fmt`][fmt], [macros], [`struct`][structs],
[`trait`][traits], 和 [use][use]

[derive]: ./trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: ./macros.html
[structs]: ./custom_types/structs.html
[traits]: ./trait.html
[use]: ./mod/use.html
