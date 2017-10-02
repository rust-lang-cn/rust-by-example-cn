对容器类型为泛型的 `trait` 有类型规范需要——`trait` 的成员**必须**指出全部关于它的泛型类型。

在下面例子中，`Contains` `trait` 允许使用泛型类型 `A` 或 `B`。然后这个 trait 针对 `Container` 类型实现，指定 `i32` 为 `A` 和 `B`，因而它可以用到 `fn difference()`。（本段原文：In the example below, the `Contains` `trait` allows the use of the generic types `A` and `B`. The trait is then implemented for the `Container` type, 
specifying `i32` for `A` and `B` so that it can be used with `fn difference()`.）

因为 `Contains` 是泛型，所以我们被迫显式地指出了针对 `fn difference()` 的所有泛型类型。实际上，我们只想要一种方式来表示由**输入**的 `C` 确定的 `A` 和 `B`。正如你就要看到的下一节内容，关联类型正好提供了这方面能力。

```rust,editable
struct Container(i32, i32);

// 这个 trait 检查 2 个项是否存到 Container（容器）中。
// 还会获得第一个值或最后一个值。
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // 显式指出需要 `A` 和 `B`
    fn first(&self) -> i32; // 未显式指出需要 `A` 或 `B`
    fn last(&self) -> i32;  // 未显式指出需要 `A` 或 `B`
}

impl Contains<i32, i32> for Container {
    // 如果存储的数字相等则为真。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}

// `C` 包含 `A` 和 `B` 。鉴于此，必须重复表达 `A` 和 `B` 真麻烦。
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
```

### 参见：

[`struct`][structs], 和 [`trait`][traits]

[structs]: ../../custom_types/structs.html
[traits]: ../../trait.html
