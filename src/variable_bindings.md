# 变量绑定

Rust 通过静态类型确保类型安全。变量绑定可以在声明变量时标注类型。不过在多数情况下，编译器能够
从字面内容推导出变量的类型，大大减少了标注类型的负担。

使用 `let` 绑定操作可以将值（像具体数据）绑定到变量中。

```rust,editable
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用变量绑定产生警告；可在变量名加上下划线的前缀来消除这些警告内容。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名加上下划线前缀消除警告
}
```
