//! 练习 3：`Option<T>` —— 彻底告别 Go 里的 `nil` 空指针解引用
//!
//! Rust 里没有 `nil` / `null`。一个值如果“可能不存在”，类型系统会显式标记为 `Option<T>`：
//! ```rust
//! enum Option<T> {
//!     Some(T),
//!     None,
//! }
//! ```
//! 编译器逼你处理 `None` 分支，因此运行期不会出现空指针 panic。
//! 查方法可以翻根目录的 `CHEATSHEET.md` 第 5 节！

/// 3.1 安全除法：如果 `b == 0.0` 返回 `None`，否则返回 `Some(a / b)`。
pub fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

/// 3.2 返回切片中第一个偶数；如果切片为空或没有偶数，返回 `None`。
pub fn first_even(nums: &[i32]) -> Option<i32> {
    let mut res: Option<i32> = None;
    for x in nums {
        if x % 2 == 0 {
            res = Some(*x);
            break;
        }
    }
    res
}

/// 3.3 如果 `nick` 有值（且去掉两端空白后非空），返回其全大写形式；
/// 否则返回 `"ANONYMOUS".to_string()`。
///
/// 示例：
/// - `Some(" alice ")` -> `"ALICE"`
/// - `Some("   ")` -> `"ANONYMOUS"`
/// - `None` -> `"ANONYMOUS"`
///
/// 提示：可以先用 `match` 写出来，再试试用 `.map()` + `.filter()` + `.unwrap_or_else()` 链式写！
pub fn nickname_or_default(nick: Option<&str>) -> String {
    let no = "ANONYMOUS".to_string();
    match nick {
        Some(s) => {
            if s.trim() != "" {
                s.trim().to_string().to_uppercase()
            } else {
                no
            }
        }
        None => no,
    }
}

/// 3.4 解析有效端口号：
/// 1. 去掉两端空白（`.trim()`）
/// 2. 解析为 `u16`（`.parse::<u16>().ok()` 把 `Result` 转成 `Option`）
/// 3. 端口号必须大于 0（即 `1..=65535`），0 视为无效返回 `None`
///
/// 提示：可以试试 Rust 的 `let Some(port) = ... else { return None; };` 或 `.filter(|&p| p > 0)`。
pub fn parse_port(input: &str) -> Option<u16> {
    let x = input.trim().parse::<u16>().ok();
    if let Some(n) = x {
        match n {
            1..=65535 => x,
            _ => None,
        }
    } else {
        None
    }
}

/// 3.5 从可变借用的 `slot` 中把值“拿走”并返回；如果原本是 `None`，返回 `0`。
/// 要求：调用结束后，`*slot` 必须变成 `None`！
///
/// 提示：查一下 `CHEATSHEET.md` 里的 `Option::take()`。
pub fn take_or_zero(slot: &mut Option<i32>) -> i32 {
    slot.take().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10.0, 2.0), Some(5.0));
        assert_eq!(safe_div(10.0, 0.0), None);
    }

    #[test]
    fn test_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn test_nickname_or_default() {
        assert_eq!(nickname_or_default(Some(" alice ")), "ALICE");
        assert_eq!(nickname_or_default(Some("   ")), "ANONYMOUS");
        assert_eq!(nickname_or_default(None), "ANONYMOUS");
    }

    #[test]
    fn test_parse_port() {
        assert_eq!(parse_port("8080"), Some(8080));
        assert_eq!(parse_port(" 443 \n"), Some(443));
        assert_eq!(parse_port("0"), None);
        assert_eq!(parse_port("70000"), None);
        assert_eq!(parse_port("abc"), None);
    }

    #[test]
    fn test_take_or_zero() {
        let mut s = Some(42);
        assert_eq!(take_or_zero(&mut s), 42);
        assert_eq!(s, None);
        assert_eq!(take_or_zero(&mut s), 0);
    }
}
