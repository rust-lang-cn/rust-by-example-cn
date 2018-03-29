# 测试

Rust 是一门非常重视正确性的语言，这门语言本身就提供了对编写软件测试的支持。

测试有三种风格：

* [单元][unit]测试。
* [文档][doc]测试。
* [集成][integration]测试。

Rust 也支持在测试中指定额外的依赖：

* [dev 依赖][dev-dependencies]

## 参见

* [TRPL][doc-testing] 中关于测试的章节
* [API 指导原则][doc-nursery]中关于文档测试的部分

[unit]: testing/unit_testing.html
[doc]: testing/doc_testing.html
[integration]: testing/integration_testing.html
[dev-dependencies]: testing/dev_dependencies.html
[doc-testing]: https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html
[doc-nursery]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html
