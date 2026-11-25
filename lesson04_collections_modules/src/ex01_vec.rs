//! 练习 1：`Vec<T>` 高频操作与所有权

/// 1.1 原地移除 `nums` 中所有小于 `min_val` 的元素。
/// 提示：用 `nums.retain(|&x| ...)` 一行就能搞定！
pub fn remove_below(nums: &mut Vec<i32>, min_val: i32) {
    todo!()
}

/// 1.2 返回一个**升序排序且去重**的新 `Vec<i32>`（不修改输入切片）。
/// 提示：`nums.to_vec()` 转成 `Vec`，然后 `.sort()` + `.dedup()`。
pub fn unique_sorted(nums: &[i32]) -> Vec<i32> {
    todo!()
}

/// 1.3 给定一个字符串切片列表，原地给每个字符串末尾追加 `"!"`。
/// 提示：`for s in items.iter_mut()` 或 `for s in items`（因为 `items` 本身就是 `&mut [String]`）。
pub fn exclaim_all(items: &mut [String]) {
    todo!()
}

/// 1.4 把两个已按升序排列的切片合并为一个新的升序 `Vec<i32>`（归并，时间复杂度 O(n+m)）。
pub fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_below() {
        let mut v = vec![1, 5, 2, 8, 3, 10];
        remove_below(&mut v, 4);
        assert_eq!(v, vec![5, 8, 10]);
    }

    #[test]
    fn test_unique_sorted() {
        assert_eq!(unique_sorted(&[3, 1, 2, 3, 1, 4]), vec![1, 2, 3, 4]);
        assert_eq!(unique_sorted(&[]), vec![]);
    }

    #[test]
    fn test_exclaim_all() {
        let mut v = vec!["go".to_string(), "rust".to_string()];
        exclaim_all(&mut v);
        assert_eq!(v, vec!["go!", "rust!"]);
    }

    #[test]
    fn test_merge_sorted() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 3, 6]), vec![1, 2, 3, 3, 5, 6]);
        assert_eq!(merge_sorted(&[], &[1, 2]), vec![1, 2]);
    }
}
