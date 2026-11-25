//! 练习 3：`Arc<Mutex<T>>` 与 `Arc<RwLock<T>>`

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// 3.1 启动 `num_threads` 个线程，每个线程对同一个 `Arc<Mutex<u64>>` 累加 `increments_per_thread` 次，
/// 等待全部线程完成后返回最终计数值。
pub fn concurrent_increment(num_threads: usize, increments_per_thread: u64) -> u64 {
    todo!()
}

/// 3.2 封装一个线程安全的内存缓存 `SafeCache`（底层用 `Arc<RwLock<HashMap<String, String>>>`）。
/// 注意派生 `Clone`，这样克隆句柄后可以轻松传给多个线程共享同一份缓存！
#[derive(Clone, Default)]
pub struct SafeCache {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl SafeCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// 写入键值（写锁）
    pub fn set(&self, key: String, value: String) {
        todo!()
    }

    /// 读取键值（读锁，返回克隆后的 `Option<String>`）
    pub fn get(&self, key: &str) -> Option<String> {
        todo!()
    }

    /// 返回当前键个数
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_increment() {
        assert_eq!(concurrent_increment(8, 1000), 8000);
    }

    #[test]
    fn test_safe_cache_multithread() {
        let cache = SafeCache::new();
        let mut handles = Vec::new();
        for i in 0..4 {
            let c = cache.clone();
            handles.push(thread::spawn(move || {
                c.set(format!("k{i}"), format!("v{i}"));
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(cache.len(), 4);
        assert_eq!(cache.get("k2"), Some("v2".to_string()));
        assert_eq!(cache.get("missing"), None);
    }
}
