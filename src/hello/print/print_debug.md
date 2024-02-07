# 调试（Debug）

所有的类型，若想用 `std::fmt` 的格式化打印，都要求实现至少一个可打印的 `traits`。仅有一些类型提供了自动实现，比如 `std` 库中的类型。所有其他类型都**必须**手动实现。

`fmt::Debug` 这个 `trait` 使这项工作变得相当简单。所有类型都能派生（`derive`，即自动创建）`fmt::Debug` 的实现。但是 `fmt::Display` 需要手动实现。

```rust
// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
```

所有 `std` 库类型都天生可以使用 `{:?}` 来打印：

```rust,editable
// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));
    
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

所以 `fmt::Debug` 确实使这些内容可以打印，但是牺牲了一些美感。Rust 也通过 `{:#?}` 提供了 “美化打印” 的功能：

```rust,editable
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
```

你可以通过手动实现 `fmt::Display` 来控制显示效果。

### 参见：

[attributes][attributes], [`derive`][derive], [`std::fmt`][fmt] 和 [`struct`][structs]

[attributes]: https://rustwiki.org/zh-CN/reference/attributes.html
[derive]: ../../trait/derive.md
[fmt]: https://rustwiki.org/zh-CN/std/fmt/
[structs]: ../../custom_types/structs.md
