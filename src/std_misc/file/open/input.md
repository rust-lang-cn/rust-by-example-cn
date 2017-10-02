`open` 静态方法能够以只读模式（read-only mode）打开一个文件。

`File` 拥有一个资源，文件描述符（file descriptor），以及在文件丢弃时管理好关闭文件的操作。（原文：A `File` owns a resource, the file descriptor and takes care of closing the file when it is `drop`ed.）

{open.rs}

下面是预期成功的输出：

```
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

（我们鼓励您在不同的失败条件下测试前面的例子：hello.txt 不存在，或 hello.txt 不可读，等等。）

