# 数组/切片

像元组一样，数组和切片也可以这样解构:

```rust,editable
fn main() {
    // 尝试改变数组中的值，或者将其做成切片!
    let array = [1, -2, 6];

    match array {
        // 将第二个和第三个元素绑定到各自的变量
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // 单个值可以用 ‘_’ 忽略
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // 也可以绑定一些而忽略其余的
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 下面的代码无法编译
        // [-1, second] => ...

        // 或者将它们存储在另一个数组/切片中(类型取决于所匹配的值的类型)。
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // 结合这些模式，我们可以绑定第一个和最后一个值，并将其余的值存储在单个数组中
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
```

### 参见：
[数组和切片](../../../primitives/array.md) 和 `@`符号用法[绑定](../binding.md) 