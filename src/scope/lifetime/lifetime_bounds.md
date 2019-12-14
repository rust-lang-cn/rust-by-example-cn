# 约束

就如泛型类型能够被约束一样，生命周期（它们本身就是泛型）也可以使用约束。`:` 字符
的意义在这里稍微有些不同，不过 `+` 是相同的。注意下面的说明：

1. `T: 'a`：在 `T` 中的**所有**引用都必须比生命周期 `'a` 活得更长。
2. `T: Trait + 'a`：`T` 类型必须实现 `Trait` trait，并且在 `T` 中的**所有**引用
   都必须比 `'a` 活得更长。

下面例子展示了上述语法的实际应用：

```rust,editable
use std::fmt::Debug; // 用于约束的 trait。

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` 包含一个指向指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期
// `'a`。`T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。另外
// `Ref` 的生命周期也不能超出 `'a`。

// 一个泛型函数，使用 `Debug` trait 来打印内容。
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的
// 所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长。
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```

### 参见：

[泛型][generics], [泛型中的约束][bounds], 以及
[泛型中的多重约束][multibounds]

[generics]: ../../generics.md
[bounds]: ../../generics/bounds.md
[multibounds]: ../../generics/multi_bounds.md