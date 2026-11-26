//! 练习 1：结构体（struct）与方法（impl）
//!
//! 对照 Go：
//! - Go 的方法写在 struct 外面：`func (r *Rect) Area() uint32`
//! - Rust 的方法写在 `impl Rect { ... }` 块里，第一个参数明确区分：
//!   - `&self`：只读借用（最常用）
//!   - `&mut self`：可变借用（修改字段）
//!   - `self`：拿走所有权（调用后原变量失效，常用于转换或 builder）

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// 1.1 关联函数（构造函数）：根据宽高创建一个 `Rectangle`。
    /// 提示：字段名和参数名相同时可以简写为 `Self { width, height }`。
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    /// 1.2 计算矩形面积（只读借用 `&self`）。
    pub fn area(&self) -> u32 {
        (self.width as u64 * self.height as u64) as u32
    }

    /// 1.3 判断当前矩形是否能严格装下另一个矩形 `other`
    /// （即 `self.width > other.width && self.height > other.height`）。
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }

    /// 1.4 将当前矩形的宽和高都按 `factor` 倍原地放大（需要 `&mut self`）。
    pub fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor
    }

    /// 1.5 消费当前矩形，返回一个正方形（边长取宽和高的较大值 `self.width.max(self.height)`）。
    /// 注意：参数是 `self`，调用后原矩形的所有权被移走。
    pub fn into_square(self) -> Self {
        let side = self.width.max(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
}

impl User {
    /// 1.6 结构体更新语法（Struct Update Syntax）：
    /// 消费当前 `User`，返回一个只替换了 `email` 的新 `User`，其余字段保持不变。
    /// 提示：`Self { email: new_email, ..self }`
    pub fn with_email(self, new_email: String) -> Self {
        User {
            email: new_email,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_area() {
        let r = Rectangle::new(6, 7);
        assert_eq!(r.width, 6);
        assert_eq!(r.height, 7);
        assert_eq!(r.area(), 42);
    }

    #[test]
    fn test_can_hold() {
        let big = Rectangle::new(10, 8);
        let small = Rectangle::new(9, 7);
        let equal = Rectangle::new(10, 8);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
        assert!(!big.can_hold(&equal));
    }

    #[test]
    fn test_scale() {
        let mut r = Rectangle::new(3, 4);
        r.scale(3);
        assert_eq!(r, Rectangle::new(9, 12));
    }

    #[test]
    fn test_into_square() {
        let r = Rectangle::new(4, 9);
        let sq = r.into_square();
        assert_eq!(sq, Rectangle::new(9, 9));
    }

    #[test]
    fn test_with_email() {
        let u = User {
            username: "gopher".to_string(),
            email: "old@example.com".to_string(),
            active: true,
        };
        let u2 = u.with_email("rustacean@example.com".to_string());
        assert_eq!(u2.username, "gopher");
        assert_eq!(u2.email, "rustacean@example.com");
        assert!(u2.active);
    }
}
