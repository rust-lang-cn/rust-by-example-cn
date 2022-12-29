# 散列集 HashSet

请把 `HashSet` 当成这样一个 `HashMap`：我们只关心其中的键而非值（`HashSet<T>`
实际上只是对 `HashMap<T, ()>` 的封装）。

你可能会问：“这有什么意义呢？我完全可以将键存储到一个 `Vec` 中呀。”
 
`HashSet` 的独特之处在于，它保证了不会出现重复的元素。这是任何 set 集合类型（set
collection）遵循的规定。`HashSet` 只是它的一个实现。（参见：[`BTreeSet`][treeset]）

如果插入的值已经存在于 `HashSet` 中（也就是，新值等于已存在的值，并且拥有相同的散列值），那么新值将会替换旧的值。

如果你不想要一样东西出现多于一次，或者你要判断一样东西是不是已经存在，这种做法就很有用了。

不过集合（set）可以做更多的事。

集合（set）拥有 4 种基本操作（下面的调用全部都返回一个迭代器）：

* `union`（并集）：获得两个集合中的所有元素（不含重复值）。

* `difference`（差集）：获取属于第一个集合而不属于第二集合的所有元素。

* `intersection`（交集）：获取同时属于两个集合的所有元素。

* `symmetric_difference`（对称差）：获取所有只属于其中一个集合，而不同时属于
  两个集合的所有元素。

在下面的例子中尝试使用这些操作。

```rust,editable,ignore,mdbook-runnable
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 如果值已经存在，那么 `HashSet::insert()` 返回 false。
    assert!(b.insert(4), "Value 4 is already in set B!");
    // 改正 ^ 将此行注释掉。

    b.insert(5);

    // 若一个集合（collection）的元素类型实现了 `Debug`，那么该集合也就实现了 `Debug`。
    // 这通常将元素打印成这样的格式 `[elem1, elem2, ...]
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // 乱序打印 [1, 2, 3, 4, 5]。
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // 这将会打印出 [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // 乱序打印 [2, 3, 4]。
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // 打印 [1, 5]
    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
```

（例子改编自[文档][hash-set]。）

[treeset]: https://rustwiki.org/zh-CN/std/collections/struct.BTreeSet.html
[hash-set]: https://rustwiki.org/zh-CN/std/collections/struct.HashSet.html#method.difference
