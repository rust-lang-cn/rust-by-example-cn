# 可变参数接口

可变参数接口可以接受任意数目的参数。比如说 `println` 就可以，其参数的数目是由格式化字符串指定的。

我们可以把之前的 `calculate!` 宏改写成可变参数接口：

```rust,editable
macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // 妈妈快看，可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
```

输出：

```txt
1 + 2 = 3
3 + 4 = 7
(2 * 3) + 1 = 7
```
