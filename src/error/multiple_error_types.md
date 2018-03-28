# 处理多种错误类型

前面出现的例子都是很方便的情况；都是 `Result` 和其他 `Result` 交互，还有 `Option`
和其他 `Option` 交互。

有时 `Option` 需要和 `Result` 进行交互，或是 `Result<T, Error1>` 需要和
`Result<T, Error2>` 进行交互。在这类情况下，我们想要以一种方式来管理不同的错误
类型，使得它们可组合且易于交互。

在下面代码中，`unwrap` 的两个实例生成了不同的错误类型。`Vec::first` 返回一个
`Option`，而 `parse::<i32>` 返回一个 `Result<i32, ParseIntError>`：

```rust,editable
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误 1
    2 * first.parse::<i32>().unwrap() // 生成错误 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    
    println!("The first doubled is {}", double_first(numbers));
    
    println!("The first doubled is {}", double_first(empty));
    // 错误1：输入 vector 为空
    
    println!("The first doubled is {}", double_first(strings));
    // 错误2：此元素不能解析成数字
}
```

在下面几节中，我们会看到处理这类问题的几种策略。
