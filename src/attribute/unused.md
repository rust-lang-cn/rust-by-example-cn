# 死代码 `dead_code`

编译器提供了 `dead_code`（死代码，无效代码） [*lint*][lint]，这会对未使用的函数产生警告。可以加上**属性**来抑制这个 lint。

```rust,editable
fn used_function() {}

// `#[allow(dead_code)]` 属性可以抑制 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
```

注意在实际程序中，需要将死代码清除掉。在这些例子中，我们是出于知识点讲解的需要才特意加上了一些死代码。

[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
