//! 练习 2：自定义 Trait、`Display`、`From` 与动态分发 `&dyn Trait`

use std::fmt;

pub trait Summarizable {
    fn summary(&self) -> String;

    /// 默认实现：子类可以不重写直接复用
    fn brief(&self) -> String {
        format!("[摘要] {}", self.summary())
    }
}

#[derive(Debug, PartialEq)]
pub struct Article {
    pub title: String,
    pub author: String,
}

#[derive(Debug, PartialEq)]
pub struct Tweet {
    pub user: String,
    pub body: String,
}

/// 2.1 为 `Article` 实现 `Summarizable`：返回 `"<title> by <author>"`。
impl Summarizable for Article {
    fn summary(&self) -> String {
        todo!()
    }
}

/// 2.2 为 `Tweet` 实现 `Summarizable`：返回 `"@<user>: <body>"`。
impl Summarizable for Tweet {
    fn summary(&self) -> String {
        todo!()
    }
}

/// 2.3 为 `Tweet` 实现 `std::fmt::Display`，输出与 `summary()` 相同的内容。
impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// 2.4 动态分发：接收一组不同类型的 `&dyn Summarizable`，把它们的 `summary()` 收集成 `Vec<String>`。
pub fn collect_summaries(items: &[&dyn Summarizable]) -> Vec<String> {
    todo!()
}

/// 2.5 实现 `From<(&str, &str)>` 方便用 `Article::from(("Rust", "Jesse"))` 构造。
impl From<(&str, &str)> for Article {
    fn from((title, author): (&str, &str)) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits_and_dyn() {
        let a = Article::from(("Rust 101", "Jesse"));
        let t = Tweet {
            user: "gopher".to_string(),
            body: "learning rust!".to_string(),
        };

        assert_eq!(a.summary(), "Rust 101 by Jesse");
        assert_eq!(a.brief(), "[摘要] Rust 101 by Jesse");
        assert_eq!(t.to_string(), "@gopher: learning rust!");

        let list: Vec<&dyn Summarizable> = vec![&a, &t];
        assert_eq!(
            collect_summaries(&list),
            vec!["Rust 101 by Jesse", "@gopher: learning rust!"]
        );
    }
}
