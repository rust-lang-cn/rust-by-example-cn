`while` 关键字可以用作当型循环（当条件满足时循环）。

让我们用 `while` 循环写一个不怎么出名的 [FizzBuzz][fizzbuzz] 程序。

```rust,editable
fn main() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时进入循环操作
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加1
        n += 1;
    }
}
```

[fizzbuzz]: http://en.wikipedia.org/wiki/Fizz_buzz
