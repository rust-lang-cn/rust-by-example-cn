从前面的例子可以看到，当我们调用 `parse` 时，直接反应就是从一个错误消息库中映射到一个新的自定义错误类型（原文：the immediate reaction is to `map` the error from a library error into our new custom error type）。

```rust
.and_then(|s| s.parse::<i32>()
    .map_err(DoubleError::Parse)
```

这是一个很简单且常见的操作，要是能够编辑它的话将会相当方便，可惜的是，不能编辑。`and_then` 并没有灵活到可以处理这种情况；但是 `try!` 可以。

`try!` 在前面已经解释过，它可以充当 `unwrap` 或 `return Err(err)`，这描述只有 `93%` 是正确的（原文：`try!` has previously been explained as either `unwrap` or `return Err(err)` which is only `93%` correct.）。实际上它意味着 `unwrap` 或者 `return Err(From::from(err))`。由于 `From::from` 是一个不同类型间相互转换的工具，所以如果你 `try!` 一些内容，里面的错误是能够转换成返回类型，这将会自动转换。这意味着如果当 `From::from` 已对我们的错误类型提供实现时，我们使用 `try!` 重写这个例子，`map_err` 将会消失不见：

{rethink.play}

现在变得整洁多了。如果和原始的 `panic` 进行比较，可以看到它好像就是使用 `try!` 替换 `unwrap`，除了返回类型是 `Result` 类型这点不同，所以它们必须在顶层被解构出来。

然而，不要指望这种错误处理在实际中完全取代 `unwrap` 的用法。这种错误处理会使代码行数扩大三倍，即便是很小的一段代码也不能称之为简单。实际中要是将 1000 行库代码从 `unwrap` 移动到更适当的错误处理，可能会导致代码量增加 100 行，但这做法是可取的，当然这重构的过程并非易事。

这点非常合理。很多库可能只是抛弃了实现 `Display`，然后在所需的基础上增加 `From`（原文：Many libraries might get away with only implementing `Display` and then adding `From` on an as needed basis.）。正式的库甚至还包括了用户使用适当的期望方式来应对应该怎样实现错误处理（原文：A serious library though will have users with certain expections about how it should implement error handling.）。在这些情况下，错误处理需要被下一步操作来处理（原文：In those cases, the error handling will need to be taken one step further.）。

### 参见：

[`From::from`][from] 和 [`try!`][try]

[from]: http://doc.rust-lang.org/std/convert/trait.From.html
[try]: http://doc.rust-lang.org/std/macro.try!.html
