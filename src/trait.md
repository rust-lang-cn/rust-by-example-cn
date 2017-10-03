# 特性 trait

`trait` 是对未知类型定义的方法集：`Self`。它们可以访问同一个 trait 中定义的方法。

对任何数据类型实现 trait 都是可行的。在下面例子中，我们定义了包含一系列方法的 `Animal`。然后针对 `Sleep` 数据类型实现 `Animal` `trait`，允许使用来自带有 `Sheep` 的 `Animal` 的方法（原文：allowing the use of methods from `Animal` with a `Sheep`）。

```rust,editable
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // 静态方法标记；`Self` 表示实现者类型（implementor type）。
    fn new(name: &'static str) -> Self;

    // 实例方法（instance method）标记；这些方法将返回一个字符串。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // trait 可以提供默认方法定义（method definition）。
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // 实现者（implementor）可以使用实现者的 trait 方法。
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// 对 `Sheep` 实现 `Animal` trait。
impl Animal for Sheep {
    // `Self` 是该实现者类型：`Sheep`。
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // 默认 trait 方法可以重载。
    fn talk(&self) {
        // 例如完们可以增加一些安静的沉思（quiet contemplation）。
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // 这种情况需要类型标注。
    let mut dolly: Sheep = Animal::new("Dolly");
    // 试一试 ^ 移除类型标注。

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```
