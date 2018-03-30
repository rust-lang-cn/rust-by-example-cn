# 文档测试

为 Rust 工程编写文档的主要方式是在源代码中写注释。文档注释使用 [markdown] 语法
书写，支持代码块。Rust 很注重正确性，这些注释中的代码块也会被编译并且用作测试。

```rust,ignore
/// 第一行是对函数的简短描述。
///
/// 接下来数行是详细文档。代码块用三个反引号开启，Rust 会隐式地在其中添加
/// `fn main()` 和 `extern crate <cratename>`。比如测试 `doccomments` crate：
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常可能带有 "Examples"、"Panics" 和 "Failures" 这些部分。
///
/// 下面的函数将两数相除。
///
/// # Examples
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// 如果第二个参数是 0，函数将会 panic。
///
/// ```rust,should_panic
/// // panics on division by zero
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}
```

这些测试仍然可以通过 `cargo test` 执行：

```bash
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests doccomments

running 3 tests
test src/lib.rs - add (line 7) ... ok
test src/lib.rs - div (line 21) ... ok
test src/lib.rs - div (line 31) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 文档测试的目的

文档测试的主要目的是作为使用函数功能的例子，这是最重要的[指导
原则][question-instead-of-unwrap]之一。文档测试应当可以作为完整的代码段被直接
使用（很多用户会复制文档中的代码来用，所以例子要写得完善）。但使用 `?` 符号会
导致编译失败，因为 `main` 函数会返回 `unit` 类型。幸运的是，我们可以在文档中
隐藏几行源代码：你可以写 `fn try_main() -> Result<(), ErrorType>` 这样的
函数，把它隐藏起来，然后在隐藏的 `main` 中展开它。听起来很复杂？请看例子：

```rust,ignore
/// 在文档测试中使用隐藏的 `try_main`。
///
/// ```
/// # // 被隐藏的行以 `#` 开始，但它们仍然会被编译！
/// # fn try_main() -> Result<(), String> { // 隐藏行包围了文档中显示的函数体
/// let res = try::try_div(10, 2)?;
/// # Ok(()) // 从 try_main 返回
/// # }
/// # fn main() { // 开始主函数，其中将展开 `try_main` 函数
/// #    try_main().unwrap(); // 调用并展开 try_main，这样出错时测试会 panic
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
```

## 参见

* 关于文档风格的 [RFC505][RFC505]
* [API 指导原则][doc-nursery]中关于文档的原则

[doc-nursery]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html
[markdown]: https://daringfireball.net/projects/markdown/
[RFC505]: https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md
[question-instead-of-unwrap]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html#examples-use--not-try-not-unwrap-c-question-mark
