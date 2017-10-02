这是传统的 Hello World 程序的源码。

```rust,editable
// 这是注释内容，将会被编译器忽略掉
// 可以单击前面的按钮 "Run" 来测试这段代码 ->
// 若想用键盘操作，可以使用快捷键"Ctrl + Enter"来运行

// 这段代码支持编辑，你可以自由地改进代码！
// 通过单击 "Reset" 按钮可以使代码恢复到初始状态 ->

// 这是主函数
fn main() {
    // 调用已编译成的可执行文件时，在这里面的语句将会运行

    // 将文本打印到控制台
    println!("Hello World!");
}
```

`println!` 是一个 [**宏**][macros]（macros），可以将文本输出到控制台（console）。

使用 Rust 的编译器 `rustc` 可以将源程序生成可执行文件：

```bash
$ rustc hello.rs
```

`rustc` 编译后将得到可执行文件 `hello`。

```bash
$ ./hello
Hello World!
```

### 动手试一试

单击上面的 'Run' 按钮并观察输出结果。然后增加一行代码，再一次使用宏 `println!` 得到下面结果：

```text
Hello World!
I'm a Rustacean!
```

[macros]: ./macros.html
