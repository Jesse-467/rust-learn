//! 练习 2：高频迭代器链（尝试全程不写 `for` 循环！）

/// 2.1 计算切片中所有正数的平方和。
/// 提示：`.iter().copied().filter(...).map(...).sum()`
pub fn sum_of_positive_squares(nums: &[i32]) -> i32 {
    todo!()
}

/// 2.2 解析一组字符串，只保留能成功解析为 `i32` 的数字，丢弃非法字符串。
/// 提示：`.iter().filter_map(|s| s.trim().parse::<i32>().ok()).collect()`
pub fn parse_valid_ints(inputs: &[&str]) -> Vec<i32> {
    todo!()
}

/// 2.3 计算两个等长切片的点积（`a[0]*b[0] + a[1]*b[1] + ...`）。
/// 提示：`a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()`
pub fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    todo!()
}

/// 2.4 把多行文本拆成所有非空单词的扁平列表（按空白切分）。
/// 提示：`lines.iter().flat_map(|line| line.split_whitespace()).collect()`
pub fn flatten_words<'a>(lines: &[&'a str]) -> Vec<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_positive_squares() {
        assert_eq!(sum_of_positive_squares(&[-3, 0, 2, -1, 4]), 20);
        assert_eq!(sum_of_positive_squares(&[-5, -2]), 0);
    }

    #[test]
    fn test_parse_valid_ints() {
        assert_eq!(parse_valid_ints(&["10", "abc", " -5 ", "", "42"]), vec![10, -5, 42]);
    }

    #[test]
    fn test_dot_product() {
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
    }

    #[test]
    fn test_flatten_words() {
        let lines = ["hello   world", "", "  rust  iterators "];
        assert_eq!(flatten_words(&lines), vec!["hello", "world", "rust", "iterators"]);
    }
}
