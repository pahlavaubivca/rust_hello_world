use std::fmt::Display;

/**
Rules:
1) each value in Rust has an owner;
2) there can only be one owner at a time
3) when the owner goes out of scope the value will be dropped

About "stack" and "heap".
In stack can be placed data know size at compile type
For example str literal {let a = "some"} have know length, immutably so can be stored in stack,
but for complex String type we don't know size of data so we will store in heap

Work with stack faster than with heap
 */
pub fn example_ownership() {
    let a = String::from("asd");
    let b = a.clone();
    println!("{:?}", a);//error because a is moved to b
}

pub fn example_ownership_clone() {
    let a = Some {
        some_string: String::from("asd")
    };
    
    let b = a.clone();
    println!("{:?}", a.some_string);
}


#[derive(Debug, Clone)]
struct Some {
    pub some_string: String,
}

impl Some {
    pub fn display(&self) {
        println!("asdf");
        // return $self
    }
}

// impl Clone for Some {
//     fn clone(&self) -> Self {
//         return Self {
//             some_string: self.some_string.clone()
//         };
//     }
// }

impl Display for Some {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.some_string)
    }
}

// impl Copy for Some {
// }

