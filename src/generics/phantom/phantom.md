虚位类型参量（phantom type parameter）是一种在运行时（runtime）不出现，而在（且只在）编译期进行静态方式检查的参量。

数据类型可以使用额外的泛型类型参量来充当标记或在编译期执行类型检查。这些额外的参量没有存储值，且没有运行时行为（runtime behavior）。

在下面例子中，我们把 [std::marker::PhantomData] 和虚位类型参量概念结合起来创建包含不同数据类型的元组。

```rust,editable
use std::marker::PhantomData;

// 虚位元组结构体，这是一个带有 `A` 和隐藏参量（hidden parameter） `B` 的泛型。
#[derive(PartialEq)] // 允许这种类型进行相等测试（equality test）。
struct PhantomTuple<A, B>(A,PhantomData<B>);

// 模型元组结构体，这是一个带有 `A` 和隐藏参量 `B` 的泛型。
#[derive(PartialEq)] // 允许这种类型进行相等测试。
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// 注意：对于泛型 `A` 会分配存储空间，但 `B` 不会。
//       因此，`B` 不能参与运算。

fn main() {
    // 这里的 `f32` 和 `f64` 是隐藏参量。
    // 被指定为 `<char, f32>` 的虚位元组（PhantomTuple）类型。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f64>` 的虚位元组。
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 被指定为 `<char, f32>` 的类型。
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 被指定为 `<char, f64>` 的类型。
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    
    // 编译期（compile-time）报错！类型不匹配，所以这些值不能够比较：
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);
    
    // 编译期报错！类型不匹配，所以这些值不能够比较：
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
}
```

### 参见：

[Derive], [结构体][struct], 和 [元组结构体][TupleStructs]

[Derive]: ../trait/derive.html
[struct]: ../custom_types/structs.html
[TupleStructs]: ../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
