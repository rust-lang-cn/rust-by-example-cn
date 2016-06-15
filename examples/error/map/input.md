`match` 是处理 `Option` 的一个有效方法。但是你最终会发现很多用例都相当繁琐；操作只有一个有效输入的情况尤其如此。

对于一些需要用到简单映射`Some -> Some` 和 `None -> None` 的情况，`Option` 有一个内置方法 `map()`。

多个不同的 `map()` 调用为更灵活运用可以链式连接在一起。在下面例子中，`process()` 轻松取代了前面的所有函数，且更加紧凑。

{map.play}

### 参见：

[闭包][closures]

[closures]: ../fn/closures.html
