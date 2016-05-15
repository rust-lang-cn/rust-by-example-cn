对一个结构体来说，须对各个元素逐个实现 `fmt::Display` 可能会很麻烦。问题在于每个 `write!`
都要生成一个 `fmt::Result`。彻底地实现需要处理**所有**的结果。出于这方面考虑，Rust 提供了 `try!` 宏。

在 `write!` 上使用 `try!`类似这样：

```rust
// 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
// 否则（没有出错）继续执行后面的语句。
try!(write!(f, "{}", value));
```

在有 `try!` 的基础上，对一个 `Vec` 实现 `fmt::Display` 是相当直接的：

{testcase_list.play}

### 动手试一试：
更改程序使 vector 里面元素的索引也能够打印出来。新的结果类似这样：
``` rust
[0: 1, 1: 2, 2: 3]
```

### 参见：

[`for`][for], [`ref`][ref], [`Result`][result], [`struct`][struct],
[`try!`][try], [`vec!`][vec]

[for]: ../../../flow_control/for.html
[result]: ../../../std/result.html
[ref]: ../../../scope/borrow/ref.html
[struct]: ../../../custom_types/structs.html
[try]: ../../../std/result/try.html
[vec]: ../../../std/vec.html
