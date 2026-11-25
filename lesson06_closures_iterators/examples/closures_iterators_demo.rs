//! 第 6 课随堂演示：闭包、迭代器链与自定义 Iterator
//! 运行方式：cargo run --example closures_iterators_demo

struct Counter {
    cur: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.max {
            self.cur += 1;
            Some(self.cur)
        } else {
            None
        }
    }
}

fn main() {
    let factor = 10;
    let scale = |x: i32| x * factor; // Fn: 不可变借用 factor
    println!("scale(4) = {}", scale(4));

    let nums = [1, 2, 3, 4, 5, 6];
    let even_squares_sum: i32 = nums
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("偶数平方和 = {even_squares_sum}");

    let c = Counter { cur: 0, max: 5 };
    let doubled: Vec<u32> = c.map(|n| n * 2).collect();
    println!("自定义 Counter 迭代器 * 2 = {doubled:?}");
}
