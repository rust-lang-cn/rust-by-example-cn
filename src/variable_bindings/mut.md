# 可变变量

变量绑定默认是不可变的（immutable），但加上 `mut` 修饰语后变量就可以改变。

```rust,editable,ignore,mdbook-runnable
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确代码
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    _immutable_binding += 1;
    // 改正 ^ 将此行注释掉
}
```

编译器会给出关于变量可变性的详细诊断信息。
