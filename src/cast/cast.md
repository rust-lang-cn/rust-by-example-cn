Rust 在基本类型之间没有提供隐式类型转换（强制类型转换）（implicit type conversion，coercion
）。不过使用 `as` 关键字进行显式类型转换（explict type conversion，casting）。

一般来说，Rust 的整型类型的转换规则遵循 C 语言的惯例，除了那些在 C 语言是未定义行为的情况。在
Rust 中，所有的整型类型转换的行为都得到了很好的定义。

{cast.play}
