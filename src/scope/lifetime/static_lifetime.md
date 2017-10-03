# 静态

`'static` 生命周期在可能存在的生命周期中是最长的，并在运行程序的周期中持续存在。`static` 生命周期也可能被强制转换成一个更短的生命周期。有两种方式使变量拥有 `static` 生命周期，这两种方式都是保存在可执行文件的只读内存区：

* 使用 `static` 声明来产生常量（constant）。
* 产生一个拥有 `&'static str` 类型的 `string` 字面量。

看下面的例子，了解列举到的各个方法：

```rust,editable
// 产生一个拥有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，其中`NUM` 的 `'static`
// 生命周期被强制转换成和输入参数的一样。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // 产生一个 `string` 字面量并打印它：
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 离开作用域时，该引用不能再使用，不过
        // 数据会保留在二进制文件里面。
    }
    
    {
        // 产生一个整型给 `coerce_static` 使用：
        let lifetime_num = 9;

        // 将 `NUM` 强制转换成 `lifetime_num` 的生命周期：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    
    println!("NUM: {} stays accessible!", NUM);
}
```

### 参见：

[`'static` 常量][static_const]

[static_const]: ../../custom_types/constants.html
