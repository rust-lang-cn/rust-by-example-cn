我们可以看到格式化就是通过**格式字符串**得到特定格式：

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

根据使用的**参数类型**，同样的变量（`foo`）能够格式化成不同的形式：`X`， `o` 和**未指定形式**。

这个格式化的功能是通过 trait 实现，并且是一种 trait 来实现各种参数类型。最常见的格式化 trait
就是 `Display`，它可以处理多种情形，但没有指明参数类型，比如 `{}`。

```rust,editable
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl Display for City {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入到一个缓冲区
        // 中（第一个参数f）
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 一旦添加了针对 fmt::Display 的实现，则要用 {} 对输出内容进行转换
        println!("{:?}", *color)
    }
}
```

在 [`fmt::fmt`][fmt] 文档中可以查看[全部系列的格式 traits][fmt_traits]和它们的参数类型。

### 动手试一试
在上面的 `Color` 结构体加上一个 `fmt::Display` 的实现，得到如下的输出结果：

```text
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```
如果感到疑惑，可看下面两条提示：
 * 你[可能需要多次列出各种颜色][argument_types]，
 * 你可以使用 `:02` [补零使位数为2位]。

### 参见：
[`std::fmt`][fmt]

[argument_types]: http://doc.rust-lang.org/std/fmt/#argument-types
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: http://doc.rust-lang.org/std/fmt/
[fmt_traits]: http://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: http://doc.rust-lang.org/std/fmt/#width
