在前面的例子中，我们使用组合算子显式地处理错误。 另一种处理这种情形分解的方法是使用 `match` 语句和**提前返回**（*early returns*）的组合形式。

也就是说，我们可以简单地停止执行函数并返回错误（若发生的话）。 而且这种形式的代码更容易阅读和编写。考虑如下版本，这是将之前的例子使用提前返回方式重写的：

```rust,editable
// 使用 `String` 作为错误类型
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    // 若存在值时，则将 `Option` 转换成 `Result`。
    // 否则提供一个包含此 `String` 的 `Err`。
    let first = match vec.first() {
        Some(first) => first,
        None => return Err("Please use a vector with at least one element.".to_owned())
    };

    // 若 `parse` 操作正常的话，则将内部的数字扩大 2 倍。
    // 否则映射任意错误，来自 `parse` 产生的 `String`。
    // （原文：Double the number inside if `parse` works fine.
    // Otherwise, map any errors that `parse` yields to `String`.）
    match first.parse::<i32>() {
        Ok(i) => Ok(2 * i),
        Err(e) => Err(e.to_string()),
    }
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

现在我们已经学会了使用组合算子和提前返回来显式地处理错误。虽然我们通常希望避免 `panic`，但显式处理我们所有的错误将会麻烦。

在下一节，我们将介绍 `try!`，用在这样的场景，我们只需要简单地 `unwrap` 而避免可能的 `panic`。
