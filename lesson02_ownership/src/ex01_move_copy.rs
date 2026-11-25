//! 练习 1：Move（移动）、Copy（复制）、Clone（深拷贝）

/// 辅助函数（已写好，不要修改）：拿走 String 的所有权，转成大写并在末尾加 '!' 后返回。
fn shout(mut s: String) -> String {
    s = s.to_uppercase();
    s.push('!');
    s
}

/// 1.1 接收 s 的所有权，在末尾追加 `"!!!"` 后返回。
///
/// 提示：参数可以写成 `mut s: String`，用 `s.push_str("!!!")` 追加。
pub fn add_exclaim(s: String) -> String {
    let mut ans = s;
    ans.push_str("!!!");
    ans
}

/// 1.2 调用上面的 `shout` 函数，但同时把**原始字符串**和**shout 后的结果**作为元组一起返回。
/// 例如："rust" -> ("rust", "RUST!")
///
/// 思考：如果你直接写 `let shouted = shout(s); (s, shouted)`，编译器会报什么错？
/// 提示：用 `.clone()` 显式深拷贝一份再传给 `shout`。
pub fn shout_and_keep(s: String) -> (String, String) {
    let s1 = s.clone();
    (s, shout(s1))
    // 直接let shouted = shout(s); (s, shouted)的话，第二句里面的s已经在第一句把所有权交出去了，所以s不存在
}

/// 1.3 计算 `Vec<i32>` 中所有元素的和，并返回 `(总和, 元素个数)`。
///
/// 踩坑体验：如果你直接写 `for x in v { ... }`，循环会**消费（Move）**掉 `v`，
/// 循环结束后再写 `v.len()` 就会编译报错！
/// 请尝试理解原因，并用两种方式之一解决：
/// - 方式 A：在循环之前先取 `let len = v.len();`
/// - 方式 B：用借用遍历 `for x in &v { ... }`（推荐）
pub fn sum_and_count(v: Vec<i32>) -> (i32, usize) {
    let mut ans = 0;
    for x in &v {
        ans += x;
    }
    (ans, v.len())
    // 你这个方式a本质上不还是消费了v里面的所有内容吗，感觉一个sum函数求和的时候就把内容释放消费了不太合适吧？
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_exclaim() {
        let s = String::from("hello");
        let s = add_exclaim(s);
        assert_eq!(s, "hello!!!");
    }

    #[test]
    fn test_shout_and_keep() {
        let s = String::from("rust");
        assert_eq!(
            shout_and_keep(s),
            (String::from("rust"), String::from("RUST!"))
        );
    }

    #[test]
    fn test_sum_and_count() {
        assert_eq!(sum_and_count(vec![1, 2, 3, 4]), (10, 4));
        assert_eq!(sum_and_count(vec![]), (0, 0));
        assert_eq!(sum_and_count(vec![-5, 10]), (5, 2));
    }
}
