我们可以看到格式化就是通过*格式字符串*得到特定格式：

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

根据使用的*描述类型*，同样的变量（`foo`）能够格式化成不同的形式：`X`， `o` 和 *未指定形式*。

这个格式化的功能是通过 trait 实现，并且是一种 trait 来实现各种描述类型。最常见的格式化 trait
就是 `Display`，它可以处理多种情形，但没有指明描述类型，比如 `{}`。

{show.play}

在 [`fmt::fmt`][fmt] 文档中可以查看[全部系列的格式 traits][fmt_traits]和它们的描述类型。

### 动手试一试
在上面的 `Color` 结构体加上一个 `fmt::Display` 的实现，得到如下的输出结果：

```
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```
如果感到疑惑，可看下面两条提示：
 * 你[可能需要多次列出各种颜色][argument_types]，
 * 你可以使用 `:02` [补零使位数为2位]。

### 参考
[`std::fmt`][fmt]

[argument_types]: http://doc.rust-lang.org/std/fmt/#argument-types
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: http://doc.rust-lang.org/std/fmt/
[fmt_traits]: http://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: http://doc.rust-lang.org/std/fmt/#width
