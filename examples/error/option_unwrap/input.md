在上个例子中，我们显示出我们能够任意引入程序失败（program failure）。当公主收到蛇这件不合适的礼物时，我们就告诉程序产生 `panic`。但是，如果公主期待一件礼物却没收到呢？这同样是一件糟糕的事情，所以我们要想办法来解决这个问题！

我们**可以**检查空字符串（`""`），就像处理蛇那样的方式。既然我们使用了 Rust，那我们就让编译器指出没有礼物的情况。

在标准库（`std`）中有个叫做 `Option<T>` （option 中文意思是“选项”）的枚举类型，用于可能发生变量不存在的情景（原文：An `enum` called `Option<T>` in the `std` library is used when absence is a possibility. ）。它表现为以下两个 “options”（选项）中的其中一个：

* `Some(T)`：找到一个属于 `T` 类型的元素
* `None`：找不到相应元素

这些选项可以通过 `match` 显式地处理，或使用 `unwrap` 隐式地处理。隐式处理会返回内部元素或 `panic`。

请注意，手动使用 [expect][expect] 方法自定义 `panic` 是可能的，而 `unwrap` 相比显式处理则留下不太有意义的输出。在下面例子中，显式处理得到更具可控性的结果，同时若需要的话，可将选项保留为 `panic`。（本段原文：Note that it's possible to manually customize `panic` with [expect][expect], but `unwrap` otherwise leaves us with a less 
meaningful output than explicit handling. In the following example, explicit handling yields a more controlled result while retaining the option to `panic` if desired. ）

{option_unwrap.play}

[expect]: http://doc.rust-lang.org/std/option/enum.Option.html#method.expect
