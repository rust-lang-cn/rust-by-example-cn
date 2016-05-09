// 定义一个函数，可以接受另一个函数作为参数并调用它。
fn call_function<F: Fn()>(f: F) {
    f()
}

// 定义一个简单的函数用作输入量。
fn print() {
    println!("I'm a function!")
}

fn main() {
    // 定义一个和上面的 `print()` 函数类似的闭包。
    let closure = || println!("I'm a closure!");
    
    call_function(closure);
    call_function(print);
}
