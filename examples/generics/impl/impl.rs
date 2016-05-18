struct Val (f64,);
struct GenVal<T>(T,);

// Val 的实现（impl）
impl Val {
    fn value(&self) -> &f64 { &self.0 }
}

// GenVal 针对泛型类型 `T` 的实现
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.0 }
}

fn main() {
    let x = Val(3.0);
    let y = GenVal(3i32);
    
    println!("{}, {}", x.value(), y.value());
}

