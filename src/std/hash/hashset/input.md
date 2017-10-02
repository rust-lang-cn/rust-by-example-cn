考虑 `HashSet` 作为一个 `HashMap`，在此处我们只关心键（`HashSet<T>` 实际上只是一个包围 `HashMap<T, ()>` 的装包（wrapper））。（原文：Consider a `HashSet` as a `HashMap` where we just care about the keys (`HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`).）

“关键点是什么呢？”你可能会这样问。“我可以将键只存储到一个 `Vec` 中。”
 
`HashSet` 的独特之处在于，它保证了不会拥有重复的元素。这是任何集合组合遵循的规定。`HashSet` 只是一个实现。（参见：[`BTreeSet`][treeset]）

如果插入的值已经存在于 `HashSet` 中（也就是，新值等于已存在的值，并且拥有相同的散列值），那么新值将会替换旧的值。

对于从来不多次保存同一事物，以及判断是否已经得到某个事物的情况，这是相当棒的。（原文：This is great for when you never want more than one of something, or when you want to know if you've already got something.）

不过集合（set）可以做更多的事。

集合拥有 4 种基本操作（下面的调用全部都返回一个迭代器）：

* `union`（并集）：获得两个集合中的所有元素（不含重复值）。

* `difference`（差集）：获取落在第一个集合而不在第二集合的所有元素。

* `intersection`（交集）：获取同时属于两个集合的所有元素。

* `symmetric_difference`（对称差）：获取所有只属于其中一个元素的集合，但不同属于两个集合的所有元素。

在下面的例子中尝试使用这些操作。

{hashset.play}

（例子修改自[文档][hash-set]。）

[treeset]: http://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[hash-set]: http://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference
