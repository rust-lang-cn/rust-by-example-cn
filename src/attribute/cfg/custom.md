# 自定义条件

有部分条件如 `target_os` 是由 `rustc` 隐式地提供的，但是自定义条件必须使用
 `--cfg` 标记来传给 `rustc`。

```rust,editable,ignore,mdbook-runnable
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}
```

试试不使用自定义的 `cfg` 标记会发生什么：

```bash
$ rustc custom.rs && ./custom
No such file or directory (os error 2)
```

使用自定义的 `cfg` 标记：

```bash
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
