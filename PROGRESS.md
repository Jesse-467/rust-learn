# Rust 学习进度

学员背景：Go 开发者，零 Rust 基础。教学方式：老师讲解 + 布置习题（`cargo test` 验收）+ 批改反馈。
分支规范：第 1–2 课保留在 `main` 分支；第 3 课起每课新建独立分支（如 `lesson03`–`lesson10`）。

| 课次 | 主题 | 目录 | 分支 | 状态 |
|---|---|---|---|---|
| 第 1 课 | 基础语法：变量/可变性/shadowing、标量类型与溢出、函数与表达式、控制流、元组与数组 | `lesson01_basics` | `main` | 已完成 ✅ |
| 第 2 课 | 所有权、借用、`String` vs `&str`、切片 | `lesson02_ownership` | `main` | 已完成 ✅ |
| 第 3 课 | struct、enum、match、`Option` / `Result` 与错误处理 | `lesson03_structs_enums` | `lesson03` | 已布置 ✅ |
| 第 4 课 | Vec / HashMap / 模块与 crate | `lesson04_collections_modules` | `lesson04` | 已布置 ✅ |
| 第 5 课 | 泛型、trait、生命周期 | `lesson05_generics_traits_lifetimes` | `lesson05` | 已布置 ✅ |
| 第 6 课 | 闭包、迭代器、测试 | `lesson06_closures_iterators` | `lesson06` | 已布置 ✅ |
| 第 7 课 | 智能指针：Box / Rc / RefCell | `lesson07_smart_pointers` | `lesson07` | 已布置 ✅ |
| 第 8 课 | 并发：thread / channel / Arc<Mutex> / Send+Sync | `lesson08_concurrency` | `lesson08` | 已布置 ✅ |
| 第 9 课 | async + tokio | `lesson09_async_tokio` | `lesson09` | 已布置 ✅ |
| 第 10 课 | 实战项目：多线程内存 KV 存储引擎 | `lesson10_mini_kv` | `lesson10` | 已布置 ✅ |

## 做题流程
1. 切换到对应课程分支（如 `git checkout lesson09`），进入课程目录（如 `cd lesson09_async_tokio`）
2. 先阅读目录下的 `LESSON.md` 教案，再运行随堂演示 `cargo run --example ...`
3. 把 `src/exNN_*.rs` 里的 `todo!()` 换成你的实现
4. `cargo test ex01` 跑单个练习，`cargo test && cargo clippy` 跑全部验收
5. 完成后推送到对应分支并告诉老师“第 N 课做完了”，老师拉取后批改点评
