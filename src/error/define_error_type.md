# 定义一个错误类型

前面我们一直使用字符串（`String`）作为错误消息。实际上，字符串作为错误类型是存在一些局限的。下面是友好的错误类型标准。字符串（`String`）很好地实现了前两点，但无法做到后两点：
Rust 允许自定义错误类型。一般而言，一个“良好”的错误类型：

* 使用相同类型来表达不同的错误
* 给用户提供友好的错误信息
* 方便和其他类型比较
    - Good: `Err(EmptyVec)`
	- Bad: `Err("Please use a vector with at least one element".to_owned())`
* 能够保存错误的信息（原文：Can hold information about the error.）：
    - Good: `Err(BadChar(c, position))`
	- Bad: `Err("+ cannot be used here".to_owned())`

可以看到字符串（`String`）（前面我一们一值在用）可以地满足前两点标准，但后两条无法满足。这使得 `String` 错误既难以创建，也难以达到要求。仅仅为了优雅地显示，实在不应该使用 `String` 格式化方式污染大量的逻辑代码（原文：It should not be necessary to pollute logic heavy code with `String` formatting simply to display nicely.）。

```rust,editable
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
// 定义我们的错误类型。不管对我们的错误处理情况有多重要，这些都可能自定义。
// 现在我们能够按照底层工具的错误实现，写下我们的错误，或者两者之间的内容。
// （原文：Define our error types. These may be customized however is useful for our error
// handling cases. Now we will be able to defer to the underlying tools error
// implementation, write our own errors, or something in between.）
enum DoubleError {
    // 我们不需要任何额外的信息来描述这个错误。
    EmptyVec,
    // 我们将推迟对于这些错误的解析错误的实现。（原文：We will defer to the parse
    // error implementation for their error.）提供额外信息将要增加更多针对类型的数据。
    Parse(ParseIntError),
}

// 类型的展示方式的和类型的产生方式是完全独立的。我们无需担心显示样式会搞乱我们
// 工具集所需的复杂逻辑。它们是独立的，就是说它们处理起来是相互独立的。
//
// 我们没有存储关于错误的额外信息。若确实想要，比如，要指出哪个字符串无法解析，
// 那么我们不得不修改我们类型来携带相应的信息。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // 这是一个 wrapper，所以按照底层类型来给出我们的 `fmt` 实现。
            // （原上：This is a wrapper so defer to the underlying types' own implementation
            // of `fmt`.）
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // 将错误改成我们新的类型。
       .ok_or(DoubleError::EmptyVec)
       .and_then(|s| s.parse::<i32>()
             // 在这里也更新成新的错误类型。    
            .map_err(DoubleError::Parse)
            .map(|i| 2 * i))
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

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: ./error/option_with_result/result_string_errors.html
