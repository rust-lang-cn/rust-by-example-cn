# Clone

当处理资源时，默认的行为是在赋值或函数调用的同时将它们转移。但是我们有时候也需要
把资源复制一份。

[`Clone`][clone] trait 正好帮助我们完成这任务。通常，我们可以使用由 `Clone`
trait 定义的 `.clone()` 方法。

```rust,editable
// 不含资源的单元结构体
#[derive(Debug, Clone, Copy)]
struct Nil;

// 一个包含资源的结构体，它实现了 `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // 实例化 `Nil`
    let nil = Nil;
    // 复制 `Nil`，没有资源用于移动（move）
    let copied_nil = nil;

    // 两个 `Nil` 都可以独立使用
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // 实例化 `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // 将 `pair` 绑定到 `moved_pair`，移动（move）了资源
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // 报错！`pair` 已失去了它的资源。
    //println!("original: {:?}", pair);
    // 试一试 ^ 取消此行注释。

    // 将 `moved_pair`（包括其资源）克隆到 `cloned_pair`。
    let cloned_pair = moved_pair.clone();
    // 使用 std::mem::drop 来销毁原始的 pair。
    drop(moved_pair);

    // 报错！`moved_pair` 已被销毁。
    //println!("copy: {:?}", moved_pair);
    // 试一试 ^ 将此行注释掉。

    // 由 .clone() 得来的结果仍然可用！
    println!("clone: {:?}", cloned_pair);
}
```

[clone]: https://rustwiki.org/zh-CN/std/clone/trait.Clone.html
