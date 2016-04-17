`type` 语句可以给一个已存在类型起一个新的名字。类型必须要有 `CamelCase`（驼峰方式）的名称，否则
编译器会产生一个警告。对规则为例外的是基本类型： `usize`，`f32`等等。

{alias.play}

别名的主要作用是减少按键，举个例子，`IoResult<T>` 类型是 `Result<T, IoError>` 类型的别名。

### 参见：

[属性](../attribute.html)
