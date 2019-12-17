# 函数

同样的规则也适用于函数：在使用类型 `T` 前给出 `<T>`，那么 `T` 就变成了泛型。

调用泛型函数有时需要显式地指明类型参量。这可能是因为调用了返回类型是泛型的
函数，或者编译器没有足够的信息来推断类型参数。

调用函数时，使用显式指定的类型参数会像是这样：`fun::<A, B, ...>()`。

```rust,editable
struct A;          // 具体类型 `A`。
struct S(A);       // 具体类型 `S`。
struct SGen<T>(T); // 泛型类型 `SGen`。

// 下面全部函数都得到了变量的所有权，并立即使之离开作用域，将变量释放。

// 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
// 因为没有 `<T>` 这样的泛型类型参数，所以这不是泛型函数。
fn reg_fn(_s: S) {}

// 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
// `SGen<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
// 泛型类型参数，所以函数不是泛型的。
fn gen_spec_t(_s: SGen<A>) {}

// 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
// `SGen<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
// 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
fn gen_spec_i32(_s: SGen<i32>) {}

// 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
// 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // 使用非泛型函数
    reg_fn(S(A));          // 具体类型。
    gen_spec_t(SGen(A));   // 隐式地指定类型参数 `A`。
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数 `i32`。

    // 为 `generic()` 显式地指定类型参数 `char`。
    generic::<char>(SGen('a'));

    // 为 `generic()` 隐式地指定类型参数 `char`。
    generic(SGen('c'));
}
```

### 参见：

[函数][fn] 和 [`struct`s][structs]

[fn]: ../fn.md
[structs]: ../custom_types/structs.md
