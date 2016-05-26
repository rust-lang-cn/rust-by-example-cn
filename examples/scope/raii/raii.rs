// raii.rs
fn create_box() {
    // 在堆上分配一个整型数据
    let _box1 = Box::new(3i32);

    // `_box1` 在这里销毁，而且内存得到释放
}

fn main() {
    // 在堆上分配一个整型数据    
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // `_box3` 在这里销毁，而且内存得到释放        
    }

    // 创建很多 box，纯属娱乐。
    // 完全不需要手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` 在这里销毁，而且内存得到释放    
}
