`'static` 生命周期在可能存在的生命周期中是最长的，并在运行程序的周期中持续存在。`static` 生命周期也可能被强制转换成一个更短的生命周期。有两种方式使变量拥有 `static` 生命周期，这两种方式都是保存在可执行文件的只读内存区：

* 使用 `static` 声明来产生常量（constant）。
* 产生一个拥有 `&'static str` 类型的 `string` 字面量。

看下面的例子，了解列举到的各个方法：

{static_lifetime.play}

### 参见：

[`'static` 常量][static_const]

[static_const]: ../../custom_types/constants.html
