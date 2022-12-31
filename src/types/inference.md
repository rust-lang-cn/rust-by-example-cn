# 类型推断

Rust 的类型推断引擎是很聪明的，它不只是在初始化时看看[右值][rvalue]（r-value）的类型而已，它还会考察变量之后会怎样使用，借此推断类型。这是一个类型推导的进阶例子：

```rust,editable
fn main() {
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;

    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）
    
    // 在向量中插入 `elem`。
    vec.push(elem);
    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。
    // 试一试 ^ 注释掉 `vec.push(elem)` 这一行。

    println!("{:?}", vec);
}
```

没有必要写类型说明，编译器和程序员皆大欢喜！

[rvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
