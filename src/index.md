# 通过例子学 Rust

> 中文翻译注（Chinese translation of the [Rust By Example][website]）：
>
> - 👉 查看更多 <a href="https://rustwiki.org/" style="color:#97ca00;font-weight:bold;">Rust 官方文档中英文双语教程</a>，包括双语版[《Rust 程序设计语言》][book-cn]（出版书名为《Rust 权威指南》），本站还提供了 [Rust 标准库中文版][std]。
> - 《通过例子学 Rust》(Rust By Example 中文版)翻译自 [Rust By Example][website]，中文版最后更新时间：2022-1-21。查看此书的 [Github 翻译项目和源码][home]。
> - <a href="https://rustwiki.org/en/rust-by-example" style="color:red;">本站支持文档中英文切换</a>，点击页面右上角语言图标可切换到相同章节的英文页面，**英文版每天都会自动同步一次官方的最新版本**。
> - 若发现本页表达错误或帮助我们改进翻译，可点击右上角的编辑按钮打开本页对应源码文件进行编辑和修改，Rust 中文资源的开源组织发展离不开大家，感谢您的支持和帮助！

[Rust][rust] 是一门注重安全（safety）、速度（speed）和并发（concurrency）的现代系统编程语言。Rust 通过内存安全来实现以上目标，但不使用垃圾回收机制（garbage collection, GC)。

《通过例子学 Rust》（Rust By Example, RBE）内容由一系列可运行的实例组成，通过这些例子阐明了各种 Rust 的概念和基本库。想获取这些例子外的更多内容，不要忘了[安装 Rust 到本地][install]并查阅[官方标准库文档][std]。另外为了满足您的好奇心，您还可以[查阅本网站的源代码][home]。

现在让我们开始学习吧！

- [Hello World](hello.html) - 从经典的 “Hello World” 程序开始学习。

- [原生类型](primitives.html) - 学习有符号整型，无符号整型和其他原生类型。

- [自定义类型](custom_types.html) - 结构体 `struct` 和 枚举 `enum`。

- [变量绑定](variable_bindings.html) - 变量绑定，作用域，变量遮蔽。

- [类型系统](types.html) - 学习改变和定义类型。

- [类型转换](conversion.html)

- [表达式](expression.html)

- [流程控制](flow_control.html) - `if`/`else`，`for`，以及其他流程控制有关内容。

- [函数](fn.html) - 学习方法、闭包和高阶函数。

- [模块](mod.html) - 使用模块来组织代码。

- [Cartes](crates.html) - crate 是 Rust 中的编译单元。学习创建一个库。

- [Cargo](cargo.html) - 学习官方的 Rust 包管理工具的一些基本功能。

- [属性](attribute.html) - 属性是应用于某些模块、crate 或项的元数据（metadata）。

- [泛型](generics.html) - 学习编写能够适用于多种类型参数的函数或数据类型。

- [作用域规则](scope.html) - 作用域在所有权（ownership）、借用（borrowing）和生命周期（lifetime）中起着重要作用。

- [特性 trait](trait.html) - trait 是对未知类型（`Self`）定义的方法集。

- [宏](macros.html)

- [错误处理](error.html) - 学习 Rust 语言处理失败的方式。

- [标准库类型](std.html) - 学习 `std` 标准库提供的一些自定义类型。

- [标准库更多介绍](std_misc.html) - 更多关于文件处理、线程的自定义类型。

- [测试](testing.html) - Rust 语言的各种测试手段。

- [不安全操作](unsafe.html)

- [兼容性](compatibility.html)

- [补充](meta.html) - 文档和基准测试

[website]: https://doc.rust-lang.org/rust-by-example/
[book-cn]: https://rustwiki.org/zh-CN/book/
[rust]: https://www.rust-lang.org/
[install]: https://www.rust-lang.org/tools/install
[std]: https://rustwiki.org/zh-CN/std/
[home]: https://github.com/rust-lang-cn/rust-by-example-cn
