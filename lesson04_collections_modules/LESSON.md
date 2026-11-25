# 第 4 课：`Vec`、`HashMap` 与模块系统（`mod` / `pub` / `use`）

> 面向 Go 开发者的核心对照：这一课把你在 Go 里每天都在用的 `slice`、`map` 和 `package` 组织方式，一一映射到 Rust 里。

## 1. `Vec<T>`：进阶操作与所有权陷阱

第 2 课已经见过 `Vec<T>` 的创建与切片，这一课重点掌握实际开发中最常用的修改、过滤与遍历写法：

| 操作 | Rust 写法 | Go 对照 |
|---|---|---|
| 创建与预分配 | `Vec::with_capacity(n)` / `vec![1, 2, 3]` | `make([]int, 0, n)` |
| 尾部增删 | `v.push(x)` / `v.pop() -> Option<T>` | `append` / 手动切片 |
| 原地过滤 | `v.retain(\|x\| *x > 0)` | 手写双指针过滤 |
| 去重（需先排序） | `v.sort(); v.dedup();` | `slices.Sort` + `slices.Compact` |
| 按条件查找 | `v.iter().position(\|x\| *x == target)` | `slices.Index` |

**三种 `for` 遍历的区别（所有权关键点）**：
- `for x in &v`：`x` 是 `&T`，只读借用，循环后 `v` 还能继续用。
- `for x in &mut v`：`x` 是 `&mut T`，可原地修改元素（`*x += 1`）。
- `for x in v`：`x` 是 `T`，**消费 `v` 的所有权**，循环结束后 `v` 不能再使用！

## 2. `HashMap<K, V>` 与杀手级 `entry` API

Go 的 `map[K]V` 读取不存在的 key 会静默返回零值，且经常需要写 `v, ok := m[k]`；Rust 的 `HashMap` 放在 `std::collections::HashMap`（需要显式 `use`），而且没有零值陷阱：

- **查值**：`m.get(&k)` 返回 `Option<&V>`（不会插入默认值，也不会 panic）。
- **插值**：`m.insert(k, v)` 会拿走 `k` 和 `v` 的所有权，如果 key 已存在会覆盖并返回旧值 `Option<V>`。
- **高频神器 `entry` API（一次哈希查找搞定“不存在就插入、存在就修改”）**：
  ```rust
  use std::collections::HashMap;
  let mut counts: HashMap<&str, usize> = HashMap::new();
  for word in ["go", "rust", "rust"] {
      // 只查一次哈希表！不存在则插入 0，然后返回 &mut usize
      *counts.entry(word).or_insert(0) += 1;
  }
  ```
  如果默认值构造开销大（比如 `Vec::new()` 或 `String::new()`），用 `entry(k).or_default()` 或 `entry(k).or_insert_with(|| ...)`，避免每次都提前构造。

## 3. 模块系统：`mod`、`pub`、`use` 与 Crate

Go 按**目录即 package**、靠**首字母大小写**控制可见性；Rust 的规则更显式：

1. **一切默认私有**：不管是结构体、字段、函数还是子模块，不写 `pub` 外部就看不到（同模块及其子模块内部可以看到父模块的私有项）。
2. **`mod` 声明模块树**：在 `lib.rs` 中写 `pub mod ex01_vec;` 编译器才会去加载 `src/ex01_vec.rs`（或 `src/ex01_vec/mod.rs`）。
3. **精细可见性**：
   - `pub`：对外部 crate 完全公开。
   - `pub(crate)`：仅在当前 crate 内部公开（非常适合写内部工具函数，类比 Go 的 `internal/`）。
   - `pub(super)`：仅对父模块公开。
4. **路径与重导出**：
   - `crate::...` 从当前 crate 根部出发（绝对路径），`super::...` 指父模块，`self::...` 指当前模块。
   - `pub use internal::Config;` 可以把深层模块里的类型重导出到顶层，让调用方少写冗长路径。

## 4. 本课练习安排

1. 先运行随堂演示：`cargo run --example collections_demo`
2. 依次完成：
   - `src/ex01_vec.rs`：`Vec` 过滤、原地修改、排序去重与合并
   - `src/ex02_hashmap.rs`：词频统计、分组聚合（`entry` API）与双 Map 合并
   - `src/ex03_modules.rs`：模块嵌套、可见性控制与 `pub use` 重导出
3. 运行 `cargo test && cargo clippy` 验收，完成后推送到 `lesson04` 分支。
