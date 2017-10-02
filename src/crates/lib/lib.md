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

```
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

库的前缀为 “lib”，默认情况下它们跟随着 crate 文件命名（原文：by default they get named after their
crate file），但此默认名称可以使用 [`crate_name` 属性][crate-name] 覆盖。

[crate-name]: ../attribute/crate.html
