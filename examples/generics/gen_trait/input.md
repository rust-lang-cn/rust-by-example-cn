当然 `trait` 也可以是泛型。我们在这里定义了一个实现 `Drop` 的 `trait`，作为泛型方法来 `drop`（丢弃） 它本身和输入参数。

{trait.play}

### 参见：

[`Drop`][Drop], [`struct`][structs], 和 [`trait`][traits]

[Drop]: http://doc.rust-lang.org/std/ops/trait.Drop.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
