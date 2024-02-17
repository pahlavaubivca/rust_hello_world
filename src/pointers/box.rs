use std::ops::Deref;
use crate::pointers::r#box::List::{Cons, Nil};

/**
Box<T> for allocating values on the heap

Most cases when to use:
 - for type whose size can't be known at compile time and
you want to use a value of that type in a context that requires an exact size
 - when you have a large amount of data and you want to transfer ownership
 but ensure the data won't be copied when you do so
 - when you want to own a value and you care only that it's a type that implements a particular trait
 rather than being of a specific type
 */

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

// use crate::List::{Cons, Nil};

fn nested_list() {
    let list = Cons(1,
                    Box::new(
                        Cons(2,
                             Box::new(
                                 Cons(3,
                                      Box::new(Nil),
                                 )
                             ),
                        )
                    ),
    );
    // Cons()
}

