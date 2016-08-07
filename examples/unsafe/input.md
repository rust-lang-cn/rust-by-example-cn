为了介绍本章内容，我们借用[官方文档](http://doc.rust-lang.org/book/unsafe.html)的一句话, “在基本代码中尽可能减少不安全的代码”（"one should try to minimize the amount of unsafe code in a code base."）。记住这句话，接着我们进入学习！在 Rust 中，不安全代码块是用于避开编译器的保护策略；具体地说，不安全代码块主要有 4 方面内容：

* 解引用裸指针
* 通过 FFI 调用函数（这个内容在本书其他章节介绍过了）
* 使用 `std::cast::transmute` 来强制转型（change type）
* 内联汇编(inline assembly)

### 原始指针
原始指针（裸指针） `*` 和引用 `&T` 有类似的功能，但引用总是安全的，因为它们保证指向一个有效的数据，这得益于借用检查器（borrow checker）。解引用一个裸指针只能通过不安全代码块中来完成。

{pointer.rs}

### Transmute（转变）
从一种类型变到另一种类型的允许简单转换，但是两种类型必须拥有相同的大小和排列：

{transmute.rs}
