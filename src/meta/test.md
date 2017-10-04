# 测试

函数可以通过这些[属性][attributes]（attribute） 进行测试：

* `#[test]` 将一个函数标记为一个单元测试。该函数不能接受参数且返回空。
* `#[should_panic]` 将一个函数标记为 panic 测试。

```rust,editalbe
// 当且仅当测试套件没有运行时，才条件编译 `main` 函数。
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// 当且仅当测试套件运行时，才条件编译 `test` 模块。
#[cfg(test)]
mod test {
    // 需要一个辅助函数 `distance_test`。
    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        (
            (b.0 - a.0).powi(2) +
            (b.1 - a.1).powi(2)
        ).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
    }
    
    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }
}
```

通过 `cargo test` 或 `rustc --test` 运行测试。

```bash
$ rustc --test unit_test.rs
$ ./unit_test 

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

若 `--test` 没有包含进来，则会出现这样的情况：

```bash
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

### 参见：

[属性][attributes], [条件编译][cfg], 和 [`mod`][mod].

[attributes]: ./attribute.html
[cfg]: ./attribute/cfg.html
[mod]: ./mod.html
