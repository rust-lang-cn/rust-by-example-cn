使用匹配链接结果会得到极其繁琐的内容；幸运的是，`try!` 宏可以使事情再次变得干净漂亮。`try!` 宏展开一个匹配表达式，其中 `Err(err)` 分支扩展了提前（返回）`return Err(err)`，同时 `Ok(ok)` 分支扩展成 `ok` 表达式。

{try.play}

记得查阅[文档][docs]，里面有很多匹配/组合 `Result`。

[docs]: http://doc.rust-lang.org/std/result/index.html
