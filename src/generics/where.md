# where 从句

限定也可以使用 `where` 从句来表达，这样可以让限定写在 `{` 紧邻的前面，而不需写在类型第一次提到的位置上。另外 `where` 从句可以用于任意类型的限定，而不局限于类型参量。

`where` 在一些情况下有很用：

* 当分开指定泛型类型和限定时更清晰情况：

```rust,ignore
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// 使用 `where` 从句来表达限定
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

* 当使用 `where` 从句比正常语法更富表现力的情况。要是没有 `where` 从句的话，例子中的 `impl` 就不能直接表达出来：

```rust,editable
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`
// 或使用另一种间接的方法。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们要将 `Option<T>: Debug` 作为限定，因为那是要打印的内容。
    // 不这样做的话，很可能就用到错误的限定。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
```

### 参见：

[RFC][where], [`struct`][struct], 和 [`trait`][trait]

[struct]: ../custom_types/structs.html
[trait]: ../trait.html
[where]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md
