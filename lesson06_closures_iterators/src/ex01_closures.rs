//! 练习 1：闭包（`Fn`、`FnMut`、`move`）

/// 1.1 高阶函数：对 `x` 连续应用两次闭包 `f`，返回 `f(f(x))`。
pub fn apply_twice<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    todo!()
}

/// 1.2 返回一个“加法器”闭包：每次调用传入 `x`，返回 `x + delta`。
/// 提示：因为闭包离开函数后仍要存活，必须用 `move |x| ...` 捕获 `delta`！
pub fn make_adder(delta: i32) -> impl Fn(i32) -> i32 {
    move |_x| todo!()
}

/// 1.3 遍历切片 `items`，对每个满足 `pred` 的元素调用一次 `on_match`（注意 `on_match` 是 `FnMut`，可修改外部状态）。
pub fn for_each_matching<P, F>(items: &[i32], pred: P, mut on_match: F)
where
    P: Fn(i32) -> bool,
    F: FnMut(i32),
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_twice() {
        assert_eq!(apply_twice(3, |x| x * 2), 12);
        assert_eq!(apply_twice(5, |x| x + 1), 7);
    }

    #[test]
    fn test_make_adder() {
        let add10 = make_adder(10);
        assert_eq!(add10(5), 15);
        assert_eq!(add10(-3), 7);
    }

    #[test]
    fn test_for_each_matching() {
        let mut sum = 0;
        let mut count = 0;
        for_each_matching(&[1, 2, 3, 4, 5, 6], |x| x % 2 == 0, |x| {
            sum += x;
            count += 1;
        });
        assert_eq!(sum, 12);
        assert_eq!(count, 3);
    }
}
