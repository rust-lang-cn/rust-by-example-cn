# 类型匿名

闭包从周围的作用域中捕获变量是简单明了的。这样会有某些后果吗？确实有。观察一下使用闭包作为函数参数，这要求闭包是[泛型][generics]的，闭包定义的方式决定了这是必要的。

```rust
// `F` 必须是泛型的。
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
```

当闭包被定义，编译器会隐式地创建一个匿名类型的结构体，用以储存闭包捕获的变量，同时为这个未知类型的结构体实现函数功能，通过 `Fn`、`FnMut` 或 `FnOnce` 三种 `trait` 中的一种。

若使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是泛型的。然而，使用未限定类型的参数 `<T>` 过于不明确，并且是不允许的。事实上，指明为该结构体实现的是 `Fn`、`FnMut`、或 `FnOnce` 中的哪种 `trait`，对于约束该结构体的类型而言就已经足够了。

```rust,editable
// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的
// 要求恰好一样。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}
```

### 参见：

[详尽分析][thorough_analysis], [`Fn`][fn], [`FnMut`][fn_mut],
和 [`FnOnce`][fn_once]

[generics]: ../../generics.md
[fn]: https://rustwiki.org/zh-CN/std/ops/trait.Fn.html
[fn_mut]: https://rustwiki.org/zh-CN/std/ops/trait.FnMut.html
[fn_once]: https://rustwiki.org/zh-CN/std/ops/trait.FnOnce.html
[thorough_analysis]: https://huonw.github.io/blog/2015/05/finding-closure-in-rust/
