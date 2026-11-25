// 第 2 课演示：Move / Copy / Borrow / Slice
// 运行：cargo run --example ownership_demo
#![allow(unused)]

fn main() {
    demo_move_vs_copy();
    demo_borrowing();
    demo_nll();
    demo_slices();
}

fn demo_move_vs_copy() {
    println!("===== 1. Move vs Copy =====");
    // 栈上简单类型（i32, bool, char, f64, 以及只含 Copy 类型的元组/数组）默认 Copy
    let a = 42;
    let b = a; // Copy：复制了一份
    println!("a = {a}, b = {b}"); // ✅ 两个都能用

    // 堆上拥有资源的类型（String, Vec<T>, HashMap 等）默认 Move（移动所有权）
    let s1 = String::from("hello");
    let s2 = s1; // Move：s1 的堆指针所有权转给 s2，s1 失效
    // println!("{s1}"); // ❌ 取消注释试试：value borrowed here after move
    println!("s2 = {s2}");

    // 真要复制堆数据，必须显式 .clone()
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");
}

fn demo_borrowing() {
    println!("===== 2. 借用规则 =====");
    let mut s = String::from("rust");

    // 规则：同一时间，要么多个只读借用(&T)，要么唯一一个可变借用(&mut T)
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}"); // ✅ 多个 &T 共存

    // r1 和 r2 在上面那行之后就不再使用了（NLL），所以现在可以可变借用！
    let r3 = &mut s;
    r3.push_str("acean");
    println!("r3 = {r3}");
}

fn demo_nll() {
    println!("===== 3. 为什么不能同时 & 和 &mut =====");
    let mut v = vec![1, 2, 3];
    // 试试取消下面三行注释，看编译器如何阻止"扩容导致悬垂指针"：
    // let first = &v[0];
    // v.push(4);
    // println!("{first}");

    // 正确做法：先读完/拷贝出来，再修改
    let first = v[0];
    v.push(4);
    println!("first = {first}, v = {v:?}");
}

fn demo_slices() {
    println!("===== 4. String vs &str =====");
    let owned: String = String::from("hello world");
    let slice: &str = &owned[0..5]; // 零拷贝，只是一个 (ptr, len) 胖指针
    let literal: &str = "hello"; // 字符串字面量直接存在二进制只读区，类型是 &'static str
    println!("owned = {owned}, slice = {slice}, literal = {literal}");
}
