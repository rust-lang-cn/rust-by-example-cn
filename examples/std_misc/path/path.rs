use std::path::Path;

fn main() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new(".");

    // `display` 方法返回一个可显示（showable）的结构体
    let display = path.display();

    // `join` 使用操作系统的特定分隔符来合并路径，并返回新的路径
    let new_path = path.join("a").join("b");

    // 将路径转换成一个字符串 slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
