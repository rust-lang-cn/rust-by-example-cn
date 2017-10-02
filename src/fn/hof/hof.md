Rust 提供了高阶函数（Higher Order Function, HOF）。执行一个或多个函数来产生一个用处更大的函数。HOF 和惰性迭代器（lazy iterator）给 Rust 带来了函数式的风格（英文原文：HOFs 
and lazy iterators give Rust its functional flavor.）。

```rust,editable
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式方式（imperative approach）
    // 声明累加器变量
    let mut acc = 0;
    // 重复：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限（upper limit）则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就累加值
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式方式（functional approach）
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数的平方
             .take_while(|&n| n < upper) // 小于上限
             .filter(|&n| is_odd(n))     // 为奇数
             .fold(0, |sum, i| sum + i); // 最后其后
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
```

[Option][option] 和 [迭代器][iter] 实现了它们自己的高阶函数（英语原文：Option and Iterator implement their fair share of HOFs.）。

[option]: http://doc.rust-lang.org/core/option/enum.Option.html
[iter]: http://doc.rust-lang.org/core/iter/trait.Iterator.html
