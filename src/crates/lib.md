# 库

让我们创建一个库，然后看看如何把它链接到另一个 crate。

```rust,editable
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

```bash
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

默认情况下，库会使用 crate 文件的名字，前面加上 “lib” 前缀，但这个默认名称可以使用 [`crate_name` 属性][crate-name] 覆盖。

[crate-name]: ../attribute/crate.md
