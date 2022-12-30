# 标准库类型

标准库提供了很多自定义类型，在**原生类型**基础上进行了大量扩充。这是部分自定义类型：

* 可增长的 `String`（字符串），如: `"hello world"`
* 可增长的向量（vector）: `[1, 2, 3]`
* 选项类型（optional types）: `Option<i32>`
* 错误处理类型（error handling types）: `Result<i32, i32>`
* 堆分配的指针（heap allocated pointers）: `Box<i32>`

### 参见：

[原生类型][primitives] 和 [标准库][std]

[primitives]: primitives.md
[std]: https://rustwiki.org/zh-CN/std/
