`Iterator::any` 是一个函数，在处理一个迭代器（iterator）时，当其中任一元素符合条件（predicate）时将返回 `true`，否则返回 `false`。它的原型如下：

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

```rust,editable
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 产出 `&i32`（原文：`iter()` for vecs yields
    // `&i32`）。解构成 `i32` 类型。
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 产出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组（array）的 `iter()` 产出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常产出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
```

### 参见：

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
