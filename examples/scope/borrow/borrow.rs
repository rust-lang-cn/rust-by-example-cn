// 此函数拥有 box 的所有权并销毁它
fn eat_box(boxed_int: Box<i32>) {
    println!("Destroying box that contains {}", boxed_int);
}

// 此函数借用了一个 i32 类型
fn borrow_box(borrowed_int: &i32) {
    println!("This int is: {}", borrowed_int);
}

fn main() {
    // 创建了一个装箱的整型（boxed integer）
    let boxed_int = Box::new(5);

    // 借用了  box 的内容，但没有取得所有权，所以 box 的内容可以
    // 再次借用。
    borrow_box(&boxed_int);
    borrow_box(&boxed_int);

    {
        // 给出一个指向 box 里面所包含数据的引用
        let _ref_to_int: &i32 = &boxed_int;

        // 报错！
        // 当 `boxed_int` 里面的值被借用时，不能销毁 `boxed_int`。
        eat_box(boxed_int);
        // 改正 ^ 注释掉此行

        // `_ref_to_int` 离开作用域且不再被借用。
    }

    // box 现在可以放弃 `eat_box` 的所有权且可以销毁
    eat_box(boxed_int);
}
