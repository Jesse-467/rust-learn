# 第 9 课：异步编程（`async`/`await` 与 `tokio`）

> 面向 Go 开发者的核心对照：Go 语言内置了 goroutine 调度器（有栈协程）；Rust 标准库只定义了 `Future` trait 和 `async`/`await` 语法（无栈状态机，零运行时开销），而把调度器交给生态——**`tokio`** 就是 Rust 异步生态的事实标准。

## 1. `Future` 是惰性的！

在 Go 里 `go doWork()` 立刻就会开跑；在 Rust 里调用一个 `async fn` **什么都不会执行**，它只是返回一个实现了 `Future` 的状态机对象，直到你 `.await` 或交给 `tokio::spawn` 才真正驱动它运行！

## 2. 四大高频异步原语（与 Go 逐一对照）

| 场景 | Rust (`tokio`) | Go 对照 |
|---|---|---|
| 启动后台轻量任务 | `let h = tokio::spawn(async move { ... });` | `go func() { ... }()` |
| 并发等待多个任务 | `let (a, b) = tokio::join!(f1(), f2());` | `sync.WaitGroup` / `errgroup` |
| 谁先完成取谁（取消其余） | `tokio::select! { a = f1() => ..., b = f2() => ... }` | `select { case <-ch1: ... case <-ch2: ... }` |
| 超时控制 | `tokio::time::timeout(Duration::from_millis(50), fut).await` | `context.WithTimeout` + `select` |
| 异步通道 | `tokio::sync::mpsc::channel(cap)` / `oneshot::channel()` | `make(chan T, cap)` |

## 3. Go 开发者最容易踩的 2 个异步坑

1. **千万不要在 `async` 里调用阻塞操作**：例如 `std::thread::sleep` 或长时间 CPU 密集计算会卡死 Tokio 的工作线程！定时等待用 `tokio::time::sleep(...).await`，重 CPU / 同步 IO 丢给 `tokio::task::spawn_blocking`。
2. **不要跨 `.await` 持有 `std::sync::MutexGuard`**：标准库互斥锁守卫不是 `Send`，跨 `.await` 会导致编译报错（如果锁内需要 `.await`，用 `tokio::sync::Mutex`，否则尽量缩小锁作用域在 `.await` 前释放）。

## 4. 本课练习安排

1. 先运行随堂演示：`cargo run --example async_tokio_demo`
2. 依次完成：
   - `src/ex01_async_basics.rs`：`async fn` 与 `tokio::join!` 并发加速
   - `src/ex02_tasks_channels.rs`：`tokio::spawn`、`tokio::sync::mpsc` 与 `oneshot` 请求-响应模式
   - `src/ex03_select_timeout.rs`：`tokio::time::timeout` 超时控制与 `tokio::select!` 竞速
3. 运行 `cargo test && cargo clippy` 验收，完成后推送到 `lesson09` 分支。
