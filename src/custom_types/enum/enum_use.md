# 使用 use

使用 `use` 声明的话，就可以不写出名称的完整路径了：

```rust,editable
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
```

### 参见：

[`match`][match] 和 [`use`][use]

[use]: ../../mod/use.md
[match]: ../../flow_control/match.md
