# 嵌套循环和标签

在处理嵌套循环的时候可以 `break` 或 `continue` 外层循环。在这类情形中，循环必须用一些 `'label`（标签）来注明，并且标签必须传递给 `break`/`continue` 语句。

```rust,editable
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
```
