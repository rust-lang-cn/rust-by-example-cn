# 引用计数 `Rc`

当需要多个所有权时，可以使用 `Rc`（引用计数，Reference Counting 缩写）。`Rc` 跟踪引用的数量，这相当于包裹在 `Rc` 值的所有者的数量.

每当克隆一个 `Rc` 时，`Rc` 的引用计数就会增加 1，而每当克隆得到的 `Rc` 退出作用域时，引用计数就会减少 1。当 `Rc` 的引用计数变为 0 时，这意味着已经没有所有者，`Rc` 和值两者都将被删除。

克隆 `Rc` 从不执行深拷贝。克隆只创建另一个指向包裹值的指针，并增加计数。

```rust,editable
use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        {
            println!("--- rc_a is cloned to rc_b ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            // 如果两者内部的值相等的话，则两个 `Rc` 相等。
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
                        
            // 我们可以直接使用值的方法
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            
            println!("--- rc_b is dropped out of scope ---");
        }
        
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a is dropped out of scope ---");
    }
    
    // 报错！`rc_examples` 已经移入 `rc_a`。
    // 而且当 `rc_a` 被删时，`rc_examples` 也被一起删除。
    // println!("rc_examples: {}", rc_examples);
    // 试一试 ^ 注释掉此行代码
}
```

### 参见：

[std::rc][1] 和 [std::sync::arc][2]。

[1]: https://rustwiki.org/zh-CN/std/rc/index.html
[2]: https://rustwiki.org/zh-CN/std/sync/struct.Arc.html
