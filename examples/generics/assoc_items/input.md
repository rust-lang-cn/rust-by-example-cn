“关联项”（associted items）是指一系列有关各种变量类型的 [`item`][items]（项） 的规则。它是 `trait` 泛型的扩展（extension），允许 `trait` 在内部定义新的项。

**关联类型**（*associated type*）就是这种项的其中一个。当 `trait` 在其容器类型（container type）上是泛型时，关联类型提供了更简单的使用模式。（原文：One such item is called an *associated type*, providing simpler usage patterns when the `trait` is generic over its container type.）

### 参见：

[RFC][RFC]

[items]: http://doc.rust-lang.org/reference.html#items
[RFC]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
