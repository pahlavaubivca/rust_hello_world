pub mod rc;
pub mod r#box;
mod defer_trait;
mod drop_trait;
mod refcell;
mod memory_leak;

/**
 Recap ot the reasons to choose `Box<T>`, `Rc<T>` or `RefCell<T>`
- `Rc<T>` enables mutliple owners of the same data; `Box<T>` and `RefCell<T>` have single owners
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows
checked at complile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the
`RefCell<T>` is immutable
 */

fn e() {}