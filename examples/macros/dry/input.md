Macros allow writing DRY code by factoring out the common parts of functions
and/or test suites. Here is an example that implements and tests the `+=`, `*=`
and `-=` operators on `Vec<T>`:
通过提取函数或测试单元的公共部分，宏允许编写 DRY 代码（DRY 是 Don't Repeat Yourself 的缩写，意思为“不要写重复代码”）。这里给出一个例子，实现并测试了关于 `Vec<T>` 的 `+=`、`*=` 和 `-=` 等运算符。

{dry.rs}

```
$ rustc --test dry.rs && ./dry
running 3 tests
test test::mul_assign ... ok
test test::add_assign ... ok
test test::sub_assign ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```
