# `From` 和 `Into`

 [`From`] 和 [`Into`] 两个 trait 是内部相关联的，实际上这是它们实现的一部分。如果我们能够从类型 B 得到类型 A，那么很容易相信我们也能把类型 B 转换为类型 A。

## `From`

[`From`] trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中有无数 `From` 的实现，规定原生类型及其他常见类型的转换功能。

比如，可以很容易地把 `str` 转换成 `String`：

```rust
let my_str = "hello";
let my_string = String::from(my_str);
```

也可以为我们自己的类型定义转换机制：

```rust,editable
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

## `Into`

[`Into`] trait 就是把 `From` trait 倒过来而已。也就是说，如果你为你的类型实现了 `From`，那么同时你也就免费获得了 `Into`。

使用 `Into` trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。不过考虑到我们免费获得了 `Into`，这点代价不值一提。

```rust,editable
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```

[`From`]: https://rustwiki.org/zh-CN/std/convert/trait.From.html
[`Into`]: https://rustwiki.org/zh-CN/std/convert/trait.Into.html
