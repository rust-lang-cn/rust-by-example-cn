虽然 Rust 在捕获临时变量的方式大多选择不带标注，但在编写函数时，这种不确定性是不允许的。当以闭包作为输入参数时，闭包的完整类型必须使用以下的其中一种 `trait` 来标注。它们的受限程度依次递减，依次是（原文：In order of decreasing restriction, they are）：

* `Fn`：闭包需要通过引用（`&T`）捕获
* `FnMut`：闭包需要通过可变引用（`&mut T`）捕获
* `FnOnce`：闭包需要通过值（`T`）捕获

在值传值（variable-by-variable）的基础上，编译器将以限制最少的方式来捕获变量。

例如考虑一个标注为 `FnOnce` 的参量。这意味着闭包可能通过 `&T`，`&mut T` 或 `T` 来捕获，但是编译器将根据所捕获变量在闭包的使用情况做出最终选择。

这是因为若移动语义（move）可能的话，则任意借用类型也应该是可行的。注意反过来就不再成立：如果参量是 `Fn`，那么通过 `&mut T` 或 `T` 捕获的情况就不允许了。

在下面的例子中，试着换换 `Fn`、`FnMut` 和 `FnOnce` 的使用，看看会发生什么：

{input_parameters.play}

### 参见：

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], 和 [`FnOnce`][fnonce]

[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
