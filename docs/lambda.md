我将从不同维度来组织闭包的知识，并通过表格形式展示：

## lambda 总结

```
继承关系：Fn -> FnMut -> FnOnce（箭头表示实现）
```

| 捕获方式 | 编译器推导规则 | 推导类型 | 可调用次数 | 示例代码 | 适用场景与注意事项 |
|---------|-------|-------|--------|--------|----------|
| 不可变借用 | 仅读取变量时 | `Fn` | 多次 | `let x = 1;`<br>`let f = \|\| println!("{}", x);` | - 适用于只需读取的场景<br>- 允许多个闭包同时捕获<br>- 原变量仍可使用 |
| 可变借用 | 需要修改变量时 | `FnMut` | 多次| `let mut x = 1;`<br>`let f = \|\| x += 1;` | - 适用于需要修改的场景<br>- 同时只能有一个闭包捕获<br>- 借用规则限制 |
| 所有权转移 | 需要消费变量时<br>或使用 move 显式声明所有权转移 |  `FnOnce` |一次 | `let s = String::new();`<br>`let f = move \|\| drop(s);` | - 跨线程场景必需<br>- 生命周期超出作用域<br>- 原变量不再可用 |


### 场景

| 场景 | 推荐使用的特征 | 是否需要 move | 示例代码 |
|-----|--------------|--------------|----------|
| 线程间通信 | `FnOnce` | 必需 | `thread::spawn(move \|\| {...})` |
| 迭代器转换 | `FnMut` | 可选 | `iter.map(\|x\| x * 2)` |
| 事件回调 | `Fn` | 可选 | `button.on_click(\|\| {...})` |
| 资源清理 | `FnOnce` | 推荐 | `defer(move \|\| cleanup())` |
| 状态维护 | `FnMut` | 根据需要 | `accumulate(\|acc\| acc += 1)` |

### 特殊情况

1. **Copy 类型的特殊处理**：
   - 即使使用 move，也是复制而非转移
   - 原变量仍可使用

2. **闭包类型的推导**：
   ```rust
   // 编译器根据使用方式推导
   let c1 = || println!("hello");           // Fn
   let c2 = || mut_var += 1;               // FnMut
   let c3 = move || drop(owned_var);       // FnOnce
   ```

3. **生命周期考虑**：
   - 借用捕获：闭包生命周期不能超过被捕获变量
   - move 捕获：闭包获得所有权，不受原变量生命周期限制

4. **性能考虑**：
   - 借用通常更高效（避免数据复制）
   - move 适用于小数据或必要场景
   - 对于 Copy 类型，性能差异通常可忽略

### 示例代码

注意观察下面代码每种 Lambda 的编译器是如何推导对环境的捕获方式以及推导出的 Lambda 的具体类型；

```rust
fn main() {

    // 默认推导为只读借用
    let data = vec![1, 2, 3];
    let closure = || println!("Data: {:?}", data);
    closure();
    println!("Can still use data: {:?}", data);

    // 使用 move 强制所有权转移
    let data = vec![1, 2, 3];
    let closure = move || println!("Data: {:?}", data);
    closure();
    // println!("Cannot use data: {:?}", data);  // 编译错误：data 已被移动

    // 推导为可变借用
    let mut data = vec![1, 2, 3];
    let mut closure = || {
        data.push(4);
        println!("Data: {:?}", data);
    };
    closure();
    closure();
    println!("Can still use data: {:?}", data);

    // 在需要跨线程使用闭包时，必须使用 move
    let data = vec![1, 2, 3];
    std::thread::spawn(move || {
        println!("Data in thread: {:?}", data);
    }).join().unwrap();
    // println!("{:?}", data);  // 编译错误：data 已被移动到新线程

    // Copy 类型的行为
    let x = 42; 
    let closure = move || println!("x: {}", x);
    closure();
    println!("Can still use x: {}", x);  // 正常工作，因为 i32 是 Copy 类型

    // 在返回闭包时使用 move
    let outer = String::from("hello");
    let factory = || {
        let capture = outer;  // 会转移所有权
        move |x| format!("{} {}", capture, x)  // 需要 move 来拥有 capture
    };
    let closure = factory();
    println!("Result: {}", closure("world"));
    // println!("{}", outer);  // 编译错误：outer 已被移动
    
}
```

### 流式处理

```rust
fn main() {
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve"];
    let scores = vec![85, 92, 78, 95, 88];

    let results: Vec<String> = names.iter()
        .zip(scores.iter())
        .filter(|(_, &score)| score >= 90)
        .map(|(&name, &score)| format!("{}: {}", name, score))
        .collect();

    println!("High Score Students:");
    results.iter().for_each(|r| println!("  {}", r));

    let (sum, count) = names.iter()
        .zip(scores.iter())
        .fold((0, 0), |acc, (_, &score)| {
            (acc.0 + score, acc.1 + 1)
        });

    println!("\nClass Average: {:.2}", sum as f64 / count as f64);
}
```