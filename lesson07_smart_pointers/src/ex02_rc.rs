//! 练习 2：`Rc<T>` 单线程多所有权共享

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct AppConfig {
    pub env: String,
    pub max_conn: usize,
}

pub struct Service {
    pub name: String,
    pub config: Rc<AppConfig>,
}

/// 2.1 用同一份 `Rc<AppConfig>` 创建多个 `Service`，避免重复深拷贝配置对象。
/// 提示：循环中用 `Rc::clone(&config)` 增加引用计数。
pub fn spawn_services(names: &[&str], config: Rc<AppConfig>) -> Vec<Service> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_shared_config() {
        let cfg = Rc::new(AppConfig {
            env: "prod".to_string(),
            max_conn: 100,
        });
        assert_eq!(Rc::strong_count(&cfg), 1);

        let services = spawn_services(&["auth", "order", "pay"], Rc::clone(&cfg));
        assert_eq!(services.len(), 3);
        // 1 (cfg) + 3 (services) = 4
        assert_eq!(Rc::strong_count(&cfg), 4);
        assert_eq!(services[0].config.env, "prod");

        drop(services);
        assert_eq!(Rc::strong_count(&cfg), 1);
    }
}
