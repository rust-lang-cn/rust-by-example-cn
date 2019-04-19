# 运算符重载

在 Rust 中，很多运算符可以通过 trait 来重载。也就是说，这些运算符可以根据它们的
输入参数来完成不同的任务。这之所以可行，是因为运算符就是方法调用的语法糖。例
如，`a + b` 中的 `+` 运算符会调用 `add` 方法（也就是 `a.add(b)`）。这个 `add` 方
法是 `Add` trait 的一部分。因此，`+` 运算符可以被任何 `Add` trait 的实现者使用。

会重载运算符的 `trait`（比如 `Add` 这种）可以在[这里][ops]查看。

```rust,editable
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait 用来指明 `+` 的功能，这里我们实现 `Add<Bar>`，它是用于
// 把对象和 `Bar` 类型的右操作数（RHS）加起来的 `trait`。
// 下面的代码块实现了 `Foo + Bar = FooBar` 这样的运算。
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// 通过颠倒类型，我们实现了不服从交换律的加法。
// 这里我们实现 `Add<Foo>`，它是用于把对象和 `Foo` 类型的右操作数加起来的 trait。
// 下面的代码块实现了 `Bar + Foo = BarFoo` 这样的运算。
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
```

### 参见：

[Add][add], [语法索引][syntax]

[add]: http://doc.rust-lang.org/core/ops/trait.Add.html
[ops]: http://doc.rust-lang.org/core/ops/
[syntax]: https://doc.rust-lang.org/book/syntax-index.html
