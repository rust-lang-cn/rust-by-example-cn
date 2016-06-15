`map()` 以链式调用的方式来简化 `match` 语句。然而，在返回类型是 `Option<T>` 的函数中使用 `map()` 会导致出现嵌套形式 `Option<Option<T>>`。多层链式调用也会变得混乱。

所以有必要引入 `and_them()`。就像某些熟知语言中的 flatmap，`and_then()` 调用它的函数带有包裹值（wrapped value）的输入，或者当 `Option` 为 `None` 时返回 `None`（原文：Known in some languages as flatmap, `and_then()` calls its function input with the wrapped value or returns `None` if the `Option` is `None`.）。

在下面例子中，`cookable_v2()` 会产生一个 `Option<Food>`。使用 `map()` 替代 `and_then()` 将会得到 `Option<Option<Food>>`，对 `eat()` 来说是一个无效类型。

{and_then.play}

### 参见：

[`Option`][option], [`Option::map()`][map], 和 [`Option::and_then()`][and_then]

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then 
