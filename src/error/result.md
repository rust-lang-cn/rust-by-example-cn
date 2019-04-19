# 结果 `Result`

[`Result`][result] 是 [`Option`][option] 类型的更丰富的版本，描述的是可能
的**错误**而不是可能的**不存在**。

也就是说，`Result<T，E>` 可以有两个结果的其中一个：

* `Ok<T>`：找到 `T` 元素
* `Err<E>`：找到 `E` 元素，`E` 即表示错误的类型。

按照约定，预期结果是 “Ok”，而意外结果是 “Err”。

`Result` 有很多类似 `Option` 的方法。例如 `unwrap()`，它要么举出元素
`T`，要么就 `panic`。 对于事件的处理，`Result` 和 `Option` 有很多相同的组合算子。

在使用 Rust 时，你可能会遇到返回 `Result` 类型的方法，例如 [`parse()`][parse]
方法。它并不是总能把字符串解析成指定的类型，所以 `parse()` 返回一个
`Result` 表示可能的失败。

我们来看看当 `parse()` 字符串成功和失败时会发生什么：

```rust,editable,ignore,mdbook-runnable
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // 我们试着用 `unwrap()` 把数字放出来。它会咬我们一口吗？
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
```

在失败的情况下，`parse()` 产生一个错误，留给 `unwrap()` 来解包并产生 `panic`。另
外，`panic` 会退出我们的程序，并提供一个让人很不爽的错误消息。

为了改善错误消息的质量，我们应该更具体地了解返回类型并考虑显式地处理错误。

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
