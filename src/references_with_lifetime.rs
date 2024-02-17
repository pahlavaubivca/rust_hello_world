use std::fmt::Display;

/**
main aim of lifetimes is to prevent `dangling references`

When we specify lifetime parameters - we're not changing the lifetimes of any values
passed in or returned. We're specifying that the borrow checker should reject any values that don't
adhere to these constraints.

When annotating lifetimes in functions, the annotations go in the function signature,
not in the function body.

Ultimately, lifetime syntax is about connecting the lifetimes of various
parameters and return values of functions. Once they're connected - Rust has enough
information to allow memory-safe operations and disallow operations that would create
dangling pointers or otherwise violate memory safety.
 */
pub fn rwl<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() < str2.len() {
        return str1;
    }
    str2
}

// pub fn rwl2<'a, 'b>(str1: &'a str, str2: &'b str) -> str {
//     if str1.len() < str2.len() {
//         return str1;
//     }
//     str2
// }

pub fn rwlResult() {
    let str1 = String::from("long string");
    // let result;
    {
        let str2 = String::from("shrt str");
        // result = rwl(str1.as_str(), str2.as_str());//err: str2 borrowed value doesn't live long enough
    }
    // println!("The shorter string is {result}")
}

/// struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn asdf() {
    let novel = String::from("Call me Ishamael. Some years ago....");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {//hold ref to the first sentence of the String owned by `novel` variable.
        // the data in `novel` exists before the `ImportantExcerpt` instance is created.
        // also `novel` doesn't go out of scope until after the `ImportantExcerpt` goes out of scope, so the ref in the `ImportantExcerpt` instance is valid.
        part: first_sentence
    };
}

///'static' tels compiler that ref will live as long as program running
///
fn static_lifetime() {
    let s: &'static str = "static lifetime 🙄";
}

///generic type parameters, trait bounds, and lifetimes together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}