use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// 设置（setup）使它工作。才尽两个含有一些信息的文件。
fn setup() -> std::io::Result<()> {
    let mut a = try!(File::create("a"));
    try!(a.write_all(b"grape"));

    let mut b = try!(File::create("b"));
    b.write_all(b"fruit")
}

// 从各个文件中获取数据，将数据保存到 `Result`。
fn get_data(path: &str) -> Result<String> {
    // `try` 将值解包（unwrap the value）或返回错误。
    let mut file = try!(File::open(path)
        // 错误依然必须转成字符串。
        .map_err(|err| err.to_string())
    );
    let mut contents = String::new();

    // 读数据到 `contents`。
    try!(file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())
    );

    Ok(contents)
}

// 将两个文件的内容连接到一起，形成一个新的 `Result`。
fn concat(a: &str, b: &str) -> Result<String> {
    let (data_a, data_b) = (try!(get_data(a)), try!(get_data(b)));

    Ok(data_a + &data_b)
}

fn main() {
    // 忽略这个结果。
    setup().unwrap();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}
