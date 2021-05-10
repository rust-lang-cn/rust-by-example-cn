# 方法

方法的标注和函数类似：

```rust,editable
struct Owner(i32);

impl Owner {
    // 标注生命周期，就像独立的函数一样。
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner  = Owner(18);

    owner.add_one();
    owner.print();
}
```

> 译注：方法一般是不需要标明生命周期的，因为 `self` 的生命周期会赋给所有的输出
> 生命周期参数，详见 [TRPL](https://doc.rust-lang.org/book/second-edition/ch10-03-lifetime-syntax.html#lifetime-elision)。

### 参见：

[方法][methods]

[methods]: ../../fn/methods.md
