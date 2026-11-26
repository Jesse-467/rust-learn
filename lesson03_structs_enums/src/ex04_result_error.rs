//! 练习 4：`Result<T, E>`、自定义错误类型与 `?` 运算符
//!
//! 对照 Go：
//! - Go 的错误处理：`v, err := doSomething(); if err != nil { return zero, err }`
//! - Rust 把“成功值”和“错误值”打包进同一个枚举 `Result<T, E>`（不会出现同时拿到半残值和 err 的情况），
//!   并用 `?` 运算符替代重复的 `if err != nil { return err }`。

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    EmptyInput,
    MissingEquals,
    InvalidPort(String),
    PortOutOfRange(u16),
}

/// 4.1 解析一行形如 `"key=value"` 的配置，返回 `(key, value)`（两端都去掉空白）：
/// - 如果整行去掉空白后为空 -> `Err(ConfigError::EmptyInput)`
/// - 如果不含 `'='` -> `Err(ConfigError::MissingEquals)`
/// - 如果有多个 `'='`，只按第一个 `'='` 切分（提示：用 `str::split_once('=')` + `.ok_or(...)`）
pub fn parse_key_value(line: &str) -> Result<(&str, &str), ConfigError> {
    let s = line.trim();
    if s.is_empty() {
        Err(ConfigError::EmptyInput)
    } else {
        let res = s.split_once('=').ok_or(ConfigError::MissingEquals);
        if let Ok((k, v)) = res {
            Ok((k.trim(), v.trim()))
        } else {
            res
        }
    }
}

/// 4.2 将字符串解析为合法端口号（`1..=65535`）：
/// - 先 `trim()`，再用 `.parse::<u16>()` 解析
/// - 如果解析失败 -> `Err(ConfigError::InvalidPort(raw.trim().to_string()))`
///   （提示：用 `.map_err(|_| ...)` 转换错误类型，再配合 `?`）
/// - 如果解析结果是 `0` -> `Err(ConfigError::PortOutOfRange(0))`
pub fn parse_valid_port(raw: &str) -> Result<u16, ConfigError> {
    let r = raw.trim().parse::<u16>();
    match r {
        Ok(0) => Err(ConfigError::PortOutOfRange(0)),
        Ok(n) => Ok(n),
        Err(_) => Err(ConfigError::InvalidPort(raw.trim().to_string())),
    }
}

/// 4.3 组合上面两个函数：输入一行配置（如 `"port = 8080"`），解析出其中的合法端口号。
/// 要求：用 `?` 串联 `parse_key_value` 和 `parse_valid_port`！
/// （本题暂不校验 key 名，只要能切出 value 并解析成合法端口即可）
pub fn extract_port_from_line(line: &str) -> Result<u16, ConfigError> {
    let (_k, v) = parse_key_value(line)?;
    parse_valid_port(v)
}

/// 4.4 解析两个整数字符串（先 `trim()`）并返回它们的和。
/// 任意一个解析失败时，直接通过 `?` 把 `ParseIntError` 向上返回。
pub fn sum_two_numbers(a: &str, b: &str) -> Result<i32, ParseIntError> {
    match a.trim().parse::<i32>() {
        Ok(s) => match b.trim().parse::<i32>() {
            Ok(t) => Ok(s + t),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_value() {
        assert_eq!(
            parse_key_value("host = 127.0.0.1"),
            Ok(("host", "127.0.0.1"))
        );
        assert_eq!(parse_key_value("token=abc=def"), Ok(("token", "abc=def")));
        assert_eq!(parse_key_value("   "), Err(ConfigError::EmptyInput));
        assert_eq!(
            parse_key_value("no_equals_here"),
            Err(ConfigError::MissingEquals)
        );
    }

    #[test]
    fn test_parse_valid_port() {
        assert_eq!(parse_valid_port(" 8080 "), Ok(8080));
        assert_eq!(parse_valid_port("0"), Err(ConfigError::PortOutOfRange(0)));
        assert_eq!(
            parse_valid_port("not_a_num"),
            Err(ConfigError::InvalidPort("not_a_num".to_string()))
        );
        assert_eq!(
            parse_valid_port("99999"),
            Err(ConfigError::InvalidPort("99999".to_string()))
        );
    }

    #[test]
    fn test_extract_port_from_line() {
        assert_eq!(extract_port_from_line("port = 8080"), Ok(8080));
        assert_eq!(extract_port_from_line(""), Err(ConfigError::EmptyInput));
        assert_eq!(
            extract_port_from_line("port 8080"),
            Err(ConfigError::MissingEquals)
        );
        assert_eq!(
            extract_port_from_line("port = 0"),
            Err(ConfigError::PortOutOfRange(0))
        );
        assert_eq!(
            extract_port_from_line("port = abc"),
            Err(ConfigError::InvalidPort("abc".to_string()))
        );
    }

    #[test]
    fn test_sum_two_numbers() {
        assert_eq!(sum_two_numbers(" 10 ", " 32 "), Ok(42));
        assert!(sum_two_numbers("10", "oops").is_err());
        assert!(sum_two_numbers("oops", "32").is_err());
    }
}
