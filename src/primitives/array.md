# 数组和 slice 类型

数组是一组包含相同数据类型 `T` 的组合，并存储在连续的内存区中。数组使用中括号 `[]` 来创建，
另外它们的大小在编译期间就已确定，数组的类型标记为 `[T; size]`（译注：T 为元素的类型，size
表示数组的大小）。

slice（中文有“切片”之意） 类型和数组类似，但 slice 类型的大小在编译期间是不确定的。相反，
slice 是一个双字对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的
长度。slice 可以用来借用数组的一部分。slice 的类型标记为 `&[T]`。

```rust,editable,ignore,mdbook-runnable
use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定大小的数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 索引从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在堆中分配
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动地借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的索引会引发 panic（中文意思是：惊恐、恐慌等意）
    println!("{}", xs[5]);
}
```
