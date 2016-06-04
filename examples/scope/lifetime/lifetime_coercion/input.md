一个较长的生命周期可以强制转成一个较短的生命周期，使它在一个通常情况下不能工作的作用域内也能正常工作。这种形式出现在编译器推导强制转换的时候，也出现在声明生命周期不同的时候（原文：This comes in the form of inferred coercion by the Rust compiler,
and also in the form of declaring a lifetime difference）：

{coercion.play}
