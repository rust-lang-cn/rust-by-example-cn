引用和可变引用可以通过 `ref` 和 `ref mut` 获得：

```rust,editable
fn main() {
    // 先定义两个常规变量
    let value = 5;
    let mut mut_value = 6;

    // 为了解构成 `&5`（5 的引用），使用 `ref` 关键字。
    match value {
        // `println!` 可以处理常规变量和引用，所以不用关心变量是哪一种
        // 类型。`r` 将得到 `&i32` 类型。
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 同样地，为了获得一个可变引用 `&mut 6`，要使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 获得一个引用。在增加内容之前，要先得到解引用（Gotta
            // dereference it before we can add anything to it）。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
```

### 参见：

[借用 (`&`)][borrow]

[borrow]: ../../../borrow.html
