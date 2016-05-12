结构体对字段的可见性有额外的规定（Structs have an extra level of visibility with their fields）。其可见性默认为私有，也可以加上 `pub` 修饰语来改变默认属性。只有当从定义在外部的模块访问一个结构体时，这可见性才显得重要，并具有隐藏信息的目的（封装，encapsulatoin）（原文：This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation)）。

{struct.play}

### 参见：

[泛型][generics] and [方法][methods]

[generics]: ../generics.html
[methods]: ../fn/methods.html
