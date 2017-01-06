`map()` 以链式调用的方式来简化 `match` 语句。然而，在返回类型是 `Option<T>` 的函数中使用 `map()` 会导致出现嵌套形式 `Option<Option<T>>`。多层链式调用也会变得混乱。所以有必要引入 `and_them()`，就像某些熟知语言中的 flatmap。

`and_then()` 使用包裹的值（wrapped value）调用其函数输入并返回结果。 如果 `Option` 是 `None`，那么它返回 `None`。


在下面例子中，`cookable_v2()` 会产生一个 `Option<Food>`。使用 `map()` 替代 `and_then()` 将会得到 `Option<Option<Food>>`，对 `eat()` 来说是一个无效类型。

{and_then.play}

### 参见：

[闭包][closures]，[`Option::map()`][map], 和 [`Option::and_then()`][and_then]

[closures]: ../../fn/closures.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then 
