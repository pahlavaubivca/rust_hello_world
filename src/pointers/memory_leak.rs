use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum MmList {
    Cons(i32, RefCell<Rc<MmList>>),
    Nil,
}

impl MmList {
    fn tail(&self) -> Option<&RefCell<Rc<MmList>>> {
        match self {
            MmList::Cons(_, item) => Some(item),
            MmList::Nil => None,
        }
    }
}

fn mem_leak() {
    let a = Rc::new(
        MmList::Cons(5, RefCell::new(
            Rc::new(MmList::Nil)
        ),
        )
    );
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(
        MmList::Cons(10, RefCell::new(
            Rc::clone(&a)
        ),
        )
    );

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(_link) = a.tail() {
        *_link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    //this will lead to memory leak
    println!("a next item  = {:?}", a.tail());
}