# 函数

函数使用 `fn` 关键字来声明。函数的参数需要标注类型，就和变量一样，另外如果
函数返回一个值，返回类型必须在箭头 `->` 之后特别指出来。

函数最后的表达式将作为返回值。或者在函数内使用 `return` 语句来提前返回值，
甚至在循环或 `if` 内部使用。

让我们使用函数来重写 FizzBuzz 函数吧！

```rust,editable
// 和 C/C++ 不一样，Rust 的函数定义位置是没有限制的
fn main() {
    // 我们在这里使用函数，并在后面的其他位置定义它
    fizzbuzz_to(100);
}

// 函数返回一个布尔（boolean）值
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 极端情况，提前返回（Corner case, early return）
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，这里可以不用 `return` 关键字
    lhs % rhs == 0
}

// 函数不返回值，而实际上是返回一个单元类型 `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，可以从标记中删除返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
```
