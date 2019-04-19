# 约束

在使用泛型时，类型参数常常必须使用 trait 作为**约束**（bound）来明确规定
类型应实现哪些功能。例如下面的例子用到了 `Display` trait 来打印，所以它用
`Display` 来约束 `T`，也就是说 `T` **必须**实现 `Display`。

```rust,ignore
// 定义一个函数 `printer`，接受一个类型为泛型 `T` 的参数，
// 其中 `T` 必须实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

约束把泛型类型限制为符合约束的类型。请看：

```rust,ignore
struct S<T: Display>(T);

// 报错！`Vec<T>` 未实现 `Display`。此次泛型具体化失败。
let s = S(vec![1]);
```

约束的另一个作用是泛型的实例可以访问作为约束的 trait 的方法。例如：

```rust,editalbe
// 这个 trait 用来实现打印标记：`{:?}`。
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
// 都可以让下面函数正常工作。
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`。任意符合该约束的泛型的实例
// 都可访问 `HasArea` 的 `area` 函数
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ 试一试：取消上述语句的注释。
    // | 报错：未实现 `Debug` 或 `HasArea`。
}
```

多说一句，某些情况下也可使用 [`where`][where] 分句来形成约束，这拥有更好的表现力。

### 参见：

[`std::fmt`][fmt], [`struct`][structs], 和 [`trait`][traits]

[fmt]: ./hello/print.html
[methods]: ./fn/methods.html
[structs]: ./custom_types/structs.html
[traits]: ./trait.html
[where]: ./generics/where.html
