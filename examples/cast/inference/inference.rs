fn main() {
    // 借助类型标注，编译器知道 `elem` 具有 u8 类型。
    let elem = 5u8;

    // 创建一个空 vector（可增长数组）。
    let mut vec = Vec::new();
    // 此时编译器并未知道 `vec` 的确切类型，它只知道 `vec` 是一个含有某种类型的 vector（`Vec<_>`）。

    // 将 `elem` 插入 vector。
    vec.push(elem);
    // Aha！现在编译器就知道了 `vec` 是一个含有 `u8` 类型的 vector(`Vec<u8>`)
    // 试一试 ^ 尝试将 `vec.push(elem)` 那行注释掉

    println!("{:?}", vec);
}
