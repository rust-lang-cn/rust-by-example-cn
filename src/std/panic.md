# `panic!`

`panic!` 宏可用于产生一个 panic （恐慌），并开始回退（unwind）它的栈。在回退栈
的同时，运行时将会释放该线程所**拥有**的所有资源，这是通过调用线程中所有对象的
析构函数完成的。

因为我们正在处理的程序只有一个线程，`panic!` 将会引发程序报告 panic 消息并退出。

```rust,editable,ignore,mdbook-runnable
// 整型除法（/）的重新实现
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // 除以 0 会引发 panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// `main` 任务
fn main() {
    // 堆分配的整数
    let _x = Box::new(0i32);

    // 此操作将会引发一个任务失败
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` 应当会在此处被销毁
}
```

可以看到，`panic!` 不会泄露内存：

```bash
$ rustc panic.rs && valgrind ./panic
==4401== Memcheck, a memory error detector
==4401== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==4401== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==4401== Command: ./panic
==4401== 
thread '<main>' panicked at 'division by zero', panic.rs:5
==4401== 
==4401== HEAP SUMMARY:
==4401==     in use at exit: 0 bytes in 0 blocks
==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
==4401== 
==4401== All heap blocks were freed -- no leaks are possible
==4401== 
==4401== For counts of detected and suppressed errors, rerun with: -v
==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```
