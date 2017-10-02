// 使用 `String` 作为错误类型
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // 若值存在则将 `Option` 转换成 `Result`。
       // 否则提供一个包含该字符串（`String`） 的 `Err`。
       .ok_or("Please use a vector with at least one element.".to_owned())
       // 回想一下，`parse` 返回一个 `Result<T, ParseIntError>`。
       .and_then(|s| s.parse::<i32>()
                      // 映射任意错误 `parse` 产生得到 `String`。
                      // （原文：Map any errors `parse` yields to `String`.）
                      .map_err(|e| e.to_string())
                      // `Result<T, String>` 成为新的返回类型，
                      // 我们可以给里面的数字扩大两倍。
                      .map(|i| 2 * i))
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
