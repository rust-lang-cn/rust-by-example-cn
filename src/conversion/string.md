# `ToString` 和 `FromStr`

## `ToString`

要把任何类型转换成 `String`，只需要实现那个类型的 [`ToString`] trait。然而不要直接这么做，您应该实现[`fmt::Display`][Display] trait，它会自动提供 [`ToString`]，并且还可以用来打印类型，就像 [`print!`][print] 一节中讨论的那样。


```rust,editable
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

译注：一个实现 `ToString` 的例子

```rust,editable
use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

## 解析字符串

我们经常需要把字符串转成数字。完成这项工作的标准手段是用 [`parse`] 函数。我们得提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，`<>`）实现。

只要对目标类型实现了 [`FromStr`] trait，就可以用 `parse` 把字符串转换成目标类型。
标准库中已经给无数种类型实现了 `FromStr`。如果要转换到用户定义类型，只要手动实现
 `FromStr` 就行。

```rust,editable
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
```

[`ToString`]: https://rustwiki.org/zh-CN/std/string/trait.ToString.html
[Display]: https://rustwiki.org/zh-CN/std/fmt/trait.Display.html
[print]: ../hello/print.md
[`parse`]: https://rustwiki.org/zh-CN/std/primitive.str.html#method.parse
[`FromStr`]: https://rustwiki.org/zh-CN/std/str/trait.FromStr.html
