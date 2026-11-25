//! 练习 3：`RefCell<T>` 内部可变性与 `Rc<RefCell<T>>`

use std::cell::RefCell;
use std::rc::Rc;

/// 模拟一个“要求 `&self` 不可变引用”的外部接口（在单元测试里做 Mock 时极常见）
pub trait Messenger {
    fn send(&self, msg: &str);
}

#[derive(Default)]
pub struct MockMessenger {
    pub sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.sent_messages.borrow().len()
    }
}

/// 3.1 虽然 `send` 只拿到了 `&self`，但请用 `self.sent_messages.borrow_mut().push(...)` 记录消息！
impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        todo!()
    }
}

/// 3.2 `Rc<RefCell<i32>>`：传入一个共享计数器引用，给里面的值加上 `delta`，返回更新后的值。
pub fn add_to_shared(counter: &Rc<RefCell<i32>>, delta: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_messenger() {
        let mock = MockMessenger::new();
        mock.send("hello");
        mock.send("rust");
        assert_eq!(mock.count(), 2);
        assert_eq!(
            *mock.sent_messages.borrow(),
            vec!["hello".to_string(), "rust".to_string()]
        );
    }

    #[test]
    fn test_shared_counter() {
        let c = Rc::new(RefCell::new(10));
        let c2 = Rc::clone(&c);
        assert_eq!(add_to_shared(&c, 5), 15);
        assert_eq!(add_to_shared(&c2, -3), 12);
        assert_eq!(*c.borrow(), 12);
    }
}
