`fmt::Debug` 看起来并不简洁，然而它对自定义输出外观通常是有好处的。而[`fmt::Display`]
[fmt]是通过手动的方式来实现，采用了`{}`来打印标记。实现方式看起来像这样：

```rust
// (使用 `use`)导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，使用 `fmt::Display` 来实现。这只是简单地给元组结构体`Structure` 包含
// 一个 `i32` 元素。
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动实现 `fmt::Display` trait 来支持相应类型。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 带有正确的标记
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 严格将第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此结果表明操作成功
        // 或失败。注意这里的 `write!` 用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}
```

`fmt::display` 的使用形式可能比 `fmt::Debug` 简洁，但它对于标准库的处理有一个问题。模棱
两可的类型该如何显示呢？举个例子，假设标准库对所有的 `Vec<T>` 都实现了单一样式，那么它应该
是那种样式？随意一种或者包含两种？

* `Vec<path>`: `/:/etc:/home/username:/bin` (split on `:`)
* `Vec<number>`: `1,2,3` (split on `,`)

答案是否定的，因为没有合适的样式适用于所有类型，标准库也没规定一种情况。对于 `Vec<T>` 或其
他任意泛型容器(container)，`fmt::Display` 都没有实现形式。在这种含有泛型的情况下要用到
 `fmt::Debug`。

而对于非泛型的容器类型的输出， `fmt::Display` 都能够实现。

```rust,editable
use std::fmt; // 导入 `fmt`

// 带有两个数字的结构体。`Debug` 将被派生，可以看到输出结果和 `Display` 的差异。
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现 `MinMax` 的 `Display`。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 方式来表示各个数据。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 Point2D 进行实现
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义方式实现，仅让 `x` 和 `y` 标识出来。
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

`fmt::Display` 都实现了，而 `fmt::Binary` 都没有，因此 `fmt::Binary` 不能使用。
`std::fmt` 有很多这样的 [`traits`][traits]，使用这些 trait 都要有各自的实现。这些内容将
在后面的 [`std::fmt`][fmt] 章节中详细介绍。

### 动手试一试

对上面程序的运行结果检验完毕后，在上述示例程序中，仿照 `Point2` 结构体增加一个复数结构体。
使用一样的方式打印，输出结果要求这个样子：
```
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

### 参见：

[`derive`][derive], [`std::fmt`][fmt], [macros], [`struct`][structs],
[`trait`][traits], 和 [use][use]

[derive]: ../../trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: ../../macros.html
[structs]: ../../custom_types/structs.html
[traits]: ../../trait.html
[use]: ../../mod/use.html
