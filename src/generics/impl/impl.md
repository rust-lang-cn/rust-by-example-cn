和函数类似，实现（implementation）也需要关注保持泛型。（原文：Similar to functions, implementations require care to remain generic.）

```rust
struct S; // 具体类型 `S`
struct GenericVal<T>(T,); // 泛型类型 `GenericVal`

// GenericVal 的实现，此处我们显式地指定了类型参量：
impl GenericVal<f32> {} // 指定 `f32` 类型
impl GenericVal<S> {} // 指定为上面定义的 `S`

// `<T>` 必须在类型之前给出来以保持泛型。
// （原文：`<T>` Must precede the type to remain generic）
impl <T> GenericVal<T> {}
```

{impl.play}

### 参见：

[函数返回引用][fn], [`impl`][methods], 和 [`struct`][structs]


[fn]: ../scope/lifetime/fn.html
[methods]: ../fn/methods.html
[specialization_plans]: http://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: ../custom_types/structs.html
