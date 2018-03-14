# 属性

属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：

<!-- TODO: Link these to their respective examples -->
* [条件编译代码][cfg]
* [设置 crate 名称、版本和类型（二进制文件或库）][crate]
* 禁用 [lint][lint] （警告）
* 启用编译器的特性（宏、全局导入（glob import）等）
* 链接到一个非 Rust 语言的库
* 标记函数作为单元测试
* 标记函数作为基准测试的某个部分

当属性作用于整个 crate 时，它们的语法为 `#![crate_attribute]`，当它们用于模块
或项时，语法为 `#[item_attribute]`（注意少了感叹号 `!`）。

属性可以接受参数，有不同的语法形式：

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

属性可以多个值，它们可以分开到多行中：

```rust,ignore
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
```

[cfg]: ./attribute/cfg.html
[crate]: ./attribute/crate.html
[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
