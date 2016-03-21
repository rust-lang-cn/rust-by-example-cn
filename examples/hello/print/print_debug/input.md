所有要用到`std::fmt`格式化的`traits`类型都需要转化成可打印的实现。`std`库这些类型能够自动实现。但所有其他类型都必须手动来实现。

`fmt::Debug` `trait` 使上面工作变得相当简单。所有类型都能推导（自动创建）`fmt::Debug`
的实现。但是 `fmt::Display` 需要手动来实现。

```rust
// 这种结构体不能使用`fmt::Display`或`fmt::Debug`来进行打印。
struct UnPrintable(i32);

// `derive`属性会自动创建实现，借助`fmt::Debug`使得这个`struct`能够打印。
#[derive(Debug)]
struct DebugPrintable(i32);
```

所有`std`库类型加上`{:?}`后也能够自动打印：

{debug.play}

所以 `fmt::Debug` 确实使这些内容可以打印，但是牺牲了美感。手动执行 `fmt::Display` 将能够弥补这些问题。

### 参考

[attributes][attributes], [`derive`][derive], [`std::fmt`][fmt] 和 [`struct`][structs]

[attributes]: http://doc.rust-lang.org/reference.html#attributes
[derive]: /trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[structs]: /custom_types/structs.html
