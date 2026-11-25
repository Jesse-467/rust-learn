//! 练习 1：泛型函数、泛型结构体与 Trait Bound

/// 1.1 返回切片中最大元素的引用；切片为空时返回 `None`。
/// 注意约束：`T: PartialOrd` 才能用 `>` 比较大小。
pub fn max_in_slice<T: PartialOrd>(items: &[T]) -> Option<&T> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    /// 1.2 构造一个 `Pair<T>`。
    pub fn new(first: T, second: T) -> Self {
        todo!()
    }

    /// 1.3 交换 `first` 和 `second` 的值，返回一个新的 `Pair<T>`（消费 `self`）。
    pub fn swap(self) -> Self {
        todo!()
    }
}

/// 只有当 `T` 支持比较（`PartialOrd`）时，才给 `Pair<T>` 提供 `max()` 方法！
impl<T: PartialOrd> Pair<T> {
    /// 1.4 返回 `first` 和 `second` 中较大者的引用（相等时返回 `&self.first`）。
    pub fn max(&self) -> &T {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_in_slice() {
        assert_eq!(max_in_slice(&[3, 9, 2, 7]), Some(&9));
        assert_eq!(max_in_slice(&["a", "z", "m"]), Some(&"z"));
        assert_eq!(max_in_slice::<i32>(&[]), None);
    }

    #[test]
    fn test_pair() {
        let p = Pair::new(10, 25);
        assert_eq!(p.max(), &25);
        let swapped = p.swap();
        assert_eq!(swapped, Pair::new(25, 10));
    }
}
