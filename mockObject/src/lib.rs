pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger>{
    messenger : &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
        LimitTracker{
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let max_percent = self.value as f32 / self.max as f32 ;
        if max_percent >= 1.0{
            self.messenger.send("Error max value is exceed");
        }else if max_percent >= 0.9 {
            self.messenger.send("Warning you've used 90% or more!");
        }else if max_percent >= 0.75 {
            self.messenger.send("Warning you've used 75% or more!")
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use super::*;

    struct MessageMock {
        messages_sent: RefCell<Vec<String>>,
    }

    impl MessageMock {
        fn new() -> MessageMock{
            MessageMock{
                messages_sent: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MessageMock {
        fn send(&self, msg: &str) {
            self.messages_sent.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn send_message_at_75() {
        let messenger_mock = MessageMock::new();
        let mut tracker = LimitTracker::new(&messenger_mock, 100);

        tracker.set_value(80);

        assert_eq!(messenger_mock.messages_sent.borrow().len(), 1);
    }
}