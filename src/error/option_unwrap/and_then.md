# 组合算子：`and_then`

`map()` 以链式调用的方式来简化 `match` 语句。然而，如果以返回类型是 `Option<T>`
 的函数作为 `map()` 的参数，会导致出现嵌套形式 `Option<Option<T>>`。这样多层串联调用就会变得混乱。所以有必要引入 `and_then()`，在某些语言中它叫做 flatmap。

`and_then()` 使用被 `Option` 包裹的值来调用其输入函数并返回结果。 如果 `Option`
 是 `None`，那么它返回 `None`。

在下面例子中，`cookable_v2()` 会产生一个 `Option<Food>`。如果在这里使用 `map()`
而不是 `and_then()` 将会得到 `Option<Option<Food>>`，这对 `eat()` 来说是一个无效类型。

```rust,editable
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// 我们没有制作寿司所需的原材料（ingredient）（有其他的原材料）。
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// 我们拥有全部食物的食谱，除了法国蓝带猪排（Cordon Bleu）的。
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}


// 要做一份好菜，我们需要原材料和食谱。
// 我们可以借助一系列 `match` 来表达这个逻辑：
fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None       => None,
        Some(food) => match have_recipe(food) {
            None       => None,
            Some(food) => Some(food),
        },
    }
}

// 也可以使用 `and_then()` 把上面的逻辑改写得更紧凑：
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

[closures]: ../../fn/closures.md
[map]: https://rustwiki.org/zh-CN/std/option/enum.Option.html#method.map
[and_then]: https://rustwiki.org/zh-CN/std/option/enum.Option.html#method.and_then
