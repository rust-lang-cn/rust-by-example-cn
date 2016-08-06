Rust 提供了外部语言函数接口（Foreign Function Interface，FFI）到 C 语言库。外部语言函数必须声明在一个 `extern` 代码块，且该代码块要带有一个包含外部语言库名称的 `#[link]` 属性。

{ffi.rs}

{ffi.out}

由于调用外部语言函数通常被认为是不安全的，因此围绕它们编写安全的装包代码是相当普遍的。

{safe.rs}

{safe.out}
