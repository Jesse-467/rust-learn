//! 练习 1：`async fn` 与 `tokio::join!` 并发执行

use tokio::time::{Duration, sleep};

async fn fetch_part(val: i32, delay_ms: u64) -> i32 {
    sleep(Duration::from_millis(delay_ms)).await;
    val
}

/// 1.1 并发调用两次 `fetch_part`：一次取 `a`（延迟 40ms），一次取 `b`（延迟 40ms），
/// 返回两者之和。
/// 要求：用 `tokio::join!` 并发执行，使总耗时明显小于 80ms（测试会校验耗时）。
pub async fn concurrent_add(a: i32, b: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_concurrent_add() {
        let start = Instant::now();
        let sum = concurrent_add(15, 27).await;
        let elapsed = start.elapsed();
        assert_eq!(sum, 42);
        assert!(
            elapsed < Duration::from_millis(75),
            "应该并发执行而不是串行 await，实际耗时: {elapsed:?}"
        );
    }
}
