//! 练习 2：`tokio::spawn`、`tokio::sync::mpsc` 与 `oneshot`

use tokio::sync::{mpsc, oneshot};

/// 2.1 为 `inputs` 中的每个数字 `tokio::spawn` 一个异步任务计算 `x * 2`，
/// 通过 `tokio::sync::mpsc::channel` 发回汇总，返回排序后的结果 `Vec<i32>`。
pub async fn parallel_double_async(inputs: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// 2.2 `oneshot` 一次性通道（常用于给后台 actor 发送“带回执的请求”）：
/// 启动一个后台 task 等待从 `rx` 收到 `(a, b, reply_tx)`，计算 `a + b` 后通过 `reply_tx` 发回。
/// 本函数返回发送端，供调用方发起一次异步加法请求。
pub struct AddRequest {
    pub a: i32,
    pub b: i32,
    pub resp: oneshot::Sender<i32>,
}

pub fn spawn_adder_worker() -> mpsc::Sender<AddRequest> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_double_async() {
        let res = parallel_double_async(vec![3, 1, 4, 2]).await;
        assert_eq!(res, vec![2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn test_oneshot_worker() {
        let tx = spawn_adder_worker();
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(AddRequest {
            a: 19,
            b: 23,
            resp: resp_tx,
        })
        .await
        .unwrap();
        assert_eq!(resp_rx.await.unwrap(), 42);
    }
}
