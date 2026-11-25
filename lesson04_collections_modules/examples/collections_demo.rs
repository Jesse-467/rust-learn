//! 第 4 课随堂演示：Vec、HashMap (entry API) 与模块可见性
//! 运行方式：cargo run --example collections_demo

use std::collections::HashMap;

mod auth {
    #[derive(Debug)]
    pub struct Token {
        pub user: String,
        secret: String, // 私有字段：外部无法直接构造或读取
    }

    impl Token {
        pub fn issue(user: &str) -> Self {
            Self {
                user: user.to_string(),
                secret: format!("sec_{user}"),
            }
        }

        pub fn verify(&self, candidate: &str) -> bool {
            self.secret == candidate
        }
    }
}

fn main() {
    println!("=== 1. Vec 常用操作 ===");
    let mut nums = vec![3, -1, 2, 3, -5, 2, 1];
    nums.retain(|x| *x > 0);
    nums.sort();
    nums.dedup();
    println!("正数排序去重后: {nums:?}");

    println!("
=== 2. HashMap & entry API ===");
    let text = "rust go rust rust go python";
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("词频统计: {freq:?}");

    let pairs = [("backend", "go"), ("backend", "rust"), ("frontend", "ts")];
    let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k, v) in pairs {
        groups.entry(k).or_default().push(v);
    }
    println!("分组聚合: {groups:?}");

    println!("
=== 3. 模块封装 ===");
    let tok = auth::Token::issue("alice");
    println!("token user = {}, verify = {}", tok.user, tok.verify("sec_alice"));
}
