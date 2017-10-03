# 外部语言函数接口

Rust 提供了外部语言函数接口（Foreign Function Interface，FFI）到 C 语言库。外部语言函数必须声明在一个 `extern` 代码块，且该代码块要带有一个包含外部语言库名称的 `#[link]` 属性。

```rust,ignore
use std::fmt;

// 此外部代码块链接到 libm 库
#[link(name = "m")]
extern {
    // 这是外部语言函数
    // 这计算了一个单精度复数的平方根
    fn csqrtf(z: Complex) -> Complex;
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // 调用一个外部语言函数是一种不安全的操作
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("the square root of {:?} is {:?}", z, z_sqrt);
}

// 最小化实现单精度复数
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```

由于调用外部语言函数通常被认为是不安全的，因此围绕它们编写安全的装包代码是相当普遍的。

```rust,ignore
use std::fmt;

#[link(name = "m")]
extern {
    fn ccosf(z: Complex) -> Complex;
}

// 安全装包（原文：safe wrapper）
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = 0 + 1i
    let z = Complex { re: 0., im: 1. };

    println!("cos({:?}) = {:?}", z, cos(z));
}

// 最小化实现单精度复数
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```
