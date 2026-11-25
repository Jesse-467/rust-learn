# 第 5 课：泛型（Generics）、Trait 与生命周期（Lifetimes）

> 面向 Go 开发者的核心对照：这一课是 Rust 类型系统的灵魂——把 Go 的泛型约束、`interface` 和隐式内存逃逸，换成零成本抽象和编译期借用检查。

## 1. 泛型与单态化（零成本抽象）

Rust 的泛型写法和 Go 1.18+ 很像，但底层实现更彻底：编译器会在编译期为每个具体类型生成专用机器码（**单态化 Monomorphization**），运行期**零额外开销**。

```rust
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> { ... }
```

## 2. Trait：Rust 的“显式接口”

Go 的 `interface` 是**隐式实现**（鸭子类型，方法签名对上就算实现）；Rust 的 `trait` 必须**显式 `impl Trait for Type`**，并且支持默认方法实现和标准库运算符重载：

- **静态分发（推荐默认）**：`fn print_it(x: &impl Summary)` 或 `fn print_it<T: Summary>(x: &T)`，编译期展开，零虚表开销。
- **动态分发（需要异构集合时）**：`Vec<Box<dyn Summary>>` 或 `&dyn Summary`，通过胖指针 `(data_ptr, vtable_ptr)` 运行期派发，**这才是 Go `interface` 的真正等价物**！
- **常用标准库 Trait**：
  - `Display`（`{}` 用户友好格式化）vs `Debug`（`{:?}` 调试打印）
  - `From<T>` / `Into<T>`（类型转换，实现 `From` 会自动白送 `Into`，而且 `?` 运算符会自动调 `From` 做错误类型转换！）
  - `PartialOrd` / `Ord`（比较大小）、`PartialEq` / `Eq`（判等）

## 3. 生命周期标注（`'a`）

很多初学者看到 `<'a>` 会发怵，其实一句话就能看透：**生命周期标注永远不会改变变量的真实存活时间，它只是向编译器描述“输入引用和输出引用之间的存活关系”。**

```rust
// 告诉编译器：返回的 &str 至少和 x、y 中活得较短的那个一样久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

- **什么时候必须手写 `'a`？**
  1. 函数返回引用，且有**多个输入引用**（编译器猜不出返回值借自哪一个）。
  2. **结构体里存了引用字段**（如 `struct Excerpt<'a> { part: &'a str }`），表示结构体不能活得比它借用的数据更久。
- **省略规则（Elision）**：单输入引用、或 `&self` 方法返回引用时，编译器会自动推断，不用手写 `'a`。
- **`'static`**：存活于整个程序运行期（比如字符串字面量 `&'static str`）。

## 4. 本课练习安排

1. 先运行随堂演示：`cargo run --example generics_traits_demo`
2. 依次完成：
   - `src/ex01_generics.rs`：泛型函数、泛型结构体 `Pair<T>` 与条件 `impl`
   - `src/ex02_traits.rs`：自定义 Trait、`Display`、`From` 以及 `&dyn Trait` 动态分发
   - `src/ex03_lifetimes.rs`：函数生命周期标注、结构体引用字段与零拷贝解析
3. 运行 `cargo test && cargo clippy` 验收，完成后推送到 `lesson05` 分支。
