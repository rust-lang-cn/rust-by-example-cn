# 别名使用

数据可以多次不可变借用，但是在不可变借用的同时，原始数据不能使用可变借用。或者说，同一时间内只允许**一次**可变借用。仅当最后一次使用可变引用**之后**，原始数据才可以再次借用。

```rust,editable
struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 数据可以通过引用或原始类型来访问
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);
    
    // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
    // let mutable_borrow = &mut point;
    // TODO ^ 试一试去掉此行注释

    // 被借用的值在这里被重新使用
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
    // let y = &point.y;
    // TODO ^ 试一试去掉此行注释

    // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ 试一试去掉此行注释

    // 正常运行！可变引用能够以不可变类型传入 `println!`
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // 可变引用不再用于其余的代码，因此可以重新借用
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}
```
