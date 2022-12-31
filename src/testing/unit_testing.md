# 单元测试

测试（test）是这样一种 Rust 函数：它保证其他部分的代码按照所希望的行为正常运行。测试函数的函数体通常会进行一些配置，运行我们想要测试的代码，然后断言（assert）结果是不是我们所期望的。

大多数单元测试都会被放到一个叫 `tests` 的、带有 `#[cfg(test)]` 属性的[模块][mod]中，测试函数要加上 `#[test]` 属性。

当测试函数中有什么东西 [panic][panic] 了，测试就失败。有一些这方面的辅助[宏][macros]：

* `assert!(expression)` - 如果表达式的值是 `false` 则 panic。
* `assert_eq!(left, right)` 和 `assert_ne!(left, right)` - 检验左右两边是否
  相等/不等。

```rust,ignore
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 这个加法函数写得很差，本例中我们会使它失败。
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // 这个断言会导致测试失败。注意私有的函数也可以被测试！
        assert_eq!(bad_add(1, 2), 3);
    }
}
```

可以使用 `cargo test` 来运行测试。

```bash
$ cargo test

running 2 tests
test tests::test_bad_add ... FAILED
test tests::test_add ... ok

failures:

---- tests::test_bad_add stdout ----
        thread 'tests::test_bad_add' panicked at 'assertion failed: `(left == right)`
  left: `-1`,
 right: `3`', src/lib.rs:21:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::test_bad_add

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

## 测试 panic

一些函数应当在特定条件下 panic。为测试这种行为，请使用 `#[should_panic]` 属性。这个属性接受可选参数 `expected = ` 以指定 panic 时的消息。如果你的函数能以多种方式
panic，这个属性就保证了你在测试的确实是所指定的 panic。

```rust,ignore
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
```

运行这些测试会输出：

```bash
$ cargo test

running 3 tests
test tests::test_any_panic ... ok
test tests::test_divide ... ok
test tests::test_specific_panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests tmp-test-should-panic

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 运行特定的测试

要运行特定的测试，只要把测试名称传给 `cargo test` 命令就可以了。

```bash
$ cargo test test_any_panic
running 1 test
test tests::test_any_panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out

   Doc-tests tmp-test-should-panic

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

要运行多个测试，可以仅指定测试名称中的一部分，用它来匹配所有要运行的测试。

```bash
$ cargo test panic
running 2 tests
test tests::test_any_panic ... ok
test tests::test_specific_panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out

   Doc-tests tmp-test-should-panic

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 忽略测试

可以把属性 `#[ignore]` 赋予测试以排除某些测试，或者使用 `cargo test -- --ignored`
命令来运行它们。

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
        assert_eq!(add(2, 100), 102);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}
```

```bash
$ cargo test
running 1 test
test tests::ignored_test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

   Doc-tests tmp-ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -- --ignored
running 1 test
test tests::ignored_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests tmp-ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

[attribute]: ../attribute.md
[panic]: ../std/panic.md
[macros]: ../macros.md
[mod]: ../mod.md
