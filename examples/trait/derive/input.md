通过 `#[derive]` [属性][attribute]，编译器能够提供一些对于 trait 的基本实现。如果需要一个更复杂的业务，这些 trait 仍然可以手动实现。（原文：The compiler is capable of providing basic implementations for some traits via the `#[derive]` [attribute][attribute]. These traits can still be manually implemented if a more complex behavior is required.）

下面列举了 “derivable”（可派生的）trait：
* 比较 traits:
  [`Eq`][eq],
  [`PartialEq`][partial-eq],
  [`Ord`][ord],
  [`PartialOrd`][partial-ord]
* [`Clone`][clone],
  采用复制（copy）方式从 `&T` 创建 `T`。
* [`Hash`][hash], 
  从 `&T` 计算哈希值（hash）。
* [`Default`][default],
  创建数据类型的一个空实例。
* `Zero`, 
  创建数字数据类型的一个零值实例（zero instance）。
* [`Debug`][debug], 
  使用 `{:?}` 格式化程序（formatter）格式化一个值。

{derive.play}

[attribute]: ../attribute.html
[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: http://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: http://doc.rust-lang.org/std/clone/trait.Clone.html
[hash]: http://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: http://doc.rust-lang.org/std/default/trait.Default.html
[debug]: http://doc.rust-lang.org/std/fmt/trait.Debug.html
