# 输入函数

既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个
接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数。

```rust,editable
// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(function);
}
```

多说一句，`Fn`、`FnMut` 和 `FnOnce` 这些 `trait` 明确了闭包如何从周围的作用域
中捕获变量。

### 参见：

[`Fn`][fn], [`FnMut`][fn_mut], 和 [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
