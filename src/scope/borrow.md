多数情况下，我们更希望访问数据本身而不需要取得它的所有权。为实现这点，Rust 使用了**借用**（*borrowing*）机制。对象可以通过引用（`&T`）来传递，从而取代通过值（`T`）来传递。

编译器静态地保证了（通过借用检查器）引用**总是**（*always*）指向有效的对象。也就是说，当存在引用指向一个对象时，该对象不能被销毁。

```rust,editable
// 此函数拥有 box 的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 此函数借用了一个 i32 类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
    let boxed_int = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了  box 的内容，但没有取得所有权，所以 box 的内容可以
    // 再次借用。
    borrow_i32(&boxed_i32);
    borrow_i32(&tacked_i32);

    {
        // 给出一个指向 box 里面所包含数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错！
        // 当 `boxed_i32` 里面的值被借用时，不能销毁 `boxed_int`。
        eat_box_i32(boxed_i32);
        // 改正 ^ 注释掉此行

        // `_ref_to_i32` 离开作用域且不再被借用。
    }

    // box 现在可以放弃 `eat_i32` 的所有权且可以销毁
    eat_i32(boxed_i32);
}
```
