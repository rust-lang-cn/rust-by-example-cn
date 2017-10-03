# 作用域和隐藏

变量绑定有一个作用域，并且限定在一个**代码块**（block）中存活（live）。代码块是一个被 `{}` 包围的
语句集合。另外也允许[变量隐藏][variable-shadow]。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // 此绑定存在于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，比 main 函数拥有一个更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*隐藏*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    println!("outer short: {}", short_lived_binding);
    // 改正 ^ 注释掉这行

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*隐藏*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
```

[variable-shadow]: https://en.wikipedia.org/wiki/Variable_shadowing
