可变数据可以使用 `&mut T` 进行可变借用。这叫做**可变引用**（*mutable reference*），并赋予了借用者读/写访问能力。相反，`&T` 通过不可变引用（immutable reference）来借用数据，借用者可以读数据而不能更改数据：

{mut.play}

### 参见：
[`static`][static]

[static]: ../../scope/lifetime/static_lifetime.html
