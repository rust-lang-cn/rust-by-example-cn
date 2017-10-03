# try!` 的其他用法

注意在前面的例子中，我们对调用 `parse` 的最直接反应就是将错误从库错误映射到我们的新的自定义错误类型（原文：Notice in the previous example that our immediate reaction to calling `parse` is to `map` the error from a library error into our new custom error type）：

```rust,ignore
.and_then(|s| s.parse::<i32>()
    .map_err(DoubleError::Parse)
```

这是一个很简单且常见的操作，要是它能够省略的话将会相当方便。可惜的是，因为 `and_then` 不够灵活，所以它不能。但是，我们可改用 `try!`。

`try!` 在前面已经解释过，它可以充当 `unwrap` 或 `return Err(err)`，这说法只是很大程度上是对的。实际上它意味着 `unwrap` 或者 `return Err(From::from(err))`。由于 `From::from` 是一个不同类型间相互转换的工具，所以如果你使用 `try!`，当中的错误若能够转换成返回类型，这将会自动转换。

在这里，我们使用 `try!` 重写前面的例子。结果可看到，`From::from` 已对我们的错误类型提供实现时，`map_err` 将会消失：

```rust,editable
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

// 实现从 `ParseIntError` 到 `DoubleError` 的转换。如果一个 `ParseIntError`
// 需要转换成 `DoubleError`，这将会被 `try!` 自动调用。
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

// 和前面的结构一样，但没有将全部的 `Results` 和 `Options` 链接在一起，
// 我们使用 `try!` 立即得到内部的值。
// （原文：// The same structure as before but rather than chain all `Results`
// and `Options` along, we `try!` to get the inner value out immediately.）
fn double_first(vec: Vec<&str>) -> Result<i32> {
    // 仍然转为 `Result`，通过规定怎样转为 `None`。
    // （原上：// Still convert to `Result` by stating how to convert `None`.）
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

现在变得整洁多了。如果和原始的 `panic` 进行比较，可以看到它好像就是使用 `try!` 替换 `unwrap`，除了返回类型是 `Result` 类型这点不同，所以它们必须在顶层被解构出来。

然而，不要指望这种错误处理经常取代 `unwrap` 的用法。这种错误处理会使代码行数扩大三倍，即便是很小的一段代码也不能称之为简单。

实际中要是将 1000 行库代码从 `unwrap` 移动到更适当的错误处理，可能会导致代码量增加 100 行，但这做法是可取的，当然这重构的过程并非易事。

很多库可能只是抛弃了实现 `Display`，然后在所需的基础上增加 `From`（原文：Many libraries might get away with only implementing `Display` and then adding `From` on an as needed basis.）。然而，越是正式的库最后越是需要面临实现更高级的错误处理的需求。

### 参见：

[`From::from`][from] 和 [`try!`][try]

[from]: http://doc.rust-lang.org/std/convert/trait.From.html
[try]: http://doc.rust-lang.org/std/macro.try!.html
