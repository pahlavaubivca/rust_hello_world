use std::ops::Deref;

/// custom Box implementation
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/**
To be able dereference value we need implement deref trait

The `deref` method gives the compiler the ability ti take a value of any type that implements
`Deref` and call the `deref` method to get a & reference that it knows how to dereference

When we entered `*y` behind the scenes Rust actually ran `*(y.deref())`
 */
impl<T> Deref for MyBox<T> {
    //defines an associated type for the `Deref` trait to use
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // deref returns a reference to the value we want to access with the * operator.
        // self.0 and .0 accesses to first value in a tuple struct.
        &self.0
    }
}

/**
 Deref Coercion - converts a reference to a type that implements the `Defer` trait into a reference to
another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the
`Defer` trait such that it returns `&str`.
 */
fn hello(name: &str) {
    println!("hello {name}");
}

/**
 Here we;re calling to `hello` function with the argument `&m`, which
is a reference to a `MyBox<String>` value. Because implemented `Deref` trair on `MyBox<T>`,
Rust can turn `&MyBox<String>` into `&String` by calling `deref`. The standart library provides an
implementation of `Deref` on `String` that returns a string slice. Rust calls `deref` again to turn the `&String` into `&str`
 */
fn deref_coercion_example() {
    let m = MyBox::new(String::from("some str"));
    hello(&m);
}

/**
 If Rust didn't implement deref coercion, code would be like below.
The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and `[..]`
take a string slice of the `String` that is equal to the whole string to match the signature of `hello`


*!!! For mut reference use `DerefMut`*
 */
fn no_deref_coercion_example() {
    let m = MyBox::new(String::from("some string"));
    hello(&(*m)[..])
}
