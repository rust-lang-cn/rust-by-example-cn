# 常量

Rust 有两种常量，可以在任意作用域声明，包括全局作用域。它们都需要显式的类型声明：

* `const`：不可改变的值（通常使用这种）。
* `static`：具有 [`'static`][static] 生命周期的，可以是可变的变量（译注：须使用
  `static mut` 关键字）。

有个特例就是 `"string"` 字面量。它可以不经改动就被赋给一个 `static` 变量，因为它的类型标记：`&'static str` 就包含了所要求的生命周期 `'static`。其他的引用类型都必须特地声明，使之拥有`'static` 生命周期。这两种引用类型的差异似乎也无关紧要，因为无论如何，`static` 变量都得显式地声明。

```rust,editable,ignore,mdbook-runnable
// 全局变量是在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}
```

### 参见：

[`const`/`static` RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md),
[`'static` 生命周期][static]

[static]: ../scope/lifetime/static_lifetime.md
