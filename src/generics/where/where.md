限定也可以使用 `where` 从句来表达，这样可以让限定写在 `{` 紧邻的前面，而不需写在类型第一次提到的位置上。另外 `where` 从句可以用于任意类型的限定，而不局限于类型参量。

`where` 在一些情况下有很用：

* 当分开指定泛型类型和限定时更清晰情况：

```rust
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// 使用 `where` 从句来表达限定
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

* 当使用 `where` 从句比正常语法更富表现力的情况。要是没有 `where` 从句的话，例子中的 `impl` 就不能直接表达出来：

{where.play}

### 参见：

[RFC][where], [`struct`][struct], 和 [`trait`][trait]

[struct]: ../custom_types/structs.html
[trait]: ../trait.html
[where]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md
