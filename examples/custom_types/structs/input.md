结构体（structure，缩写成 struct）有 3 种类型，使用 `struct` 关键字来创建：

* 元组结构体，总的来说是根据元组来命名。
* C 语言风格的结构体 [c_struct]。
* 单元结构体，不带字段，在泛型中很有用。

{structs.play}

### 动手试一试：

1. 增加一个计算长方形面积的函数 `rect_area`（尝试使用嵌套的解构方式）。
2. 增加一个函数 `square`，接受的参数是一个 `Point` 和一个 `f32`，并返回一个 `Rectangle`（长方形）的信息，包括左下角的点，以及长和宽的浮点数值。

### 参见：

[`attributes`][attributes] 和 [解构][destructuring]

[attributes]: ../attribute.html
[c_struct]: http://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ../flow_control/match/destructuring.html
