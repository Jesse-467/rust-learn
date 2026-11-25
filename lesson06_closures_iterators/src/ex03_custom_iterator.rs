//! 练习 3：实现自定义 `Iterator`

/// 3.1 无限斐波那契数列迭代器：依次产出 `0, 1, 1, 2, 3, 5, 8, 13, ...`
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// 每次调用返回当前的 `curr`，并把 `(curr, next)` 推进到 `(next, curr + next)`。
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// 3.2 利用上面的 `Fibonacci` 迭代器，计算前 `n` 个**偶数**斐波那契数的总和。
/// 提示：`Fibonacci::new().filter(|x| x % 2 == 0).take(n).sum()`
pub fn sum_first_n_even_fibs(n: usize) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_first_8() {
        let seq: Vec<u64> = Fibonacci::new().take(8).collect();
        assert_eq!(seq, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_sum_first_n_even_fibs() {
        // 偶数斐波那契数依次为：0, 2, 8, 34, ...
        assert_eq!(sum_first_n_even_fibs(4), 44);
    }
}
