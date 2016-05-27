因为变量要负责释放它们拥有的资源，所以**资源只能拥有一个所有者**。这也防止了资源的重复释放。注意并非所有变量都拥有资源（例如 [references]）。

在进行赋值（`let x = y`）或通过值来传递函数参数的时候，资源的**所有权**（*ownership*)会发生转移（transfer）。按照 Rust 的说法，这种方式被称为**移动**（*move*）。

After moving resources, the previous owner can no longer be used. This avoids
creating dangling pointers.
在移动资源之后，原来的所有者可不可以再使用，这避免了悬垂指针的产生。

{move.play}

[references]: ../flow_control/match/destructuring/destructure_pointers.html
