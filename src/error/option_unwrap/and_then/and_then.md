`map()` 以链式调用的方式来简化 `match` 语句。然而，在返回类型是 `Option<T>` 的函数中使用 `map()` 会导致出现嵌套形式 `Option<Option<T>>`。多层链式调用也会变得混乱。所以有必要引入 `and_them()`，就像某些熟知语言中的 flatmap。

`and_then()` 使用包裹的值（wrapped value）调用其函数输入并返回结果。 如果 `Option` 是 `None`，那么它返回 `None`。


在下面例子中，`cookable_v2()` 会产生一个 `Option<Food>`。使用 `map()` 替代 `and_then()` 将会得到 `Option<Option<Food>>`，对 `eat()` 来说是一个无效类型。

```rust,editable
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// 我们没有原材料（ingredient）来制作寿司。
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// 我们拥有全部食物的食谱，除了欠缺高超的烹饪手艺。
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// 做一份好菜，我们需要原材料和食谱这两者。
// 我们可以借助一系列 `match` 来表达相应的逻辑：
// （原文：We can represent the logic with a chain of `match`es:）
fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None       => None,
        Some(food) => match have_recipe(food) {
            None       => None,
            Some(food) => Some(food),
        },
    }
}

// 这可以使用 `and_then()` 方便重写出更紧凑的代码：
fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
```

### 参见：

[闭包][closures]，[`Option::map()`][map], 和 [`Option::and_then()`][and_then]

[closures]: ../../fn/closures.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then 
