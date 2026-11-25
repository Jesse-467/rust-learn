//! 第 9 课随堂演示：tokio::join!、spawn、mpsc、select! 与 timeout
//! 运行方式：cargo run --example async_tokio_demo

use tokio::sync::mpsc;
use tokio::time::{Duration, Instant, sleep, timeout};

async fn fake_rpc(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} ok ({ms}ms)")
}

#[tokio::main]
async fn main() {
    println!("=== 1. tokio::join! 并发执行 ===");
    let start = Instant::now();
    let (r1, r2) = tokio::join!(fake_rpc("user-svc", 40), fake_rpc("order-svc", 40));
    println!("结果: {r1} | {r2}，总耗时仅约 {:?}（非 80ms）", start.elapsed());

    println!("
=== 2. tokio::spawn + mpsc ===");
    let (tx, mut rx) = mpsc::channel(8);
    for i in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("task-{i}")).await.unwrap();
        });
    }
    drop(tx);
    while let Some(msg) = rx.recv().await {
        println!("收到异步消息: {msg}");
    }

    println!("
=== 3. timeout 与 select! ===");
    let res = timeout(Duration::from_millis(20), fake_rpc("slow-svc", 100)).await;
    println!("超时测试结果: {res:?}");

    tokio::select! {
        v = fake_rpc("fast", 15) => println!("select 胜出: {v}"),
        v = fake_rpc("slow", 60) => println!("select 胜出: {v}"),
    }
}
