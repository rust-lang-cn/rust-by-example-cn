# 闭包

Rust 中的闭包（closure），也叫做 lambda 表达式或者 lambda，是一类能够捕获周围
作用域中变量的函数。例如，一个可以捕获 x 变量的闭包如下：

```Rust
|val| val + x
```

它们的语法和能力使它们在临时（on the fly）使用时相当方便。调用一个闭包和调用一个
函数完全相同，不过调用闭包时，输入和返回类型两者都**可以**自动推导，而输入变量
名**必须**指明。

其他的特点包括：
* 声明时使用 `||` 替代 `()` 将输入参数括起来。
* 函数体定界符（`{}`）对于单个表达式是可选的，其他情况必须加上。
* 有能力捕获外部环境的变量。

```rust,editable
fn main() {
    // 通过闭包和函数分别实现自增。
    // 译注：下面这行是使用函数的实现
    fn  function            (i: i32) -> i32 { i + 1 }

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;
    
    // 译注：将闭包绑定到引用的说法可能不准。
    // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
    // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
    // `closure_xxx` 变量解引用。
    
    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());
}
```
