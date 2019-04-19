# 泛型

**泛型**（generic）是关于泛化类型和函数功能，以扩大其适用范围的话题。泛型极大地
减少了代码的重复，但它自身的语法很要求细心。也就是说，采用泛型意味着仔细地指定
泛型类型具体化时，什么样的具体类型是合法的。泛型最简单和常用的用法是用于类型参数。

> 译注：定义泛型类型或泛型函数之类的东西时，我们会用 `<A>` 或者 `<T>` 这类标记
> 作为类型的代号，就像函数的形参一样。在使用时，为把 `<A>`、`<T>` 具体化，我们
> 会把类型说明像实参一样使用，像是 `<i32>` 这样。这两种把（泛型的或具体的）类型
> 当作参数的用法就是**类型参数**。

泛型的类型参数是使用尖括号和[大驼峰命名][camelcase]的名称：`<Aaa, Bbb, ...>`
 来指定的。泛型类型参数一般用 `<T>` 来表示。在 Rust 中，“泛型的” 除了表示
类型，还表示可以接受一个或多个泛型类型参数 `<T>` 的任何内容。任何用泛型类型参数
表示的类型都是泛型，其他的类型都是具体（非泛型）类型。

例如定义一个名为 `foo` 的 **泛型函数**，它可接受类型为 `T` 的任何参数 `arg`：

```rust,ignore
fn foo<T>(arg: T) { ... }
```

因为我们使用了泛型类型参数 `<T>`，所以这里的 `(arg: T)` 中的 `T` 就是泛型
类型。即使 `T` 在之前被定义为 `struct`，这里的 `T` 仍然代表泛型。

下面例子展示了泛型语法的使用：

```rust,editable
// 一个具体类型 `A`。
struct A;

// 在定义类型 `Single` 时，第一次使用类型 `A` 之前没有写 `<A>`。
// 因此，`Single` 是个具体类型，`A` 取上面的定义。
struct Single(A);
//            ^ 这里是 `Single` 对类型 `A` 的第一次使用。

// 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
// 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
struct SingleGen<T>(T);

fn main() {
    // `Single` 是具体类型，并且显式地使用类型 `A`。
    let _s = Single(A);
    
    // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为 `SingleGen('a')`
    // 这里的 `SingleGen` 的类型参数是显式指定的。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 的类型参数也可以隐式地指定。
    let _t    = SingleGen(A); // 使用在上面定义的 `A`。
    let _i32  = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char`。
}
```

### 参见：

[`struct`][structs]

[structs]: ./custom_types/structs.html
[camelcase]: https://en.wikipedia.org/wiki/CamelCase
