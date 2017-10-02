`Iterator` trait 用来实现关于集合（collection）类型（比如数组）的迭代器。

这个 trait 只需定义一个指向 `next`（下一个）元素的方法，这可手动在 `impl` 代码块中定义，或者自动定义（比如在数组或区间中）。

为方便起见，`for` 结构通常使用 [`.into_iterator()`][intoiter] 方法将一些集合类型转换为迭代器。

下面例子展示了如何访问使用 `Iterator` trait 的方法，关于这方面的更多内容可[点击这里][iter]查看。

{iter.play}

[intoiter]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[iter]: http://doc.rust-lang.org/core/iter/trait.Iterator.html
