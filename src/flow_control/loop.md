# loop 循环

Rust 提供了 `loop` 关键字来实现一个无限循环。

可以使用 `break` 语句在任何时候退出一个循环，另外可用 `continue` 跳过循环体的剩
余部分并开始下一轮循环。

```rust,editable
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}
```
