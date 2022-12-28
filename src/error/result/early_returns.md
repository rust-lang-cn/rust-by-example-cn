# 提前返回

在上一个例子中，我们显式地使用组合算子处理了错误。另一种处理错误的方式是使用
`match` 语句和**提前返回**（early return）的结合。

这也就是说，如果发生错误，我们可以停止函数的执行然后返回错误。对有些人来说，这样的代码更好写，更易读。这次我们使用提前返回改写之前的例子：

```rust,editable
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

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

到此为止，我们已经学会了如何使用组合算子和提前返回显式地处理错误。我们一般是想要避免 panic 的，但显式地处理所有错误确实显得过于繁琐。

在下一部分，我们将看到，当只是需要 `unwrap` 并且不产生 `panic` 时，可以使用
`?` 来达到同样的效果。
