# `crate`

`crate_type` 属性可以告知编译器  crate 是一个二进制的可执行文件还是一个库（甚至是哪种类型的库），`crate_time` 属性可以设定 crate 的名称。

```rust,editable
// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

当用到 `crate_type` 属性时，就不再需要给 `rustc` 命令加上 `--crate-type` 标记。

```bash
$ rustc lib.rs
$ ls lib*
library.rlib
```
