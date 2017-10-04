# 各种错误类型

前面出现的例子确实很方便；都是 `Result` 和其他 `Result` 交互，还有 `Option` 和其他 `Option` 交互。

有时 `Option` 需要和 `Result` 进行交互，或是 `Result<T, Error1>` 需要和 `Result<T, Error2` 进行交互。在这类情况下，我们想要以一种方式来管理不同的错误类型，使得它们可组合且易于交互。

在下面代码中，`unwrap` 的两个实例生成了不同的错误类型。`Vec::first` 返回一个 `Option`，而 `parse::<i32>` 返回一个 `Result<i32, ParseIntError>`：

```rust,ignore,mdbook-runnable
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误1
    2 * first.parse::<i32>().unwrap() // 生成错误2
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    
    println!("The first doubled is {}", double_first(empty));
    // 错误1：输入 vector 为空
    
    println!("The first doubled is {}", double_first(strings));
    // 错误2：此元素不能解析成数字
}
```

使用组合算子的知识，我们能够重写上述代码来显式地处理错误。为了做到两种错误类型都能够出现，我们需要将他们转换为一种通用类型，比如 `String` 类型。

就这样，我们将 `Option` 和 `Result` 都转换成 `Result`，从而将他们的错误类型映射成相同的类型：

```rust,editable
// 使用 `String` 作为错误类型
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // 若值存在则将 `Option` 转换成 `Result`。
       // 否则提供一个包含该字符串（`String`） 的 `Err`。
       .ok_or("Please use a vector with at least one element.".to_owned())
       // 回想一下，`parse` 返回一个 `Result<T, ParseIntError>`。
       .and_then(|s| s.parse::<i32>()
                      // 映射任意错误 `parse` 产生得到 `String`。
                      // （原文：Map any errors `parse` yields to `String`.）
                      .map_err(|e| e.to_string())
                      // `Result<T, String>` 成为新的返回类型，
                      // 我们可以给里面的数字扩大两倍。
                      .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}
```

在下一节，我们将学到一个替代方法来显式处理这型错误。

### 参见：

[`Option::ok_or`][okor], [`Result::map_err`][maperr]

[okor]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[maperr]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
