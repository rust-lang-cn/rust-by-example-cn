命令行参数可使用 `std::env::args` 进行接收，这将返回一个迭代器，该迭代器会对各个参数产生一个字符串。

```rust,editable
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 第一个参数是调用本程序的路径
    println!("My path is {}.", args[0]);

    // 其余的参数充当一般的命令行参量。
    // 调用程序方式如下：
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
```

```
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```
