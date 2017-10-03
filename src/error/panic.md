# `panic`

我们将要看到的最简单的错误处理机制就是 `panic`。它会打印一个错误消息，开始展开任务（译注：感觉此句翻译不好，望指正，原文为 starts unwinding the task），且通常退出程序。这里我们显式地在错误条件上调用 `panic`：

```rust,editalbe,ignore,mdbook-runnable
fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
```
