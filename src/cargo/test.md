# 测试

我们知道测试是任何软件不可缺少的一部分！Rust 对单元和集成测试提供一流的支持（参见《Rust 程序设计语言》中的关于[测试的章节](https://doc.rust-lang.org/book/ch11-00-testing.html)）。

通过上面链接的关于测试章节，我们看到了如何编写单元测试和集成测试。在代码目录组织上，我们可以将单元测试放在需要测试的模块中，并将集成测试放在源码中 `tests/` 目录中：

```txt
foo
├── Cargo.toml
├── src
│   └── main.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs
```

`tests` 目录下的每个文件都是一个单独的集成测试。

`cargo` 很自然地提供了一种便捷的方法来运行所有测试！

```sh
cargo test
```

你将会看到像这样的输出：

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

你还可以运行如下测试，其中名称匹配一个模式：

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

需要注意的一点是：`cargo` 可能同时进行多项测试，因此请确保它们不会相互竞争。例如，如果它们都输出到文件，则应该将它们写入不同的文件。
