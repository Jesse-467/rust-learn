# Rust 高频标准库方法速查（按课更新）

> 面向 Go 开发者的随查手册：每学完一个核心类型，就在这里补上它的高频方法和底层原理。

---

## 0. 先看懂两种调用语法：`Type::func()` vs `val.method()`

| 写法 | 含义 | Go 类比 |
|---|---|---|
| `String::from("hi")` | **关联函数**（不依赖实例，常用于构造） | `strings.NewReader("hi")` / `NewUser()` |
| `s.push_str("!")` | **方法**（第一个参数是 `self` / `&self` / `&mut self`） | `buf.WriteString("!")` |
| `vec![1, 2, 3]` | **宏**（带 `!`，编译期展开成代码） | 无直接对应，类似 `[]int{1, 2, 3}` |

---

## 1. 通用方法：`.clone()`

- **作用**：显式深拷贝一份值（比如 `String` 会新分配一块堆内存并把字节复制过去）。
- **底层原理**：实现自 `Clone` trait。Rust 故意不把堆拷贝做成隐式的，因为拷贝大数据有性能成本——你在代码里看到 `.clone()`，就一眼知道这里发生了内存分配和拷贝。
- **Go 对比**：Go 里没有统一的 `.Clone()`，通常要手写 `append([]T(nil), s...)` 或 `slices.Clone(s)`。

---

## 2. `String`（拥有堆内存、可增长）与 `&str`（只读字符串切片）

### 创建 `String`（从 `&str` 转 `String`）

| 写法 | 原理与区别 | 推荐度 |
|---|---|---|
| `String::from("hello")` | 在堆上分配内存，把 `"hello"` 的 UTF-8 字节拷进去 | ⭐⭐⭐ 最直观 |
| `"hello".to_string()` | 通过 `Display` trait 转成 `String`，对 `&str` 效果和 `String::from` 完全一样 | ⭐⭐⭐ 写起来最短 |
| `"hello".to_owned()` | 明确表达“把借用的 `&str` 变成拥有所有权的 `String`” | ⭐⭐ 表达意图清晰 |
| `String::new()` | 创建空 `String`（**此时不分配堆内存**，cap=0，第一次 push 才分配） | ⭐⭐⭐ |
| `String::with_capacity(64)` | 预分配 64 字节堆容量，避免频繁扩容（等价 Go `strings.Builder.Grow(64)`） | ⭐⭐⭐ 性能敏感时用 |
| `format!("hi, {name}")` | 类似 Go 的 `fmt.Sprintf`，返回新 `String`，**不会 Move 传入的变量** | ⭐⭐⭐ 拼接首选 |

### 修改 `String`（需要 `mut` 或 `&mut String`）

| 方法 | 说明 | 原理 |
|---|---|---|
| `s.push('!')` | 追加单个 `char` | 直接写入尾部缓冲区，容量不够时按倍增策略扩容（同 Go `append`） |
| `s.push_str("abc")` | 追加一段 `&str` | 批量 memcpy 字节到尾部 |
| `s.clear()` | 清空内容，**保留已分配容量**（cap 不变，len 归 0） | 复用内存，等价 Go `b.Reset()` |

### 查询与切分（`&str` 和 `String` 都能调，因为 `String` 会自动解引用为 `&str`）

| 方法 | 返回类型 | 说明与原理 | Go 对照 |
|---|---|---|---|
| `s.len()` | `usize` | **字节长度**（O(1)，直接读字段） | `len(s)` |
| `s.is_empty()` | `bool` | 是否为空（clippy 推荐用它代替 `s.len() == 0`） | `len(s) == 0` |
| `s.trim()` | `&str` | 去掉两端空白，**零拷贝**，只调整返回的 `(ptr, len)` 范围 | `strings.TrimSpace(s)` |
| `s.find(' ')` | `Option<usize>` | 找第一个匹配的**字节下标**，找不到返回 `None` | `strings.IndexByte` |
| `s.contains("ab")` | `bool` | 是否包含子串 | `strings.Contains` |
| `s.starts_with("a")` / `ends_with` | `bool` | 前缀 / 后缀判断 | `strings.HasPrefix/Suffix` |
| `s.to_uppercase()` / `to_lowercase()` | `String` | 返回**新** `String`（Unicode 大小写可能改变字节长度，无法原地改） | `strings.ToUpper` |
| `s.bytes()` | 字节迭代器 | 逐字节遍历 `u8` | `for i := 0; i < len(s); i++` |
| `s.chars()` | 字符迭代器 | 逐 Unicode 字符遍历 `char` | `for _, r := range s` |
| `s.split(' ')` | 切片迭代器 | 按分隔符切分，**零拷贝**产出 `&str` | `strings.Split` |
| `s.as_bytes()` | `&[u8]` | 直接把底层 UTF-8 字节借出来当字节切片看（零拷贝） | `[]byte(s)`（但 Go 会拷贝！） |

---

## 3. `Vec<T>`（拥有堆内存、动态数组）与 `&[T]`（切片）

### 创建 `Vec<T>`

| 写法 | 说明与原理 | Go 对照 |
|---|---|---|
| `Vec::new()` | 空 `Vec`，**不分配堆内存**（cap=0） | `var s []T` |
| `Vec::with_capacity(n)` | 预分配容量 `n`，减少扩容拷贝 | `make([]T, 0, n)` |
| `vec![1, 2, 3]` | 宏：直接初始化元素 | `[]int{1, 2, 3}` |
| `vec![0; 10]` | 宏：创建 10 个 0 | `make([]int, 10)` |

### 修改 `Vec<T>`（需要 `mut` 或 `&mut Vec<T>`）

| 方法 | 说明与原理 | Go 对照 |
|---|---|---|
| `v.push(x)` | 尾部追加，容量不够时重新分配更大堆内存并搬家（所以会导致旧引用失效！） | `v = append(v, x)` |
| `v.pop()` | 弹出尾部元素，返回 `Option<T>`（空时返回 `None`，不会 panic） | 手动取尾+缩容 |
| `v.extend_from_slice(&[4, 5])` | 批量追加切片元素（要求 `T: Copy` 或 `Clone`） | `v = append(v, other...)` |
| `v.clear()` | 清空元素并逐个 Drop，保留容量 | `v = v[:0]` |

### 读取与切片操作（`&[T]`、`Vec<T>`、`[T; N]` 通用）

| 方法 | 返回类型 | 说明与原理 | Go 对照 |
|---|---|---|---|
| `s.len()` / `s.is_empty()` | `usize` / `bool` | 元素个数（O(1)） | `len(s)` |
| `s[i]` | `T`（借用） | 下标访问，越界会 **panic** | `s[i]` |
| `s.get(i)` | `Option<&T>` | 安全下标访问，越界返回 `None`，**不 panic** | 需手写判断 |
| `s.first()` / `s.last()` | `Option<&T>` | 取首/尾元素引用，空切片返回 `None` | 需手写判断 |
| `&s[a..b]` | `&[T]` | 子切片（零拷贝，只改 `(ptr, len)`） | `s[a:b]` |
| `s.split_at(mid)` | `(&[T], &[T])` | 在 `mid` 处拆成左右两个**零拷贝**子切片 | `s[:mid], s[mid:]` |
| `s.contains(&x)` | `bool` | 线性查找是否包含 `x` | `slices.Contains` |
| `s.swap(i, j)` | `()` | 原地交换两个下标的元素（需 `&mut`） | `s[i], s[j] = s[j], s[i]` |
| `std::mem::swap(a, b)` | `()` | 交换任意两个 `&mut T` 的值（零拷贝，直接换内存位） | `a, b = b, a` |
| `s.iter()` | 迭代器 | 借用遍历，产出 `&T`（不消费原集合） | `for _, v := range s` |
| `s.iter().enumerate()` | 迭代器 | 带下标遍历，产出 `(usize, &T)` | `for i, v := range s` |
