# 使用库

要将一个 crate 链接到上节新建的库，可以使用 `rustc` 的 `--extern` 选项。然后将所有的物件导入到与库名相同的模块下。此模块的操作通常与任何其他模块相同。

```rust,ignore
// extern crate rary; // 在 Rust 2015 版或更早版本需要这个导入语句

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}
```

```bash
# library.rlib 是已编译好的库的路径，这里假设它在同一目录下：
$ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```
