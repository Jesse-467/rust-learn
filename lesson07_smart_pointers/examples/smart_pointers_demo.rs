//! 第 7 课随堂演示：Box、Rc、RefCell 与 Drop
//! 运行方式：cargo run --example smart_pointers_demo

use std::cell::RefCell;
use std::rc::Rc;

struct ScopeGuard(&'static str);
impl Drop for ScopeGuard {
    fn drop(&mut self) {
        println!("[Drop] 自动释放资源: {}", self.0);
    }
}

fn main() {
    println!("=== 1. Box 堆分配与 Drop ===");
    {
        let _g = ScopeGuard("db_conn");
        let b = Box::new(42);
        println!("Box 值 = {}", *b);
    } // _g 在此处自动触发 Drop

    println!("
=== 2. Rc 多所有权共享 ===");
    let shared = Rc::new(String::from("global-config"));
    println!("初始引用计数 = {}", Rc::strong_count(&shared));
    {
        let c1 = Rc::clone(&shared);
        let c2 = Rc::clone(&shared);
        println!("克隆两次后计数 = {}, 内容 = {c1}, {c2}", Rc::strong_count(&shared));
    }
    println!("子作用域结束后计数 = {}", Rc::strong_count(&shared));

    println!("
=== 3. Rc<RefCell<T>> 共享且可变 ===");
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;
    println!("共享计数器最终值 = {}", counter.borrow());
}
