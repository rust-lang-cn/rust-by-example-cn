当我们要重复多次使用特定的 `Result` 类型怎么办呢？要写下完整类型名称的话很快就会变得烦琐，还好 Rust 允许我们创建[别名][typealias]。对问题中提到的特定 `Result` 可以很方便地定义泛型别名（generic alias）：

{alias.play}

在单个模块的级别上创建别名特别有帮助。在特定模块中发现的错误常常会有相同的 `Err` 类型，所以一个单一的别名就能简便地定义**所有的**关联 `Result`。这点太重要了，甚至标准库也提供了一个： `io::Result`！

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[typealias]: ../../cast/alias.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
