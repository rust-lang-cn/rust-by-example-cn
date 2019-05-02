# 原始标志符

与许多编程语言一样，Rust 拥有“关键字”的概念。 这些标识符对语言有特定意义，所以不能在变量名、函数名和其他位置使用它们。 原始标识符允许你使用通常不允许的关键字。 当 Rust 引入新关键字时，使用旧版 Rust 的库拥有与新版本中引入的关键字同名的变量或函数，这一点就特别有用。

举个例子，使用 2015 版 Rust 编译的 crate `foo`，它导出一个名为 `try` 的函数。 此关键字（*try*）在 2018 版本的新功能中保留下来，因此如果没有原始标识符，我们将无法命名该功能。

```rust,ignore
extern crate foo;

fn main() {
    foo::try();
}
```

将得到如下错误：

```text
error: expected identifier, found keyword `try`
 --> src/main.rs:4:4
  |
4 | foo::try();
  |      ^^^ expected identifier, found keyword
```

使用原始标志符重写上述代码：

```rust,ignore
extern crate foo;

fn main() {
    foo::r#try();
}
```
