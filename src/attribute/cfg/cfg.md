条件编译可能通过两种不同的操作：

* `cfg` 属性：在属性位置中使用 `#[cfg(...)]`
* `cfg!` 宏：在布尔表达式中使用 `cfg!(...)`

两种形式使用参数的语法都相同。

```rust,editable
// 这个函数仅当操作系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当操作系统**不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

### 参见：

[引用][ref], [`cfg!`][cfg], 和 [宏][macros].

[cfg]: http://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ../macros.html
[ref]: http://doc.rust-lang.org/reference.html#conditional-compilation
