# 打开文件 `open`

`open` 静态方法能够以只读模式（read-only mode）打开一个文件。

`File` 拥有资源，即文件描述符（file descriptor），它会在自身被 `drop` 时关闭文件。

```rust,editable,ignore
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 创建指向所需的文件的路径
    let path = Path::new("hello.txt");
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
}
```

下面是所希望的成功的输出：

```bash
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

（我们鼓励您在不同的失败条件下测试前面的例子：hello.txt 不存在，或 hello.txt 不可读，等等。）
