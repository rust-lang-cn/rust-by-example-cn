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

        // 可变引用离开作用域
    }

    // `point` 的不可变引用再次可用。
    println!("Point now has coordinates: ({}, {}, {})",
             point.x, point.y, point.z);
}
