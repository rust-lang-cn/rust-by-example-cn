# 存在问题

`trait` 如果对实现了它的容器类型是泛型的，则须遵守类型规范要求——`trait` 的
使用者**必须**指出 `trait` 的全部泛型类型。

在下面例子中，`Contains` `trait` 允许使用泛型类型 `A` 和 `B`。然后我们为
 `Container` 类型实现了这个 trait，将 `A` 和 `B` 指定为 `i32`，这样就可以对
它们使用 `difference()` 函数。

因为 `Contains` 是泛型的，我们必须在 `fn difference()` 中显式地指出**所有的**泛型
类型。但实际上，我们想要表达，`A` 和 `B` 究竟是什么类型是由输入 `C` 决定的。在
下一节会看到，关联类型恰好提供了这样的功能。

```rust,editable
struct Container(i32, i32);

// 这个 trait 检查给定的 2 个项是否储存于容器中
// 并且能够获得容器的第一个或最后一个值。
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // 显式地要求 `A` 和 `B`
    fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
    fn last(&self) -> i32;  // 未显式地要求 `A` 或 `B`
}

impl Contains<i32, i32> for Container {
    // 如果存储的数字和给定的相等则为真。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}

// 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦。
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

[structs]: ../../custom_types/structs.md
[traits]: ../../trait.md
