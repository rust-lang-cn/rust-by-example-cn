`Iterator::find` 是一个函数，在处理一个迭代器（iterator）时，将返回第一个满足条件的元素作为一个 `Option` 类型。它的原型如下：

```rust
pub trait Iterator {
	// 迭代相关的类型。
    type Item;

	// `find` 接受 `&mut self` 作为调用者可能被借用和修改，但不会消费掉。
	// （原文：`find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.）
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
		// `FnMut` 表示任意捕获变量很可能都被修改，而非消费。
		// `&Self::Item` 表明了通过引用接受闭包类型的参数。
		// （原文：`FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.）
        P: FnMut(&Self::Item) -> bool {}
}
```

{iter_find.play}

### 参见：

[`std::iter::Iterator::find`][find]

[find]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
