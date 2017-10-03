# 属性

属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：

<!-- TODO: Link these to their respective examples -->
* [代码的条件编译][cfg]
* [设置 crate 名称、版本和类型（二进制文件或库）][crate]
* 禁用 [lint][lint] （警告）
* 启用编译器的特性（宏、全局导入（glob import））等]
* 链接到一个非 Rust 语言的库
* 标记函数作为单元测试（unit test）
* 标记作为基准某个部分的函数

当属性用于一个完整的 crate 时，它们的语法为 `#![crate_attribute]`，当它们用于模块或项时，语法为 `#[item_attribute]`（注意少了感叹号 `!`）。

属性可以接受参数，有不同的语法形式：

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

[cfg]: ./attribute/cfg.html
[crate]: ./attribute/crate.html
[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
