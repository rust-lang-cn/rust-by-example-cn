模块可以分配到文件/目录的层次结构中。让我们将[可见性小节例子][visibility]
的代码拆开分到多个文件中：

```
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

{split.rs}

{my/mod.rs}

{my/nested.rs}

{my/inaccessible.rs}

我们看到代码仍然正常运行，就和前面的一样：

```
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

[visibility]: ../mod/visibility.html
