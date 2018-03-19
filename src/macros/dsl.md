# 领域专用语言（DSL）

DSL 是 Rust 的宏中集成的微型 “语言”。这种语言是完全合法的，因为宏系统会把它转换
成普通的 Rust 语法树，它只不过看起来像是另一种语言而已。这就允许你为一些特定功能
创造一套简洁直观的语法（当然是有限制的）。

比如说我想要定义一套小的计算器 API，可以传给它表达式，它会把结果打印到控制台上。

```rust,editable
macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // 强制类型为整型
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2 // 看到了吧，`eval` 可并不是 Rust 的关键字！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
```

输出:

```txt
1 + 2 = 3
(1 + 2) * (3 / 4) = 0
```

这个例子非常简单，但是已经有很多利用宏开发的复杂接口了，比如
[`lazy_static`](https://crates.io/crates/lazy_static) 和
[`clap`](https://crates.io/crates/clap)。
