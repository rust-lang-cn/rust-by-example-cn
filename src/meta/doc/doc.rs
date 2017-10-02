#![crate_name = "doc"]

/// 这里给出一个人类
pub struct Person {
    /// 一个人必须有名字，不管 Juliet 多讨厌他/她。
    name: String,
}

impl Person {
    /// 返回给定名字的人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串 slice，代表人物的名称
    ///
    /// # 示例：
    ///
    /// ```
    /// // 可以在注释的特定标记内编写 Rust。
    /// // 如果可以通过 --- 测试传递给 Rustdoc，它将会帮你进行测试！
    /// let person = Person::new("name);
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
