# 动态数组 vector

vector 是可变大小的数组。和 slice（切片）类似，它们的大小在编译期不可预知，但他们可以随时扩大或缩小。一个 vector 使用 3 个词来表示：一个指向数据的指针，它的长度，还有它的容量。此容量表明了分配多少内存给这 vector。vector 只要小于该容量，就可以随意增长。当临界值就要达到时，vector 会重新分配一个更大的容量。

```rust,editable,ignore,mdbook-runnable
fn main() {
    // 迭代器可以收集到 vector
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` 宏可用来初始化一个 vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // 在 vector 的尾部插入一个新的元素
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // 报错！不可变 vector 不可增长
    collected_iterator.push(0);
    // 改正 ^ 将此行注释掉

    // `len` 方法获得一个 vector 的当前大小
    println!("Vector size: {}", xs.len());

    // 在中括号上加索引（索引从 0 开始）
    println!("Second element: {}", xs[1]);

    // `pop` 移除 vector 的最后一个元素并将它返回
    println!("Pop last element: {:?}", xs.pop());

    // 超出索引范围将抛出一个 panic
    println!("Fourth element: {}", xs[3]);
}
```

更多 `Vec` 方法可以在 [std::vec][vec] 模块中找到。

[vec]: http://doc.rust-lang.org/std/vec/
