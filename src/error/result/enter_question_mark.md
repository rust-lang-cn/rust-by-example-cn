# 引入 `?`

有时我们只是想 `unwrap` 且避免产生 `panic`。到现在为止，对 `unwrap` 的错误处理都在强迫我们一层层地嵌套，然而我们只是想把里面的变量拿出来。`?` 正是为这种情况准备的。

当找到一个 `Err` 时，可以采取两种行动：

1. `panic!`，不过我们已经决定要尽可能避免 panic 了。
2. 返回它，因为 `Err` 就意味着它已经不能被处理了。

`?` **几乎**[^†] 就等于一个会返回 `Err` 而不是 `panic` 的 `unwrap`。我们来看看怎样简化之前使用组合算子的例子：

```rust,editable
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

## `try!` 宏

在 `?` 出现以前，相同的功能是使用 `try!` 宏完成的。现在我们推荐使用 `?` 运算符，但是在老代码中仍然会看到 `try!`。如果使用 `try!` 的话，上一个例子中的 `multiply` 函数看起来会像是这样：

```rust,editable,edition2015
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```


[^†]: 更多细节请看[`?` 的更多用法][re_enter_?]。

[re_enter_?]: ../multiple_error_types/reenter_question_mark.md
