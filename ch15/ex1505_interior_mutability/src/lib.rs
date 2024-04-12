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

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        dbg!("it_sends_an_over_75_percent_warning_message");
    }
}
