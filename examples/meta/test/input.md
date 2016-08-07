函数可以通过这些[属性][attributes]（attribute） 进行测试：

* `#[test]` 将一个函数标记为一个单元测试。该函数不能接受参数且返回空。
* `#[should_panic]` 将一个函数标记为 panic 测试。

{unit_test.rs}

通过 `cargo test` 或 `rustc --test` 运行测试。

```
$ rustc --test unit_test.rs
$ ./unit_test 

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

若 `--test` 没有包含进来，则会出现这样的情况：

```
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

### 参见：

[属性][attributes], [条件编译][cfg], 和 [`mod`][mod].

[attributes]: ../../attribute.html
[cfg]: ../../attribute/cfg.html
[mod]: ../../mod.html
