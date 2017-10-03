# 散列表 HashMap

vector 通过整型索引来存储值，而 `HashMap` （散列表）通过键（key）来存储值。`HashMap` 的键可以是布尔型、整型、字符串，或任意实现了 `Eq` 和 `Hash` trait 的其他类型。在下一节将进一步介绍。

和 vector 类似，`HashMap` 也是可增长的，但 HashMap 在空间多余时能够缩小自身（原文：HashMaps can also shrink themselves when they have excess space. ）。创建 HashMap，可以使用适当的初始化容器（starting capacity） `HashMap::with_capacity(unit)`，或者使用 `HashMap::new()` 来获得一个带有默认初始容器的 HashMap（推荐方式）。

```rust,editable
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() { 
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 接受一个引用并返回 Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // 如果被插入的值为新内容，那么 `HashMap::insert()` 返回 `None`，
    // 否则返回 `Some(value)`
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&("Ashley")); 

    // `HashMap::iter()` 返回一个迭代器，该迭代器获得
    // 任意顺序的 (&'a key, &'a value) 对。
    // （原文：`HashMap::iter()` returns an iterator that yields 
    // (&'a key, &'a value) pairs in arbitrary order.）
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number)); 
    }
}
```

了解更多关于映射（map）和散列映射（hash map）（通常也称作散列表，哈希表）的实现原理，可以查看 Wikipedia 的词条[散列表][wiki-hash]。

[wiki-hash]: http://en.wikipedia.org/wiki/Hash_table
