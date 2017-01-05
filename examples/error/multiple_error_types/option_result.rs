// 第一次尝试使用 `unwrap` 和没帮助的 panic（崩溃）。
fn double_first(vec: Vec<&str>) -> i32 {
    // 若输入的 vector 为空则引发 panic：
    let first = vec.first().unwrap();

    // 若元素不能解析成一个数字则引发 panic：
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let strings = vec!["tofu", "cheese", "bell pepper"];
    let empty = vec![];

    println!("The first doubled is {}", double_first(numbers));
    
    // 此行引发第一个 panic：
    println!("The first doubled is {}", double_first(strings));
    // ^ 注释掉此行，接着看第二个 panic。
    
    // 此行引发第二个 panic：
    println!("The first doubled is {}", double_first(empty));
}
