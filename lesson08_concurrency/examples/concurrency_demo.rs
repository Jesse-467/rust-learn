//! 第 8 课随堂演示：thread::scope、mpsc::channel、Arc<Mutex<T>>
//! 运行方式：cargo run --example concurrency_demo

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    println!("=== 1. thread::scope 零拷贝并行借用 ===");
    let data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at(3);
    let total: i32 = thread::scope(|s| {
        let h1 = s.spawn(|| left.iter().sum::<i32>());
        let h2 = s.spawn(|| right.iter().sum::<i32>());
        h1.join().unwrap() + h2.join().unwrap()
    });
    println!("并行求和 = {total}");

    println!("
=== 2. mpsc 多生产者 Channel ===");
    let (tx, rx) = mpsc::channel();
    for worker_id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("worker-{worker_id} done")).unwrap();
        });
    }
    drop(tx); // 释放原始发送端，这样所有子线程结束后 rx 才会自然关闭
    for msg in rx {
        println!("收到: {msg}");
    }

    println!("
=== 3. Arc<Mutex<T>> 共享状态 ===");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("最终计数 = {}", *counter.lock().unwrap());
}
