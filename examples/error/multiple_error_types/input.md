前面出现的例子确实很方便；都是 `Result` 和其他 `Result` 交互，还有 `Option` 和其他 `Option` 交互。

有时 `Option` 需要和 `Result` 进行交互，或是 `Result<T, Error1>` 需要和 `Result<T, Error2` 进行交互。在这类情况下，我们想要以一种方式来管理不同的错误类型，使得它们可组合且易于交互。

在下面代码中，`unwrap` 的两个实例生成了不同的错误类型。`Vec::first` 返回一个 `Option`，而 `parse::<i32>` 返回一个 `Result<i32, ParseIntError>`：

```rust
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

使用组合器的知识，我们能够重写上述代码来显式地处理错误。为了做到两种错误类型都能够出现，我们需要将他们转换为一种通用类型，比如 `String` 类型。

就这样，我们将 `Option` 和 `Result` 都转换成 `Result`，从而将他们的错误类型映射成相同的类型：

{multiple_error_types.play}

在下一节，我们将学到一个替代方法来显式处理这型错误。

### 参见：

[`Option::ok_or`][okor], [`Result::map_err`][maperr]

[okor]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[maperr]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
