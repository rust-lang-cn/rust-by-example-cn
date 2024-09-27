# 使用 use

使用 `use` 声明的话，就可以不写出名称的完整路径了：

```rust,editable
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // 明确地使用每个`use`，以便它们可以在没有指定作用域的情况下使用。
    use crate::Stage::{Beginner, Advanced};
    // 自动地使用`Role`内部定义的所有名称。
    use crate::Role::*;

    // `Beginner` 等价于 `Stage::Beginner`。
    let stage = Beginner;
    // `Student` 等价于 `Role::Student`。
    let role = Student;

    match stage {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // 再次注意到没有用完整路径。
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
```

### 参见：

[`match`][match] 和 [`use`][use]

[use]: ../../mod/use.md
[match]: ../../flow_control/match.md
