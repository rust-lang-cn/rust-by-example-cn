# 元组

元组是一个可以包含各种类型的组合。元组使用括号 `()` 来构成，每个元组的值都是 `(T1, T2, ...)`
类型标记的形式，其中 `T1`,`T2` 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以
拥有任意数量的值。

```rust,editable
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 来绑定元组的各个变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 在“动手试一试”的练习中要用到下面这个结构体。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的索引来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和括号包含的普通数据作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 解构元组，将值赋给创建的绑定变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

}
```

### 动手试一试

 1. **重新试一下**：在上面的例子中给 Matrix `结构体` 加上 `fmt::Display` trait，让你能够将
    打印调试格式 `{:?}` 切换成一般显示格式 `{}`，得到如下的输出：
```
( 1.1 1.2 )
( 2.1 2.2 )
```
    可以回顾之前学过的例子 [打印显示][print_display]。
 2. 以 `reverse` 函数作为样本，添加一个 `transpose` 函数，使它可以接受一个 Matrix 的参数，并
    返回一个两个元素元素交换后 Matrix。举个例子：
```
println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
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
