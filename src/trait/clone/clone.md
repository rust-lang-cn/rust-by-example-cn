当处理资源时，默认的行为是在赋值或函数调用的同时将它们转移。但是我们有时候也需要得到一份资源的复制。

[`Clone`][clone] trait 正好帮助我们完成这任务。更普遍地，我们可以使用由 `Clone` trait 定义的方法。

{clone.play}

[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
