借用检查器使用显式的生命周期来明确引用的有效时间应该持续多久。在生命周期没有省略[^1]的情况，Rust 需要显式标注来确定引用的生命周期应该是什么样的。对于显式地标注引用的生命周期的语法如下：

```rust
foo<'a>
// `foo` 带有一个生命周期参量 `'a`
```

和[闭包][annoymity]类似，使用生命周期需要泛型。另外这个生命周期的语法也表明了 `foo` 的生命周期不能超出 `'a` 的周期。类型的显式标注有 `&'a T` 这样的形式，其中 `'a` 已引入。

In cases with multiple lifetimes, the syntax is similar:
对于多个生命周期的情况，语法是类似的：

```rust
foo<'a, 'b>
// `foo` 带有生命周期参量 `'a` 和 `'b`
```

在这种情形中，`foo` 的生命周期不能超出 `'a` 或 `'b` 的周期。

看下面的例子，了解显式生命周期标注的运用：

{explicit.play}

[^1]: [省略][elision] 隐式地标注了生命周期，所以情况不同。
### See also:

[泛型][generics] 和 [闭包][closures]

[anonymity]: ../../fn/closures/anonymity.html
[closures]: ../../fn/closures.html
[elision]: elision.html
[generics]: ../../generics.html
