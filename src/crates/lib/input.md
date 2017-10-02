让我们创建一个库，然后看看如何把它链接到另一个 crate。

{rary.rs}

```
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

库的前缀为 “lib”，默认情况下它们跟随着 crate 文件命名（原文：by default they get named after their
crate file），但此默认名称可以使用 [`crate_name` 属性][crate-name] 覆盖。

[crate-name]: ../attribute/crate.html
