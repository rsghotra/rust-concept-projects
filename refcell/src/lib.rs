pub trait Messenger {
    fn send(&self, msg: &str);
}


pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { 
            messenger, 
            value: 0, 
            max, 
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("You are over your quote!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You have used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("You have used up over 75% of your quote!!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        //create mock messenger
        let mock_messenger = MockMessenger::new();
        //create limit tracker
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        
        //call limit tracker method set_value
        limit_tracker.set_value(80);
        //assert
        assert_eq!(mock_messenger.sent_messages.len(), 1);

    }
}


