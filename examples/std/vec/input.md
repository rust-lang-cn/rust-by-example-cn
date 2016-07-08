vector 是可变大小的数组。和 slice（切片）类似，它们的大小在编译期不可预知，但他们可以随时扩大或缩小。一个 vector 使用 3 个词来表示：一个指向数据的指针，它的长度，还有它的容量。此容量表明了分配多少内存给这 vector。vector 只要小于该容量，就可以随意增长。当临界值就要达到时，vector 会重新分配一个更大的容量。

{vec.play}

更多 `Vec` 方法可以在 [std::vec][vec] 模块中找到。

[vec]: http://doc.rust-lang.org/std/vec/
