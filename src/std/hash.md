# 散列表 HashMap

vector 通过整型下标来存储值，而 `HashMap`（散列表）通过键（key）来存储值。`HashMap` 的键可以是布尔型、整型、字符串，或任意实现了 `Eq` 和 `Hash` trait 的其他类型。在下一节将进一步介绍。

和 vector 类似，`HashMap` 也是可增长的，但 HashMap 在占据了多余空间时还可以缩小自己。可以使用 `HashMap::with_capacity(unit)` 创建具有一定初始容量的 HashMap，也可以使用 `HashMap::new()` 来获得一个带有默认初始容量的 HashMap（这是推荐方式）。

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

    // `HashMap::iter()` 返回一个迭代器，该迭代器以任意顺序举出
    // (&'a key, &'a value) 对。
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number)); 
    }
}
```

想要了解更多关于散列（hash）与散列表（hash map）（有时也称作 hash table）的工作原理，可以查看 Wikipedia 的[散列表][wiki-hash]词条。

[wiki-hash]: https://en.wikipedia.org/wiki/Hash_table
