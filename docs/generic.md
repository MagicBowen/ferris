## 泛型

| 特性               | 场景                     | 示例                                                                 |
|--------------------|--------------------------|------------------------------------------------------------------------|
| 泛型函数           | 编写与类型无关的函数       | `fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T { a + b }`       |
| 泛型结构体         | 编写与类型无关的结构体     | `struct Point<T> { x: T, y: T }`                                       |
| 特征系统 (`trait bounds`) | 通过 trait 对类型进行约束 | `fn process<T>(value: T) where T: std::fmt::Display + Clone,` |
| 关联类型 (`type alias`) | 定义类型族 | `trait Container { type Item; fn get(&self) -> Option<&Self::Item>; }` |
| 常量泛型 (`const generics`) | 编译期确定大小的数组   | `struct Array<T, const N: usize> { data: [T; N] }`     |
| 常量函数 (`const fn`) | 编译期计算               | `const fn square(x: i32) -> i32 { x * x }`     |
| 声明宏 (`Declarative Macros`) | 语法扩展 | `sql!(SELECT * FROM users WHERE id = 1)` |
| 过程宏 (`Procedural Macros`)  | 代码生成 | `#[derive(Debug, Clone)]` |
