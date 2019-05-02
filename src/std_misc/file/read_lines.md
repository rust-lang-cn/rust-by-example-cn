# 读取行

方法 `lines()` 在文件的行上返回一个迭代器。

`File::open` 需要一个泛型 `AsRef<Path>`。这正是 `read_lines()` 期望的输入。

```rust,no_run
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // 在生成输出之前，文件主机必须存在于当前路径中
    if let Ok(lines) = read_lines("./hosts") {
        // 使用迭代器，返回一个（可选）字符串
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }      
        }   
    }
}

// 输出包裹在 Result 中以允许匹配错误，
// 将迭代器返回给文件行的读取器（Reader）。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

运行此程序将一行行将内容打印出来。

```bash
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

这个过程比在内存中创建 `String` 更有效，特别是处理更大的文件。
