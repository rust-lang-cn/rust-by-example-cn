既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！不过因为函数是**不能**捕获变量的，闭包则明显更加灵活（原文：closures are strictly more flexible）。因此任何函数可以接受一个闭包作为参数也可以接受一个函数。

{input_functions.play}

额外说明，`Fn`，`FnMut`，和 `FnOnce` 这些 `trait` 明确了闭包如何从封闭的作用域中捕获变量。

### 参见：

[`Fn`][fn], [`FnMut`][fn_mut], 和 [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
