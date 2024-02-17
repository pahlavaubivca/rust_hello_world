use std::fmt::Display;

/**
 A trait defines functionality a particular type has and can share with other types.
Trait somehow similar to interface from other languages.
 */
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_with_implementation(&self) -> String {
        String::from("lol...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/**
Traits as parameters.

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the
trait name. This parameter accepts any type that implements the specified trait
 */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

/**
 Trait Bound Syntax.

This form is equivalent to the example in the previous section but is more verbose.
 */
pub fn notify_bound_syntax<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_multiple_trait(item: &(impl Summary + Display)) {}

pub fn notify_multiple_trait_generic<T: Summary + Display>(item: &T) {}

/// alternative to example above
pub fn notify_mtg_alternative<T>(item: &T)
    where
        T: Summary + Display,
{}

///return types that implement trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user name"),
        content: String::from("some content"),
        reply: false,
        retweet: false,
    }
}


/// to use in one collection structs with implementation one trait
/// we need wrap it with Box<dyn>
struct S1 {
    prop: i32,
}

struct S2 {
    prop: i32,
}

impl Summary for S1 {
    fn summarize(&self) -> String {
        todo!()
    }
}

impl Summary for S2 {
    fn summarize(&self) -> String {
        todo!()
    }
}

fn S1S2Using() -> Box<dyn Summary> {
    let s1: S1 = S1 { prop: 2 };
    let s2: S2 = S2 { prop: 3 };
    let asd: Vec<Box<dyn Summary>> = vec![Box::new(s1), Box::new(s2)];

    for s in asd.iter() {
        println!("{}", s.summarize())
    }
    todo!();
    // if true {
    //     return &asd[0];
    // } else {
    //     return &asd[1];
    // }
}

/// Using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd`
/// trait that enables comparison and the `Display` trait.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x ={}", self.x);
        } else {
            println!("The largest member is y ={}", self.y);
        }
    }
}