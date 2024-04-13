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
    // the set_value() function of the LimitTracker doesn’t return anything to make assertions on
    // what we want to achieve with this test is to be able to say that 
    // if we create a LimitTracker 
    // -> with something that implements the Messenger trait 
    // -> and a particular value for max
    // then, when we pass different numbers for value, 
    // the messenger should be told to send the appropriate messages

    // for the test we a mock object that will keep track of the messages it’s told to send
    //. We can create a new instance of the mock object, create a LimitTracker that uses the mock object, call the set_value method on LimitTracker, and then check that the mock object has the messages we expect. Listing 15-21 shows an attempt to implement a mock object to do just that, but the borrow checker won’t allow it:
    
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
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        dbg!("it_sends_an_over_75_percent_warning_message");
    }
}
