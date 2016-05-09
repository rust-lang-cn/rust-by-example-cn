我们已经指出 Rust 在没有类型标注时选择怎样的方式去捕获临时的变量（原文：It has been
noted that Rust chooses how to capture variables on the fly without annotation）。
这在通常的使用中是相当方便的，然而无论何时编写函数这歧义都是不允许的。
闭包的完整类型，包括捕获的类型，都必须类型标注。捕获闭包功能的方式是由下面的
`traits` 来标注类型（原文：The manner of capture a closure uses is annotated as
one of the following `traits`）：

* `Fn`：通过引用（`&T`）
* `FnMut`：通过可变引用（`&mut T`）
* `FnOnce`：通过值（`T`）
	

即使类型已经标注，这些用法也是相当灵活的：`FnOnce` 参量明确规定了闭包**可能**通过
`T` 或 `&mut T` 或 `&T` 的随意一种方式来捕获（若移动语义（move）可能的话，
任意借用类型也应该是可行的）。反过来就不一定成立：如果参量是 `Fn`，那么其他情况就不允许了
（原文：then nothing lower is allowed）。所以有如下规则：

* 任何已标注的参数都限制捕获自身和上述内容（原文：any annotated parameter restricts capture to itself and above）

另外，Rust 在 variable-by-variable 基础上将更倾向以可能的最少限制方式来捕获变量（原文：Rust 
will preferentially capture variables in the least restrictive manner possible on 
a variable-by-variable basis）：

{input_parameters.play}

### 参见：

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], and [`FnOnce`][fnonce]

[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
