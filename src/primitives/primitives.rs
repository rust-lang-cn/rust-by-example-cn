fn main() {
    // 变量可以声明类型。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规声明
    let an_integer   = 5i32; // 后缀声明

    // 否则自动推断类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // 可变类型 `i32`。

    // 报错！变量的类型不可改变。
    mutable = true;
}
