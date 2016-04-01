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
    // This trait requires `fmt` with this exact signature.
    // 这个 trait 要求 `tmt` 带有正确的标记
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

{display.play}

`fmt::Display` 都实现了，而 `fmt::Binary` 都没有，因此 `fmt::Binary` 不能使用。
`std::fmt` 有很多这样的 [`traits`][traits]，使用这些 trait 都要有各自的实现。这些内容将
在后面的 [`std::fmt`][fmt] 章节中详细介绍。

### 动手试一试

对上面程序的运行结果检验完毕后，在上述示例程序中，仿照 `Point2` 结构体增加一个复杂的结构体。
使用一样的方式打印，输出结果要求这个样子：
```
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

### 参考

[`derive`][derive], [`std::fmt`][fmt], [macros], [`struct`][structs],
[`trait`][traits], 和 [use][use]

[derive]: /trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: /macros.html
[structs]: /custom_types/structs.html
[traits]: /trait.html
[use]: /mod/use.html
