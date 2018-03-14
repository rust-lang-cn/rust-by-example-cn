# 死代码 `dead_code`

编译器提供了 `dead_code`（死代码，无效代码） [*lint*][lint]，这会对未使用的函数
产生警告。可以用一个**属性**来禁用这个 lint。

```rust,editable
fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
```

注意在实际程序中，需要将死代码清除掉。在本书的例子中，由于它们是交互性的，因而
需要允许一些死代码的出现。

[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
