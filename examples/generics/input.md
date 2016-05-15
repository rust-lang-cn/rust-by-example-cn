**泛型**（generic）可以一般化类型和功能，以扩大适用范围（原文：*Generics* is the topic of generalizing types and functionalities to broader
cases.）。减少代码的重复是相当重要的，这可以通过多种方式实现，不过需要相当繁琐的语法（原文：This is extremely useful for reducing code duplication in many ways,
but can call for rather involving syntax.）。也就是说，用到泛型需要特别谨慎地指出哪种类型对于泛型类型来说是有效的（原文：Namely, being generic requires 
taking great care to specify over which types a generic type 
is actually considered valid.）。使用泛型最简单且最常见的方式就是用到类型参量（type parameter）。

类型参量指定为泛型要使用尖括号和 [CamelCase][camelcase]（驼峰式命名）：`<Aaa, Bbb, ...>` 。“泛型类型参量”一般用 `<T>` 来表示。在 Rust 中，“泛型”也表示可以接受一个或多个泛型类型参量 `<T>` 的任何内容。任何指定为泛型类型参量的类型都是泛型，其他的都是具体类型（非泛型）。

举个例子，定义一个名为 `foo` 的 **泛型函数**，可接受一个任意类型的参数 `T`：

```rust
fn foo<T>(T) { ... }
```

因为 `T` 被指定为一个使用 `<T>` 的泛型类型参量，所以在这里用到的 `(T)` 会变成泛型 。即使 `T` 在前面被定义为 `struct` 也是如此。

下面例子展示了一些操作中的语法：

{generics.play}

### 参见：

[`struct`][structs]

[structs]: ./custom_types/structs.html
[camelcase]: https://en.wikipedia.org/wiki/CamelCase
