// 第 1 课答疑（二），运行：cargo run --example qa2
#![allow(unused)]

fn main() {
    q1_shadow_does_not_free();
    q2_get_chinese_char();
    q3_hide_details();
}

// ---------------- 问题 1：遮蔽不会回收旧变量 ----------------
struct Noisy(&'static str);
impl Drop for Noisy {
    // Drop 相当于"析构函数"：变量被释放时自动调用（第 7 课细讲）
    fn drop(&mut self) {
        println!("  释放 {}", self.0);
    }
}

fn q1_shadow_does_not_free() {
    println!("===== Q1 遮蔽 ≠ 回收 =====");
    {
        let x = Noisy("旧的 x");
        let x = Noisy("新的 x"); // 遮蔽：旧的 x 此时并没有被释放！
        println!("  遮蔽完成，作用域即将结束");
    } // 两个都在这里才释放，顺序和声明相反：先新后旧

    // 证据 2：持有旧变量的引用，遮蔽之后引用依然有效
    let s = String::from("我是旧的 s");
    let r = &s; // r 指向旧 s
    let s = 123; // 遮蔽
    println!("  r = {r}, s = {s}"); // 旧 s 还活着，r 能正常用

    // 实用技巧："冻结"：先可变地构建，构建完遮蔽成不可变
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    let v = v; // 从这里开始 v 不能再修改
    // v.push(3); // ❌ cannot borrow `v` as mutable
}

// ---------------- 问题 2：取出某个汉字 ----------------
fn q2_get_chinese_char() {
    println!("===== Q2 取汉字 =====");
    let s = "Hi你好";

    // 方法 A：按"第几个字符"取 → chars().nth(i)，返回 Option<char>
    let c = s.chars().nth(2); // Some('你')
    println!("  chars().nth(2) = {c:?}");
    let c = s.chars().nth(10); // 越界不 panic，返回 None
    println!("  chars().nth(10) = {c:?}");

    // 方法 B：按"字节范围"切片 → 得到 &str（不是 char）
    // H=0, i=1, 你=2..5, 好=5..8
    let sub = &s[2..5];
    println!("  &s[2..5] = {sub}");
    // let bad = &s[2..4]; // ❌ 运行时 panic: byte index 4 is not a char boundary
    println!("  s.get(2..4) = {:?}", s.get(2..4)); // 安全版本：切错了返回 None

    // 想知道每个字符从第几个字节开始 → char_indices()
    for (byte_idx, ch) in s.char_indices() {
        print!("  ({byte_idx}, {ch})");
    }
    println!();

    // 方法 C：需要频繁随机访问 → 一次性转成 Vec<char>（等价于 Go 的 []rune(s)）
    let chars: Vec<char> = s.chars().collect();
    println!("  chars[3] = {}", chars[3]);
}

// ---------------- 问题 3：隐藏内部细节 ----------------
mod user {
    // 元组结构体（tuple struct）：有名字的元组，字段不加 pub 就是私有的
    pub struct UserId(u64);

    impl UserId {
        pub fn new(raw: u64) -> UserId {
            UserId(raw)
        }
        pub fn display(&self) -> String {
            format!("U{:06}", self.0) // 模块内部可以访问 .0
        }
    }

    // 普通结构体：字段私有，只暴露方法（和 Go 的小写字段 + 大写方法一个意思）
    pub struct Account {
        id: UserId,
        balance: i64,
    }

    impl Account {
        pub fn new(id: UserId) -> Account {
            Account { id, balance: 0 }
        }
        pub fn deposit(&mut self, amount: i64) {
            self.balance += amount;
        }
        pub fn balance(&self) -> i64 {
            self.balance
        }
    }
}

type Point = (f64, f64); // 类型别名：只是换个名字，完全透明，(f64, f64) 和 Point 可以随便混用

fn q3_hide_details() {
    println!("===== Q3 隐藏细节 =====");
    let p: Point = (1.0, 2.0);
    let q: (f64, f64) = p; // ✅ 别名和原类型是同一个类型
    println!("  p.0 = {}", p.0); // 内部完全暴露

    let id = user::UserId::new(42);
    println!("  id = {}", id.display());
    // println!("{}", id.0);        // ❌ field `0` of struct `UserId` is private
    // let fake = user::UserId(7);  // ❌ 外部不能直接构造，只能走 new()

    let mut acc = user::Account::new(id);
    acc.deposit(100);
    println!("  balance = {}", acc.balance());
    // acc.balance = 999;           // ❌ field `balance` of struct `Account` is private
}
