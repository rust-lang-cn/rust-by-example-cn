# 枚举

和前面相似，解构 `enum` 方式如下：

```rust,editable
// 需要 `allow` 来消除警告，因为只使用了一个变量。
#[allow(dead_code)]
enum Color {
    // 这三者仅由它们的名字来表示。
    Red,
    Blue,
    Green,
    // 这些元组含有类似的 `u32` 元素，分别对应不同的名字：颜色模型（color models）。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}
```

### 参见：

[`#[allow(...)]`][allow], [color models][color_models] 和 [`enum`][enum] 

[allow]: ./attribute/unused.html
[color_models]: http://en.wikipedia.org/wiki/Color_model
[enum]: ./custom_types/enum.html
