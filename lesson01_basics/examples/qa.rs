// 第 1 课答疑示例，运行：cargo run --example qa
#![allow(unused)]

fn main() {
    q1_shadowing();
    q2_str_vs_char();
    q3_block_expression();
    q4_labels();
    q5_tuples();
}

// ---------------- 问题 1：shadowing ----------------
fn q1_shadowing() {
    println!("===== Q1 shadowing =====");
    let input = "  42 "; // 变量①：类型 &str
    let input = input.trim(); // 变量②：类型 &str，值 "42"，①被"遮住"
    let input: i32 = input.parse().unwrap(); // 变量③：类型 i32，值 42
    println!("input = {input}");

    // 内层作用域里的遮蔽，出了作用域就失效，外层的旧变量"重新露出来"
    let x = 1;
    {
        let x = "hello"; // 只在这对花括号里有效
        println!("内层 x = {x}");
    }
    println!("外层 x = {x}"); // 还是 1

    // 对比 mut：mut 能改值，但不能改类型
    let mut y = 1;
    y = 2; // ✅
    // y = "two"; // ❌ expected integer, found `&str`
}

// ---------------- 问题 2：str 不是 char 数组 ----------------
fn q2_str_vs_char() {
    println!("===== Q2 str vs char =====");
    let s = "Hi你好";
    println!("len() = {} （字节数，不是字符数）", s.len()); // 2 + 3*2 = 8
    println!("chars().count() = {} （字符数）", s.chars().count()); // 4

    // let c = s[0]; // ❌ Rust 不允许按下标取字符（Go 的 s[0] 会给你一个 byte）

    for c in s.chars() {
        // 相当于 Go 的 for _, r := range s
        print!("[{c}] ");
    }
    println!();
    for b in s.bytes() {
        // 相当于 Go 的 for i := 0; i < len(s); i++ { s[i] }
        print!("{b:#x} ");
    }
    println!();

    println!("size_of::<char>() = {} 字节", std::mem::size_of::<char>()); // 4

    // 真要"char 数组"，就显式转换成 Vec<char>
    let v: Vec<char> = s.chars().collect();
    println!("Vec<char> 第 3 个字符 = {}", v[2]); // 你
}

// ---------------- 问题 3：表达式与分号 ----------------
fn q3_block_expression() {
    println!("===== Q3 表达式 =====");
    let label = 1 > 0; // let 是语句，必须有分号；右边 `1 > 0` 是表达式，值为 true
    println!("label = {label}");

    // 代码块 {} 也是表达式：最后一个没有分号的表达式就是整个块的值
    let area = {
        let w = 3;
        let h = 4;
        w * h // 没分号 → 块的值是 12
    };
    println!("area = {area}");

    println!("is_positive(5) = {}", is_positive(5));
}

fn is_positive(n: i32) -> bool {
    n > 0 // 函数体也是一个块，最后这个表达式就是返回值
}

// ---------------- 问题 4：循环标签 ----------------
fn q4_labels() {
    println!("===== Q4 循环标签 =====");
    // 场景 1：在二维网格里找目标，找到就同时跳出两层
    let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut pos = None;
    'search: for (r, row) in grid.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val == 5 {
                pos = Some((r, c));
                break 'search;
            }
        }
    }
    println!("5 在 {pos:?}");

    // 场景 2：continue 外层 —— 跳过"含有负数的行"，只累加全正的行
    let data = [[1, 2], [3, -1], [5, 6]];
    let mut sum = 0;
    'rows: for row in data {
        for v in row {
            if v < 0 {
                continue 'rows; // 直接处理下一行
            }
        }
        sum += row[0] + row[1];
    }
    println!("全正行之和 = {sum}"); // 1+2+5+6 = 14

    // 场景 3：带标签的块（不是循环也能 break），用来"提前结束一段逻辑并给出值"
    let n = 15;
    let desc = 'check: {
        if n % 15 == 0 {
            break 'check "FizzBuzz";
        }
        if n % 3 == 0 {
            break 'check "Fizz";
        }
        "other"
    };
    println!("{n} -> {desc}");
}

// ---------------- 问题 5：函数返回值与元组 ----------------
fn q5_tuples() {
    println!("===== Q5 元组 =====");
    say_hi(); // 没写 -> 的函数返回 ()

    let t: (i32, String, bool) = (1, String::from("a"), true); // 元素类型可以不同
    let nested: ((i32, i32), (f64, char)) = ((1, 2), (3.5, 'x')); // 可以嵌套
    println!("nested.1.1 = {}", nested.1.1); // 'x'

    type Point = (f64, f64); // 起个类型别名
    let p: Point = (1.0, 2.0);

    // 元组是"一等公民"：能存进变量、放进集合、作为参数
    let r = div_rem(17, 5); // 整个元组存进一个变量（Go 的多返回值做不到）
    let list: Vec<(String, u32)> = vec![("Tom".to_string(), 18), ("Amy".to_string(), 20)];
    println!("r = {r:?}, list = {list:?}");

    let unit: () = (); // 空元组，叫 unit，相当于"没有值"
    let one = (5,); // 单元素元组必须带逗号
    let not_tuple = (5); // 这只是带括号的 5，类型是 i32
}

fn say_hi() {
    println!("hi");
}

fn div_rem(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
