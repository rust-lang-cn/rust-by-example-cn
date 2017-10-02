// `F` 必须针对一个没有输入参数和返回值的闭包实现 `Fn`
// —— 确切地讲是 `print` 要求的类型。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获的 `x` 成为一个匿名类型并为它实现 `Fn`。
    // 将它存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}
