//! 练习 3：生命周期标注（`'a`）

/// 3.1 返回两个字符串切片中字节长度更长的那个（相等时返回 `a`）。
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// 3.2 零拷贝解析：结构体持有对原始日志行的切片借用，不需要分配新 `String`。
#[derive(Debug, PartialEq)]
pub struct LogEntry<'a> {
    pub level: &'a str,
    pub message: &'a str,
}

impl<'a> LogEntry<'a> {
    /// 解析形如 `"[INFO] server started"` 或 `"ERROR: timeout"` 的简化日志：
    /// 本题约定格式为 `"LEVEL: message"`，按第一个 `':'` 切分并 `trim()` 两端空白；
    /// 若不含 `':'` 或 level 为空，返回 `None`。
    pub fn parse(line: &'a str) -> Option<Self> {
        todo!()
    }
}

/// 3.3 从 `text` 中提取以 `prefix` 开头的**第一行**（保留整行，去掉首尾空白）；找不到返回 `None`。
/// 注意：返回值的生命周期只和 `text` 有关，和 `prefix` 无关！
pub fn first_line_with_prefix<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("go", "rust"), "rust");
        assert_eq!(longest("same", "size"), "same");
    }

    #[test]
    fn test_log_entry_parse() {
        let raw = String::from("ERROR: connection refused");
        let entry = LogEntry::parse(&raw).unwrap();
        assert_eq!(
            entry,
            LogEntry {
                level: "ERROR",
                message: "connection refused",
            }
        );
        assert_eq!(LogEntry::parse("invalid line"), None);
        assert_eq!(LogEntry::parse(": missing level"), None);
    }

    #[test]
    fn test_first_line_with_prefix() {
        let doc = "hello
  TODO: finish lesson 5 
TODO: lesson 6";
        let found = {
            let p = String::from("TODO:");
            first_line_with_prefix(doc, &p)
        }; // 注意：p 在这里已经销毁，但 found 依然有效！
        assert_eq!(found, Some("TODO: finish lesson 5"));
    }
}
