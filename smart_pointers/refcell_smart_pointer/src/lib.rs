pub trait Messenger {
    // Applications that use our library will be expected to provide the mechanism for sending the messages
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    // We want to be able to say that if we create a LimitTracker with something that implements the Messenger trait and a particular value for max.  
    // When we pass different numbers for value, the messenger is told to send the appropriate messages.

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// We need a mock object that, instead of sending an email or text message when we call send, will only keep track of the messages it’s told to send. 
// We can create a new instance of the mock object, create a LimitTracker that uses the mock object, call the set_value method on LimitTracker, and then check that the mock object has the messages we expect.

/* ERROR: Cannot modify the sent_message log in the MockMessenger object since self is not mutable!
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![]
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // ERROR: `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
            // self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
*/

// ERROR ERROR:
// We can’t modify the MockMessenger to keep track of the messages, because the send method takes an immutable reference to self!
// Can't use &mut self instead, because then the signature of send wouldn’t match the signature in the Messenger trait definition!

// This is a situation in which interior mutability can help! We’ll store the sent_messages within a RefCell<T>

// We will wrap the sent_messages vector in a RefCell<T>, in order to enable interior mutability!

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    // For the implementation of the send method, the first parameter is still an immutable borrow of self, which matches the trait definition. 
    // We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>, which is the vector.
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // FIXED: `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // sent_messages here is still an immutable borrow
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


// Keeping Track of Borrows at Runtime Using RefCell<T>

// With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>
// The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>
// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.

/* ERROR: Can't do two mutable borrows, against borrowing rules!

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

*/

// Notice that the code panicked with the message already borrowed: BorrowMutError. This is how RefCell<T> handles violations of the borrowing rules at runtime.
// However, using RefCell<T> makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while you’re using it in a context where only immutable values are allowed.
// And it also incurs a small performance penalty at runtime, so best used sparingly.

