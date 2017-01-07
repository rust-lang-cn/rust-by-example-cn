// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 限定的装包函数（wrapper function）。
fn function() {
    println!("I'm a function!")
}

fn main() {
    // 定义一个满足 `Fn` 限定的闭包。
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(function);
}
