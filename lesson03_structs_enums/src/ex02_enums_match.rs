//! 练习 2：枚举（enum）、穷尽模式匹配（match）、`if let` 与 `matches!`
//!
//! 对照 Go：
//! - Go 没有真正的 sum type（带数据的枚举），通常只能用 `const + iota` 或 `interface{}` + type switch 模拟。
//! - Rust 的 `enum` 每个变体可以携带不同类型、不同数量的数据，而且 `match` 强制你覆盖所有情况（漏掉任何一种编译就会报错）。

use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rect { width: f64, height: f64 },
    Point,
}

impl Shape {
    /// 2.1 用 `match` 计算图形面积：
    /// - `Circle(r)` -> `PI * r * r`
    /// - `Rect { width, height }` -> `width * height`
    /// - `Point` -> `0.0`
    pub fn area(&self) -> f64 {
        todo!()
    }
}

/// 2.2 用 `match` 的范围模式和或模式分类 HTTP 状态码：
/// - `200..=299` -> `"success"`
/// - `300..=399` -> `"redirect"`
/// - `401 | 403` -> `"auth_error"`
/// - `404` -> `"not_found"`
/// - `400..=499` -> `"client_error"`
/// - `500..=599` -> `"server_error"`
/// - 其余 -> `"unknown"`
///
/// 注意：分支顺序很重要！更具体的分支（如 `401 | 403`、`404`）要写在宽泛范围（`400..=499`）前面。
pub fn classify_status(code: u16) -> &'static str {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

impl Command {
    /// 2.3 用 `match` 返回指令描述字符串：
    /// - `Quit` -> `"quit"`
    /// - `Echo(msg)` -> `"echo: <msg>"`
    /// - `Move { x, y }` -> `"move to (<x>, <y>)"`
    /// - `ChangeColor(r, g, b)` -> `"color(<r>, <g>, <b>)"`
    ///
    /// 提示：可以用 `format!(...)` 构造 `String`。
    pub fn describe(&self) -> String {
        todo!()
    }

    /// 2.4 判断当前指令是否是 `Command::Quit`。
    /// 提示：试试标准库宏 `matches!(self, Command::Quit)`，非常简洁！
    pub fn is_quit(&self) -> bool {
        todo!()
    }
}

/// 2.5 用 `if let` 提取 `Command::Echo` 里的文本借用：
/// - 如果是 `Command::Echo(s)`，返回 `Some(s.as_str())`（或者 `Some(s)`，Rust 会自动解引用 `&String` 为 `&str`）
/// - 其他变体一律返回 `None`
pub fn extract_echo(cmd: &Command) -> Option<&str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_area() {
        let c = Shape::Circle(2.0);
        assert!((c.area() - 4.0 * PI).abs() < 1e-9);

        let r = Shape::Rect {
            width: 3.0,
            height: 5.0,
        };
        assert!((r.area() - 15.0).abs() < 1e-9);

        assert_eq!(Shape::Point.area(), 0.0);
    }

    #[test]
    fn test_classify_status() {
        assert_eq!(classify_status(200), "success");
        assert_eq!(classify_status(204), "success");
        assert_eq!(classify_status(301), "redirect");
        assert_eq!(classify_status(401), "auth_error");
        assert_eq!(classify_status(403), "auth_error");
        assert_eq!(classify_status(404), "not_found");
        assert_eq!(classify_status(429), "client_error");
        assert_eq!(classify_status(502), "server_error");
        assert_eq!(classify_status(100), "unknown");
    }

    #[test]
    fn test_command_describe_and_quit() {
        assert_eq!(Command::Quit.describe(), "quit");
        assert!(Command::Quit.is_quit());

        let echo = Command::Echo("hi".to_string());
        assert_eq!(echo.describe(), "echo: hi");
        assert!(!echo.is_quit());

        let mv = Command::Move { x: 10, y: -5 };
        assert_eq!(mv.describe(), "move to (10, -5)");

        let color = Command::ChangeColor(255, 128, 0);
        assert_eq!(color.describe(), "color(255, 128, 0)");
    }

    #[test]
    fn test_extract_echo() {
        let echo = Command::Echo("hello".to_string());
        assert_eq!(extract_echo(&echo), Some("hello"));
        assert_eq!(extract_echo(&Command::Quit), None);
    }
}
