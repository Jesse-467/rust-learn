# 第 8 课：并发编程（`thread`、`channel`、`Arc<Mutex>`、`Send + Sync`）

> 面向 Go 开发者的核心对照：Go 用 `goroutine` + `channel` + `sync.Mutex` 写并发，数据竞争靠 `go test -race` 运行期抓；Rust 把**无畏并发（Fearless Concurrency）**写进了类型系统——**数据竞争在编译期就无法通过编译！**

## 1. OS 线程：`std::thread::spawn` 与 `thread::scope`

- `std::thread::spawn(move || { ... })`：启动一个操作系统线程，闭包必须用 `move` 接管数据所有权（因为子线程可能活得比父栈帧更久），返回 `JoinHandle<T>`，用 `.join().unwrap()` 等待结果。
- **`std::thread::scope`**（Rust 1.63+ 神器）：作用域线程！保证所有子线程在 `scope` 结束前全部 `join`，因此**子线程可以直接借用父线程栈上的局部切片，无需 `Arc` 或 `clone`**！

## 2. 消息传递：`std::sync::mpsc` Channel

Go 的哲学同样适用于 Rust：*Do not communicate by sharing memory; share memory by communicating.*

- `mpsc` = **Multiple Producer, Single Consumer**（多生产者、单消费者）：
  ```rust
  let (tx, rx) = std::sync::mpsc::channel();
  let tx2 = tx.clone(); // 克隆发送端给第二个生产者
  ```
- 发送 `tx.send(val)` 会把 `val` 的**所有权转移**到接收端，发送后原线程再也碰不到它，从根源消灭数据竞争。
- 当所有 `tx` 都被 `drop` 时，`for msg in rx` 循环会自动结束（类似 Go `close(ch)` 后 `for msg := range ch` 退出）。

## 3. 共享状态：`Arc<Mutex<T>>` 与 `Arc<RwLock<T>>`

对照 Go 的 `sync.Mutex` 最大区别：**Go 的锁和数据是分开的（靠人自觉先加锁再读数据）；Rust 的 `Mutex<T>` 把数据关在锁里面，不调 `.lock()` 根本拿不到 `T`！**

- `Arc<T>`：Atomic Reference Counted（原子引用计数，`Rc<T>` 的线程安全版）。
- `let mut guard = mutex.lock().unwrap();`：返回 `MutexGuard<T>`，**离开作用域自动解锁**（等价 Go 的 `mu.Lock(); defer mu.Unlock()`，但永远忘不了 `defer`；想提前解锁就显式 `drop(guard);`）。
- 读多写少场景用 `Arc<RwLock<T>>`（`.read().unwrap()` / `.write().unwrap()`）。

## 4. `Send` 与 `Sync`（编译期防并发 Bug 的守门员）

- **`Send`**：类型的所有权可以安全地跨线程转移（几乎所有基础类型都是 `Send`，但 `Rc<T>` 不是！）。
- **`Sync`**：类型的不可变引用 `&T` 可以安全地在多线程间共享（`T: Sync` 等价于 `&T: Send`；`RefCell<T>` 不是 `Sync`，所以多线程必须换 `Mutex<T>` / `RwLock<T>`）。

## 5. 本课练习安排

1. 先运行随堂演示：`cargo run --example concurrency_demo`
2. 依次完成：
   - `src/ex01_threads.rs`：`thread::spawn` 与 `thread::scope` 并行分块求和
   - `src/ex02_channels.rs`：`mpsc::channel` 多生产者汇总任务结果
   - `src/ex03_shared_state.rs`：`Arc<Mutex<T>>` 并发计数器与 `Arc<RwLock<HashMap>>` 线程安全缓存
3. 运行 `cargo test && cargo clippy` 验收，完成后推送到 `lesson08` 分支。
