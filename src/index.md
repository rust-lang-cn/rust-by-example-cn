# 通过例子学 Rust
>《通过例子学 Rust》(Rust by Example 中文版)翻译自 [Rust by Example][website]，内容已全部翻译完成，中文版最后更新时间：2017-10-03。查看此书的 [Github 翻译项目][home]。(Chinese translation of the [Rust by Example][website].)

[Rust][rust] 是一门注重安全（safety）、速度（speed）和并发（concurrency）的现代系统编程语言。Rust 通过内存安全来实现上述目标，但不用垃圾回收机制（Garbage collection, GC)。

《通过例子学 Rust》（Rust by Example, RBE）内容由一系列可运行的实例组成，通过这些例子阐明了各种 Rust 的概念和基本库。想获取这些例子外的更多内容，不要忘了[安装 Rust 到本地][install]并查阅[官方文档][std]。另外为了满足您的好奇心，你可以[查阅本网站的源代码][home]。

现在让我们开始学习吧！

- [Hello World](hello.html) - 从经典的 “Hello World” 程序开始学习。

- [Primitives](primitives.html) - 学习有符号整型，无符号整型和其他原生类型。

- [Custom Types](custom_types.html) - 结构体 `struct` 和 枚举 `enum`。

- [Variable Bindings](variable_bindings.html) - 变量绑定，作用域，隐藏。

- [Casting](cast.html) - 学习类型之间的显式转换。

- [Expressions](expression.html)

- [Flow Control](flow_control.html) - `if`/`else`，`for`，以及其他流程控制有关内容。

- [Functions](fn.html) - 学习方法、闭包和高阶函数。

- [Modules](mod.html) - 使用模块来组织代码。

- [Crates](crates.html) - crate 是 Rust 中的编译单元。学习创建一个库。

- [Attributes](attribute.html) - 属性是应用于某些模块、crate 或项的元数据（metadata）。

- [Generics](generics.html) - 学习编写能够适用于多类型参数的函数或数据类型。

- [Scoping rules](scope.html) - 作用域在所有权（ownership）、借用（borrowing）和生命周期（lifetime）中起着重要作用。

- [Traits](trait.html) - trait 是对未知类型定义的方法集：Self。

- [Macros](macros.html)

- [Error handling](error.html) - 学习 Rust 语言处理失败的方式。

- [Std library types](std.html) - 学习 `std` 标准库提供的一些自定义类型。

- [Std misc](std_misc.html) - 更多关于文件处理、线程的自定义类型。

- [Meta](meta.html) - 文档和测试

- [Unsafe Operations](unsafe.html)

[website]: http://rustbyexample.com
[rust]: http://www.rust-lang.org/
[install]: http://www.rust-lang.org/install.html
[std]: http://doc.rust-lang.org/std/
[home]: https://github.com/rust-lang-cn/rust-by-example-cn
