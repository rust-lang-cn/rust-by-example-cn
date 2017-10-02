条件编译可能通过两种不同的操作：

* `cfg` 属性：在属性位置中使用 `#[cfg(...)]`
* `cfg!` 宏：在布尔表达式中使用 `cfg!(...)`

两种形式使用参数的语法都相同。

{cfg.play}

### 参见：

[引用][ref], [`cfg!`][cfg], 和 [宏][macros].

[cfg]: http://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ../macros.html
[ref]: http://doc.rust-lang.org/reference.html#conditional-compilation
