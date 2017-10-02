// 使用 `String` 作为错误类型
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    // 若存在值时，则将 `Option` 转换成 `Result`。
    // 否则提供一个包含此 `String` 的 `Err`。
    let first = match vec.first() {
        Some(first) => first,
        None => return Err("Please use a vector with at least one element.".to_owned())
    };

    // 若 `parse` 操作正常的话，则将内部的数字扩大 2 倍。
    // 否则映射任意错误，来自 `parse` 产生的 `String`。
    // （原文：Double the number inside if `parse` works fine.
    // Otherwise, map any errors that `parse` yields to `String`.）
    match first.parse::<i32>() {
        Ok(i) => Ok(2 * i),
        Err(e) => Err(e.to_string()),
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}
