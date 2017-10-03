# 使用 use

使用 `use` 声明，这样就不必手动加上作用域了：

```rust,editable
// 隐藏未使用代码警告的属性。
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
    // 明确地 `use` 各个名称使他们直接可用而不需要手动加上作用域。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // 等价于 `Status::Poor`。
    let status = Poor;
    // 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里少了作用域，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到这里没有作用域。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
```

### 参见：

[`match`][match] 和 [`use`][use]

[use]: ../../mod/use.html
[match]: ../../flow_control/match.html
