# 文档

文档注释对于需要文档的大型项目来说非常重要。当运行 [Rustdoc][1]，文档注释就会
编译成文档。它们使用 `///` 标记，并支持 [`Markdown`][2]。

```rust,editable,ignore,mdbook-runnable
#![crate_name = "doc"]

/// 这里给出一个人类
pub struct Person {
    /// 一个人必须有名字，不管 Juliet 多讨厌他/她。
    name: String,
}

impl Person {
    /// 返回具有指定名字的一个人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串切片，代表人物的名称
    ///
    /// # 示例
    ///
    /// ```
    /// // 在文档注释中，你可以书写代码块
    /// // 如果向 Rustdoc 传递 --test 参数，它还会帮你测试注释文档中的代码！
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

要运行测试，首先将代码构建为库，然后告诉 `rustdoc` 在哪里找到库，这样它就可以
使每个文档中的程序链接到库：

```bash
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rs"
```

（当你对库 crate 上运行 `cargo test` 时，Cargo 将自动生成并运行正确的 `rustc`
和 `rustdoc` 命令。）

[1]: http://doc.rust-lang.org/book/documentation.html
[2]: https://en.wikipedia.org/wiki/Markdown
