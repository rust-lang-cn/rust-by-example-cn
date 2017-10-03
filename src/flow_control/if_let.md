# if let

在一些例子中，`match` 使用起来并不优雅。比如：

```rust
// 将 `optional` 定为 `Option<i32>` 类型
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ 行首需要2个缩进，就这样可以从 option 类型中对 `i`
        // 进行解构
    },
    _ => {},
    // ^ 必需内容，因为 `match` 需要覆盖全部情况。难道不觉得冗余吗？
};

```

`if let` 对这样的用法要简洁得多，并且允许指明特定的各种不同的失败可选项
内容（options）：

```rust,editable
fn main() {
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构解读：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。换到失败情形（Change to the failure case）。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供一个改变的失败条件（Provide an altered failing condition）。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // 解构失败。执行 `else if` 条件来判断轮到的失败分支是否需要执行
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件执行错误。这是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}
```

### 参见：

[`枚举`][enum]，[`Option`][option]，和 [RFC][if_let_rfc]

[enum]: ../../custom_types/enum.html
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: ../../std/option.html
