//! 练习 1：`std::thread::spawn` 与 `std::thread::scope`

use std::thread;

/// 1.1 启动 `n` 个线程（编号 `0..n`），每个线程计算 `id * id` 并返回，
/// 主线程收集所有结果并按升序排序后返回。
pub fn parallel_squares(n: usize) -> Vec<usize> {
    todo!()
}

/// 1.2 使用 `thread::scope` 把切片 `nums` 按 `chunk_size` 切块，每块开一个线程求和，
/// 最后把所有子线程的和汇总返回（空切片直接返回 0）。
/// 提示：`nums.chunks(chunk_size)`，注意 `chunk_size == 0` 时可当成 1 处理避免 panic。
pub fn scoped_parallel_sum(nums: &[i64], chunk_size: usize) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_squares() {
        assert_eq!(parallel_squares(5), vec![0, 1, 4, 9, 16]);
        assert_eq!(parallel_squares(0), vec![]);
    }

    #[test]
    fn test_scoped_parallel_sum() {
        let data: Vec<i64> = (1..=100).collect();
        assert_eq!(scoped_parallel_sum(&data, 25), 5050);
        assert_eq!(scoped_parallel_sum(&[], 10), 0);
    }
}
