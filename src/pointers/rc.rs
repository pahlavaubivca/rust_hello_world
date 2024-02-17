use std::rc::Rc;

/**
Rc<T> reference counting type that enables multiple ownership

cases when a single value might have multiple owners. Example - graph data structure
where multiple edges might point to the same node, and that node is
conceptually owned by all of the edges that point to it.

 To enable multiple ownership explicitly by using the Rust type `Rc<T>` (reference counting).

`Rc<T>` type keep track of the number of references to a value to deremine whether or not the value is still in use.
if there are zero references to a value, the value can be cleaned up without any references becoming invalid
 */

pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_example() {
    let a = Rc::new(RcList::Cons(5,
                                 Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    let b = RcList::Cons(3, Rc::clone(&a));
    let c = RcList::Cons(4, Rc::clone(&a));
    println!("Count of references to a is {}", Rc::strong_count(&a));
}