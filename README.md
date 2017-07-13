# 通过例子学 Rust

[![Build Status][travis-image]][travis-link]  
> 已于 2016-08-07 翻译完全部内容，欢迎纠正——最后更新时间 2017-02-14  
> Chinese translation of the [Rust by Example][rust-by-example]

## 这是什么？

这是 [Rust by Example-CN][website-cn]（Rust by Example 中文版）网站的源代码，翻译自英文网站 [Rust by Example][website]！

## 如何贡献

参考 [CONTRIBUTING.md][how-to-contribute]。

## 如何生成静态网站

### Debian (Ubuntu) 发行版

安装 [Rust](http://www.rust-lang.org/install.html) 并执行以下命令：

```
sudo apt-get install nodejs npm subversion calibre
sudo ln -s /usr/bin/nodejs /usr/bin/node
```

### 非 Debian 类型的发行版

安装 Rust [nightly](http://www.rust-lang.org/install.html)，
`node`, `npm`, `subversion` 和 `calibre`。

### 编译指示

运行：

```
make all
make html pdf epub mobi
make test
```

使用 `make serve` 来查看结果。

### 详情

我们使用下述工具来生成静态页：

* [Rust][rust-lang] \o/
* [GitBook][gitbook]

`gitbook` 把 Markdown 文件生成静态页面 (查看工作原理[点击这里][gitbook-format])。

在运行 `gitbook` 之前，我们先使用 [src/main.rs][main-rs] 进行预处理。

此预处理过程有两个步骤：

### 生成 `SUMMARY.md` 文件

`SUMMARY.md` 是由
[examples/structure.json][structure] 文件生成得到。这个 JSON 文件对每个“示例”都包含类树形的结构。

每个示例包括：

* 一个 id, 如 `hello`
* 一个标题（title）, 如 `Hello World`
* 子节点（children），为可选项， 指向子节点，如 `null`
* 一个在 `examples` 下的子目录，如 [examples/hello][hello-folder]
* 一个在 examples/structure.json 里面的入口点，如 `{ "id": "hello", "title": "Hello World", "children": null }`
* 一些源文件，如 [examples/hello/hello.rs][hello-rs]
* 一个输入 Markdown 文件，如 [examples/hello/input.md][hello-md]

在处子级示例时，其目录必须包含父级示例的 id 值，如 `examples/variable/mut/input.md` 表明 `mut` 示例是在分级 `variable` 下。

### 处理 `input.md` 文件

我们将代码放到单独的源文件中，而不是在 `input.md` 文件中直接编辑。然后在预处理阶段将会把代码代码插入到 Markdown 文件中。

举个例子，为了插入源码文件 `hello.rs`，我们在 Markdown 文件中采用以下语法：

* `{hello.play}` 会把代码嵌入到一个在线编辑器中。
* `{hello.rs}` 会插入到静态文本中。
* `{hello.out}` 展示源程序的输出结果。

Makefile 提供了以下指令：

* `make`：构建 `update.rs` 并执行上述的前置步骤
* `make html`： 运行 `gitbook` 来生成 HTML 静态页文档
* `make pdf`： 运行 `gitbook` 来生成 PDF 格式的电子书
* `make epub`： 运行 `gitbook` 来生成 ePub 格式的电子书
* `make mobi`： 运行 `gitbook` 来生成 Mobi 格式的电子书
* `make serve`： 运行 `gitbook --serve` 来生成书籍内容并发布到 `localhost:4000` 进行预览
* `make test`：检查所有的 Rust 源文件是否有编译错误

## 其他语言版本

* [English](https://github.com/rust-lang/rust-by-example)
* [Japanese](https://github.com/rust-lang-ja/rust-by-example-ja)

## 授权协议

《通过例子学 Rust》（中文版与英文版 Rust by Example 均） 使用 Apache 2.0 license 和 MIT
license 两种协议进行授权。

详情参见 LICENSE-APACHE 和 LICENSE-MIT。

[rust-by-example]: https://github.com/rust-lang/rust-by-example
[travis-image]: https://travis-ci.org/rust-lang-cn/rust-by-example-cn.svg?branch=master
[travis-link]: https://travis-ci.org/rust-lang-cn/rust-by-example-cn
[website]: http://rustbyexample.com
[website-cn]: http://rustwiki.org/rust-by-example
[how-to-contribute]: CONTRIBUTING.md
[rust-lang]: http://www.rust-lang.org/
[gitbook]: http://www.gitbook.io
[gitbook-format]: https://github.com/GitbookIO/gitbook#book-format
[main-rs]: src/main.rs
[structure]: examples/structure.json
[hello-folder]: examples/hello
[hello-rs]: examples/hello/hello.rs
[hello-md]: examples/hello/input.md
