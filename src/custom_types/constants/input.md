Rust 有两种常量，可以在任意作用域声明，包括全局作用域。这两种常量都要显式地标注：

* `const`： 不可改变的值（常用类型）。
* `static`： 在 [`'static`][static] 生命周期内可能发生改变的变量。

有个特例就是 `"string"` 原始类型。可以给它直接赋一个不可改变的 `static` 变量，是因为它的
类型标记：`&'static str` 包含了生命周期 `'static`。其他的引用类型都必须特别注明从而拥有
`'static` 生命周期。这似乎是无关紧要的，因为所需的显式标记会隐藏差异（This may seem minor
 though because the required explicit annotation hides the distinction.）。

{constants.play}

### 参见：

[`const`/`static` RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md),
[`'static` 生命周期][static]

[static]: ../scope/lifetime/static_lifetime.html
