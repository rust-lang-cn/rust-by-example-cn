这是一个经典的 Hello World 程序的源码。

{hello.play}

`println!` 是一个 [*宏*][macros]（macros），它可以将文本输出到控制台（console）。

使用 Rust 的编译器 `rustc` 可以将源程序生成可执行文件：

```
$ rustc hello.rs
```

`rustc` 编译后将得到可执行文件 `hello`。

```
$ ./hello
Hello World!
```

### 动手试一试

单击上面的 'Run' 按钮并观察输出结果。然后增加一行代码，再一次使用宏 `println!` 得到下面结果：
```
Hello World!
I'm a Rustacean!
```

[macros]: ./macros.html
