//! 练习 3：循环 —— for / while / loop / break 带值 / 循环标签

/// 3.1 斐波那契数列第 n 项：fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)
///
/// 要求：用循环实现（不要用递归），因为 fib(90) 用朴素递归会算到天荒地老。
/// 提示：维护两个 `let mut` 变量 a、b；Rust 的 for 循环不需要用到的循环变量可以写成 `_`
///       例如 `for _ in 0..n { ... }`
pub fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0_u128, 1_u128);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a as u64
}

/// 3.2 返回所有小于 limit 的、3 或 5 的倍数之和。
/// 例如 limit = 10：3 + 5 + 6 + 9 = 23
///
/// 要求：用 `while` 循环实现（体会一下 Rust 的 while 和 Go 的 `for cond {}` 是一回事）。
pub fn sum_of_multiples(limit: u32) -> u32 {
    let (mut n, mut sum) = (1_u32, 0_u32);
    while n < limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
        n += 1;
    }
    sum
}

/// 3.3 返回**严格大于** n 的最小的 2 的幂。
/// 例如：0 -> 1，1 -> 2，5 -> 8，8 -> 16
///
/// 要求：用 `loop` + `break 值` 的写法，形如：
///     let result = loop {
///         ...
///         if 条件 { break 某个值; }
///     };
pub fn next_power_of_two_over(n: u64) -> u64 {
    let mut ans: u128 = 1;
    // while ans <= n as u128 {
    //     ans *= 2;
    // }
    // ans as u64
    (loop {
        if ans > n as u128 {
            break ans;
        };
        ans <<= 1
    }) as u64
}

/// 3.4 找到满足下面条件的三个正整数 (a, b, c)：
///     a < b < c，  a + b + c == sum，  a² + b² == c²
/// 题目保证解存在且唯一。例如 sum = 12 -> (3, 4, 5)
///
/// 要求：用两层嵌套 for 循环 + 循环标签 `'outer:` ，找到后用 `break 'outer;` 一次跳出两层。
/// 提示：枚举 a 和 b，c 就等于 sum - a - b，不需要第三层循环。
///       可以先 `let mut ans = (0, 0, 0);`，找到后赋值再跳出。
pub fn pythagorean_triplet(sum: u32) -> (u32, u32, u32) {
    let mut ans = (0, 0, 0);
    'outer: for a in 1..sum {
        for b in (a + 1)..(sum - a) {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                ans = (a, b, c);
                break 'outer;
            }
        }
    }
    ans
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(90), 2880067194370816120);
    }

    #[test]
    fn test_sum_of_multiples() {
        assert_eq!(sum_of_multiples(1), 0);
        assert_eq!(sum_of_multiples(10), 23);
        assert_eq!(sum_of_multiples(1000), 233168);
    }

    #[test]
    fn test_next_power_of_two_over() {
        assert_eq!(next_power_of_two_over(0), 1);
        assert_eq!(next_power_of_two_over(1), 2);
        assert_eq!(next_power_of_two_over(5), 8);
        assert_eq!(next_power_of_two_over(8), 16);
        assert_eq!(next_power_of_two_over(1000), 1024);
    }

    #[test]
    fn test_pythagorean_triplet() {
        assert_eq!(pythagorean_triplet(12), (3, 4, 5));
        assert_eq!(pythagorean_triplet(30), (5, 12, 13));
        assert_eq!(pythagorean_triplet(1000), (200, 375, 425));
    }
}
