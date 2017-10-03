# 类型匿名

闭包从封闭的作用中域捕获变量简单明了。这样会有某些后果吗？当然会。观察一下使用闭包作为函数参量的方式是要求为[泛型][generics]的，它们定义的方式决定了这是必要的（原文：Observe how using a closure as a function parameter requires [generics], which is necessary because of how they are defined）：

```rust
// `F` 必须是泛型。
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
```

当定义一个闭包时，编译器将隐式地创建一个新的匿名结构体来存储内部的捕获变量，
同时针对此未知类型通过其中的一种 `trait`：`Fn`，`FnMut`，或 `FnOnce` 来实现功能
（原文：implementing the functionality via one of the `traits`: `Fn`, `FnMut`, 
or `FnOnce` for this unknown type）。这个类型被赋给所存储的变量直到调用（原文：
This type is assigned to the variable which is stored until calling）。

由于这个新类型是未知的类型，所以在函数中的任何用法都要求是泛型。然而，
未限定的类型参量 `<T>` 仍然是不明确的并且是不允许的。因此通过其中一种 
`trait`：`Fn`，`FnMut`，或 `RnOnce`（已经实现）就足以指明它的类型。

```rust,editable
// `F` 必须针对一个没有输入参数和返回值的闭包实现 `Fn`
// —— 确切地讲是 `print` 要求的类型。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获的 `x` 成为一个匿名类型并为它实现 `Fn`。
    // 将它存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}
```

### 参见：

[透彻分析][thorough_analysis], [`Fn`][fn], [`FnMut`][fn_mut],
和 [`FnOnce`][fn_once]

[generics]: ../../generics.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
[thorough_analysis]: http://huonw.github.io/blog/2015/05/finding-closure-in-rust/
