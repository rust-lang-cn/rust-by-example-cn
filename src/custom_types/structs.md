# 结构体

结构体（structure，缩写成 struct）有 3 种类型，使用 `struct` 关键字来创建：

* 元组结构体（tuple struct），事实上就是具名元组而已。
* 经典的 [C 语言风格结构体][c_struct]（C struct）。
* 单元结构体（unit struct），不带字段，在泛型中很有用。

```rust,editable
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段（field）的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    
    // 以 Debug 方式打印结构体
    println!("{:?}", peter);
    
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let new_point = Point { x: 0.1, ..point };
    
    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", new_point.x, new_point.y);
    
    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

### 动手试一试：

1. 增加一个计算长方形面积的函数 `rect_area`（尝试使用嵌套的解构方式）。
2. 增加一个函数 `square`，接受的参数是一个 `Point` 和一个 `f32`，并返回一个
    `Rectangle`（长方形），其左下角的点等于 `Point` 参数，长和宽都等于 `f32`
    参数。

### 参见：

[`attributes`][attributes] 和 [解构][destructuring]

[attributes]: ../attribute.md
[c_struct]: http://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ../flow_control/match/destructuring.md
