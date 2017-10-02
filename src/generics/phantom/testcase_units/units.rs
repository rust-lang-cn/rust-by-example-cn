use std::ops::Add;
use std::marker::PhantomData;

/// 创建空枚举来定义单位类型。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` 是一个带有虚位类型参量的 `Unit`（单位），
/// 而且不是关于长类型（即 `f64`）的泛型。
///
/// `f64` 已经实现了 `Clone` 和 `Copy` trait.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// `Add` trait 定义了 `+` 运算符的行为。
impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    // add() 返回一个全新的包含总和的 `Length` 结构体。
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` 调用了针对 `f64` 类型的 `Add` 实现。
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // 指出 `one_foot` 拥有虚位类型参量 `Inch`。
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` 拥有虚位类型参量 `Mm`。
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` 调用了 `add()` 方法，该方法对 `Length<Unit>` 进行了实现。
    //
    // 由于 `Length` 了实现了 `Copy`，于是 `add()` 不会消费 `one_foot`
    // 和 `one_meter`，但会复制它们到 `self` 和 `rhs`。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法正常执行。
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // 无意义的操作将会失败，因为它们会导致：
    // 编译期报错：类型不匹配（Compile-time Error: type mismatch.）。
    //let one_feter = one_foot + one_meter;
}

