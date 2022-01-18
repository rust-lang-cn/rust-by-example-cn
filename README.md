# 通过例子学 Rust

[![Build Status][travis-image]][travis-link]
[![LICENSE-MIT](https://img.shields.io/badge/license-MIT-green)](https://raw.githubusercontent.com/rust-lang-cn/rust-by-example-cn/master/LICENSE-MIT)
[![LICENSE-APACHE](https://img.shields.io/badge/license-Apache%202-blue)](https://raw.githubusercontent.com/rust-lang-cn/rust-by-example-cn/master/LICENSE-APACHE)
[![GitHub last commit](https://img.shields.io/github/last-commit/rust-lang-cn/rust-by-example-cn?color=gold)](https://github.com/rust-lang-cn/rust-by-example-cn/commits/master)
[![GitHub contributors](https://img.shields.io/github/contributors/rust-lang-cn/rust-by-example-cn?color=pink)](https://github.com/rust-lang-cn/rust-by-example-cn/graphs/contributors)
![Locatized 100%](https://img.shields.io/badge/localized-100%25-purple)
[![rustwiki.org](https://img.shields.io/website?up_message=rustwiki.org&url=https%3A%2F%2Frustwiki.org)](https://rustwiki.org)

> Chinese translation of the [Rust By Example][rust-by-example]
>
> 本文档按照 [**Rust 文档翻译指引**](https://github.com/rust-lang-cn/rust-translation-guide)规范进行翻译。
> 首次于 2016-08-07 翻译完全部内容，欢迎纠正——最后更新时间 2022.1.19

通过例子学 Rust，Rust By Example 中文版（包含在线代码编辑器）。

## 使用说明

如果想阅读《通过例子学 Rust》，可以直接访问 [https://rustwiki.org/zh-CN/rust-by-example/][website-cn] 进行在线上阅读（**支持同一页面中英双语切换**）。（英文阅读地址：[https://doc.rust-lang.org/rust-by-example/][website]）

若想在本地阅读，请先[安装 Rust][install Rust]，然后进行下面操作：

```bash
$ git clone https://github.com/rust-lang-cn/rust-by-example-cn
$ cd rust-by-example-cn
$ cargo install mdbook
$ mdbook build
$ mdbook serve
```

为了能够运行这些示例，你必须要连接到网络；当然你可以离线阅读所有这些内容。

## 如何贡献

请查看 [CONTRIBUTING.md][how-to-contribute] 文件了解详细内容。

## 其他语言版本

* [英语](https://github.com/rust-lang/rust-by-example)
* [法语](https://github.com/Songbird0/FR_RBE)
* [日语](https://github.com/rust-lang-ja/rust-by-example-ja)
* [俄语](https://github.com/ruRust/rust-by-example)

## 授权协议

《通过例子学 Rust》（中文版与英文版 Rust By Example 均） 使用以下两种协议的任一种进行授权：

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
