`trait` 是对未知类型定义的方法集：`Self`。它们可以访问同一个 trait 中定义的方法。

对任何数据类型实现 trait 都是可行的。在下面例子中，我们定义了包含一系列方法的 `Animal`。然后针对 `Sleep` 数据类型实现 `Animal` `trait`，允许使用来自带有 `Sheep` 的 `Animal` 的方法（原文：allowing the use of methods from `Animal` with a `Sheep`）。

{trait.play}
