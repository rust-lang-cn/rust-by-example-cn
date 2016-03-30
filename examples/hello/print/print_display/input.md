`fmt::Debug` 看起来并不简洁，然而它对自定义输出外观通常是有好处的。而[`fmt::Display`]
[fmt]是通过手动的方式来实现，采用了`{}`来打印标记。实现方式看起来像这样：

```rust
// (使用 `use`)导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，使用 `fmt::Display` 来实现。这只是简单地给元组结构体`Structure` 包含
// 一个 `i32` 元素。
struct Structure(i32);

// In order to use the `{}` marker, the trait `fmt::Display` must be implemented
// 为了用到 `{}` 标记，`fmt::Display` trait 必须手动实现来支持类型。
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    // 这个 trait 要求
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display` may be cleaner than `fmt::Debug` but this presents
a problem for the `std` library. How should ambiguous types be displayed?
For example, if the `std` library implemented a single style for all
`Vec<T>`, what style should it be? Either of these two?

* `Vec<path>`: `/:/etc:/home/username:/bin` (split on `:`)
* `Vec<number>`: `1,2,3` (split on `,`)

No, because there is no ideal style for all types and the `std` library
doesn't presume to dictate one. `fmt::Display` is not implemented for `Vec<T>`
or for any other generic containers. `fmt::Debug` must then be used for these
generic cases.

This is not a problem though because for any new *container* type which is
*not* generic,`fmt::Display` can be implemented.

{display.play}

So, `fmt::Display` has been implemented but `fmt::Binary` has not, and
therefore cannot be used. `std::fmt` has many such [`traits`][traits] and
each requires its own implementation. This is detailed further in
[`std::fmt`][fmt].

### 动手试一试

After checking the output of the above example, use the `Point2` struct as
guide to add a Complex struct to the example. When printed in the same
way, the output should be:
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
