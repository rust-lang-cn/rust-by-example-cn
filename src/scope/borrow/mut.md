# 可变性

可变数据可以使用 `&mut T` 进行可变借用。这叫做**可变引用**（mutable
reference），它使借用者可以读/写数据。相反，`&T` 通过不可变引用（immutable
reference）来借用数据，借用者可以读数据而不能更改数据：

```rust,editable,ignore,mdbook-runnable
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` 是一个对分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 此函数接受一个对 Book 类型的引用
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // 创建一个名为 `immutabook` 的不可变的 Book 实例
    let immutabook = Book {
        // 字符串字面量拥有 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
    let mut mutabook = immutabook;
    
    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);
    
    // 可变地借用一个可变对象
    new_edition(&mut mutabook);
    
    // 报错！不能可变地借用一个不可变对象
    new_edition(&mut immutabook);
    // 改正 ^ 注释掉此行
}
```

### 参见：
[`static`][static]

[static]: ./lifetime/static_lifetime.html
