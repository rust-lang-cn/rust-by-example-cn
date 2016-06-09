use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait 在这里用来指明 `+` 的功能，我们给出 `Add<Bar>`——关于
// 加法的 trait，带有一个 `Bar` 类型的右操作数（RHS）。下面代码块实现了这样的
// 运算： Foo + Bar = FooBar。
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// 通过反转类型，我们以实现非交换的加法作为结束。
// 这里我们给出 `Add<Foo>`——关于加法的 trait，带有一个 `Foo` 类型的右操作数。
// 这个代码块实现了这样的操作：Bar + Foo = BarFoo。
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
