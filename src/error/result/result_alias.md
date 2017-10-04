# 给 `Result` 起别名

当我们要重复多次使用特定的 `Result` 类型怎么办呢？回忆一下，Rust 允许我们创建[别名][typealias]。对问题中提到的特定 `Result`，我们可以很方便地给它定义一个别名。

在单个模块的级别上创建别名特别有帮助。在特定模块中发现的错误常常会有相同的 `Err` 类型，所以一个单一的别名就能简便地定义**所有的**关联 `Result`。这点太重要了，甚至标准库也提供了一个： `io::Result`！

下面给出一个快速示例来展示语法：

```rust,editable
use std::num::ParseIntError;

// 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
type AliasedResult<T> = Result<T, ParseIntError>;

// 使用上面定义过的别名来表示我们特指的 `Result` 类型。
fn double_number(number_str: &str) -> AliasedResult<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// 这里的别名又让我们节省了一些空间（save some space）。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}
```

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[typealias]: ./cast/alias.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
