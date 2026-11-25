# 第 6 课：闭包（Closures）、迭代器（Iterators）与测试

> 面向 Go 开发者的核心对照：Go 习惯写显式 `for` 循环，而地道的 Rust 代码大量使用**惰性迭代器链**（`iter().filter().map().collect()`）——关键在于 Rust 的迭代器是**零成本抽象**，编译优化后速度和手写 `for` 循环一样快！

## 1. 闭包的三种捕获方式：`Fn`、`FnMut`、`FnOnce`

闭包语法 `|x, y| x + y`，编译器会根据闭包体如何使用外部变量自动决定捕获方式：

| Trait | 捕获方式 | 可调用次数 | 常见场景 |
|---|---|---|---|
| `Fn` | 不可变借用（`&T`） | 多次 | `filter`、`map` |
| `FnMut` | 可变借用（`&mut T`） | 多次 | 累加器、计数器 |
| `FnOnce` | 消费所有权（`T`） | 至少一次（可能只一次） | `unwrap_or_else`、`thread::spawn` |

- **`move` 关键字**：`move |x| x + offset` 强制把外部变量的所有权移进闭包（写多线程和 `async` 时几乎必用）。

## 2. 迭代器的三大核心原则

1. **惰性求值（Lazy）**：`v.iter().map(...).filter(...)` 只是拼装“流水线配方”，**不会真正遍历**，直到你调用消费方法（`collect`、`sum`、`fold`、`count`、`any`、`all`、`find`、`for_each`）才会一口气跑完。
2. **三种创建方式**：
   - `.iter()` -> 产出 `&T`（借用）
   - `.iter_mut()` -> 产出 `&mut T`（可变借用）
   - `.into_iter()` -> 产出 `T`（消费所有权）
3. **自定义迭代器只需实现一个方法**：`impl Iterator for MyIter { type Item = ...; fn next(&mut self) -> Option<Self::Item> }`，写完 `next` 就能白嫖标准库几十个适配器（`map`、`filter`、`take`、`zip`、`enumerate`、`fold`...）！

## 3. 测试小贴士

- `#[test]` 单元测试、`#[should_panic(expected = "...")]` 测试预期 panic。
- 文档注释 `///` 里的代码块会自动作为 **doctest** 被 `cargo test` 编译运行，保证文档永不过期。

## 4. 本课练习安排

1. 先运行随堂演示：`cargo run --example closures_iterators_demo`
2. 依次完成：
   - `src/ex01_closures.rs`：高阶函数接收 `Fn` / `FnMut` 与 `move` 闭包工厂
   - `src/ex02_iterators.rs`：用迭代器链完成过滤、映射、归约（`fold`）、`zip` 与 `flat_map`
   - `src/ex03_custom_iterator.rs`：手写一个斐波那契 `Iterator` 并配合 `.take()` / `.filter()` 使用
3. 运行 `cargo test && cargo clippy` 验收，完成后推送到 `lesson06` 分支。
