在运用泛型时，类型参量常常必须使用 trait 作为**限定**（bound）来明确规定一个类型实现了哪些功能。例如下面的例子用到了 `Display` trait 来打印，所以它要求 `T` 由 `Display` 限定，也就是说 `T` **必须**实现 `Display`。

```rust,ignore
// 定义一个函数 `printer`，接受一个泛型类型 `T`，其中 `T` 必须
// 实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

限定限制了泛型为符合限定的类型。即：

```rust,ignore
struct S<T: Display>(T);

// 报错！`Vec<T>` 未实现 `Display`。
// 此特例化将失败。
let s = S(vec![1]);
```

限定的另一个作用是泛型实例允许访问在指定在限定中的 trait 的方法。例如：

```rust,editalbe
// 这个 trait 实现了打印标记：`{:?}`。
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

// 泛型 `T` 必须实现 `Debug`。不管什么类型，都可以正常工作。
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`。任意符合限定的函数都可以访问
// `HasArea` 的 `area` 函数。
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ 试一试：将上述语句的注释去掉。
    // | 报错：未实现 `Debug` 或 `HasArea`。
}
```

额外补充内容，某些情况下为了提高代码的表现力，[`where`][where] 从句也可以在限定上使用。

### 参见：

[`std::fmt`][fmt], [`struct`][structs], 和 [`trait`][traits]

[fmt]: ../hello/print.html
[methods]: ../fn/methods.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
[where]: ../generics/where.html
