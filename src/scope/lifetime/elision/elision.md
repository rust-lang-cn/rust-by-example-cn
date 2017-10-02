有些生命周期的模式太过普遍了，所以借用检查器将会隐式地添加它们来以减少字母输入和增强可读性。这种隐式添加生命周期的过程称为省略（elision）。在 Rust 使用省略仅仅是因为这些模式太普遍了。

下面代码展示了一些省略的例子。对于省略的详细描述，可以参考官方文档的 [生命周期省略][elision]。

{elision.play}

### 参见：

[省略][elision]

[elision]: http://doc.rust-lang.org/book/lifetimes.html#lifetime-elision
