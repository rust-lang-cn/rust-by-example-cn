# 定义一个错误类型

有时候把所有不同的错误都视为一种错误类型会简化代码。我们将用一个自定义错误类型来
演示这一点。

Rust 允许我们定义自己的错误类型。一般来说，一个 “好的” 错误类型应当：

* 用同一个类型代表了多种错误
* 向用户提供了清楚的错误信息
* 能够容易地与其他类型比较
    - 好的例子：`Err(EmptyVec)`
    - 坏的例子：`Err("Please use a vector with at least one element".to_owned())`
* 能够容纳错误的具体信息
    - 好的例子：`Err(BadChar(c, position))`
    - 坏的例子：`Err("+ cannot be used here".to_owned())`
* 能够与其他错误很好地整合

```rust,editable
use std::error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
// 定义我们的错误类型，这种类型可以根据错误处理的实际情况定制。
// 我们可以完全自定义错误类型，也可以在类型中完全采用底层的错误实现，
// 也可以介于二者之间。
struct DoubleError;

// 错误的生成与它如何显示是完全没关系的。没有必要担心复杂的逻辑会导致混乱的显示。
//
// 注意我们没有储存关于错误的任何额外信息，也就是说，如果不修改我们的错误类型定义的话，
// 就无法指明是哪个字符串解析失败了。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// 为 `DoubleError` 实现 `Error` trait，这样其他错误可以包裹这个错误类型。
impl error::Error for DoubleError {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // 把错误换成我们的新类型。
       .ok_or(DoubleError)
       .and_then(|s| s.parse::<i32>()
            // 这里也换成新类型。
            .map_err(|_| DoubleError)
            .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```
