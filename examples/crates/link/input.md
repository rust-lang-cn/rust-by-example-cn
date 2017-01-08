链接一个 crate 到这个新库，必须使用 `extern crate` 声明。这不仅会链接库，还会导入与库名相同的模块里面的所有项。适用于模块的可见性规则也适用于库。

{executable.rs}

```
# library.rlib 是已编译好的库的路径，假设在这里它在同一目录下：
# （原文：Where library.rlib is the path to to the compiled library, 
# assumed that it's in the same directory here:）
$ rustc executable.rs --extern rary=library.rlib && ./executable
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```
