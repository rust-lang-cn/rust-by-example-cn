# 开发依赖

有时仅在测试中才需要一些依赖（比如基准测试相关的）。这种依赖要写在 `Cargo.toml`
的 `[dev-dependencies]` 部分。这些依赖不会传播给其他依赖于这个包的包。

比如说使用 `pretty_assertions`，这是扩展了标准的 `assert!` 宏的一个 crate。

文件 `Cargo.toml`:

```ignore
# 这里省略了标准的 crate 数据
[dev-dependencies]
pretty_assertions = "0.4.0"
```

文件 `src/lib.rs`:

```rust,ignore
// 仅用于测试的外部 crate
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

## 参见
[Cargo][cargo] 文档中关于指定依赖的部分。

[cargo]: https://doc.crates.io/specifying-dependencies.html
