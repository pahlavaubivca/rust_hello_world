use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct Node<T> {
    prev: RefCell<Option<Rc<Node<T>>>>,
    value: RefCell<T>,
    next: RefCell<Option<Rc<Node<T>>>>,
}


impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            prev: RefCell::new(None),
            value: RefCell::new(value),
            next: RefCell::new(None),
        }
    }


}

pub fn ll_proto_1() {
    let first = Rc::new(Node::new("first"));
    println!("first count {}", Rc::strong_count(&first));
    let second = Rc::new(
        Node {
            prev: RefCell::new(Some(Rc::clone(&first))),
            value: RefCell::new("second"),
            next: RefCell::new(None),
        }
    );
    let third = Rc::new(
        Node {
            prev: RefCell::new(Some(Rc::clone(&second))),
            value: RefCell::new("third"),
            next: RefCell::new(None),
        }
    );
    {
        {
            // *second.next.borrow_mut() = Some(Rc::clone(&third));
            *second.next.borrow_mut() = Some(third.clone());
            println!("asdfd")
        }
        let aasdf = Rc::clone(second.prev.borrow().as_ref().unwrap());
        println!("get prev.value from second {:?}", aasdf.value);
        println!("get next.value from second {:?}", second.next.borrow().as_ref().unwrap().value);
        println!("first count {}", Rc::strong_count(&first))
    }
    println!("end of program");


    // let asd =  Rc::clone(&second);
    // *first.next.borrow_mut() = asd;
}
//
// fn get_rand_num_node() -> Node<i32> {
//     let num = rand::thread_rng().gen_range(0..10);
//     Node::Value(num)
// }


pub struct NodeV2<T> {
    prev: Option<Rc<RefCell<NodeV2<T>>>>,
    value: T,
    next: Option<Rc<RefCell<NodeV2<T>>>>,
}

impl<T> NodeV2<T> {
    pub fn new(value: T) -> Self {
        Self {
            prev: None,
            value,
            next: None,
        }
    }

    pub fn set_next(&mut self, node: NodeV2<T>) {
        let asd = Rc::new(RefCell::new(node));
        self.next = Some(asd);
    }
    pub fn get_value(&self)->&T{
        &self.value
    }
    // pub fn set_prev(&mut self, node: NodeV2<T>){
    //     let asd = Rc::new(RefCell::new(node));
    //     self.prev = Some(asd);
    // }
}

pub fn ll_proto_1_1() {
    let first = Rc::new(RefCell::new(NodeV2::new("first")));
    let second = Rc::new(RefCell::new(NodeV2 {
        prev: Some(Rc::clone(&first)),
        value: "second",
        next: None,
    }));
    let third = NodeV2 {
        prev: Some(Rc::clone(&second)),
        value: "third",
        next: None,
    };
    second.borrow_mut().set_next(third);
    println!("second.prev.value {}", second.borrow().prev.as_ref().unwrap().borrow().get_value());
    println!("second.next.value {}", second.borrow().next.as_ref().unwrap().borrow().get_value());
}

