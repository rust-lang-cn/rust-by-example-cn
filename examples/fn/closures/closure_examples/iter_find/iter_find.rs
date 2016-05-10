fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 产出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec 产出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 产出内容的引用是 `&&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 产出内容的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组 `iter()`  产出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常产出 `&i32``。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

