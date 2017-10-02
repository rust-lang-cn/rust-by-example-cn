`Path` 结构体代表了底层文件系统的文件路径。`Path` 分为两种：`posix::Path`，针对类 UNIX 系统；以及 `windows::Path`，针对 Windows。预处理会导入适合特定平台的 `Path` 变量（原文：The prelude exports the appropriate platform-specific `Path` variant.）。

`Path` 可从多种类型创建，几乎所有实现了 `BytesContainer` trait 的类型都可以，比如 string，并提供了几种方法从路径指向的文件/目录中获取信息。（原文：A `Path` can be created from almost any type that implements the `BytesContainer` trait, like a string, and provides several methods to get information from the file/directory the path points to.）

注意 `Path` 在内部并没有表示为一个 UTF-8 字符串，而是存储为若干字节（`Vec<u8>`）的 vector。因此，将 Path 转化成 &str 并非零开销（free），且可能失败（返回一个 Option）。

```rust,editable
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
```

要记得查阅其他的 `Path` 方法（`posix::Path` 或 `windows::Path`），还有 `FileStat` 结构体。
