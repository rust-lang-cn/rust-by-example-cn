# 函数

忽视[省略][elision]（elision）情况，带上生命周期的函数签名（function signature）有一些限制：

* 任何引用**都必须**拥有标注好的生命周期。
* 任何被返回的引用**都必须**有一个和输入量相同的生命周期或是静态类型（`static`）。

另外要注意，若会导致返回的引用指向无效数据，则返回不带输入量的引用是被禁止的。下面例子展示了一些带有生命周期的函数的有效形式：

```rust,editable
// 一个拥有生命周期 `'a` 的输入引用，其中 `'a` 的存活时间
// 至少与函数的一样长。
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// 可变引用同样也可能拥有生命周期。
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// 拥有不同生命周期的多个元素。对下面这种情形，两者即使拥有
// 相同的生命周期 `'a` 也没问题，但对一些更复杂的情形，可能
// 就需要不同的生命周期了。
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// 返回传递进来的引用也是可行的。
// 但必须返回正确的生命周期。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a i32 { &7 }
// 上面代码是无效的：`'a` 存活的时间必须比函数的长。
// 这里的 `&7` 将会创建一个 `i32` 类型，跟在引用后面。
// 然后数据在离开作用域时删掉，留下一个指向无效数据的引用，
// 此引用将被返回。

fn main() {
    let x = 7;
    let y = 9;
    
    print_one(&x);
    print_multi(&x, &y);
    
    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
```

### 参见：

[函数][fn]

[elision]: elision.html
[fn]: ../../fn.html
