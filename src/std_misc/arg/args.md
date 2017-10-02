命令行参数可使用 `std::env::args` 进行接收，这将返回一个迭代器，该迭代器会对各个参数产生一个字符串。

{args.play}

```
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```
