[LEARN DOCS](https://doc.rust-lang.org/book/ch00-00-introduction.html)

## Task

1) implement tree
2) linked list
3) recursion
4) async

## Ownership

For simple data which stored in stack rust make copy automatic because size is know on compile time

```
let s1 = 4
let s2 = s1
```

s1 and s2 stored in different stack, no need store that value in heap

## Reference

```
let asd = 4;
let qwe = &asd;// qwe it is reference to asd, qwe not own the value of asd
```

### Mutable reference

```
let mut asd = 4;
let qwe = &mut asd;
```

***Mutable reference restriction***: if you have a mutable reference to a value, you can have
no other references to that value. This code that attempts to create two mutable
references to variable will fail.

```
 let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

also combining mutable and immutable reference cause error

### Summary about rules of references

1) at any given time, you can have either one mutable reference or any number of immutable references.
2) references must always be valid.



### Section what is done

1) data types - in general simple datatypes which stores in stack (i16,i32,str, etc...) and complext which most of the time stores in heap 
2) ownership - by default all data have owner - variable, on = clause we change the owner with futher consequences
3) smart pointers
- Box<T> - single owner, allow immutable or mutable borrows(checked at compile time)
- Rc<T> - multiple owner to the same data, immutable, at compile time
- RefCell<T> - single owner, allow mutate immutable data, at runtime time

## TODO
1) [modules, package and crates](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
2) [common collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
3) [error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
4) [io](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
5) [closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
6) [cargo](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
7) [concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
8) [oop](https://doc.rust-lang.org/book/ch17-00-oop.html)
9) [patterns and matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
10) [multithread](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)