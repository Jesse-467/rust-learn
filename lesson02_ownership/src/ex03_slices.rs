//! 练习 3：切片（&str 与 &[T]）—— 零拷贝的视图

/// 3.1 返回字符串 `s` 的第一个单词（以空格 `' '` 分隔）。
/// 如果没有空格，返回整个字符串。
/// 例如："hello world" -> "hello"，"rust" -> "rust"，"" -> ""
///
/// 要求：返回 `&str` 切片（零拷贝，不要创建新 String）。
/// 提示：
/// - 方式 A：遍历字节 `for (i, b) in s.bytes().enumerate()`，遇到 `b == b' '` 就 `return &s[..i];`
/// - 方式 B：`if let Some(i) = s.find(' ') { &s[..i] } else { s }`
pub fn first_word(s: &str) -> &str {
    if let Some(i) = s.find(' ') {
        &s[..i]
    } else {
        s
    }
}

/// 3.2 返回字符串 `s` 的第二个单词（假设单词之间用单个空格分隔）；如果没有第二个单词，返回 `""`。
/// 例如："hello world rust" -> "world"，"one two" -> "two"，"only" -> ""
///
/// 要求：返回 `&str` 切片（零拷贝）。
/// 提示：先找到第一个空格的位置 `i`，剩下部分就是 `&s[i + 1..]`，再对剩下部分调用 `first_word`！
pub fn second_word(s: &str) -> &str {
    let s: &str = s.trim();
    if let Some(i) = s.find(' ') {
        let s1 = &s[i + 1..];
        if let Some(j) = s1.find(' ') {
            &s1[..j]
        } else {
            s1
        }
    } else {
        ""
    }
}

/// 3.3 计算切片 `&[i32]` 中所有元素的和。
///
/// 体会：参数写成 `&[i32]` 而不是 `&Vec<i32>`，
/// 下面的测试里既可以传数组引用 `&[1, 2, 3]`，也可以传 `&vec![...]`，还可以传子切片 `&arr[1..4]`！
pub fn sum_slice(nums: &[i32]) -> i32 {
    let mut sum: i64 = 0;
    for x in nums {
        sum += *x as i64
    }
    sum as i32
}

/// 3.4 给定切片 `nums` 和窗口长度 `k`（保证 `1 <= k <= nums.len()`），
/// 返回所有长度为 `k` 的连续子切片中，元素和的最大值。
/// 例如：nums = [1, 4, 2, 10, 2, 3], k = 3 -> 4 + 2 + 10 = 16
///
/// 提示：枚举起始下标 `i in 0..=nums.len() - k`，对子切片 `&nums[i..i + k]` 调用 `sum_slice`。
pub fn max_window_sum(nums: &[i32], k: usize) -> i32 {
    let mut sum = 0;
    for x in nums.iter().take(k) {
        sum += x
    }
    let mut mx = sum;
    for i in k..nums.len() {
        sum += nums[i] - nums[i - k];
        if sum > mx {
            mx = sum;
        }
    }
    mx
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word(" leading"), "");
    }

    #[test]
    fn test_second_word() {
        assert_eq!(second_word("hello world rust"), "world");
        assert_eq!(second_word("one two"), "two");
        assert_eq!(second_word("only"), "");
        assert_eq!(second_word(""), "");
    }

    #[test]
    fn test_sum_slice() {
        let arr = [1, 2, 3, 4, 5];
        let vec = vec![10, 20, 30];
        assert_eq!(sum_slice(&arr), 15); // 数组自动转切片
        assert_eq!(sum_slice(&vec), 60); // Vec 自动转切片
        assert_eq!(sum_slice(&arr[1..4]), 9); // 子切片 [2, 3, 4]
        assert_eq!(sum_slice(&[]), 0);
    }

    #[test]
    fn test_max_window_sum() {
        assert_eq!(max_window_sum(&[1, 4, 2, 10, 2, 3], 3), 16);
        assert_eq!(max_window_sum(&[-5, -2, -8, -1], 2), -7);
        assert_eq!(max_window_sum(&[7], 1), 7);
    }
}
