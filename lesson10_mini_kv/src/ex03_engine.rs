//! 模块 3：KV 引擎（命令分发 + 快照导入导出 + 并发压测）

use crate::ex01_protocol::{Command, KvError};
use crate::ex02_store::KvStore;

#[derive(Clone, Default)]
pub struct KvEngine {
    store: KvStore,
}

impl KvEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// 3.1 解析并执行单行文本命令：
    /// - `Ping` -> `Ok("PONG".into())`
    /// - `Set { key, value, ttl_ms }` -> 写入后返回 `Ok("OK".into())`
    /// - `Get(key)` -> 找到返回 `Ok(val)`，否则返回 `Ok("(nil)".into())`
    /// - `Del(key)` -> 删掉返回 `Ok("1".into())`，否则返回 `Ok("0".into())`
    /// - `Keys` -> 返回 `Ok(keys.join(","))`
    pub fn execute(&self, line: &str) -> Result<String, KvError> {
        todo!()
    }

    /// 3.2 导出文本快照：把当前所有未过期的键值按 key 升序导出为多行 `"key=value"`（每行一条，用换行符 `\n` 连接）。
    pub fn dump_snapshot(&self) -> String {
        todo!()
    }

    /// 3.3 从文本快照恢复数据（忽略空行；每行按第一个 `'='` 切分，若某行缺少 `'='` 返回 `Err(KvError::InvalidArgs(...))`），
    /// 返回成功加载的条数。
    pub fn load_snapshot(&self, snapshot: &str) -> Result<usize, KvError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_engine_commands_and_snapshot() {
        let eng = KvEngine::new();
        assert_eq!(eng.execute("PING"), Ok("PONG".into()));
        assert_eq!(eng.execute("SET user jesse"), Ok("OK".into()));
        assert_eq!(eng.execute("SET lang rust"), Ok("OK".into()));
        assert_eq!(eng.execute("GET user"), Ok("jesse".into()));
        assert_eq!(eng.execute("KEYS"), Ok("lang,user".into()));

        let snap = eng.dump_snapshot();
        assert_eq!(snap, "lang=rust
user=jesse");

        let eng2 = KvEngine::new();
        assert_eq!(eng2.load_snapshot(&snap), Ok(2));
        assert_eq!(eng2.execute("GET lang"), Ok("rust".into()));
        assert_eq!(eng2.execute("DEL user"), Ok("1".into()));
        assert_eq!(eng2.execute("GET user"), Ok("(nil)".into()));
    }

    #[test]
    fn test_engine_concurrent_access() {
        let eng = KvEngine::new();
        let mut handles = Vec::new();
        for i in 0..8 {
            let e = eng.clone();
            handles.push(thread::spawn(move || {
                e.execute(&format!("SET k{i} v{i}")).unwrap();
                assert_eq!(e.execute(&format!("GET k{i}")).unwrap(), format!("v{i}"));
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(eng.execute("KEYS").unwrap(), "k0,k1,k2,k3,k4,k5,k6,k7");
    }
}
