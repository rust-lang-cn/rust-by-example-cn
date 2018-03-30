# 字面量

对数值字面量，只要把类型作为后缀加上去，就完成了类型说明。比如指定字面量 `42` 的
类型是 `i32`，只需要写 `42i32`。

无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用
 `i32`，对浮点数使用 `f64`。

```rust,editable
fn main() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```

上面的代码使用了一些还没有讨论过的概念。心急的读者可以看看下面的简短解释：

* `fun(&foo)` 用**传引用**（pass by reference）的方式把变量传给函数，而非
  传值（pass by value，写法是 `fun(foo)`）。更多细节请看[借用][borrow]。
* `std::mem::size_of_val` 是一个函数，这里使用其**完整路径**（full path）调用。代
  码可以分成一些叫做**模块**（module）的逻辑单元。在本例中，`size_of_val` 函数是
  在 `mem` 模块中定义的，而 `mem` 模块又是在 `std` **crate** 中定义的。更多细节
  请看[模块][mod]和[crate][crate].

[borrow]: scope/borrow.html
[mod]: mod.html
[crate]: crates.html
