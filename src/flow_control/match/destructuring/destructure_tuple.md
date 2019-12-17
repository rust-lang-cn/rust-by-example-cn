# 元组

元组可以在 `match` 中解构，如下所示：

```rust,editable
fn main() {
    let pair = (0, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    // match 可以解构一个元组
    match pair {
        // 解构出第二个值
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}
```

### 参见：

[元组](../../../primitives/tuples.md)
