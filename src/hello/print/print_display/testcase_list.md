# 测试实例：List

对一个结构体实现 `fmt::Display`，其中的元素需要一个接一个地处理到，这可能会很麻
烦。问题在于每个 `write!` 都要生成一个 `fmt::Result`。正确的实现需要
处理**所有**的 Result。Rust 专门为解决这个问题提供了 `?` 操作符。

在 `write!` 上使用 `?` 会像是这样：

```rust,ignore
// 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
// 否则（没有出错）继续执行后面的语句。
write!(f, "{}", value)?;
```

另外，你也可以使用 `try!` 宏，它和 `?` 是一样的。这种写法比较罗嗦，故不再推荐，
但在老一些的 Rust 代码中仍会看到。使用 `try!` 看起来像这样：

```rust,ignore
try!(write!(f, "{}", value));
```

有了 `?`，对一个 `Vec` 实现 `fmt::Display` 就很简单了：

```rust,editable
use std::fmt; // 导入 `fmt` 模块。

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的脚标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

### 动手试一试：
更改程序使 vector 里面每个元素的脚标也能够打印出来。新的结果如下：

```rust
[0: 1, 1: 2, 2: 3]
```

### 参见：

[`for`][for], [`ref`][ref], [`Result`][result], [`struct`][struct],
[`try!`][try], [`vec!`][vec]

[for]: ./flow_control/for.html
[result]: ./std/result.html
[ref]: ./scope/borrow/ref.html
[struct]: ./custom_types/structs.html
[try]: ./std/result/try.html
[vec]: ./std/vec.html
