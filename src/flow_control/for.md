# for 循环

## for 与区间

`for in` 结构可以遍历一个 `Iterator`（迭代器）。创建迭代器的一个最简单的方法是使用区间标记 `a..b`。这会生成从 `a`（包含此值） 到 `b`（不含此值）的，步长为 1 的一系列值。

让我们使用 `for` 代替 `while` 来写 FizzBuzz 程序。

```rust,editable
fn main() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

或者，可以使用`a..=b`表示两端都包含在内的范围。上面的代码可以写成：

```rust,editable
fn main() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

## for 与迭代器

`for in` 结构能以几种方式与 `Iterator` 互动。在 [迭代器][iter] trait 一节将会谈到，如果没有特别指定，`for` 循环会对给出的集合应用 `into_iter` 函数，把它转换成一个迭代器。这并不是把集合变成迭代器的唯一方法，其他的方法有 `iter` 和`iter_mut` 函数。

这三个函数会以不同的方式返回集合中的数据。

- `iter` - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。

```rust,editable
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
```

译注：Ferris 是 Rust 的[非官方吉祥物](https://www.rustacean.net/)。

- `into_iter` - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。

```rust,editable
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
```

- `iter_mut` - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。

```rust,editable
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
```

在上面这些代码中，注意 `match` 的分支中所写的类型不同，这是不同迭代方式的关键区别。因为类型不同，能够执行的操作当然也不同。

### 参见：

[Iterator][iter]

[iter]: ../trait/iter.md
