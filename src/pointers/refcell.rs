use std::cell::RefCell;
use std::rc::Rc;

/**
 `RefCell<T>` represents single ownership over the data it holds.

With references and `Box<T>` the borrowing rules inveraints are enforced at compile time.
With `RefCell<T>` these inveriants are enforced *at runtime*.

`RefCell<T>` *ONLY FOR SINGLE_THREADED SCENATIOS*

Interior Mutability - mutable borrow to an immutable value
 */

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
            self.messenger.send("Error: you are over quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: you've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/**
 For the implementation of the `send` method, the first parameter is still an immutable borrow
of `self` which matches the trait definition. Call `borrow_mut` on the `RefCell<Vec<String>>` in
`self.sent_messages` to get a mutable reference to the value inside the `RefCell<Vec<String>>`.

To see how many items are in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an immutable reference
to the vector.


*! we can't create 2 or more borrow_mut at time to one struct in one scope*

```
let mut one = self.sent_messages.borrow_mut();
let mut two = self.sent_messages.borrow_mut(); // rust will panic at runtime
```
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{RefCell};

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/**
#### Multiple ownes of mutable data by combining `Rc<T>` and `RefCell<T>`

  Create a value that is an instance of `Rc<RefCell<i32>>` and store it in a variable `value`.
Then create a `List` in `a` with a `Cons` variant that holds `value`. We need to clone `value`
so both `a` and `value` have ownership of the inner `5` value rather than transferring ownership from
`value` to `a` or having `a` borrow from `value`.

  Wrap the list `a` i an `Rc<T>` so when we create list `b` and `c`, the can both refer to `a`.

  *output*
```
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

`RefCell<T>` work only in single-thread. If you need multithread use - `Mutex<T>`
 */
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn rc_rccell_list_usage() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(
        List::Cons(
            Rc::clone(&value),
            Rc::new(List::Nil),
        )
    );
    let b = List::Cons(
        Rc::new(
            RefCell::new(3)
        ),
        Rc::clone(&a),
    );
    let c = List::Cons(
        Rc::new(
            RefCell::new(4)
        ),
        Rc::clone(&a),
    );
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}