# 关联类型

通过把容器内部的类型放到 `trait` 中作为**输出类型**，使用 “关联类型” 增加了代码的可读性。这样的 `trait` 的定义语法如下：

```rust
// `A` 和 `B` 在 trait 里面通过 `type` 关键字来定义。
// （注意：此处的 `type` 不同于为类型取别名时的 `type`）。
trait Contains {
    type A;
    type B;

	// 这种语法能够泛型地表示这些新类型。
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
```

注意使用了 `Contains` `trait` 的函数就不需要写出 `A` 或 `B` 了：

```rust,ignore
// 不使用关联类型
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// 使用关联类型
fn difference<C>(container: &C) -> i32 where 
    C: Contains { ... }
```

让我们使用关联类型来重写上一小节的例子：

```rust,editable
struct Container(i32, i32);

// 这个 trait 检查给定的 2 个项是否储存于容器中
// 并且能够获得容器的第一个或最后一个值。
trait Contains {
    // 在这里定义可以被方法使用的泛型类型。
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
    // 为 `Container(i32, i32)`，那么 `output`（输出）类型
    // 会被确定为 `i32` 和 `i32`。
    type A = i32;
    type B = i32;

    // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}

fn difference<C>(container: &C) -> i32 
    where C: Contains 
{
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
