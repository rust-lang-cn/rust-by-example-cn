// 隐藏未使用代码警告的属性。
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
// 没有明确赋值区分的 enum（计数从0开始）
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
// 明确给定值的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enum` 可以转成整形。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
