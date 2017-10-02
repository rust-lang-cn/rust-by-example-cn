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
