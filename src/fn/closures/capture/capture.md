闭包本身是相当灵活的，可以实现所需功能来让闭包运行而不用类型标注（原文：Closures are inherently flexible and will do what the functionality requires
to make the closure work without annotation）。这允许变量捕获灵活地适应使用
情况，有时是移动（moving）有时是借用（borrowing）（原文：This allows capturing to
flexibly adapt to the use case, sometimes moving and sometimes borrowing.）。闭包可以捕获变量：

* 通过引用：`&T`
* 通过可变引用：`&mut T`
* 通过值：`T`

它们更倾向于通过引用来捕获变量并且只在需要时才用后面用法（原文：They preferentially capture variables by reference and only go lower when
required.）。

{capture.play}

### 参见：

[`Box`][box] 和 [`std::mem::drop`][drop]

[box]: ../../std/box.html
[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
