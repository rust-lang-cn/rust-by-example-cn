# 多重约束

多重约束（multiple bounds）可以用 `+` 连接。和平常一样，类型之间使用 `,` 隔开。

```rust,editable
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // 试一试 ^ 将此行注释去掉。

    compare_types(&array, &vec);
}
```

### 参见：

[`std::fmt`][fmt] 和 [`trait`][traits]

[fmt]: ../hello/print.md
[traits]: ../trait.md