use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// 设置（setup）使它工作。创建两个含有一些信息的文件。
// 忽略掉返回值，因为我们在这里并不关心这点。
fn setup() {
    File::create("a")
        .and_then(|mut file| file.write_all(b"grape"))
        .unwrap();

    File::create("b")
        .and_then(|mut file| file.write_all(b"fruit"))
        .unwrap();
}

// 从各个文件中获取数据，将数据保存到 `Result`。
// （原文：Get the data from each file with the data stored in a `Result`.）
fn get_data(path: &str) -> Result<String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();

            // 读数据到 `contents`（原文：Read the data into `contents`.）。
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                // 忽略掉输出 `read_to_string` 返回，并返回 `contents`。
                //（原文：Ignore the output `read_to_string` returns and return `contents`.）
                .map(|_| contents)
        })
}

// 将两个文件的内容连接到一起，形成一个新的 `Result`。
fn concat(filename_a: &str, filename_b: &str) -> Result<String> {
    let (data_a, data_b) = (get_data(filename_a), get_data(filename_b));
    
    data_a.and_then(|a|
        // 当 `a` 和 `b` 都为 `Ok` 时，返回 `Ok`。否则返回先出错的那一个。
        data_b.and_then(|b| Ok(a + &b))
    )
}

fn main() {
    setup();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}
