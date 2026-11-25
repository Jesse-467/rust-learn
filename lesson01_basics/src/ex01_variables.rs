//! 练习 1：变量、可变性、shadowing、整数溢出

/// 1.1 返回 1 + 2 + ... + n 的和（n = 0 时返回 0）。
///
/// 要求：用 `let mut` 声明累加变量，用 `for i in 1..=n` 循环实现。
/// （先别用 `(1..=n).sum()` 这种迭代器写法，后面的课会讲）
pub fn sum_to(n: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

/// 1.2 shadowing：输入一个字符串形式的数字（两边可能有空格），返回它的平方。
/// 例如：" 12 " -> 144
///
/// 要求：全程只用 `input` 这一个变量名，通过 shadowing 依次把它变成：
///       去掉空格的字符串  ->  i64 整数  ->  平方
///
/// 提示：
///   - `input.trim()` 去掉两边空格
///   - `input.parse::<i64>().unwrap()` 把字符串转成 i64
///     （`unwrap()` 的意思是"转换失败就直接 panic"，第 3 课会详细讲）
///   - `&str` 是"字符串切片"，你暂时把它当成 Go 的 string 就行，第 2 课详细讲
pub fn square_of_str(input: &str) -> i64 {
    let input = input.trim();
    let input = input.parse::<i64>().unwrap();
    input * input
}

/// 1.3 两个 u8 相加，如果超过 255 就返回 255（"饱和加法"）。
/// 例如：200 + 100 -> 255，  1 + 2 -> 3
///
/// 注意：直接写 `a + b` 在溢出时会 panic（debug 模式），不信可以先试试看测试报什么错！
///
/// 提示（两种思路，都试试）：
///   - 思路 A：先用 `as` 转成更大的类型 u16 再相加、比较，最后转回 u8
///   - 思路 B：u8 自带了一个方法能直接做这件事，在 VS Code 里输入 `a.` 看看自动补全，
///     找名字里带 saturating 的方法
pub fn add_u8_clamped(a: u8, b: u8) -> u8 {
    let c = a as u32 + b as u32;
    if c > 255 { 255 } else { c as u8 }
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn test_square_of_str() {
        assert_eq!(square_of_str("3"), 9);
        assert_eq!(square_of_str(" 12 "), 144);
        assert_eq!(square_of_str("-5"), 25);
    }

    #[test]
    fn test_add_u8_clamped() {
        assert_eq!(add_u8_clamped(1, 2), 3);
        assert_eq!(add_u8_clamped(200, 55), 255);
        assert_eq!(add_u8_clamped(200, 100), 255);
        assert_eq!(add_u8_clamped(255, 255), 255);
    }
}
