// 隐藏未使用代码警告的属性。
#![allow(dead_code)]

// 创建一个 `enum` （枚举）来划分人的类别。注意命名和类型的信息是如何一起
// 明确规定变量的：
// `Skinny != Fat` 和 `Height(i32) != Weight(i32)`。每者都是不同的且
// 相互独立。
enum Person {
    // 一个 `enum` 可能是个 `unit-like`（类单元结构体），
    Skinny,
    Fat,
    // 或像一个元组结构体，
    Height(i32),
    Weight(i32),
    // 或像一个普通的结构体。
    Info { name: String, height: i32 }
}

// 此函数将一个 `Person` enum 作为参数，无返回值。
fn inspect(p: Person) {
    // `enum` 的使用必须覆盖所有情形（无可辩驳的），所以使用 `match`
    // 以分支方式覆盖所有类型。
    match p {
        Person::Skinny    => println!("Is skinny!"),
        Person::Fat       => println!("Is fat!"),
        // 从 `enum` 内部解构 `i`
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // 将 `Info` 解构成 `name` 和 `height`。
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let danny  = Person::Weight(10);
    // `to_owned()` 从一个字符串 slice 创建一个具有所有权的 `String`。
    let dave   = Person::Info { name: "Dave".to_owned(), height: 72 };
    let john   = Person::Fat;
    let larry  = Person::Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}
