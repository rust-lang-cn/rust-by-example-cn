# 捕获

闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行。这使得捕获（capture）能够灵活地适应用例，既可移动（move），又可借用（borrow）。闭包可以通过以下方式捕获变量：

* 通过引用：`&T`
* 通过可变引用：`&mut T`
* 通过值：`T`

闭包优先通过引用来捕获变量，并且仅在需要时使用其他方式。

```rust,editable
fn main() {
    use std::mem;
    
    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。
}
```

在竖线 `|` 之前使用 `move` 会强制闭包取得被捕获变量的所有权：

```rust,editable
fn main() {
    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
    
    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}
```

### 参见：

[`Box`][box] 和 [`std::mem::drop`][drop]

[box]: ../../std/box.md
[drop]: https://rustwiki.org/zh-CN/std/mem/fn.drop.html
