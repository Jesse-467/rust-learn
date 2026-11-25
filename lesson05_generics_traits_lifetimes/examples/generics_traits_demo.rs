//! 第 5 课随堂演示：泛型、Trait（静态/动态分发）与生命周期
//! 运行方式：cargo run --example generics_traits_demo

use std::fmt;

trait Describable {
    fn describe(&self) -> String;
}

struct Metric {
    name: String,
    value: f64,
}

impl Describable for Metric {
    fn describe(&self) -> String {
        format!("{}={:.2}", self.name, self.value)
    }
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Metric] {}", self.describe())
    }
}

struct TextExcerpt<'a> {
    head: &'a str,
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let m = Metric {
        name: "qps".to_string(),
        value: 1234.567,
    };
    println!("Display 输出: {m}");

    // 动态分发（等价 Go 的 []Describable）
    let items: Vec<&dyn Describable> = vec![&m];
    for item in items {
        println!("dyn Trait: {}", item.describe());
    }

    let s1 = String::from("golang");
    let s2 = "rustlang";
    println!("longer = {}", longer(&s1, s2));

    let excerpt = TextExcerpt { head: &s1[..2] };
    println!("excerpt.head = {}", excerpt.head);
}
