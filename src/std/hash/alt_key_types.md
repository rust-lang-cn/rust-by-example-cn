任何实现了 `Eq` 和 `Hash` trait 的类型都可以充当 `HashMap` 的键。这包括：

* `bool` （当然这个用处不大，因为只有两个可能的键）
* `int`，`unit`，以及所有这类型的变量
* `String` 和 `&str`（友情提示：可以创建一个由 `String` 构成键的 `HashMap`，并以一个 `&str` 来调用 `.get()`）（原文：`String` and `&str` (protip: you can have a `HashMap` keyed by `String` and call `.get()` with an `&str`)）

需要注意的是 `f32` 和 `f64` **没有**实现 `Hash`，很大程度上是由于[浮点精度误差][floating]（floating-point precision error）会使浮点类型作为散列映射键发生严重的错误。

对于所有的集合类（collection），如果它们包含的类型都分别实现 `Eq` 和 `Hash`，那么这些集合类也都会实现 `Eq` 和 `Hash`。例如，若 `T` 实现了 `Hash`，则 `Vec<T>` 也会实现 `Hash`。

对自定义类型可以轻松地实现 `Eq` 和 `Hash`，只需加上一行代码： `#[derive(PartialEq, Eq, Hash)]`。

编译器将会完成余下的工作。如果你想控制更多的细节内容，你可以实现自己定制的 `Eq` 和/或 `Hash`。本指南不包含实现 `Hash` 的细节内容。

为了玩玩怎么使用 `HashMap` 中的 `struct`，让我们试着做一个非常简易的登录系统：

```rust,editable
use std::collections::HashMap;

// Eq 要求你对此类型派生了 PartiaEq。
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
        username: &'a str, password: &'a str){
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username: username,
        password: password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login failed!"),
    }
}

fn main(){
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}
```

[hash]: http://en.wikipedia.org/wiki/Hash_function
[floating]: http://en.wikipedia.org/wiki/Floating_point#Accuracy_problems
