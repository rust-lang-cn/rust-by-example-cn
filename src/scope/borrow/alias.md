# 别名使用

数据可以进行多次不可变借用，但是在不可变借用的期间，原始数据不可进行可变借用。也就是说，在同一段时间内只允许**单独一个**可变借用。原始数据在可变引用离开作用域**之后**可再次被借用。

```rust,editable
struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // 通过引用和原始所有者来访问数据
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // 报错！不能借用 `point` 作为可变内容，因为目前已被借用成为
        // 不可变内容。
        //let mutable_borrow = &mut point;
        // 动手试一试 ^ 将此行注释去掉。

        // 不可变引用离开作用域
    }

    {
        let mutable_borrow = &mut point;

        // 通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错！不能借用 `point` 作为不可变内容，因为目前它已被借用成为
        // 可变内容。
        //let y = &point.y;
        // 动手试一试 ^ 将此行注释去掉。

        // 报错！不能打印，因为 `println!` 接受了一个不可变引用。
        //println!("Point Z coordinate is {}", point.z);
        // 动手试一试 ^ 将此行注释去掉。

        // 好！可变引用可以作为不可变的传给 `println!`。
        println!("Point has coordinates: ({}, {}, {})",
                 mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // 可变引用离开作用域
    }

    // `point` 的不可变引用再次可用。
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             borrowed_point.x, borrowed_point.y, borrowed_point.z);
}
```
