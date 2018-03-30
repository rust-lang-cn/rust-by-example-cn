# 子进程

`process::Output` 结构体表示已结束的子进程（child process）的输出，而
`process::Command` 结构体是一个进程创建者（process builder）。

```rust,editable
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}
```

（再试试上面的例子，给 `rustc` 命令传入一个错误的 flag）
