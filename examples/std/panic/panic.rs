// 再次实现整型的除法（/）
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // 除以一个 0 时会引发一个 panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// `main` 任务
fn main() {
    // 堆分配的整数
    let _x = Box::new(0i32);

    // 此操作将会引发一个任务失败
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` 在此处将被销毁
}
