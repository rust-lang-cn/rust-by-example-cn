元组是一个可以包含各种类型的组合。元组使用括号 `()` 来构成，每个元组的值都是 `(T1, T2, ...)`
类型标记的形式，其中 `T1`,`T2` 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以
拥有任意数量的值。

{tuples.play}

### 动手试一试

 1. *重新试一下*：在上面的例子中给 Matrix `结构体` 加上 `fmt::Display` trait，让你能够将
    打印调试格式 `{:?}` 切换成一般显示格式 `{}`，得到如下的输出：
```
( 1.1 1.2 )
( 2.1 2.2 )
```
    可以回顾之前学过的例子 [打印显示][print_display]。
 2. 以 `reverse` 函数作为样本，添加一个 `transpose` 函数，使它可以接受一个 Matrix 的参数，并
    返回一个两个元素元素交换后 Matrix。举个例子：
```
println!("Matrix:\n{}", matrix)
println!("Transpose:\n{}", transpose(matrix))
```
输出结果：
```
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

[print_display]: ../hello/print/print_display.html
