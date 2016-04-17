类型推导引擎是相当智能的。它不仅仅在初始化期间分析[右值][rvalue]的类型，还会通过分析变量在后面是
怎么使用的来推导该变量的类型。这里给出一个类型推导的高级例子：

{inference.play}

无需变量的类型标注，编译器和程序员都很开心（the compiler is happy and so is the
programmer）！

[rvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
