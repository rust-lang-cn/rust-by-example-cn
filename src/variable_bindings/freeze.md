# 冻结

当数据被相同的名称不变地绑定时，它还会**冻结**（freeze）。在不可变绑定超出作用域之前，无法修改已冻结的数据：

```rust,editable,ignore,mdbook-runnable
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;
}
```
