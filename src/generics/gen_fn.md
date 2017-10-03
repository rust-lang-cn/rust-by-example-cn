# 函数

同样的规则也可以适用于函数：在使用前给出 `<T>` 后，类型 `T` 就变成了泛型。

使用泛型函数有时需要显式地指明类型参量。这种可能的情况包括，调用返回类型是泛型的函数，或者编译器没有足够的信息来推导类型参量。

函数调用使用显式指定的类型参量，如下所示：
`fun::<A, B, ...>()`.

```rust,editable
struct A;          // 具体类型 `A`。
struct S(A);       // 具体类型 `S`。
struct SGen<T>(T); // 泛型类型 `SGen`。

// 下面全部函数都得到了变量的所有权，传递给函数的变量在离开作用域时立即释放。
// （原文：The following functions all take ownership of the variable passed
// into them and immediately go out of scope, freeing the variable.）

// 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
// 因为没有 `<T>`，所以这不是泛型函数。
fn reg_fn(_s: S) {}

// 定义一个函数 `gen_spec_t`，接受一个 `SGen<T>` 类型的参数 `_s`。
// 这里显式地给出了类型参量 `A`，但因为 `A` 没有被指明为针对 `gen_spec_t` 的
// 泛型类型参量，所以这不是一个泛型。
fn gen_spec_t(_s: SGen<A>) {}

// 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
// 这里显式地给出了类型参量 `i32`，而 `i32` 是一个具体类型。
// 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型。
fn gen_spec_i32(_s: SGen<i32>) {}

// 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
// 因为 `SGen<T>` 之前给定了 `<T>`，所以这个函数是关于 `T` 的泛型。
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // 使用非泛型函数
    reg_fn(S(A));          // 具体类型。
    gen_spec_t(SGen(A));   // 隐式地指定类型参量 `A`。
    gen_spec_i32(SGen(6)); // 隐式地指定类型参量 `i32`。

    // 显式地指定类型参量 `char` 传给 `generic()`。
    generic::<char>(SGen('a'));

    // 隐式地指定类型参量 `char` 传给 `generic()`。
    generic(SGen('c'));
}
```

### 参见：

[函数][fn] 和 [`struct`s][structs]

[fn]: ../fn.html
[structs]: ../custom_types/structs.html
