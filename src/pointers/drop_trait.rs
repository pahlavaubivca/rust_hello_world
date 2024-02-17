/**
`Drop` allow customize what happens when a value is about to go out of scope.

The `Drop` trait requires you to implement one method name `drop` that take a
mutable reference to `self`.
 */

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}

fn drop_usage_example() {
    let c = CustomSmartPointer {
        data: String::from("some str 1"),
    };
    let d = CustomSmartPointer {
        data: String::from("some str 2"),
    };
    println!("CustomStartPointers created.");
}

/**
We can't call `drop` method manually like `instance.drop()`, so if you need forcefully free memory - use `std::mem::drop`
we can use it like `drop(instance)`
 */

fn drop_example() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    drop(c);
}