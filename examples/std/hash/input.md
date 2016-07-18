vector 通过整型索引来存储值，而 `HashMap` （散列表）通过键（key）来存储值。`HashMap` 的键可以是布尔型、整型、字符串，或任意实现了 `Eq` 和 `Hash` trait 的其他类型。在下一节将进一步介绍。

和 vector 类似，`HashMap` 也是可增长的，但 HashMap 在空间多余时能够缩小自身（原文：HashMaps can also shrink themselves when they have excess space. ）。创建 HashMap，可以使用适当的初始化容器（starting capacity） `HashMap::with_capacity(unit)`，或者使用 `HashMap::new()` 来获得一个带有默认初始容器的 HashMap（推荐方式）。

{hash.play}

了解更多关于映射（map）和散列映射（hash map）（通常也称作散列表，哈希表）的实现原理，可以查看 Wikipedia 的词条[散列表][wiki-hash]。

[wiki-hash]: http://en.wikipedia.org/wiki/Hash_table
