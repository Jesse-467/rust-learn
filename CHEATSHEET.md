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

---

## 4. `#[derive(...)]` 常用派生宏（第 3 课新增）

Rust 默认不会为自定义 `struct` / `enum` 生成打印、拷贝、判等能力，需要用 `#[derive(...)]` 让编译器在编译期自动生成代码：

| 派生 trait | 作用 | Go 对照 |
|---|---|---|
| `Debug` | 支持 `{:?}` / `{:#?}` 调试打印 | `fmt.Printf("%+v", x)` |
| `Clone` | 生成 `.clone()` 深拷贝方法 | 手写拷贝 |
| `Copy` | 标记为“按位浅拷贝、赋值不 Move”（要求所有字段都是 `Copy`，不能含 `String`/`Vec`） | Go 默认值拷贝 |
| `PartialEq` / `Eq` | 支持 `==` 和 `!=` 比较（写 `assert_eq!` 必备） | 结构体直接 `==` |
| `Default` | 生成 `T::default()` 默认值构造方法 | Go 零值（zero value） |

---

## 5. `Option<T>` 高频方法（替代 Go 的 `nil` / `(val, ok)`）

`Option<T>` 只有两个变体：`Some(T)` 和 `None`。

| 方法 / 写法 | 返回类型 | 说明与原理 | Go 对照 |
|---|---|---|---|
| `opt.is_some()` / `opt.is_none()` | `bool` | 判断有没有值 | `p != nil` / `p == nil` |
| `opt.unwrap()` | `T` | 取出值，**如果是 `None` 直接 panic**（仅建议在测试或绝对有把握时用） | 直接 `*p`（nil 时 panic） |
| `opt.expect("原因")` | `T` | 同 `unwrap()`，但 panic 时附带自定义报错信息 | 手写 panic 信息 |
| `opt.unwrap_or(default)` | `T` | `Some(v)` 返回 `v`，`None` 返回 `default`（`default` 会提前求值） | `if !ok { v = def }` |
| `opt.unwrap_or_else(\|\| ...)` | `T` | 同上，但只有 `None` 时才执行闭包（**惰性求值**，构造 `String`/`Vec` 时首选） | `if !ok { v = makeDef() }` |
| `opt.map(\|v\| ...)` | `Option<U>` | 有值就变换内部值，`None` 原样透传 | 需写 `if ok` |
| `opt.and_then(\|v\| ...)` | `Option<U>` | 变换函数本身也返回 `Option`，自动拍平（避免 `Option<Option<U>>`） | 嵌套 `if ok` |
| `opt.filter(\|v\| bool)` | `Option<T>` | 满足条件保留 `Some(v)`，不满足变成 `None` | `if ok && cond` |
| `opt.ok_or(err)` | `Result<T, E>` | 把 `Option` 转成 `Result`：`Some(v) -> Ok(v)`，`None -> Err(err)` | `if !ok { return err }` |
| `opt.take()` | `Option<T>` | （需 `&mut Option<T>`）把值**拿走**并在原地留下 `None`，常用于从结构体中移出字段 | `v := *p; *p = nil` |
| `opt.copied()` | `Option<T>` | 把 `Option<&T>` 转成 `Option<T>`（要求 `T: Copy`，如 `i32`） | 解引用拷贝 |
| `opt.as_ref()` | `Option<&T>` | 借用内部值而不消费原 `Option` | 取地址 |

---

## 6. `Result<T, E>` 高频方法与 `?` 运算符（替代 Go 的 `(val, error)`）

`Result<T, E>` 只有两个变体：`Ok(T)` 和 `Err(E)`。

| 方法 / 写法 | 返回类型 | 说明与原理 | Go 对照 |
|---|---|---|---|
| `expr?` | `T` | **错误传播运算符**：`Ok(v)` 解包出 `v`；`Err(e)` 立即从当前函数返回 `Err(e.into())` | `if err != nil { return zero, err }` |
| `res.is_ok()` / `res.is_err()` | `bool` | 判断成功还是失败 | `err == nil` / `err != nil` |
| `res.unwrap()` / `res.expect("msg")` | `T` | 成功取值，失败直接 panic（生产代码慎用，测试代码常用） | 忽略 err 或 panic |
| `res.unwrap_or(default)` | `T` | 失败时回退到默认值 | `if err != nil { v = def }` |
| `res.map(\|v\| ...)` | `Result<U, E>` | 成功时变换 `Ok` 里的值，`Err` 原样透传 | 需写 `if err != nil` |
| `res.map_err(\|e\| ...)` | `Result<T, F>` | 失败时把错误类型 `E` 转换成 `F`（常配合 `?` 使用） | `fmt.Errorf("...: %w", err)` |
| `res.ok()` | `Option<T>` | 丢弃错误细节，转成 `Option`：`Ok(v) -> Some(v)`，`Err(_) -> None` | `v, err := ...; if err == nil` |

---

## 7. 字符串解析与模式匹配常用技巧（第 3 课新增）

| 写法 | 说明与原理 | Go 对照 |
|---|---|---|
| `s.parse::<u16>()` | 解析字符串为目标类型（返回 `Result<u16, ParseIntError>`），`::<T>` 叫 **turbofish** 语法 | `strconv.Atoi` / `ParseUint` |
| `s.split_once('=')` | 按第一个分隔符拆成 `Option<(&str, &str)>`（零拷贝，比 `split` 更安全高效） | `strings.Cut(s, "=")` |
| `if let Some(x) = opt { ... }` | 只关心一种匹配模式时的简写（替代只处理一个分支的 `match`） | `if x, ok := ...; ok { ... }` |
| `let Some(x) = opt else { return None; };` | 守卫式解包：匹配不上就直接提前 `return` / `break`，减少嵌套缩进 | `if !ok { return }` |
| `matches!(val, Pat1 \| Pat2)` | 宏：判断值是否匹配某个模式，直接返回 `bool` | `switch` 返回 `bool` |

---

## 8. `Vec<T>` 进阶与 `HashMap<K, V>`（第 4 课新增）

| 方法 / 写法 | 返回类型 | 说明与原理 | Go 对照 |
|---|---|---|---|
| `v.retain(\|x\| cond)` | `()` | 原地过滤，保留满足条件的元素（O(n) 原地双指针） | 手写原地过滤 |
| `v.sort()` / `v.dedup()` | `()` | 原地排序 / 移除**相邻**重复元素（去重前通常先 `sort`） | `slices.Sort` + `slices.Compact` |
| `slice.to_vec()` | `Vec<T>` | 把切片拷贝成拥有所有权的 `Vec<T>` | `slices.Clone` |
| `HashMap::new()` / `with_capacity(n)` | `HashMap<K, V>` | 创建哈希表（需 `use std::collections::HashMap`） | `make(map[K]V, n)` |
| `m.get(&k)` | `Option<&V>` | 按引用查询，不插入默认值 | `v, ok := m[k]` |
| `m.insert(k, v)` | `Option<V>` | 插入并接管所有权，返回被覆盖的旧值 | `m[k] = v` |
| `m.entry(k).or_insert(def)` | `&mut V` | **单次哈希查找**：无则插默认值，返回可变引用 | 需先查再写 |
| `m.entry(k).or_default()` | `&mut V` | 同上，按 `V::default()` 惰性插入（如 `Vec::new()` / `0`） | 零值初始化 |
| `m.entry(k).or_insert_with(\|\| ...)` | `&mut V` | 同上，惰性闭包构造默认值 | `if !ok { m[k] = make() }` |
