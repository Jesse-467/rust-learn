//! 练习 1：`Box<T>` 递归数据结构与 `Box<dyn Trait>`

/// 1.1 递归算术表达式树
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn num(v: i64) -> Self {
        Expr::Num(v)
    }

    pub fn add(lhs: Expr, rhs: Expr) -> Self {
        Expr::Add(Box::new(lhs), Box::new(rhs))
    }

    pub fn mul(lhs: Expr, rhs: Expr) -> Self {
        Expr::Mul(Box::new(lhs), Box::new(rhs))
    }

    /// 递归求值：
    /// - `Num(v)` -> `v`
    /// - `Add(a, b)` -> `a.eval() + b.eval()`
    /// - `Mul(a, b)` -> `a.eval() * b.eval()`
    pub fn eval(&self) -> i64 {
        todo!()
    }
}

/// 1.2 异构集合 `Vec<Box<dyn Greeter>>`
pub trait Greeter {
    fn greet(&self) -> String;
}

pub struct EnGreeter;
pub struct ZhGreeter;

impl Greeter for EnGreeter {
    fn greet(&self) -> String {
        "Hello".to_string()
    }
}

impl Greeter for ZhGreeter {
    fn greet(&self) -> String {
        "你好".to_string()
    }
}

/// 调用 `greeters` 中每个对象的 `greet()`，用 `", "` 拼成一个字符串返回。
/// 提示：`.iter().map(|g| g.greet()).collect::<Vec<_>>().join(", ")`
pub fn greet_all(greeters: &[Box<dyn Greeter>]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_eval() {
        // (2 + 3) * 4 = 20
        let expr = Expr::mul(Expr::add(Expr::num(2), Expr::num(3)), Expr::num(4));
        assert_eq!(expr.eval(), 20);
    }

    #[test]
    fn test_greet_all() {
        let gs: Vec<Box<dyn Greeter>> = vec![Box::new(EnGreeter), Box::new(ZhGreeter)];
        assert_eq!(greet_all(&gs), "Hello, 你好");
    }
}
