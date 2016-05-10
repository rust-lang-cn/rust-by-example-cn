`Iterator::any` 是一个函数，在处理一个迭代器（iterator）时，当所有元素都符合条件（predicate）时将返回 `true`，否则返回 `false`。它的原型如下：

```rust
pub trait Iterator {
	// 迭代相关的类型（原文：The type being iterated over）。
    type Item;

	// `any` 接受 `&mut self` 作为调用者可能被借用和修改，但不会消费掉。
	// （原文： `any` takes `&mut self` meaning the caller may be
	// borrowed and modified, but not consumed.）
    fn any<F>(&mut self, f: F) -> bool where
		// `FnMut` 表示任意捕获变量很可能都被修改，而非消费。
		// `Self::Item` 表明了通过值来接受闭包类型参数。
        // （原文：`FnMut` meaning any captured variable may at 
		// most be modified, not consumed. `Self::Item` states it
		// takes arguments to the closure by value.）
        F: FnMut(Self::Item) -> bool {}
}
```

{iter_any.play}

### 参见：

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
