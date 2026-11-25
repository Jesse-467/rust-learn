//! 第 10 课实战项目：多线程内存 KV 存储引擎（Mini-KV）
//!
//! 做题顺序：
//! 1. 阅读 `LESSON.md` 了解整体架构与协议
//! 2. 依次完成 ex01_protocol.rs -> ex02_store.rs -> ex03_engine.rs
//! 3. 验收：cargo test && cargo clippy

#![allow(unused_variables, dead_code, unused_imports)]

pub mod ex01_protocol;
pub mod ex02_store;
pub mod ex03_engine;

pub use ex01_protocol::{Command, KvError};
pub use ex02_store::KvStore;
pub use ex03_engine::KvEngine;
