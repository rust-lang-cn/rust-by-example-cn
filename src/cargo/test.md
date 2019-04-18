# 测试

众所周知，任何软件都需要测试！Rust 对单元测试和集成测试提供了完整的支持（请看 TRPL
 中的[这一章](https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html)）。

从上面链接的章节中，我们看到了如何编写单元测试和继承测试。我们可以很条理地把单元
测试放到它们要测试的模块目录，把集成测试放到单独的 `tests/` 文件夹。

```txt
foo
├── Cargo.toml
├── src
│   └── main.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs
```

这里 `tests` 文件夹中的每个文件都是独立的集成测试。

`cargo` 提供了一种运行所有测试的简单方式！

```sh
cargo test
```

你会看到类似这样的输出：

```txt
$ cargo test
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 3 tests
test test_bar ... ok
test test_baz ... ok
test test_foo_bar ... ok
test test_foo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

你也可以运行那些名字匹配某一模式的测试：

```sh
cargo test test_foo
```

```txt
$ cargo test test_foo
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 2 tests
test test_foo ... ok
test test_foo_bar ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

注意一点：cargo 可能会并行地运行多个测试，所以确保它们不会彼此竞争。比如，它们
都往同一个文件里输出内容，那么你应该修改它们，让结果写到不同的文件中。
