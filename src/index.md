# 通过例子学 Rust
>《通过例子学 Rust》(Rust by Example 中文版)翻译自 [Rust by Example][website]，内容已全部翻译完成，中文版最后更新时间：2019-4-20。查看此书的 [Github 翻译项目][home]。(Chinese translation of the [Rust by Example][website].)

[Rust][rust] 是一门注重安全（safety）、速度（speed）和并发（concurrency）的现代系统编程语言。Rust 通过内存安全来实现上述目标，但不用垃圾回收机制（Garbage collection, GC)。

《通过例子学 Rust》（Rust by Example, RBE）内容由一系列可运行的实例组成，通过这些例子阐明了各种 Rust 的概念和基本库。想获取这些例子外的更多内容，不要忘了[安装 Rust 到本地][install]并查阅[官方文档][std]。另外为了满足您的好奇心，你可以[查阅本网站的源代码][home]。

现在让我们开始学习吧！

- [Hello World](hello.html) - 从经典的 “Hello World” 程序开始学习。

- [原生类型](primitives.html) - 学习有符号整型，无符号整型和其他原生类型。

- [自定义类型](custom_types.html) - 结构体 `struct` 和 枚举 `enum`。

- [变量绑定](variable_bindings.html) - 变量绑定，作用域，隐藏。

- [类型转换](cast.html) - 学习类型之间的显式转换。

- [表达式](expression.html)

- [流程控制](flow_control.html) - `if`/`else`，`for`，以及其他流程控制有关内容。

- [函数](fn.html) - 学习方法、闭包和高阶函数。

- [模块](mod.html) - 使用模块来组织代码。

- [`crate`](crates.html) - crate 是 Rust 中的编译单元。学习创建一个库。

- [属性](attribute.html) - 属性是应用于某些模块、crate 或项的元数据（metadata）。

- [泛型](generics.html) - 学习编写能够适用于多类型参数的函数或数据类型。

- [作用域规则](scope.html) - 作用域在所有权（ownership）、借用（borrowing）和生命周期（lifetime）中起着重要作用。

- [特性 trait](trait.html) - trait 是对未知类型定义的方法集：Self。

- [宏](macros.html)

- [错误处理](error.html) - 学习 Rust 语言处理失败的方式。

- [标准库类型](std.html) - 学习 `std` 标准库提供的一些自定义类型。

- [标准库更多介绍](std_misc.html) - 更多关于文件处理、线程的自定义类型。

- [补充](meta.html) - 文档和测试

- [不安全操作](unsafe.html)

[website]: http://rustbyexample.com
[rust]: http://www.rust-lang.org/
[install]: http://www.rust-lang.org/install.html
[std]: http://doc.rust-lang.org/std/
[home]: https://github.com/rust-lang-cn/rust-by-example-cn
