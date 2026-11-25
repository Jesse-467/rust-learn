//! 练习 3：`tokio::time::timeout` 与 `tokio::select!`

use tokio::time::{Duration, sleep, timeout};

/// 3.1 给一个异步计算加上超时限制：
/// 等待 `work_ms` 毫秒后产出 `value`，但如果超过 `timeout_ms` 毫秒未完成，返回 `None`；
/// 否则返回 `Some(value)`。
/// 提示：`timeout(Duration::from_millis(timeout_ms), async { ... }).await.ok()`
pub async fn run_with_timeout(value: i32, work_ms: u64, timeout_ms: u64) -> Option<i32> {
    todo!()
}

/// 3.2 用 `tokio::select!` 让两个分支竞速，返回先完成分支的字符串结果：
/// - 分支 A：等待 `delay_a_ms` 毫秒后返回 `"A"`
/// - 分支 B：等待 `delay_b_ms` 毫秒后返回 `"B"`
pub async fn race_two(delay_a_ms: u64, delay_b_ms: u64) -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_with_timeout() {
        assert_eq!(run_with_timeout(99, 10, 80).await, Some(99));
        assert_eq!(run_with_timeout(99, 80, 15).await, None);
    }

    #[tokio::test]
    async fn test_race_two() {
        assert_eq!(race_two(10, 60).await, "A");
        assert_eq!(race_two(60, 10).await, "B");
    }
}
