# 使用 `macro_rules!` 来创建宏

Rust 提供了一个强大的宏系统，可进行元编程（metaprogramming）。你已经在前面的章节中看到，宏看起来和函数很像，只不过名称末尾有一个感叹号 `!` 。宏并不产生函数调用，而是展开成源码，并和程序的其余部分一起被编译。Rust 又有一点和 C
以及其他语言都不同，那就是 Rust 的宏会展开为抽象语法树（AST，abstract syntax
tree），而不是像字符串预处理那样直接替换成代码，这样就不会产生无法预料的优先权错误。

宏是通过 `macro_rules!` 宏来创建的。

```rust,editable
// 这是一个简单的宏，名为 `say_hello`。
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => {
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    };
}

fn main() {
    // 这个调用将会展开成 `println!("Hello!");`
    say_hello!()
}
```

为什么宏是有用的？

1. 不写重复代码（DRY，Don't repeat yourself.）。很多时候你需要在一些地方针对不同
   的类型实现类似的功能，这时常常可以使用宏来避免重复代码（稍后详述）。
2. 领域专用语言（DSL，domain-specific language）。宏允许你为特定的目的创造特定的
   语法（稍后详述）。
3. 可变接口（variadic interface）。有时你需要能够接受不定数目参数的接口，比如
   `println!`，根据格式化字符串的不同，它需要接受任意多的参数（稍后详述）。
