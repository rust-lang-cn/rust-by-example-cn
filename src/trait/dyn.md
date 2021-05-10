# 使用 `dyn` 返回 trait

Rust 编译器需要知道每个函数的返回类型需要多少空间。这意味着所有函数都必须返回一个具体类型。与其他语言不同，如果你有个像 `Animal` 那样的的 trait，则不能编写返回 `Animal` 的函数，因为其不同的实现将需要不同的内存量。

但是，有一个简单的解决方法。相比于直接返回一个 trait 对象，我们的函数返回一个包含一些 `Animal` 的 `Box`。`box` 只是对堆中某些内存的引用。因为引用的大小是静态已知的，并且编译器可以保证引用指向已分配的堆 `Animal`，所以我们可以从函数中返回 trait！

每当在堆上分配内存时，Rust 都会尝试尽可能明确。因此，如果你的函数以这种方式返回指向堆的 trait 指针，则需要使用 `dyn` 关键字编写返回类型，例如 `Box<dyn Animal>`。

```rust,editable
struct Sheep {}
struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 实现 `Sheep` 的 `Animal` trait。
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// 实现 `Cow` 的 `Animal` trait。
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

```
