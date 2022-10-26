# RAII

Rust 的变量不只是在栈中保存数据：它们也**占有**资源，比如 `Box<T>` 占有堆（heap）中的内存。Rust 强制实行 [RAII][raii]（Resource Acquisition Is Initialization，资源获取即初始化），所以任何对象在离开作用域时，它的析构函数（destructor）就被调用，然后它占有的资源就被释放。

这种行为避免了**资源泄漏**（resource leak），所以你再也不用手动释放内存或者担心内存泄漏（memory leak）！下面是个快速入门示例：

```rust,editable
// raii.rs
fn create_box() {
    // 在堆上分配一个整型数据
    let _box1 = Box::new(3i32);

    // `_box1` 在这里被销毁，内存得到释放
}

fn main() {
    // 在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // `_box3` 在这里被销毁，内存得到释放
    }

    // 创建一大堆 box（只是因为好玩）。
    // 完全不需要手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` 在这里被销毁，内存得到释放
}
```

当然我们可以使用 [`valgrind`][valgrind] 对内存错误进行仔细检查：

```bash
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

完全没有泄漏！

## 析构函数

Rust 中的析构函数概念是通过 [`Drop`] trait 提供的。当资源离开作用域，就调用析构函数。你无需为每种类型都实现 [`Drop`] trait，只要为那些需要自己的析构函数逻辑的类型实现就可以了。

运行下列例子，看看 [`Drop`] trait 是怎样工作的。当 `main` 函数中的变量离开作用域，自定义的析构函数就会被调用：

```rust,editable
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
```

### 参见：

[Box][box]

[raii]: https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization
[box]: ../std/box.md
[valgrind]: https://valgrind.org/info/
[`Drop`]: https://rustwiki.org/zh-CN/std/ops/trait.Drop.html
