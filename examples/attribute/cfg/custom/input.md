有部分条件如 `target_os` 在使用 `rustc` 时会隐式地提供，但是自定义条件必须使用 `--cfg` 标记来传给 `rustc`。

{custom.rs}

不使用自定义的 `cfg` 标记：

{custom.out}

使用自定义的 `cfg` 标记：

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
