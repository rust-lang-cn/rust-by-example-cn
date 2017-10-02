Rust 通过 `spawn` 函数提供了创建本地操作系统（native OS）线程的机制，该函数的参数是一个转移闭包（moving closure）。

{threads.play}

这些线程由操作系统调度（schedule）。
