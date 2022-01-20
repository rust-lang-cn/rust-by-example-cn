# `?` 的其他用法

注意在上一个例子中，我们调用 `parse` 后总是立即把错误从标准库错误 `map`
到装箱的错误。

```rust,ignore
.and_then(|s| s.parse::<i32>()
    .map_err(|e| e.into())
```

因为这个操作很简单常见，如果有省略写法就好了。`and_then` 不够灵活，所以不能实现
这样的写法。不过，我们可以使用 `?` 来代替它。

之前我们说 `?` 就是 “要么 `unwrap` 要么 `return Err(error)`”，这大部分是对的，但
事实上 `?` 是 “要么 `unwrap` 要么 `return Err(From::from(err))`”。`From::from` 是
不同类型间的转换工具，也就是说，如果在错误能够转换成返回类型的地方使用 `?`，它就
会自动转换成返回类型。

这里，我们使用 `?` 重写之前的例子。这样，只要为我们的错误类型实现 `From::from`，就
可以不再使用 `map_err`。

```rust,editable
use std::error;
use std::fmt;

// 为 `Box<error::Error>` 取别名。
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
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

    fn cause(&self) -> Option<&error::Error> {
        // 泛型错误，没有记录内部原因。
        None
    }
}

// 这里的结构和之前一样，但是这次没有把所有的 `Results` 和 `Options` 串起来，
// 而是使用 `?` 立即得到内部值。
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
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

这段代码现在已经很清晰了。相比原始的 `panic`，它就像是把所有的 `unwrap` 调用都换
成 `?` 一样。与 `panic` 相比，这样做的区别在于返回类型是 `Result`，因而必须在顶层
解构它们。

### 参见：

[`From::from`][from] and [`?`][q_mark]

[from]: https://rustwiki.org/zh-CN/std/convert/trait.From.html
[q_mark]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#the--operator
