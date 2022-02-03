# 文档

用 `cargo doc` 构建文档到 `target/doc`。

用 `cargo test` 运行所有测试（包括文档测试），用 `cargo test --doc` 仅运行文档测试。

这些命令会恰当地按需调用 `rustdoc`（以及 `rustc`）。

## 文档注释

文档注释对于需要文档的大型项目来说非常重要。当运行 `rustdoc`，文档注释就会
编译成文档。它们使用 `///` 标记，并支持 [Markdown]。

```rust,editable,ignore
#![crate_name = "doc"]

/// 这里给出一个“人”的表示
pub struct Person {
    /// 一个人必须有名字（不管 Juliet 多讨厌她自己的名字）。
    name: String,
}

impl Person {
    /// 返回具有指定名字的一个人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串切片，代表人的名字
    ///
    /// # 示例
    ///
    /// ```
    /// // 在文档注释中，你可以书写代码块
    /// // 如果向 `rustdoc` 传递 --test 参数，它还会帮你测试注释文档中的代码！
    /// use doc::Person;
    /// let person = Person::new("name");
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

```shell
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```

## 文档属性

下面是一些使用 `rustdoc` 时最常使用的 `#[doc]` 属性的例子。

### `inline`

用于内联文档，而不是链接到单独的页面。

```rust,ignore
#[doc(inline)]
pub use bar::Bar;

/// bar 的文档
mod bar {
    /// Bar 的文档
    pub struct Bar;
}
```

### `no_inline`

用于防止链接到单独的页面或其他位置。

```rust,ignore
// 来自 libcore/prelude 的例子
#[doc(no_inline)]
pub use crate::mem::drop;
```

### `hidden`

使用此属性来告诉 `rustdoc` 不要包含此项到文档中：

```rust,editable,ignore
// 来自 futures-rs 库的例子
#[doc(hidden)]
pub use self::async_await::*;
```

对文档来说， `rustdoc` 被社区广泛采用。[标准库文档](https://doc.rust-lang.org/std/)也是用它生成的。

### 参见:

- [The Rust Book: Making Useful Documentation Comments][book]
- [The rustdoc Book][rustdoc-book]
- [The Reference: Doc comments][ref-comments]
- [RFC 1574: API Documentation Conventions][api-conv]
- [RFC 1946: Relative links to other items from doc comments (intra-rustdoc links)][intra-links]
- [Is there any documentation style guide for comments? (reddit)][reddit]

[markdown]: https://en.wikipedia.org/wiki/Markdown
[book]: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
[ref-comments]: https://doc.rust-lang.org/stable/reference/comments.html#doc-comments
[rustdoc-book]: https://doc.rust-lang.org/rustdoc/index.html
[api-conv]: https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text
[intra-links]: https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html
[reddit]: https://www.reddit.com/r/rust/comments/ahb50s/is_there_any_documentation_style_guide_for/
