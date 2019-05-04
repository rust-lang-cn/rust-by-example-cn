# 通过例子学 Rust

[![Build Status][travis-image]][travis-link]  
> Chinese translation of the [Rust By Example][rust-by-example]
>
> 本文档按照 [**Rust 文档翻译指引**](https://github.com/rust-lang-cn/rust-translation-guide)规范进行翻译。  
> 首次于 2016-08-07 翻译完全部内容，欢迎纠正——最后更新时间 2019.5.3  
> 近段时间将跟随英文版进行升级调整，欢迎大家踊跃参与，共同更新内容 —— 2019.4.20  

通过例子学 Rust，Rust By Example 中文版（包含在线代码编辑器）。

## 使用说明

如果想阅读《通过例子学 Rust》，可以直接访问 [https://rustwiki.org/zh-CN/rust-by-example/][website-cn] 进行在线上阅读。（英文阅读地址：[https://doc.rust-lang.org/rust-by-example/][website]）

若想在本地阅读，请先[安装 Rust][install Rust]，然后进行下面操作：

```bash
$ git clone https://github.com/rust-lang-cn/rust-by-example-cn
$ cd rust-by-example-cn
$ cargo install mdbook --version 0.2 --force
$ mdbook build
$ mdbook serve
```


## 如何贡献

请查看 [CONTRIBUTING.md][how-to-contribute] 文件了解详细内容。


## 其他语言版本

* [English](https://github.com/rust-lang/rust-by-example)
* [French](https://github.com/Songbird0/FR_RBE)
* [Japanese](https://github.com/rust-lang-ja/rust-by-example-ja)

## 授权协议

《通过例子学 Rust》（中文版与英文版 Rust by Example 均） 使用以下两种协议的任一种进行授权：

* Apache 2.0 授权协议，（[LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0）
* MIT 授权协议 ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)

可以根据自己选择来定。

除非您有另外说明，否则您在本仓库提交的任何贡献均按上述方式进行双重许可授权，就如 Apache 2.0 协议所规定那样，而无需附加任何其他条款或条件。



[install Rust]: https://www.rust-lang.org/tools/install
[rust-by-example]: https://github.com/rust-lang/rust-by-example
[travis-image]: https://travis-ci.org/rust-lang-cn/rust-by-example-cn.svg?branch=master
[travis-link]: https://travis-ci.org/rust-lang-cn/rust-by-example-cn
[website]: https://doc.rust-lang.org/rust-by-example/
[website-cn]: https://rustwiki.org/zh-CN/rust-by-example/
[how-to-contribute]: CONTRIBUTING.md
