use std::fmt; // 导入 `fmt` 模块。

// 定义一个包含不同包含 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 对 `self` 解引用，并通过解构创建一个指向 `vec` 的引用。
        let List(ref vec) = *self;

        try!(write!(f, "["));

        // 对 `vec` 进行迭代，`v` 为每次迭代的值，`count` 为计数。
        for (count, v) in vec.iter().enumerate() {
            // 在调用 `write!` 前对每个元素（第一个元素除外）加上逗号。
            // 使用 `try!` ，在出错的情况返回。
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        // 加上配对中括号，并返回一个 fmt::Result 值
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
