# `Result` 的 `map`

上一节的 `multiply` 函数的 panic 设计不是健壮的（robust）。一般地，我们希望把
错误返回给调用者，这样它可以决定回应错误的正确方式。

首先，我们需要了解需要处理的错误类型是什么。为了确定 `Err` 的类型，我们可以
用 [`parse()`][parse] 来试验。Rust 已经为 [`i32`][i32] 类型使用
[`FromStr`][from_str] trait 实现了 `parse()`。结果表明，这里的 `Err` 类型被指定为
[`ParseIntError`][parse_int_error]。

> 译注：原文没有具体讲如何确定 `Err` 的类型。由于目前用于获取类型的函数仍然是不
> 稳定的，我们可以用间接的方法。使用下面的代码：
>
> ```rust,editable,ignore
> fn main () {
>     let i: () = "t".parse::<i32>();
> }
> ```
>
> 由于不可能把 `Result` 类型赋给单元类型变量 `i`，编译器会提示我们：
>
> ```text
> note: expected type `()`
>          found type `std::result::Result<i32, std::num::ParseIntError>`
> ```
>
> 这样就知道了 `parse<i32>` 函数的返回类型详情。

在下面的例子中，使用简单的 `match` 语句导致了更加繁琐的代码。

```rust,editable
use std::num::ParseIntError;

// 修改了上一节中的返回类型，现在使用模式匹配而不是 `unwrap()`。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 这种情形下仍然会给出正确的答案。
    let twenty = multiply("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    let tt = multiply("t", "2");
    print(tt);
}
```

幸运的是，`Option` 的 `map`、`and_then`、以及很多其他组合算子也为 `Result` 实现
了。官方文档的 [`Result`][result] 一节包含完整的方法列表。

```rust,editable
use std::num::ParseIntError;

// 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
// 除去写法外，这个函数与上面那个完全一致，它的作用是：
// 如果值是合法的，计算其乘积，否则返回错误。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 这种情况下仍然会给出正确的答案。
    let twenty = multiply("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    let tt = multiply("t", "2");
    print(tt);
}
```

[parse]: https://rustwiki.org/zh-CN/std/primitive.str.html#method.parse
[from_str]: https://rustwiki.org/zh-CN/std/str/trait.FromStr.html
[i32]: https://rustwiki.org/zh-CN/std/primitive.i32.html
[parse_int_error]: https://rustwiki.org/zh-CN/std/num/struct.ParseIntError.html
[result]: https://rustwiki.org/zh-CN/std/result/enum.Result.html
