就如泛型类型能够被限定一样，生命周期（它们本身就是泛型）也可以使用限定。`:` 字符的意义在这里稍微有些不同，不过 `+` 是相同的。注意下面是怎么说明的：

1. `T: 'a`：在 `T` 中的**所有**引用都必须比生命周期 `'a` 活得更长。
2. `T: Trait + 'a`：`T` 类型必须实现 `Trait` trait，并且在 `T` 中的**所有**引用都必须比 `'a` 活得更长。

下面例子展示了上述语法的实际应用：

{bounds.play}

### 参见：

[泛型][generics], [泛型中的限定][bounds], 以及
[泛型中的多重限定][multibounds]

[generics]: ../../generics.html
[bounds]: ../../generics/bounds.html
[multibounds]: ../../generics/multi_bounds.html
