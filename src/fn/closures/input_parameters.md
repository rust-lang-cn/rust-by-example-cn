# 作为输入参数

虽然 Rust 无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊写法是不允许的。当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下 `trait` 中的一种来指定的。其受限制程度按以下顺序递减：

- `Fn`：表示捕获方式为通过引用（`&T`）的闭包
- `FnMut`：表示捕获方式为通过可变引用（`&mut T`）的闭包
- `FnOnce`：表示捕获方式为通过值（`T`）的闭包

> 译注：顺序之所以是这样，是因为 `&T` 只是获取了不可变的引用，`&mut T` 则可以改变变量，`T` 则是拿到了变量的所有权而非借用。

对闭包所要捕获的每个变量，编译器都将以限制最少的方式来捕获。

> 译注：这句可能说得不对，事实上是在满足使用需求的前提下尽量以限制最多的方式捕获。

例如用一个类型说明为 `FnOnce` 的闭包作为参数。这说明闭包可能采取 `&T`，`&mut T` 或 `T` 中的一种捕获方式，但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。

这是因为如果能以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量。注意反过来就不再成立：如果参数的类型说明是 `Fn`，那么不允许该闭包通过 `&mut T` 或 `T` 捕获变量。

在下面的例子中，试着分别用一用 `Fn`、`FnMut` 和 `FnOnce`，看看会发生什么：

```rust,editable
// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F) where
    // 闭包没有输入值和返回值。
    F: FnOnce() {
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32 where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```

### 参见：

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], 和 [`FnOnce`][fnonce]

[drop]: https://rustwiki.org/zh-CN/std/mem/fn.drop.html
[fn]: https://rustwiki.org/zh-CN/std/ops/trait.Fn.html
[fnmut]: https://rustwiki.org/zh-CN/std/ops/trait.FnMut.html
[fnonce]: https://rustwiki.org/zh-CN/std/ops/trait.FnOnce.html
