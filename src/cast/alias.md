# 别名

`type` 语句可以给一个已存在类型起一个新的名字。类型必须要有 `CamelCase`（驼峰方式）的名称，否则
编译器会产生一个警告。对规则为例外的是基本类型： `usize`，`f32`等等。

```rust,editable
// `NanoSecond` 是 `u64` 的新名字。
type NanoSecond = u64;
type Inch = u64;

// 使用一个属性来忽略警告。
#[allow(non_camel_case_types)]
type u64_t = u64;
// 试一试 ^ 试着删掉属性。

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型的别名*没有*提供任何额外的类型安全，因为别名*不是*新的类型
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
```

别名的主要作用是减少按键，举个例子，`IoResult<T>` 类型是 `Result<T, IoError>` 类型的别名。

### 参见：

[属性](../attribute.html)
