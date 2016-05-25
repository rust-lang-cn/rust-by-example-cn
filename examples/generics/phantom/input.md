幻象类型参量（phantom type parameter）是一种不会在运行时（runtime）出现，但是会（且只会）在编译期进行静态方式检查的参量。

数据类型可以使用额外的泛型类型参量来充当标记或在编译期执行类型检查。这些额外的参量没有存储值，且没有运行时行为（runtime behavior）。

In the following example, we combine [std::marker::PhantomData]
with the phantom type parameter concept to create tuples containing
different data types.
在下面例子中，我们将 [std::marker::PhantomData] 和幻象类型参量概念结合起来创建包含不同数据类型的元组。

{phantom.play}

### 参见：

[Derive], [结构体][struct], 和 [元组结构体][TupleStructs]

[Derive]: ../trait/derive.html
[struct]: ../custom_types/structs.html
[TupleStructs]: ../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
