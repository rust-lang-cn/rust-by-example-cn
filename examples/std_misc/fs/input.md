`std::io::fs` 模块包含几个处理文件系统的函数。

{fs.rs}

下面是预期成功的输出：

```
$ rustc fs.rs && ./fs
`mkdir a`
`echo hello > a/b.txt`
`mkdir -p a/c/d`
`touch a/c/e.txt`
`ln -s ../b.txt a/c/b.txt`
`cat a/c/b.txt`
> hello
`ls a`
> a/b.txt
> a/c
`walk a`
> a/c
> a/c/b.txt
> a/c/e.txt
> a/c/d
> a/b.txt
`rm a/c/e.txt`
`rmdir a/c/d`
```

且 `a` 目录的最终状态为：

```
$ tree a
a
|-- b.txt
`-- c
    `-- b.txt -> ../b.txt

1 directory, 2 files
```

### 参见：

[`cfg!`][cfg]

[cfg]: ../attribute/cfg.html
