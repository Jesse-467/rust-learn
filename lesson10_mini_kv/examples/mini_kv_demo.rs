//! 第 10 课实战演示：展示 Mini-KV 的协议定义与架构入口
//! （当你完成 ex01–ex03 后，可以把下面注释取消来端到端运行整个引擎！）
//! 运行方式：cargo run --example mini_kv_demo

fn main() {
    println!("=== Mini-KV 实战项目脚手架已就绪 ===");
    println!("支持命令: PING | SET <k> <v> [EX <ms>] | GET <k> | DEL <k> | KEYS");
    println!("请先完成 src/ex01_protocol.rs -> ex02_store.rs -> ex03_engine.rs，");
    println!("然后运行 `cargo test` 验收全部单元测试与多线程并发测试！");
}
