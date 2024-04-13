#![allow(dead_code, unused_imports)]
/// Interior Mutability (a mutable borrow to an immutable value)

/// example: creating a library that tracks a value against a maximum value 
/// messages are sent based on how close to the maximum value the current value is

/// example library will only provide 
/// -> the functionality of tracking how close to the maximum a value is 
/// -> and what the messages should be at what times
/// applications that use our library will be expected to provide the mechanism 
/// for sending the messages
/// -> the application could put a message in the application, 
///    send an email, send a text message or something else
/// 
/// all the library itself needs is something that implements a trait 
/// we’ll provide called Messenger
/// 
/// a library to keep track of how close a value is to a maximum value 
/// and warn when the value is at certain levels
/// sourcecode taken from https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

pub trait Messenger {
    // the Messenger trait has only one method called send 
    // send() takes an immutable reference to self and the text of the message
    fn send(&self, msg: &str);
}

/// the Messenger trait is the interface the LimitTracker struct needs to implement 
/// so that it can be used in the same way a real object is
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
    
    // set_value() is the behavior of the LimitTracker that we want to test
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

#[cfg(test)]
mod tests {
    use super::*;
    // adding interior mutability with RefCell
    // we’ll store the sent_messages within a RefCell<T>, 
    // and by this the send method will be able to modify sent_messages to store the messages 
    use std::cell::{Ref, RefCell};

    // the set_value() function of the LimitTracker doesn’t return anything to make assertions on
    // what we want to achieve with this test is to be able to say that 
    // if we create a LimitTracker 
    // -> with something that implements the Messenger trait 
    // -> and a particular value for max
    // then, when we pass different numbers for value, 
    // the messenger should be told to send the appropriate messages

    // for the test we a mock object that will keep track of the messages it’s told to send
    // we'll create a new instance of the mock object, 
    // create a LimitTracker that uses the mock object, 
    // call the set_value method on LimitTracker, 
    // then check that the mock object has the messages we expect
    
    // defining a MockMessenger struct that has a sent_messages field
    // to keep track of the messages it’s told to send
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        // defining an associated function new to create new MockMessenger values
        // that start with an empty list of messages
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }
    
    // implementing the Messenger trait for MockMessenger 
    // so we can give a MockMessenger to a LimitTracker
    impl Messenger for MockMessenger {
        // taking the message passed in as a parameter 
        // and storing it in the MockMessenger list of sent_messages
        fn send(&self, message: &str) {
            // we now can call borrow_mut on the RefCell<Vec<String>> 
            // in self.sent_messages to get a mutable reference 
            // to the value inside the RefCell<Vec<String>>
            // then we can call push on the mutable reference 
            // to the vector to keep track of the messages sent during the test
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // testing what happens when the LimitTracker is told 
        // to set value to something that is more than 75 percent of the max value
        
        // creating a new MockMessenger with an empty list of messages
        let mock_messenger = MockMessenger::new();
        // creating a new LimitTracker 
        // with a reference to the new MockMessenger 
        // and a max value of 100
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // calling the set_value method on the LimitTracker with a value of 80
        limit_tracker.set_value(80);
        // asserting that the list of messages has now one message in it
        // calling borrow on the RefCell<Vec<String>> to get an immutable reference to the vector
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
