## 比较

| 语言   | 关键字数量 | 特点描述                                                |
|--------|------------|---------------------------------------------------------|
| **C**  | ~32 个     | 简洁、低级别、稳定                                   |
| **C++**| ~82 个     | 丰富、 生态健全、历史负担高                 |
| **Go** | 25 个      | 简洁、高效、带 GC                   |
| **Rust**| ~52 个    | 安全、现代、学习曲线高                             |

## 分类

## 1. Rust 关键字

| 分类 | 关键字 |
|------|---------|
| 变量和类型 | `let`, `const`, `static`, `mut`, `type`, `struct`, `enum`, `trait`, `impl`, `fn`, `Self`, `self` |
| 控制流 | `if`, `else`, `match`, `loop`, `while`, `for`, `break`, `continue`, `return` |
| 模块系统 | `mod`, `pub`, `use`, `crate`, `super`, `self`, `extern` |
| 生命周期和所有权 | `move`, `ref`, `'static` |
| 其他 | `where`, `as`, `dyn`, `in`, `unsafe` , `async`, `await`|

## 2. 标准库常用词汇

| 分类 | 类型/特征 |
|------|-----------|
| 容器类型 | `Vec`, `String`, `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `VecDeque`, `LinkedList` |
| 智能指针 | `Box`, `Rc`, `Arc`, `Cell`, `RefCell`, `Mutex`, `RwLock`, `Pin`, `Cow` |
| 迭代器相关 | `Iterator`, `IntoIterator`, `FromIterator` |
| 错误处理 | `Option`, `Result`, `unwrap`, `expect`, `Ok`, `Err`, `Some`, `None`, `Try` |
| 并发相关 | `Thread`, `Send`, `Sync`, `Future` |
| 核心 Trait | `Default`, `Clone`, `Copy`, `Drop`, `Deref`, `DerefMut`|
| 比较 Trait | `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` |
| 类型转换 Trait | `From`, `Into`, `TryFrom`, `TryInto`, `AsRef`, `AsMut`, `Borrow`, `BorrowMut` |
| 运算符 Trait | `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`, `Index`, `IndexMut` |
| 功能性 Trait | `Display`, `Debug`, `ToString`, `FromStr` |

## 3. 惯用法词汇

| 分类 | 约定用语 |
|------|----------|
| 转换方法前缀 | `from_*`, `to_*`, `into_*`, `as_*` |
| 构建器模式 | `new`, `default`, `build`, `with_*`, `builder` |
| 容器操作方法 | `iter`, `iter_mut`, `into_iter`, `collect`, `map`, `filter`, `fold` |
| 错误处理方法 | `map_err`, `and_then`, `or_else`, `unwrap_or`, `unwrap_or_else` |
| 变量前后缀 | `raw_*`(原始指针), `*_ref`(引用), `*_mut`(可变引用), `*_ptr`(指针) |
| 类型后缀 | `*Iterator`, `*Error`, `*Result`, `*Builder` |
| 方法前缀 | `get_*`(获取器), `set_*`(设置器), `is_*`(布尔查询), `has_*`(存在性检查) |

注意事项：
1. `*` 表示可替换的部分
2. 这些命名约定是社区形成的最佳实践，不是强制标准
3. 某些约定可能在特定上下文中有所变化
4. 标准库 Trait 的实现通常遵循这些命名约定
