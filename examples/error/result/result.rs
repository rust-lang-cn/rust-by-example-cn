fn double_number(number_str: &str) -> i32 {
    // 将一个字符串解析成其他类型并不一定就会成功。
    // 所以 `parse()` 返回一个 `Result` 表明可能的失败。让我们尝试使用
    // `unwrap()` 把数字取出来。它会咬我们吗？
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}
