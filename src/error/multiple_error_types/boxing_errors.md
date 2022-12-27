# 把错误 “装箱”

如果又想写简单的代码，又想保存原始错误信息，一个方法是把它们[装箱][box]（`Box`）。这样做的坏处就是，被包装的错误类型只能在运行时了解，而不能被[静态地判别][dynamic_dispatch]。

对任何实现了 `Error` trait 的类型，标准库的 `Box` 通过 [`From`][from] 为它们提供了到 `Box<Error>` 的转换。

```rust,editable
use std::error;
use std::fmt;

// 为 `Box<error::Error>` 取别名。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        // 泛型错误。没有记录其内部原因。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       .ok_or_else(|| EmptyVec.into())  // 装箱
       .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into())  // 装箱
                .map(|i| 2 * i)
        })
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

### 参见：

[动态分发][dynamic_dispatch] and [`Error` trait][error]

[box]: https://rustwiki.org/zh-CN/std/boxed/struct.Box.html
[dynamic_dispatch]: https://rustwiki.org/zh-CN/book/ch17-02-trait-objects.html#trait-对象执行动态分发
[error]: https://rustwiki.org/zh-CN/std/error/trait.Error.html
[from]: https://rustwiki.org/zh-CN/std/convert/trait.From.html
