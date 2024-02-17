use std::cell::{Ref, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::training::linked_list::prototype_1::Node;

pub struct NodeV2<T> {
    prev: Option<Rc<RefCell<NodeV2<T>>>>,
    value: T,
    next: Option<Rc<RefCell<NodeV2<T>>>>,
}

pub struct LinkedList<T> {
    current: Option<Rc<RefCell<NodeV2<T>>>>,
    last: Option<Rc<RefCell<NodeV2<T>>>>,
    len: usize,
    current_index: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            current: None,
            last: None,
            len: 0,
            current_index: 0,
        }
    }
    pub fn add(&mut self, value: T) {
        let _node = NodeV2 {
            prev: None,
            value,
            next: None,
        };
        let _rc_node = Rc::new(RefCell::new(_node));
        if self.current.is_none() {
            self.current = Some(_rc_node.clone());
        }

        if let Some(_last) = &self.last {
            _last.borrow_mut().next = Some(_rc_node.clone());
            _rc_node.borrow_mut().prev = Some(_last.clone())
        }
        self.len += 1;
        self.last = Some(_rc_node.clone())
    }

    pub fn move_next(&mut self) -> Option<Ref<NodeV2<T>>> {
        if self.current.is_none() ||
            self.current.as_ref().unwrap().borrow().next.is_none() {
            return None;
        }
        let next = Some(self.current.as_ref().unwrap().borrow().next.as_ref().unwrap().clone());
        self.current = next;
        self.current.as_ref().map(|node_rc| node_rc.borrow())
    }

    // //todo -> Option<&T> (hint: use lifetime)
    // pub fn move_next(&mut self) -> Option<Rc<RefCell<NodeV2<T>>>> {
    //     // pub fn move_next(&mut self) -> Option<&T> {
    //     if self.current.is_none() ||
    //         self.current.as_ref().unwrap().borrow().next.is_none() {
    //         return None;
    //     }
    //
    //     let next = Some(self.current.as_ref().unwrap().borrow().next.as_ref().unwrap().clone());
    //     self.current = next;
    //     Some(self.current.as_ref().unwrap().clone())
    // }


    pub fn move_prev() -> Option<Rc<RefCell<NodeV2<T>>>> {
        None
    }

    pub fn remove(&mut self, index: usize) {}
}

pub fn ll_proto_2() {
    let first = String::from("first");
    let second = String::from("second");
    let mut ll = LinkedList::new();
    ll.add(first);
    ll.add(second);
    let res = ll.move_next();
    println!("next is {}", res.unwrap().value)
}