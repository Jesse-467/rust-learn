//! 练习 2：函数、表达式、if 表达式

/// 2.1 摄氏度转华氏度：F = C × 9 / 5 + 32
///
/// 要求：函数体只有**一个表达式**——不写 `return`，末尾也不加分号。
/// 注意：f64 的字面量要写成 `9.0` 而不是 `9`（Rust 不会自动把整数转成浮点数！）
pub fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// 2.2 返回两个数之差的绝对值，返回类型是 u32。
/// 例如：abs_diff(3, 10) -> 7，abs_diff(-5, 5) -> 10
///
/// 要求：用 if 表达式，形如 `if 条件 { 值1 } else { 值2 }` 直接作为返回值。
/// 提示：i32 转 u32 用 `as u32`
pub fn abs_diff(a: i32, b: i32) -> u32 {
    if a > b {
        (a as i64 - b as i64) as u32
    } else {
        (b as i64 - a as i64) as u32
    }
}

/// 2.3 根据分数返回等级：
///   90 及以上 'A'，80 及以上 'B'，70 及以上 'C'，60 及以上 'D'，否则 'F'
///
/// 要求：先写 `let g = if ... else if ... else ...;` 把结果绑定到变量 g，再返回 g。
/// 注意：char 字面量用单引号 'A'，字符串用双引号 "A"，它们是不同类型。
pub fn grade(score: u32) -> char {
    match score {
        90.. => 'A',
        80.. => 'B',
        70.. => 'C',
        60.. => 'D',
        _ => 'F',
    }
}

/// 2.4 思考题（写成代码）：返回 x 是否在闭区间 [low, high] 内。
///
/// 要求：用一个布尔表达式一行搞定，不要用 if。
pub fn in_range(x: i32, low: i32, high: i32) -> bool {
    x <= high && x >= low
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_to_f() {
        assert_eq!(c_to_f(0.0), 32.0);
        assert_eq!(c_to_f(100.0), 212.0);
        assert_eq!(c_to_f(-40.0), -40.0);
        assert!((c_to_f(37.0) - 98.6).abs() < 1e-9);
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(3, 10), 7);
        assert_eq!(abs_diff(10, 3), 7);
        assert_eq!(abs_diff(-5, 5), 10);
        assert_eq!(abs_diff(4, 4), 0);
    }

    #[test]
    fn test_grade() {
        assert_eq!(grade(100), 'A');
        assert_eq!(grade(90), 'A');
        assert_eq!(grade(89), 'B');
        assert_eq!(grade(75), 'C');
        assert_eq!(grade(60), 'D');
        assert_eq!(grade(59), 'F');
        assert_eq!(grade(0), 'F');
    }

    #[test]
    fn test_in_range() {
        assert!(in_range(5, 1, 10));
        assert!(in_range(1, 1, 10));
        assert!(in_range(10, 1, 10));
        assert!(!in_range(0, 1, 10));
        assert!(!in_range(11, 1, 10));
    }
}
