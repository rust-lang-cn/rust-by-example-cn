# 作为输出参数

闭包作为输入参数是可能的，所以返回闭包作为输出参数（output parameter）也应该是
可能的。然而返回闭包类型会有问题，因为目前 Rust 只支持返回具体（非泛型）的
类型。按照定义，匿名的闭包的类型是未知的，所以想要返回一个闭包只有使它变成具体的
类型。通过 box 操作可以实现这点。

返回值的合法 trait 和前面的略有不同：

* `Fn`：和前面的一样
* `FnMut`：和前面的一样
* `FnOnce`：不平常的事情正是发生在这里。总之现在你需要返回 [`FnBox`][fnbox]
   类型，目前该类型还是不稳定的。这个情况估计将来会改进。

除此之外，还必须使用 `move` 关键字，它表明所有的捕获都是通过值进行的。这是必须
的，因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。

```rust,editable
fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
```

### 参见：

[Boxing][box], [`Fn`][fn], [`FnMut`][fnmut], 和 [泛型][generics].

[box]: ./std/box.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnbox]: http://doc.rust-lang.org/std/boxed/trait.FnBox.html 
[generics]: ./generics.html
