既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为参数传递。

{input_functions.play}

额外说明，`Fn`，`FnMut`，和 `FnOnce` 这些 `trait` 明确了闭包如何从封闭的作用域中捕获变量。

### 参见：

[`Fn`][fn], [`FnMut`][fn_mut], 和 [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
