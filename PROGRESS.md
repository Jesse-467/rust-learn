# Rust 学习进度

学员背景：Go 开发者，零 Rust 基础。教学方式：老师讲解 + 布置习题（`cargo test` 验收）+ 批改反馈。
分支规范：第 1–2 课保留在 `main` 分支；第 3 课起每课新建独立分支（如 `lesson03`）。

| 课次 | 主题 | 目录 | 分支 | 状态 |
|---|---|---|---|---|
| 第 1 课 | 基础语法：变量/可变性/shadowing、标量类型与溢出、函数与表达式、控制流、元组与数组 | `lesson01_basics` | `main` | 已完成 ✅ |
| 第 2 课 | 所有权、借用、`String` vs `&str`、切片 | `lesson02_ownership` | `main` | 已完成 ✅ |
| 第 3 课 | struct、enum、match、`Option` / `Result` 与错误处理 | `lesson03_structs_enums` | `lesson03` | 进行中 |
| 第 4 课 | Vec / HashMap / 模块与 crate | — | `lesson04` | 未开始 |
| 第 5 课 | 泛型、trait、生命周期 | — | `lesson05` | 未开始 |
| 第 6 课 | 闭包、迭代器、测试 | — | `lesson06` | 未开始 |
| 第 7 课 | 智能指针：Box / Rc / RefCell | — | `lesson07` | 未开始 |
| 第 8 课 | 并发：thread / channel / Arc<Mutex> / Send+Sync | — | `lesson08` | 未开始 |
| 第 9 课 | async + tokio | — | `lesson09` | 未开始 |
| 第 10+ 课 | 实战项目 | — | `lesson10` | 未开始 |

## 做题流程
1. 切换到对应课程分支（如 `git checkout lesson03`），进入课程目录（如 `cd lesson03_structs_enums`）
2. 先运行 `cargo run --example structs_enums_demo` 感受随堂示例
3. 把 `src/exNN_*.rs` 里的 `todo!()` 换成你的实现
4. `cargo test ex01` 跑单个练习，`cargo test` 跑全部
5. 完成后推送到你的 GitHub 分支并告诉老师“第 N 课做完了”，老师拉取后批改点评
