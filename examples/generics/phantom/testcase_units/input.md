单位转换（unit conversion）的一个有效方法，可以通过包含虚位类型参量而实现的 `Add` 来检验（原文：A useful method of unit conversions can be examined by implementing `Add` with a phantom type parameter）。检验 `Add` `trait` 的代码如下：

```rust
// 这个结构得到加强：`Self + RHS = Qutput`，其中 RHS 要
// 是没有给出特定实现的话会默认成为 Self。
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output` 必须是 `T<U>` 类型，所以 `T<U> + T<U> = T<U>`。
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

完整实现：

{units.play}

### 参见：

[Borrowing (`&`)], [Bounds (`X: Y`)], [enum], [impl & self],
[Overloading], [ref], [Traits (`X for Y`)], 和 [TupleStructs].

[Borrowing (`&`)]: ../../scope/borrow.html
[Bounds (`X: Y`)]: ../../trait/bounds.html
[enum]: ../../custom_types/enum.html
[impl & self]: ../../fn/methods.html
[Overloading]: ../../trait/ops.html
[ref]: ../../scope/borrow/ref.html
[Traits (`X for Y`)]: ../../trait.html
[TupleStructs]: ../../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
