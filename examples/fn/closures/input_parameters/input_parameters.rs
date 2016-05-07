// 将闭包作为参数并调用它的函数。
fn apply<F>(f: F) where
    // 闭包没有输入值和返回值。
    F: FnOnce() {
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f()
}

// 使用闭包并返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32 where
// 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;
    
    let greeting = "hello";
    // 不可复制的类型。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用方式的 `greeting` 和
    // 通过值方式的 `farewell`。
    let diary = || {
        // `greeting` 使用引用方式：需要 `Fn`。
        println!("I said {}.", greeting);

        // 改变迫使 `farewell` 变成了通过可变引用来捕获。
        // （原文：Mutation forces `farewell` to be
        // captured by mutable reference.）
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 将 `farewell` 强制转成通过值来捕获。
        // （原文：Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.）
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 调用处理闭包的函数（原文：Call the function
    // which applies the closure）。
    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
