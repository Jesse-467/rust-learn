//! 练习 2：`std::sync::mpsc` 多生产者单消费者 Channel

use std::sync::mpsc;
use std::thread;

/// 2.1 给定一组任务列表 `jobs`（每个元素是一组待求和的数字），
/// 为每个 job 启动一个线程计算其总和，并通过 `mpsc::channel` 发回主线程，
/// 主线程收齐所有结果并累加返回总和。
///
/// 提示：别忘了在循环结束后 `drop(tx)`，这样 `for partial in rx` 才会正常退出！
pub fn sum_jobs_via_channel(jobs: Vec<Vec<i64>>) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_jobs_via_channel() {
        let jobs = vec![vec![1, 2, 3], vec![10, 20], vec![100]];
        assert_eq!(sum_jobs_via_channel(jobs), 136);
        assert_eq!(sum_jobs_via_channel(vec![]), 0);
    }
}
