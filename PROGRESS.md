# Rust 学习进度

学员背景：Go 开发者，零 Rust 基础。教学方式：老师讲解 + 布置习题（`cargo test` 验收）+ 批改反馈。

| 课次 | 主题 | 目录 | 状态 |
|---|---|---|---|
| 第 1 课 | 基础语法：变量/可变性/shadowing、标量类型与溢出、函数与表达式、控制流、元组与数组 | `lesson01_basics` | 已完成 ✅ |
| 第 2 课 | 所有权、借用、`String` vs `&str`、切片 | `lesson02_ownership` | 进行中 |
| 第 3 课 | struct、enum、match、`Option` / `Result` 与错误处理 | — | 未开始 |
| 第 4 课 | Vec / HashMap / 模块与 crate | — | 未开始 |
| 第 5 课 | 泛型、trait、生命周期 | — | 未开始 |
| 第 6 课 | 闭包、迭代器、测试 | — | 未开始 |
| 第 7 课 | 智能指针：Box / Rc / RefCell | — | 未开始 |
| 第 8 课 | 并发：thread / channel / Arc<Mutex> / Send+Sync | — | 未开始 |
| 第 9 课 | async + tokio | — | 未开始 |
| 第 10+ 课 | 实战项目 | — | 未开始 |

## 做题流程
1. 进入课程目录，如 `cd d:\rust\rust_code\lesson01_basics`
2. 把 `src/exNN_*.rs` 里的 `todo!()` 换成你的实现
3. `cargo test ex01` 跑单个练习，`cargo test` 跑全部
4. 完成后告诉老师"第 N 课做完了"，老师会批改代码、点评写法
