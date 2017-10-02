// 不可复制的类型。
struct Empty;
struct Null;

// 用到 `T` 的trait 泛型。
trait DoubleDrop<T> {
    // 定义一个关于调用者的方法，接受一个额外的单一参量 `T`，
    // 且没有任何操作。
    fn double_drop(self, _: T);
}

// 针对泛型参量 `T` 和调用者 `U` 实现了 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得了两个传入参数的所有权，并释放这两个参数。
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // 释放 `empty` 和 `null`。
    empty.double_drop(null);

    //empty;
    //null;
    // ^ 试一试：去掉这两行的注释。
}
