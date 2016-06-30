前面的问题相当糟糕，因为对 `unwrap` 的回避迫使我们嵌套了一层又一层，而我们想要的只不过是将相应的变量取出来。那到底有没有什么方式来处理这种情况而不用 `panic`？在发现错误（`Err`）时，有效的行为是什么样的呢？事实表明有两点：

1. `panic!`，但我们已经尽可能回避这种情况
2. `return`，因为 `Err` 意味着它不能被处理

这就是引入 `try!` 的目的；它**几乎完全**[^1]等同于一个这样的 `unwrap`——对待错误（`Err`）采用返回的方式而不是 `panic。

{try.play}

这确实有了**极大的**改进，但仍存在有关 `map_err` 的棘手问题。其实是有办法避免这点的（似乎我们在好多地方都用到了），但我们仍缺少一些细节。首先我们必须学会如何得到更好的错误。
[^1]: 参考 [re-enter try!][re_enter_try]，查看更多详细内容。

### 参见：

[`Result`][result] 和 [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[re_enter_try]: ../../error/reenter_try.html
