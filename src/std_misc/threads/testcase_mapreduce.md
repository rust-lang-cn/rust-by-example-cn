# 测试实例：map-reduce

Rust 使数据的并行化处理非常简单，在 Rust 中你无需面对并行处理的很多传统难题。

标准库提供了开箱即用的线程类型，把它和 Rust 的所有权概念与别名规则结合
起来，可以自动地避免数据竞争（data race）。

当某状态对某线程是可见的，别名规则（即一个可变引用 XOR 一些只读引用。译注：XOR
是异或的意思，即「二者仅居其一」）就自动地避免了别的线程对它的操作。（当需要同步
处理时，请使用 `Mutex` 或 `Channel` 这样的同步类型。）

在本例中，我们将会计算一堆数字中每一位的和。我们将把它们分成几块，放入不同的
线程。每个线程会把自己那一块数字的每一位加起来，之后我们再把每个线程提供的结果
再加起来。

注意到，虽然我们在线程之间传递了引用，但 Rust 理解我们是在传递只读的引用，因此
不会发生数据竞争等不安全的事情。另外，因为我们把数据块 `move` 到了线程中，Rust
会保证数据存活至线程退出，因此不会产生悬挂指针。

```rust,editable
use std::thread;

// 这是 `main` 线程
fn main() {

    // 这是我们要处理的数据。
    // 我们会通过线程实现 map-reduce 算法，从而计算每一位的和
    // 每个用空白符隔开的块都会分配给单独的线程来处理
    //
    // 试一试：插入空格，看看输出会怎样变化！
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // 创建一个向量，用于储存将要创建的子线程
    let mut children = vec![];

    /*************************************************************************
     * "Map" 阶段
     *
     * 把数据分段，并进行初始化处理
     ************************************************************************/

    // 把数据分段，每段将会单独计算
    // 每段都是完整数据的一个引用（&str）
    let chunked_data = data.split_whitespace();

    // 对分段的数据进行迭代。
    // .enumerate() 会把当前的迭代计数与被迭代的元素以元组 (index, element)
    // 的形式返回。接着立即使用 “解构赋值” 将该元组解构成两个变量，
    // `i` 和 `data_segment`。
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // 用单独的线程处理每一段数据
        //
        // spawn() 返回新线程的句柄（handle），我们必须拥有句柄，
        // 才能获取线程的返回值。
        //
        // 'move || -> u32' 语法表示该闭包：
        // * 没有参数（'||'）
        // * 会获取所捕获变量的所有权（'move'）
        // * 返回无符号 32 位整数（'-> u32'）
        //
        // Rust 可以根据闭包的内容推断出 '-> u32'，所以我们可以不写它。
        //
        // 试一试：删除 'move'，看看会发生什么
        children.push(thread::spawn(move || -> u32 {
            // 计算该段的每一位的和：
            let result = data_segment
                        // 对该段中的字符进行迭代..
                        .chars()
                        // ..把字符转成数字..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // ..对返回的数字类型的迭代器求和
                        .sum();

            // println! 会锁住标准输出，这样各线程打印的内容不会交错在一起
            println!("processed segment {}, result={}", i, result);

            // 不需要 “return”，因为 Rust 是一种 “表达式语言”，每个代码块中
            // 最后求值的表达式就是代码块的值。
            result

        }));
    }


    /*************************************************************************
     * "Reduce" 阶段
     *
     * 收集中间结果，得出最终结果
     ************************************************************************/

    // 把每个线程产生的中间结果收入一个新的向量中
    let mut intermediate_sums = vec![];
    for child in children {
        // 收集每个子线程的返回值
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // 把所有中间结果加起来，得到最终结果
    //
    // 我们用 “涡轮鱼” 写法 ::<> 来为 sum() 提供类型提示。
    //
    // 试一试：不使用涡轮鱼写法，而是显式地指定 intermediate_sums 的类型
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}


```

### 作业
根据用户输入的数据来决定线程的数量是不明智的。如果用户输入的数据中有一大堆空格
怎么办？我们**真的**想要创建 2000 个线程吗？

请修改程序，使得数据总是被分成有限数目的段，这个数目是由程序开头的静态常量决定的。

### 参见:
* [线程][thread]
* [向量][vectors]和[迭代器][iterators]
* [闭包][closures]、[移动][move]语义和[`move`闭包][move_closure]
* [解构][destructuring]赋值
* 使用[涡轮鱼写法][turbofish]帮助类型推断
* [unwrap vs. expect][unwrap]
* [枚举类型][enumerate]

[thread]: std_misc/threads.html
[vectors]: std/vec.html
[iterators]: trait/iter.html
[destructuring]: https://doc.rust-lang.org/book/second-edition/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values
[closures]: fn/closures.html
[move]: scope/move.html
[move_closure]: https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html#closures-can-capture-their-environment
[turbofish]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[unwrap]: error/option_unwrap.html
[enumerate]: https://doc.rust-lang.org/book/loops.html#enumerate
