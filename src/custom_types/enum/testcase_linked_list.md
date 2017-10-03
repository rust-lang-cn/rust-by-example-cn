# 测试实例：链表

`enum` 的一个常见用法就是创建链表（linked-list）：

```rust,editable
use List::*;

enum List {
    // Cons： 元组结构体，包含一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil： 末结点，表明链表结束
    Nil,
}

// 方法可以在 enum 定义
impl List {
    // 创建一个空列表
    fn new() -> List {
        // `Nil` 为 `List` 类型
        Nil
    }

    // 处理一个列表，得到一个头部带上一个新元素的同样类型的列表并返回此值
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    // 返回列表的长度
    fn len(&self) -> u32 {
        // `self` 必须匹配，因为这个方法的行为取决于 `self` 的变化类型
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，一个具体的 `T` 类型的匹配
        // 要参考引用 `&T` 的匹配
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 而是得到一个 tail 引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 基本情形：空列表的长度为 0
            Nil => 0
        }
    }

    // 将列表以字符串（堆分配的）的形式返回
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，而不是
                // 打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```

### 参见：

[`Box`][box] 和 [methods][methods]

[box]: ../../std/box.html
[methods]: ../../fn/methods.html
