# for 循环和区间

`for in` 结构可以通过一个计数器来迭代。创建计算器的一个最简便的方法就是使用区间标记 `a..b`。这
会生成从 `a`（包含此值） 到 `b` （不含此值）增幅为 1 的一系列值。

让我们使用 `for` 代替 `while` 来写 FizzBuzz 程序。

```rust,editable
fn main() {
    // `n` 将从 1, 2, ..., 100 这些值依次获取进行每次循环
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

### 参见：

[Iterator][iter]

[iter]: ../trait/iter.html
