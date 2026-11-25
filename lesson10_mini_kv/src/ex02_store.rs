//! 模块 2：线程安全的内存 KV 存储（支持 TTL 过期）

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

impl Entry {
    fn is_expired(&self) -> bool {
        matches!(self.expires_at, Some(exp) if Instant::now() >= exp)
    }
}

#[derive(Clone, Default)]
pub struct KvStore {
    inner: Arc<RwLock<HashMap<String, Entry>>>,
}

impl KvStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// 2.1 写入键值，若 `ttl_ms` 为 `Some(ms)`，设置过期时间为 `Instant::now() + Duration::from_millis(ms)`。
    pub fn set(&self, key: String, value: String, ttl_ms: Option<u64>) {
        todo!()
    }

    /// 2.2 读取键值：如果不存在或已过期，返回 `None`；否则返回 `Some(value)`。
    pub fn get(&self, key: &str) -> Option<String> {
        todo!()
    }

    /// 2.3 删除键：只有当键存在且未过期时返回 `true`，否则返回 `false`。
    pub fn del(&self, key: &str) -> bool {
        todo!()
    }

    /// 2.4 返回所有**未过期**的 key，按字典序升序排列。
    pub fn keys(&self) -> Vec<String> {
        todo!()
    }

    /// 2.5 主动清理所有已过期的条目，返回清理掉的数量。
    /// 提示：`map.retain(|_, entry| !entry.is_expired())`
    pub fn purge_expired(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_basic_crud() {
        let store = KvStore::new();
        store.set("b".into(), "2".into(), None);
        store.set("a".into(), "1".into(), None);
        assert_eq!(store.get("a"), Some("1".into()));
        assert_eq!(store.keys(), vec!["a", "b"]);
        assert!(store.del("a"));
        assert!(!store.del("a"));
        assert_eq!(store.get("a"), None);
    }

    #[test]
    fn test_ttl_expiration() {
        let store = KvStore::new();
        store.set("temp".into(), "val".into(), Some(30));
        assert_eq!(store.get("temp"), Some("val".into()));
        thread::sleep(Duration::from_millis(45));
        assert_eq!(store.get("temp"), None);
        assert_eq!(store.keys(), Vec::<String>::new());
        assert_eq!(store.purge_expired(), 1);
    }
}
