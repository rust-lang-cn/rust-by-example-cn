# 集成测试

[单元测试][unit]一次仅能单独测试一个模块，这种测试是小规模的，并且能测试私有
代码；集成测试是 crate 外部的测试，并且仅使用 crate 的公共接口，就像其他使用
该 crate 的程序那样。集成测试的目的是检验你的库的各部分是否能够正确地协同工作。

cargo 在与 `src` 同级别的 `tests` 目录寻找集成测试。

文件 `src/lib.rs`：

```rust,ignore
// 假设这个 crate 叫做 adder，我们需要在集成测试中用 extern 说明。
pub fn testing add(a: i32, b: i32) -> i32 {
    a + b
}
```

包含测试的文件：`tests/integration_test.rs`：

```rust,ignore
// 声明被测试的外部 crate，就像其他使用该 crate 的程序要声明的那样。
extern crate adder;

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

使用 `cargo test` 命令：

```bash
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-bcd60824f5fbfe19

running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`tests` 目录中的每一个 Rust 源文件都被编译成一个单独的 crate。在集成测试中要想
共享代码，一种方式是创建具有公用函数的模块，然后在测试中导入并使用它。

文件 `tests/common.rs`:

```rust,ignore
pub fn setup() {
    // 一些配置代码，比如创建文件/目录，开启服务器等等。
}
```

包含测试的文件：`tests/integration_test.rs`

```rust,ignore
// 被测试的外部 crate。
extern crate adder;

// 导入共用模块。
mod common;

#[test]
fn test_add() {
    // 使用共用模块。
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
```

带有共用代码的模块遵循和普通的[模块][mod]一样的规则，所以完全可以把公共模块
写在 `tests/common/mod.rs` 文件中。

[unit]: unit_testing.md
[mod]: ../mod.md
