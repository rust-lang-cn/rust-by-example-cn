在运用泛型时，类型参量常常必须使用 trait 作为**限定**（bound）来明确规定一个类型实现了哪些功能。例如下面的例子用到了 `Display` trait 来打印，所以它要求 `T` 由 `Display` 限定，也就是说 `T` **必须**实现 `Display`。

```rust
// 定义一个函数 `printer`，接受一个泛型类型 `T`，其中 `T` 必须
// 实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

限定限制了泛型为符合限定的类型。即：

```rust
struct S<T: Display>(T);

// 报错！`Vec<T>` 未实现 `Display`。
// 此特例化将失败。
let s = S(vec![1]);
```

限定的另一个作用是泛型实例允许访问在指定在限定中的 trait 的方法。例如：

{bounds.play}

额外补充内容，某些情况下为了提高代码的表现力，[`where`][where] 从句也可以在限定上使用。

### 参见：

[`std::fmt`][fmt], [`struct`][structs], 和 [`trait`][traits]

[fmt]: ../hello/print.html
[methods]: ../fn/methods.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
[where]: ../generics/where.html
