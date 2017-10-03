# 显示标注

借用检查器使用显式的生命周期来明确引用的有效时间应该持续多久。在生命周期没有省略[^1]的情况，Rust 需要显式标注来确定引用的生命周期应该是什么样的。对于显式地标注引用的生命周期的语法如下：

```rust,ignore
foo<'a>
// `foo` 带有一个生命周期参量 `'a`
```

和[闭包][anonymity]类似，使用生命周期需要泛型。另外这个生命周期的语法也表明了 `foo` 的生命周期不能超出 `'a` 的周期。类型的显式标注有 `&'a T` 这样的形式，其中 `'a` 已引入。

In cases with multiple lifetimes, the syntax is similar:
对于多个生命周期的情况，语法是类似的：

```rust,ignore
foo<'a, 'b>
// `foo` 带有生命周期参量 `'a` 和 `'b`
```

在这种情形中，`foo` 的生命周期不能超出 `'a` 或 `'b` 的周期。

看下面的例子，了解显式生命周期标注的运用：

```rust,editable,ignore,mdbook-runnable
// 生命周期 `'a` 和 `'b`。这两个生命周期都必须至少要和 `print_refs`
// 函数的一样长。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参量的函数，不过有一个生命周期参量 `'a`。
fn failed_borrow<'a>() {
    let _x = 12;

    // 报错：`_x` 存活时间长度不够（`_x` does not live long enough）
    //let y: &'a i32 = &_x;
    // 尝试使用生命周期 `'a` 作为函数内部的显式类型标注将导致失败，因为
    // `&_x` 的生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
}

fn main() {
    // 创建变量，给下面代码借用。
    let (four, nine) = (4, 9);
    
    // 两个变量的借用（`&`）都传进函数。
    print_refs(&four, &nine);
    // 任何借用得来的输入量都必须比借入者“活”得更长。
    // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs`
    // 的长。
    
    failed_borrow();
    // `failed_borrow` 未包含引用来迫使 `'a` 长于函数的生命周期，
    // 但 `'a` 寿命更长。因为该生命周期从未被约束，所以默认为 `'static`。
}
```

[^1]: [省略][elision] 隐式地标注了生命周期，所以情况不同。

### 参见：

[泛型][generics] 和 [闭包][closures]

[anonymity]: ../../fn/closures/anonymity.html
[closures]: ../../fn/closures.html
[elision]: elision.html
[generics]: ../../generics.html
