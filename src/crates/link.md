# `extern crate`

要把上一节创建的库链接到一个 crate，必须使用 `extern crate` 声明。这不仅会
链接库，还会用一个与库名相同的模块来存放库里面的所有物件。于模块的可见性规则也
适用于库。

```rust,ignore
// 链接到 `rary` 库，导入其中的物件
extern crate rary;

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}
```

```bash
# library.rlib 是已编译好的库的路径，这里假设它在同一目录下：
$ rustc executable.rs --extern rary=library.rlib && ./executable
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```
