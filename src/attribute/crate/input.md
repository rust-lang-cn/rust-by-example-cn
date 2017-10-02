`crate_type` 属性可以告知编译器  crate 是一个二进制的可执行文件还是一个库（甚至是哪种类型的库），`crate_time` 属性可以设定 crate 的名称。

{lib.rs}

当用到 `crate_type` 属性时，就不再需要给 `rustc` 命令加上 `--crate-type` 标记。

```
$ rustc lib.rs
$ ls lib*
library.rlib
```
