# 元组

元组是一个可以包含各种类型的值的组合。元组使用括号 `()` 来构成（construct），而
每个元组自身又是一个类型标记为 `(T1, T2, ...)` 的值，其中 `T1`、`T2` 是每个元素
的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多的值。

```rust,editable
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 在 “动手试一试” 的练习中要用到下面这个结构体。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。
    
    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

}
```

### 动手试一试

 1. **复习**：在上面的例子中给 Matrix `结构体` 加上 `fmt::Display` trait，这样当
    你从 Debug 格式化 `{:?}` 切换到 Display 格式化 `{}` 时，会得到如下的输出：
     ```
     ( 1.1 1.2 )
     ( 2.1 2.2 )
     ```
     可以回顾之前学过的[显示（display）][print_display]的例子。
 2. 以 `reverse` 函数作为样板，写一个 `transpose` 函数，它可以接受一个 Matrix
     作为参数，并返回一个右上 - 左下对角线上的两元素交换后的 Matrix。举个例子：
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

[print_display]: ./hello/print/print_display.html
