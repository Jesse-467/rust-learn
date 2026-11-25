//! 练习 4：和借用检查器和平共处（NLL 与避免借用冲突）

/// 4.1 取出 `v` 的第一个元素，同时把 `val` 追加到 `v` 末尾，最后返回那个第一个元素。
/// （测试保证 `v` 非空）
///
/// 思考：为什么下面这种写法会编译失败？
///     let first = &v[0];   // 不可变借用 v
///     v.push(val);         // ❌ 可变借用 v（push 可能导致扩容搬家，first 会变成悬垂指针！）
///     *first               // 这里还在使用 first
///
/// 怎么改？因为 `i32` 实现了 `Copy`，直接 `let first = v[0];` 拷贝出一个值，
/// 就不存在对 `v` 的借用了！
pub fn first_and_push(v: &mut Vec<i32>, val: i32) -> i32 {
    let t = v[0];
    v.push(val);
    t
}

/// 4.2 原地去掉 `s` 两端的空白字符，并返回去掉空格后的字节长度。
/// 例如："  rust  " -> s 变成 "rust"，返回 4。
///
/// 提示：`s.trim()` 返回的是借用 `s` 的 `&str`，不能直接赋给 `*s`。
/// 先用 `.to_string()` 转成独立的新 `String`，再赋给 `*s`：
///     let trimmed = s.trim().to_string();
///     *s = trimmed;
/// 感受一下：一旦 `.to_string()` 完成，对 `s` 的不可变借用就结束了（NLL），
/// 所以下一行 `*s = ...` 的可变借用可以顺利通过！
pub fn trim_in_place(s: &mut String) -> usize {
    let t = s.trim().to_string();
    *s = t;
    s.len()
}

/// 4.3 把切片 `nums` 在下标 `mid` 处一分为二，分别求左右两半的和，返回 `(左和, 右和)`。
/// 例如：split_sum(&[1, 2, 3, 4, 5], 2) -> (3, 12)
///
/// 提示：可以用 `&nums[..mid]` 和 `&nums[mid..]`，也可以用 `let (left, right) = nums.split_at(mid);`。
/// 注意：同时持有同一个切片的两个**不可变**子切片是完全合法的！
pub fn split_sum(nums: &[i32], mid: usize) -> (i32, i32) {
    let (mut l, mut r) = (0, 0);
    for x in &nums[..mid] {
        l += x;
    }
    for x in &nums[mid..] {
        r += x;
    }
    (l, r)
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_push() {
        let mut v = vec![10, 20, 30];
        let first = first_and_push(&mut v, 40);
        assert_eq!(first, 10);
        assert_eq!(v, vec![10, 20, 30, 40]);
    }

    #[test]
    fn test_trim_in_place() {
        let mut s = String::from("  hello rust \n");
        let len = trim_in_place(&mut s);
        assert_eq!(s, "hello rust");
        assert_eq!(len, 10);
    }

    #[test]
    fn test_split_sum() {
        assert_eq!(split_sum(&[1, 2, 3, 4, 5], 2), (3, 12));
        assert_eq!(split_sum(&[1, 2, 3], 0), (0, 6));
        assert_eq!(split_sum(&[1, 2, 3], 3), (6, 0));
    }
}
