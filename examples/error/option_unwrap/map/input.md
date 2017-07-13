`match` 是处理 `Option` 的一个有效方法。但是你最终会发现很多用例都相当繁琐，特别是操作只有一个有效输入的情况。在这些情况下，可以使用 [组合算子][combinators]（combinator）以模块化方式来管理控制流。

`Option` 有一个内置方法 `map()`，这个组合算子可用于简单映射`Some -> Some` 和 `None -> None` 的情况。多个不同的 `map()` 调用可以更灵活地链式连接在一起。

在下面例子中，`process()` 轻松取代了前面的所有函数，且更加紧凑。

{map.play}

### 参见：

[闭包][closures], [`Option`][option], 和 [`Option::map()`][map]

[combinators]: https://doc.rust-lang.org/book/glossary.html#combinators
[closures]: ../../fn/closures.html
[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
