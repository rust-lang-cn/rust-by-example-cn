对容器类型为泛型的 `trait` 有类型规范需要——`trait` 的成员**必须**指出全部关于它的泛型类型。

在下面例子中，`Contains` `trait` 允许使用泛型类型 `A` 或 `B`。然后这个 trait 针对 `Container` 类型实现，指定 `i32` 为 `A` 和 `B`，因而它可以用到 `fn difference()`。（本段原文：In the example below, the `Contains` `trait` allows the use of the generic types `A` and `B`. The trait is then implemented for the `Container` type, 
specifying `i32` for `A` and `B` so that it can be used with `fn difference()`.）

因为 `Contains` 是泛型，所以我们被迫显式地指出了针对 `fn difference()` 的所有泛型类型。实际上，我们只想要一种方式来表示由**输入**的 `C` 确定的 `A` 和 `B`。正如你就要看到的下一节内容，关联类型正好提供了这方面能力。

{problem.play}

### 参见：

[`struct`][structs], 和 [`trait`][traits]

[structs]: ../../custom_types/structs.html
[traits]: ../../trait.html
