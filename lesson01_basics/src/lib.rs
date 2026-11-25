//! 第 1 课：基础语法（变量、类型、函数、控制流、复合类型）
//!
//! 做题方式：
//!   1. 打开每个 exNN_*.rs 文件，把 `todo!()` 替换成你的实现
//!   2. 运行对应的测试，例如：  cargo test ex01
//!   3. 全部通过后运行：        cargo test
//!   4. 最后跑一次：            cargo clippy   （看看有没有不地道的写法）

// 还没写完的函数参数会产生 "unused variable" 警告，这里先关掉，写完后可以删掉这一行
#![allow(unused_variables)]

pub mod ex01_variables;
pub mod ex02_functions;
pub mod ex03_control_flow;
pub mod ex04_compound;
