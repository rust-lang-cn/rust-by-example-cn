# 字符串 String

Rust 中有两种字符串类型：`String` 和 `&str`。

`String` 被存储为一个字节形式（`Vec<u3>`）的vector ，但确保一定是一个有效的 UTF-8 序列。`String` 是堆分配的，可增大且无上限。

`&str` 是一个指向有效 UTF-8 序列的切片（`&[u8]`），并可在使用中看成是 `String`，就如同 `&[T]` 是 `Vec<T>` 的一个视图。（原文：`&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence, and can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`.）（您是否有更好的翻译？请改进此句翻译，感谢！）

```rust,editable
fn main() {
    // （所有的类型标注都是都是多余）
    // 一个指向在只读内存中堆分配字符串的引用
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，不用分配新的字符串
    // （原文：Iterate over words in reverse, no new string is allocated）    
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 复制字符到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的且可增长的 `String`
    let mut string = String::new();
    for c in chars {
        // 在字符串的尾部插入一个字符
        string.push(c);
        // 在字符串尾部插入一个字符串
        string.push_str(", ");
    }

    // 此切割的字符串是原字符串的一个切片，所以没有执行新分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
```

更多 `str`/`String` 方法可以在 [std::str][str] 和 [std::string][string] 模块中找到。

[str]: http://doc.rust-lang.org/std/str/
[string]: http://doc.rust-lang.org/std/string/
