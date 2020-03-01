# 捕获

闭包天生就是灵活的，它会自动满足函数功能的要求，使得闭包不需要类型说明就可以
工作。这允许变量捕获（capture）灵活地适应使用场合，既可移动（move）又可
借用（borrow）变量。闭包可以通过以下手段捕获变量：

* 通过引用：`&T`
* 通过可变引用：`&mut T`
* 通过值：`T`

闭包更倾向于通过引用来捕获变量，并且只在被要求时才使用其他手段。

```rust,editable
fn main() {
    use std::mem;
    
    let color = "green";

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 调用闭包，闭包又借用 `color`。
    print();
    print();

    let mut count = 0;

    // 这个闭包使 `count` 值增加。要做到这点，它需要使用 `&mut count` 或者
    // `count` 的方式，但 `&mut count` 的权限更低，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    inc();

    //let reborrow = &mut count;
    //*reborrow = 3;
    // ^ 试一试：将上面两行注释去掉。
    
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
[drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
