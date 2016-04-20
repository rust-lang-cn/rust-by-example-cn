数字字面量可以加上类型标记作为后缀来标注类型，以 `usize` 作后缀的 `usize` 类型和以 `isize`
作后缀的 `isize` 的类型除外。

未加上后缀的数字字面量的类型视使用它们的情况而定。如果没有限定，编译器会将整型定为 `i32` 类型，将
浮点数定为 `f64` 类型。

{literals.play}

前面代码中用了一些尚未解释过的概念，这里列出一些简短的说明：

* `fun(&foo)`  是**通过引用**传参给一个函数，而不是通过值来传参（`fun(foo)`）。更多内容参见
  [借用][borrow]（borrowing）。
* `std::mem::size_of_val` 是一个函数，不过是通过**完整的路径**调用的。代码可以划分到称为
  **模块**（module）的逻辑单元中。在这个例子中，`size_of_val` 函数是定义在 `mem` 模块的，
  `mem` 模块是定义在 `std` 包（crate）中。更多内容参考[模块][mod] 和 [crate][crate]。

[borrow]: ../scope/borrow.html
[mod]: ../mod.html
[crate]: ../crates.html
