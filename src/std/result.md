# 结果 `Result`

我们前面已经看到 `Option` 枚举可以用于函数可能失败的返回值，其中 `None` 可以返回以表明失败。但是有时要强调**为什么**一个操作会失败。为达成这点，我们提供了 `Result` 枚举。

`Result<T, E>` 枚举拥有两个变量：

* `Ok(value)` 表示操作成功，并装包操作返回的 `value`（`value` 拥有 `T` 类型）。
* `Err(why)`，表示操作失败，并装包 `why`，它（能按照所希望的方式）解释了失败的原因（`why` 拥有 `E` 类型）。

```rust,editalbe,ignore,mdbook-runnable
mod checked {
    // 我们想要捕获的数学“错误”
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // 此操作将会失败，反而让我们返回失败的理由，并装包成 `Err`
            Err(MathError::DivisionByZero)
        } else {
            // 此操作是有效的，返回装包成 `Ok` 的结果
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // 这是一个三层的匹配金字塔！
    // （原文：This is a three level match pyramid!）
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // 这会失败吗？
    println!("{}", op(1.0, 10.0));
}
```
