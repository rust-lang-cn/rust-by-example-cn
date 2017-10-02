有时候想要捕捉到程序某部分的失败信息，而不调用 `panic!`；这可使用 `Option` 枚举来完成。

`Option<T>` 枚举有两个变量：

* `None`，表明失败或缺少值
* `Some(value)`，元组结构体，使用 `T` 类型装包了一个值 `value`

{option.play}
