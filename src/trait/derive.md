# 派生

通过 `#[derive]` [属性][attribute]，编译器能够提供一些对于 trait 的基本实现。如果需要一个更复杂的业务，这些 trait 仍然可以手动实现。（原文：The compiler is capable of providing basic implementations for some traits via the `#[derive]` [attribute][attribute]. These traits can still be manually implemented if a more complex behavior is required.）

下面列举了 “derivable”（可派生的）trait：
* 比较 trait:
  [`Eq`][eq], [`PartialEq`][partial-eq], [`Ord`][ord], [`PartialOrd`][partial-ord]
* [`Clone`][clone], 采用复制（copy）方式从 `&T` 创建 `T`。
* [`Copy`][copy]，给出“复制语义”（'copy semantics'）来替代“移动语义”（'move semantics'）。
* [`Hash`][hash]，从 `&T` 计算哈希值（hash）。
* [`Default`][default], 创建数据类型的一个空实例。
* `Zero`，创建数字数据类型的一个零值实例（zero instance）。
* [`Debug`][debug]，使用 `{:?}` 格式化程序（formatter）格式化一个值。

```rust,editable
// `Centimeters`，可以比较的元组结构体
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`，可以打印的元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`，不带附加属性的元组结构体
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // 报错：`Seconds` 不能打印；它没有实现 `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // 试一试 ^ 将此行注释去掉

    // 报错：`Seconds`不能比较；它没有实现 `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // 试一试 ^ 将此行注释去掉

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
```

[`derive`][derive]

[attribute]: ../attribute.html
[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: http://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: http://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
[hash]: http://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: http://doc.rust-lang.org/std/default/trait.Default.html
[debug]: http://doc.rust-lang.org/std/fmt/trait.Debug.html
[derive]: https://doc.rust-lang.org/reference.html#derive
