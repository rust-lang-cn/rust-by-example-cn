# while let

和 `if let` 类似，`while let` 会产生更加难看的 `match` 的一连串内容。
考虑下面的有关增量 `i` 的一连串内容：

```rust
// 将 `optional` 设为 `Option<i32>` 类型
let mut optional = Some(0);

// Repeatedly try this test.
// 重复运行这个测试。
loop {
    match optional {
        // 如果 `optional` 解构成功，就执行下面语句块。
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ 需要三个缩进！
        },
        // 当解构失败时退出循环：
        _ => { break; }
        // ^ 为什么要这样的语句呢？肯定有更优雅的处理方式！
    }
}
```

使用 `while let` 可以使这一连串内容变得更加优雅：

```rust,editable
fn main() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 分析：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则中断退出（`break`）。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有额外可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}
```

### 参见：

[`枚举`][enum]，[`Option`][option]，和 [RFC][while_let_rfc]

[enum]: ./custom_types/enum.html
[option]: ./std/option.html
[while_let_rfc]: https://github.com/rust-lang/rfcs/pull/214
