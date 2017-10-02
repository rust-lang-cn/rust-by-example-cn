`create` 静态方法以只写模式（write-only mode）打开一个文件。若文件已经存在，则旧内容将被销毁。否则，将创建一个新文件。

{create.rs}

Here's the expected successful output:

```
$ mkdir out
$ rustc create.rs && ./create
successfully wrote to out/lorem_ipsum.txt
$ cat out/lorem_ipsum.txt
Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

（和前面例子一样，我们鼓励你在失败条件下测试这个例子。）

还有一个更通用的 `open_mode` 方法，这能够以其他方式来来打开文件，如：read+write（读+写），追加（append），等等。
