struct Container(i32, i32);

// 这个 trait 检查 2 个项是否存到容器中。
// 还会获得第一个值或最后一个值。
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // 显式指出需要 `A` 和 `B`
    fn first(&self) -> i32; // 未显式指出需要 `A` 或 `B`
    fn last(&self) -> i32;  // 未显式指出需要 `A` 或 `B`
}

impl Contains<i32, i32> for Container {
    // 如果存储的数字相等则为真。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}

// `C` 包含 `A` 和 `B` 。鉴于此，必须重复表达 `A` 和 `B` 真麻烦。
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
