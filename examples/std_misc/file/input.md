`File` 结构体表示一个被打开的文件（它装包了一个文件描述符），并赋予了针对底层文件的读和/或写能力。（原文：The `File` struct represents a file that has been opened (it wraps a file descriptor), and gives read and/or write access to the underlying file.）

由于在进行文件 I/O（输入/输出）操作时很多情形都可能出现错误，因此所有的 `File` 方法都返回 `io::Result<T>` 类型，这是 `Result<T, io::Error>` 的别名。

这使得所有 I/O 操作的失败都变成**显式**内容。借助这点，程序员可以看到所有的失败路径，并鼓励主动去处理这些情形。

