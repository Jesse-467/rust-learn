# 第 10 课实战项目：多线程内存 KV 存储引擎（Mini-KV）

> 恭喜来到毕业实战！这个项目把前 9 课学过的所有核心技能串成一套真正可运行的后端基础组件：**`enum` 协议建模 + 自定义 `Result` 错误传播 + `HashMap` + `Arc<RwLock<T>>` 并发安全 + TTL 过期控制 + 迭代器快照持久化**。

## 1. 系统架构（3 层模块化拆分）

```text
命令行文本 ("SET user alice EX 5000")
        │
        ▼
[ex01_protocol]  Command::parse(&str) -> Result<Command, KvError>
        │
        ▼
[ex02_store]     KvStore (Arc<RwLock<HashMap<String, Entry>>>)
                 - 支持惰性 TTL 过期（Instant）
                 - 读多写少使用 RwLock
        │
        ▼
[ex03_engine]    KvEngine
                 - 执行单条/批量文本命令并返回协议响应
                 - 文本快照导出（dump_snapshot）与恢复（load_snapshot）
```

## 2. 支持的文本协议（类似精简版 Redis）

| 命令 | 语法 | 成功响应 |
|---|---|---|
| `PING` | `PING` | `"PONG"` |
| `SET` | `SET <key> <value>` 或 `SET <key> <value> EX <ttl_ms>` | `"OK"` |
| `GET` | `GET <key>` | 找到返回 `"<value>"`，未找到或已过期返回 `"(nil)"` |
| `DEL` | `DEL <key>` | 删除成功返回 `"1"`，不存在返回 `"0"` |
| `KEYS` | `KEYS` | 按字典序升序返回所有未过期 key，逗号分隔（空时返回 `""`） |

## 3. 设计亮点（值得体会的 Rust 工程习惯）

- **惰性过期（Lazy Expiration）**：`Entry` 存 `expires_at: Option<Instant>`。读取时若发现 `Instant::now() >= exp`，先视为不存在（或在写操作/定期清理 `purge_expired()` 时统一清理），和 Redis 的核心策略一致。
- **句柄轻量化克隆**：`KvStore` 和 `KvEngine` 都派生了 `#[derive(Clone)]`，克隆只是 `Arc::clone`，零数据拷贝即可分发给多个线程并发读写。
- **进阶挑战（可选扩展）**：全部测试通过后，你可以试着给 `KvEngine` 套一层 `tokio::net::TcpListener`，把它升级成一个真正能用 `nc` / `telnet` 连接的异步 KV 服务器！

## 4. 本课验收流程

1. 先运行演示框架：`cargo run --example mini_kv_demo`
2. 按顺序实现三个模块：
   - `src/ex01_protocol.rs`：命令枚举解析与 `KvError`
   - `src/ex02_store.rs`：线程安全 `KvStore` 与 TTL 过期逻辑
   - `src/ex03_engine.rs`：`KvEngine` 命令分发、快照导出/恢复与多线程并发测试
3. 验收：`cargo test && cargo clippy`，全部绿灯后推上 `lesson10` 分支喊老师做最终代码 Review！
