# 高阶函数

Rust 提供了高阶函数（Higher Order Function, HOF），指那些输入一个或多个
函数，并且/或者产生一个更有用的函数的函数。HOF 和惰性迭代器（lazy iterator）给
 Rust 带来了函数式（functional）编程的风格。

```rust,editable
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数取平方
             .take_while(|&n| n < upper) // 取小于上限的
             .filter(|&n| is_odd(n))     // 取奇数
             .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
```

[Option][option] 和 [迭代器][iter] 都实现了不少高阶函数。

[option]: http://doc.rust-lang.org/core/option/enum.Option.html
[iter]: http://doc.rust-lang.org/core/iter/trait.Iterator.html
