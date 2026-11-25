//! 模块 1：命令协议解析（`Command` + `KvError`）

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum KvError {
    EmptyCommand,
    UnknownCommand(String),
    InvalidArgs(String),
    InvalidTtl(String),
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::EmptyCommand => write!(f, "empty command"),
            KvError::UnknownCommand(cmd) => write!(f, "unknown command: {cmd}"),
            KvError::InvalidArgs(msg) => write!(f, "invalid arguments: {msg}"),
            KvError::InvalidTtl(raw) => write!(f, "invalid ttl: {raw}"),
        }
    }
}

impl std::error::Error for KvError {}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Ping,
    Set {
        key: String,
        value: String,
        ttl_ms: Option<u64>,
    },
    Get(String),
    Del(String),
    Keys,
}

impl Command {
    /// 1.1 将一行文本解析为 `Command`（命令名大小写不敏感，按空白切分）：
    /// - 空行 -> `Err(KvError::EmptyCommand)`
    /// - `PING`（无额外参数）-> `Ok(Command::Ping)`
    /// - `KEYS`（无额外参数）-> `Ok(Command::Keys)`
    /// - `GET <key>` -> `Ok(Command::Get(key))`
    /// - `DEL <key>` -> `Ok(Command::Del(key))`
    /// - `SET <key> <value>` -> `Ok(Command::Set { key, value, ttl_ms: None })`
    /// - `SET <key> <value> EX <ttl_ms>`（`EX` 大小写不敏感，`ttl_ms` 须为正整数 `u64` 且 `> 0`，
    ///   解析失败或为 0 返回 `Err(KvError::InvalidTtl(...))`）
    /// - 参数个数不对或选项不是 `EX` -> `Err(KvError::InvalidArgs(...))`
    /// - 其他命令名 -> `Err(KvError::UnknownCommand(...))`
    pub fn parse(line: &str) -> Result<Self, KvError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_commands() {
        assert_eq!(Command::parse("ping"), Ok(Command::Ping));
        assert_eq!(Command::parse("  KEYS "), Ok(Command::Keys));
        assert_eq!(Command::parse("GET name"), Ok(Command::Get("name".into())));
        assert_eq!(Command::parse("del name"), Ok(Command::Del("name".into())));
        assert_eq!(
            Command::parse("SET lang rust"),
            Ok(Command::Set {
                key: "lang".into(),
                value: "rust".into(),
                ttl_ms: None,
            })
        );
        assert_eq!(
            Command::parse("set session xyz ex 500"),
            Ok(Command::Set {
                key: "session".into(),
                value: "xyz".into(),
                ttl_ms: Some(500),
            })
        );
    }

    #[test]
    fn test_parse_errors() {
        assert_eq!(Command::parse("   "), Err(KvError::EmptyCommand));
        assert!(matches!(Command::parse("FOO bar"), Err(KvError::UnknownCommand(_))));
        assert!(matches!(Command::parse("GET"), Err(KvError::InvalidArgs(_))));
        assert!(matches!(Command::parse("SET only_key"), Err(KvError::InvalidArgs(_))));
        assert!(matches!(Command::parse("SET k v EX 0"), Err(KvError::InvalidTtl(_))));
        assert!(matches!(Command::parse("SET k v EX abc"), Err(KvError::InvalidTtl(_))));
    }
}
