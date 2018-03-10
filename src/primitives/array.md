# 数组和切片

数组（array）是一组拥有相同类型 `T` 的对象的集合，在内存中是连续存储的。数组使用
中括号 `[]` 来创建，另外它们的大小在编译时会被确定。数组的类型标记为 `[T; size]`
（译注：T 为元素的类型，size 表示数组的大小）。

切片（slice）类型和数组类似，但其大小在编译期间是不确定的。相反，切片是一个双字
对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度。这
个 “字” 的宽度和 usize 相同，由处理器架构决定，比如在 x86-64 平台上就是 64 位。
slice 可以用来借用数组的一部分。slice 的类型标记为 `&[T]`。

```rust,editable,ignore,mdbook-runnable
use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 脚标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的脚标会引发致命错误（panic）
    println!("{}", xs[5]);
}
```
