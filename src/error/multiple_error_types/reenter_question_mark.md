# `?` 的其他用法

注意在上一个例子中，我们调用 `parse` 后总是立即将错误从标准库的错误 `map`（映射）到装箱错误。

```rust,ignore
.and_then(|s| s.parse::<i32>()
    .map_err(|e| e.into())
```

因为这个操作很简单常见，如果有省略写法就好了。遗憾的是 `and_then` 不够灵活，所以实现不了这样的写法。不过，我们可以使用 `?` 来代替它。

`?` 之前被解释为要么 `unwrap`，要么 `return Err(err)`，这只是在大多数情况下是正确的。`?` 实际上是指  `unwrap` 或 `return Err(From::from(err))`。由于 `From::from` 是不同类型之间的转换工具，也就是说，如果在错误可转换成返回类型地方使用 `?`，它将自动转换成返回类型。

我们在这里使用 `?` 重写之前的例子。重写后，只要为我们的错误类型实现 `From::from`，就可以不再使用 `map_err`。

```rust,editable
use std::error;
use std::fmt;

// 为 `Box<error::Error>` 取别名。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// 这里的结构和之前一样，但是这次没有把所有的 `Result` 和 `Option` 串起来，
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

这段代码已经相当清晰了。与原来的 `panic` 相比，除了返回类型是 `Result` 之外，它就像是把所有的 `unwrap` 调用都换成 `?` 一样。因此必须在顶层解构它们。

### 参见：

[`From::from`][from] 和 [`?`][q_mark]

[from]: https://rustwiki.org/zh-CN/std/convert/trait.From.html
[q_mark]: https://rustwiki.org/zh-CN/reference/expressions/operator-expr.html#问号操作符
