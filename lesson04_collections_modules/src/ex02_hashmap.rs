//! 练习 2：`HashMap<K, V>` 与 `entry` API

use std::collections::HashMap;

/// 2.1 统计一段文本中每个单词出现的次数（按空白切分，统一转小写 `to_lowercase()`）。
/// 示例：`"Go rust go"` -> `{"go": 2, "rust": 1}`
/// 提示：`text.split_whitespace()` + `counts.entry(word).or_insert(0)`。
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in text.split_whitespace() {
        *m.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    m
}

/// 2.2 按首字母（小写 `char`）对单词进行分组，忽略空字符串。
/// 示例：`&["apple", "Ant", "banana"]` -> `{'a': ["apple", "Ant"], 'b': ["banana"]}`
/// 提示：`word.chars().next()` 获取首字符，`c.to_ascii_lowercase()` 转小写，`entry(...).or_default().push(...)`。
pub fn group_by_first_char<'a>(words: &[&'a str]) -> HashMap<char, Vec<&'a str>> {
    let mut map: HashMap<char, Vec<&'a str>> = HashMap::new();
    for &word in words {
        if let Some(c) = word.chars().next() {
            map.entry(c.to_ascii_lowercase()).or_default().push(word);
        }
    }
    map
}

/// 2.3 将 `other` 的计数累加合并到 `base` 中（相同 key 相加，不存在的 key 直接插入）。
pub fn merge_counts(base: &mut HashMap<String, i32>, other: &HashMap<String, i32>) {
    for (k, v) in other {
        *base.entry(k.clone()).or_default() += v;
    }
}

/// 2.4 找出计数 Map 中值最大的那个 key（空 Map 返回 `None`；若有并列，返回任意一个均可）。
pub fn most_frequent_key(counts: &HashMap<String, usize>) -> Option<String> {
    counts
        .iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let m = word_count("Rust go   rust GO python");
        assert_eq!(m.get("rust"), Some(&2));
        assert_eq!(m.get("go"), Some(&2));
        assert_eq!(m.get("python"), Some(&1));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn test_group_by_first_char() {
        let g = group_by_first_char(&["apple", "Ant", "", "banana"]);
        assert_eq!(g.get(&'a'), Some(&vec!["apple", "Ant"]));
        assert_eq!(g.get(&'b'), Some(&vec!["banana"]));
        assert_eq!(g.len(), 2);
    }

    #[test]
    fn test_merge_counts() {
        let mut a = HashMap::from([("go".to_string(), 2), ("rust".to_string(), 1)]);
        let b = HashMap::from([("rust".to_string(), 4), ("c".to_string(), 3)]);
        merge_counts(&mut a, &b);
        assert_eq!(a.get("go"), Some(&2));
        assert_eq!(a.get("rust"), Some(&5));
        assert_eq!(a.get("c"), Some(&3));
    }

    #[test]
    fn test_most_frequent_key() {
        let empty = HashMap::new();
        assert_eq!(most_frequent_key(&empty), None);

        let m = HashMap::from([
            ("a".to_string(), 2),
            ("b".to_string(), 5),
            ("c".to_string(), 3),
        ]);
        assert_eq!(most_frequent_key(&m), Some("b".to_string()));
    }
}
