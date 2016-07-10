Rust 中有两种字符串类型：`String` 和 `&str`。

`String` 被存储为一个字节形式（`Vec<u3>`）的vector ，但确保一定是一个有效的 UTF-8 序列。`String` 是堆分配的，可增大且无上限。

`&str` 是一个指向有效 UTF-8 序列的切片（`&[u8]`），并可在使用中看成是 `String`，就如同 `&[T]` 是 `Vec<T>` 的一个视图。（原文：`&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence, and can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`.）（您是否有更好的翻译？请改进此句翻译，感谢！）

{str.play}

更多 `str`/`String` 方法可以在 [std::str][str] 和 [std::string][string] 模块中找到。

[str]: http://doc.rust-lang.org/std/str/
[string]: http://doc.rust-lang.org/std/string/
