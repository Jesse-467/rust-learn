//! 练习 4：元组（tuple）与数组（array）

/// 4.1 同时返回商和余数 —— Go 里你会写 `func divRem(a, b int) (int, int)`，
///     Rust 里没有"多返回值"，而是返回一个元组。
/// 例如：div_rem(17, 5) -> (3, 2)
pub fn div_rem(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

/// 4.2 返回数组中的 (最小值, 最大值)。
///
/// 要求：用 `for x in arr { ... }` 遍历。
/// 提示：初始值可以用 `arr[0]`，也可以用 `i32::MAX` / `i32::MIN`
pub fn min_max(arr: [i32; 5]) -> (i32, i32) {
    let mut ans = (i32::MAX, i32::MIN);
    for x in arr {
        if x < ans.0 {
            ans.0 = x;
        }
        if x > ans.1 {
            ans.1 = x;
        }
    }
    ans
}

/// 4.3 交换元组的两个元素。
///
/// 要求：用解构 `let (x, y) = pair;` 写。
/// 注意：元组的元素类型可以不同，这里输入是 (i32, bool)，输出是 (bool, i32)
pub fn swap(pair: (i32, bool)) -> (bool, i32) {
    let (x, y) = pair;
    (y, x)
    // 或者直接 (pair.1, pair.0)
}

/// 4.4 原地反转数组并返回。例如 [1,2,3,4,5] -> [5,4,3,2,1]
///
/// 要求：参数前加 `mut`（写成 `mut arr: [i32; 5]`），用下标交换，不要新建数组。
/// 提示：数组有个 `.swap(i, j)` 方法；也可以自己用临时变量交换。
///
/// 思考：调用方传进来的数组会被修改吗？（答案见下面的测试 test_reverse_is_copy）
pub fn reverse(arr: [i32; 5]) -> [i32; 5] {
    let mut arr = arr;
    for i in 0..arr.len() {
        let j = arr.len() - i - 1;
        if i < j {
            arr.swap(i, j);
        }
    }
    arr
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_rem() {
        assert_eq!(div_rem(17, 5), (3, 2));
        assert_eq!(div_rem(10, 2), (5, 0));
        // 注意：Rust 和 Go 一样，整数除法向零取整，余数符号跟随被除数
        assert_eq!(div_rem(-7, 2), (-3, -1));
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_max([3, 1, 4, 1, 5]), (1, 5));
        assert_eq!(min_max([-2, -8, 0, 7, 7]), (-8, 7));
        assert_eq!(min_max([42, 42, 42, 42, 42]), (42, 42));
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap((1, true)), (true, 1));
        assert_eq!(swap((-9, false)), (false, -9));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse([1, 2, 3, 4, 5]), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_is_copy() {
        let original = [1, 2, 3, 4, 5];
        let reversed = reverse(original);
        assert_eq!(reversed, [5, 4, 3, 2, 1]);
        // 数组是按值传递（复制）的，和 Go 的数组一样，原数组不受影响
        assert_eq!(original, [1, 2, 3, 4, 5]);
    }
}
