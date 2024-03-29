# 文件输入输出（I/O）

`File` 结构体表示一个被打开的文件（它包裹了一个文件描述符），并赋予了对所表示的文件的读写能力。

由于在进行文件 I/O（输入/输出）操作时可能出现各种错误，因此 `File` 的所有方法都返回 `io::Result<T>` 类型，它是 `Result<T, io::Error>` 的别名。

这使得所有 I/O 操作的失败都变成**显式的**。借助这点，程序员可以看到所有的失败路径，并被鼓励主动地处理这些情形。
