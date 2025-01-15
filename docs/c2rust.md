## 工具转换

| 工具名称 | 主要功能 | 特点 | 限制 | GitHub 链接 |
|----------|----------|------|------|-------------|
| Corrode | 将 Haskell 编写的 C 代码转换器，能将 C 源文件转译为 Rust | - 工具完全自动化<br>- 代码会有警告及报错提示 | - 项目几乎停滞<br>- 仅支持 C 语言 | github.com/jameysharp/corrode |
| Citrus-rs | 使用 clang 解析 C 代码，并把其转换为 Rust 代码 | - 清理 C 代码<br>- 保留原始转译的 Rust 语法<br>- 链接到 C-lang 与 Rust-lcms<br>- 重新为项目构建 Rust 代码 | - 一次只能转换一个文件<br>- 需要对代码进行大量处理<br>- 部分转换的 Rust 代码无法编译运行 | github.com/citrus-rs/citrus |
| CRust | 将工具链在已知少的努力范围内将 C++ 代码转换为 Rust | - C/Rust 完法转换包含完整文件<br>- 支持多种输入/中间文件<br>- 包含自动化的预处理阶段<br>- 将 HeaderDefine 等头文件相关内容处理完整 | - 目前仅支持基础的语法转换<br>- 部分 C++ 特性可能无法完全转换 | github.com/NishanthSpShetty/crust/ |
| C2Rust | C99 的代码转译为 Rust，支持编译器转译和重构 | - 原 C 代码生成语义等同的 Rust 的 unsafe 代码<br>- 语言特性的自动转换<br>- 支持大型项目转换<br>- 手动干预功能完善 | - 转换后的代码存在大量 unsafe 可能需要进一步优化<br>- 部分复杂的 C++ 特性可能需要手动调整<br>- 对某些特殊语法支持有限 | github.com/immunant/c2rust |

从表格可以看出：
- Corrode 的主要限制是项目维护状态
- Citrus-rs 的限制主要在于文件处理能力和输出代码质量
- CRust 的限制在于语法支持范围
- C2Rust 虽然功能最全面，但转换后的代码可能需要优化