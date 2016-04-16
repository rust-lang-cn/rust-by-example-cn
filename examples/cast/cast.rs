// 消除会溢出的类型转换的所有警告。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // 报错！不能隐式转换类型
    let integer: u8 = decimal;
    // 改正 ^ 注释掉此行

    // 显式转换类型
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当将任意整数值转成无符号类型（unsigned 类型） T 时，
    // 将会加上或减去 std::T::MAX + 1，直到值符合新的类型

    // 1000 原本就符合 u16 类型
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 在计算机底层会截取数字的低8位（the least significant bit，LSB），而高位（the most
    // significant bit，MSB）数字会被抛掉（译注：此操作是按二进数存储的数字位进行）。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数来说，上面的类型转换操作和取模效果一样
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当将整数值转成有符号类型（signed 类型）时，一样要先将数值转成相应的无符号类型（译注：
    // 如 i32 和 u32 对应，i16 和 u16对应），然后再求此值的补码（two's complement）

    // 除非值本来就已经符合所要转的类型。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128，再求数字128的8位二进制补码得到：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复上面的例子
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // 232 的补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);


}
