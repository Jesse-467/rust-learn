//! 练习 2：借用（&T 与 &mut T）

/// 2.1 原地把 x 翻倍。
///
/// 提示：基础类型通过可变引用修改值需要解引用 `*x`，和 Go 的 `*p *= 2` 一样。
pub fn double_in_place(x: &mut i32) {
    *x *= 2
}

/// 2.2 通过可变引用交换两个 i32 的值。
/// （对照 Go 的 `func swap(a, b *int)`）
///
/// 提示：可以用临时变量，也可以用标准库 `std::mem::swap(a, b)`。
/// 建议先自己用 `*a` / `*b` 写一遍，感受解引用。
pub fn swap_values(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b
}

/// 2.3 往 `buf` 末尾追加 `"Hello, <name>!"`。
/// 例如：buf 原来是 `"Hi. "`，name 是 `"Go"` -> buf 变成 `"Hi. Hello, Go!"`。
///
/// 注意：调用 `buf.push_str(...)` 时不需要写 `(*buf).push_str(...)`，
/// Rust 会在方法调用时自动解引用（和 Go 一样）。
pub fn append_greeting(buf: &mut String, name: &str) {
    buf.push_str(&format!("Hello, {name}!"));
}

/// 2.4 把 `nums` 里**现有**的每个元素都翻倍，依次追加到 `nums` 末尾。
/// 例如：[1, 2, 3] -> [1, 2, 3, 2, 4, 6]
///
/// 借用检查器初体验：
/// 如果你写 `for x in &*nums { nums.push(x * 2); }`，编译器会直接拒绝！
/// 因为循环期间 `nums` 被不可变借用着，同时又在循环体里可变借用 `nums.push(...)`。
/// （Go 里边遍历 slice 边 append 很容易写出 bug，Rust 在编译期就把这条路封死了）
///
/// 提示：先记下 `let len = nums.len();`，再用下标 `for i in 0..len` 遍历。
pub fn push_doubled(nums: &mut Vec<i32>) {
    let len = nums.len();
    for x in 0..len {
        nums.push(nums[x] * 2);
    }
}

// ======================== 以下是测试，不要修改 ========================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_in_place() {
        let mut n = 21;
        double_in_place(&mut n);
        assert_eq!(n, 42);
    }

    #[test]
    fn test_swap_values() {
        let mut a = 1;
        let mut b = 99;
        swap_values(&mut a, &mut b);
        assert_eq!((a, b), (99, 1));
    }

    #[test]
    fn test_append_greeting() {
        let mut buf = String::from("Hi. ");
        append_greeting(&mut buf, "Gopher");
        // println!("{buf}");
        assert_eq!(buf, "Hi. Hello, Gopher!");
    }

    #[test]
    fn test_push_doubled() {
        let mut v = vec![1, 2, 3];
        push_doubled(&mut v);
        assert_eq!(v, vec![1, 2, 3, 2, 4, 6]);

        let mut empty: Vec<i32> = vec![];
        push_doubled(&mut empty);
        assert_eq!(empty, vec![]);
    }
}
