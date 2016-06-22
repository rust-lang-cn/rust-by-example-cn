// 我们在第一次时尝试使用 `unwrap`，返回没帮助的错误消息。
fn double_first(vec: Vec<&str>) -> i32 {
    // 若输入的 vector 为空则返回一个错误：
    let first = vec.first().unwrap();

    // 若元素不能解析成一个数字则返回一个错误：
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    
    // 此行引发第一个错误：
    println!("The first doubled is {}", double_first(empty));
    // ^ 注释掉此行，接着看第二个错误。
    
    // 此行引发第二个错误：
    println!("The first doubled is {}", double_first(strings));
}
