# 遍历 `Result`

`Iter::map` 操作可能失败，比如：

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let possible_numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", possible_numbers);
}
```

我们来看一些处理这种问题的策略：

## 使用 `filter_map()` 忽略失败的项

`filter_map` 会调用一个函数，过滤掉为 `None` 的所有结果。

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers);
}
```

## 使用 `collect()` 使整个操作失败

`Result` 实现了 `FromIter`，因此结果的向量（`Vec<Result<T, E>>`）可以被转换成
结果包裹着向量（`Result<Vec<T>, E>`）。一旦找到一个 `Result::Err` ，遍历就被终止。

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

同样的技巧可以对 `Option` 使用。

## 使用 `Partition()` 收集所有合法的值与错误

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

当你看着这些结果时，你会发现所有东西还在 `Result` 中保存着。要取出它们，需要一些
模板化的代码。

```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
