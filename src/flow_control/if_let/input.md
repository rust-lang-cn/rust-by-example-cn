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

{if_let.play}

### 参见：

[`枚举`][enum]，[`Option`][option]，和 [RFC][if_let_rfc]

[enum]: ../../custom_types/enum.html
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: ../../std/option.html
