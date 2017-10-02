使用匹配链接结果会得到极其繁琐的内容；幸运的是，`try!` 宏可以使事情再次变得干净漂亮。`try!` 宏展开一个匹配表达式，其中 `Err(err)` 分支扩展了提前（返回）`return Err(err)`，同时 `Ok(ok)` 分支扩展成 `ok` 表达式。

```rust,editable
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败”了，那么 `DivisionByZero` 将被返回
        let ratio = try!(div(x, y));

        // 如果 `ln` “失败”了，那么 `NegativeLogarithm` 将被返回
        let ln = try!(ln(ratio));

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NegativeLogarithm
                    => "logarithm of negative number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

记得查阅[文档][docs]，里面有很多匹配/组合 `Result`。

[docs]: http://doc.rust-lang.org/std/result/index.html
