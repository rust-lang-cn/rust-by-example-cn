在处理嵌套循环的时候可以`中断（break）`或`继续（continue）`外层循环。在这类情形中，循环必须用一
些`'label`（标签）来注明，并且标签必须给 `break`/`continue` 语句通过（the label must be
passed to the `break`/`continue` statement.）。

{nested.play}
