# 所有权和移动

因为变量要负责释放它们拥有的资源，所以**资源只能拥有一个所有者**。这也防止了
资源的重复释放。注意并非所有变量都拥有资源（例如[引用][references]）。

在进行赋值（`let x = y`）或通过值来传递函数参数（`foo(x)`）`的时候，资源
的**所有权**（ownership）会发生转移。按照 Rust 的说法，这被称为资源
的**移动**（move）。

在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针（dangling
pointer）的产生。

```rust,editable
// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` 被销毁且内存得到释放
}

fn main() {
    // 栈分配的整型
    let x = 5u32;

    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *移动* `a` 到 `b`
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。
    
    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    //println!("a contains: {}", a);
    // 试一试 ^ 去掉此行注释

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
    // 报错！和前面出错的原因一样。
    //println!("b contains: {}", b);
    // 试一试 ^ 去掉此行注释
}
```

[references]: ./flow_control/match/destructuring/destructure_pointers.html
