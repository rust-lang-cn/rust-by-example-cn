Rust 语言可以先声明变量绑定，后面才将它们初始化。但是这种情况用得很少，因为这样很可能导致使用未
初始的变量。

```rust,editable
fn main() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化一个绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错！使用了未初始化的绑定
    println!("another binding: {}", another_binding);
    // 改正 ^ 注释掉此行

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
```

编译器禁止使用未经初始化的变量，因为这会产生未定义行为（undefined behavior）。
