struct Val {
    val: f64
}

struct GenVal<T>{
    gen_val: T
}

// Val 的实现（impl）
impl Val {
    fn value(&self) -> &f64 { &self.val }
}

// GenVal 针对泛型类型 `T` 的实现
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    
    println!("{}, {}", x.value(), y.value());
}

