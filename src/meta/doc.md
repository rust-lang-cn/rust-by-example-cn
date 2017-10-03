文档注释对于需要文档的大型项目来说非常重要。当运行 [Rustdoc][1]，这些注释就会编译成文档。它们使用 `///` 标记，并支持 [`Markdown`][2]。

```rust,editable,ignore,mdbook-runnable
#![crate_name = "doc"]

/// 这里给出一个人类
pub struct Person {
    /// 一个人必须有名字，不管 Juliet 多讨厌他/她。
    name: String,
}

impl Person {
    /// 返回给定名字的人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串 slice，代表人物的名称
    ///
    /// # 示例：
    ///
    /// ```
    /// // 可以在注释的特定标记内编写 Rust。
    /// // 如果可以通过 --- 测试传递给 Rustdoc，它将会帮你进行测试！
    /// let person = Person::new("name);
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
```

要运行测试，首先将代码构建为库，然后告诉 `rustdoc` 在哪里找到库，以便它可以将代码链接成各个文档测试程序：

```bash
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rs"
```

（当你在库 crate 上运行 `cargo test` 时，`Cargo` 将自动生成并运行正确的 `rustc` 和 `rustdoc` 命令。）

[1]: http://doc.rust-lang.org/book/documentation.html
[2]: https://en.wikipedia.org/wiki/Markdown
