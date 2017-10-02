[`Drop`][Drop] trait 只有一个方法：`drop`，当一个对象离开作用域时会自动调用该方法。`Drop` trait 的主要作用是释放实现者实例拥有的资源。

`Box`，`Vec`，`String`，`File`，以及 `Process` 是一些实现了 `Drop` trait 来释放资源的类型的例子。`Drop` trait 也可以针对任意自定义数据类型手动实现。

下面示例给 `drop` 函数增加了打印到控制台的功能，用于宣布它在什么时候被调用。（原文：The following example adds a print to console to the `drop` function to announce
when it is called.）

{drop.play}

[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
