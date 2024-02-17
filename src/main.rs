use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use crate::borrow::borrow_checker;
use crate::ownership::example_ownership;
use crate::references_with_lifetime::rwl;
use crate::pointers::r#box::List::{Cons, Nil};
use crate::pointers::r#rc::RcList::{Cons as RcCons, Nil as RcNil};
use crate::training::linked_list::prototype_1::{ll_proto_1, ll_proto_1_1};
use crate::training::linked_list::prototype_2::ll_proto_2;


mod basic_variable_and_types;
mod ownership;
mod borrow;
mod slice;
mod r#struct;
// mod linked_list;
mod generic;
mod r#trait;

mod references_with_lifetime;
mod pointers;
mod training;

// mod pointers#box;
// fn main() {
//     println!("Hello, world!");
//     let mut asd = 89;
//     asd = 4;
//     const SOME_VARIABLE: i32 = 8;
//     let asdd = '😶';
//     println!("{}", asdd);
//     // borrow_checker();
//     // example_ownership();
//     // example_of_shadowing()
//
//     let str1 = "asdf";
//     let str2 = "aasdf";
//     let shorter = rwl(&str1, &str2);
//     println!("Shorter str is {shorter}")
// }
//

fn main() {
    // ll_proto_1();
    // ll_proto_1_1();
    // ll_proto_2();
    let fifo_path = "/tmp/xterm_fifo";

    // Create a FIFO (named pipe)
    Command::new("mkfifo")
        .arg(fifo_path)
        .status()
        .unwrap();
    let xterm_cmd = format!("bash -c 'while true; do read -r cmd < {}; $cmd; done'", fifo_path);
    Command::new("xterm")
        .args(&["-e", &xterm_cmd])
        .spawn()
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    // Open the FIFO for writing
    let mut fifo = OpenOptions::new()
        .write(true)
        .open(fifo_path)
        .unwrap();

    // Write commands to the FIFO
    writeln!(fifo, "echo Hello, World!").unwrap();
    writeln!(fifo, "ls -la").unwrap();
    // Add more commands as needed
    let mut index = 0;
    loop {
        writeln!(fifo, "clear").unwrap();
        thread::sleep(Duration::from_millis(500));
        writeln!(fifo, "echo {};",index).unwrap();
        index+=1;
        thread::sleep(Duration::from_secs(1));
    }

    // Optional: Keep the Rust program running for a bit
    thread::sleep(Duration::from_secs(5));


    // let commands = "echo Hello, World!; ls -la; exec bash";
    // let mut xterm = Command::new("xterm")
    //     .arg("-geometry")
    //     .arg("110x40")
    //     // .args(&["-e", "bash", "-c",commands])
    //     // .args(&["-e","bash"])
    //     .stdin(Stdio::piped())
    //     .spawn()
    //     .unwrap();
    // thread::sleep(Duration::from_secs(2));
    // if let Some(ref mut stdin) = xterm.stdin {
    //     // Write commands to xterm
    //     println!("try to write into xterm...");
    //     stdin.write_all("-e bash -c echo Hello, World!; exec bash".as_bytes()).unwrap();
    //     // writeln!(stdin, "echo Hello, World!").unwrap();//.expect("Failed to write to stdin");
    //     // writeln!(stdin, "ls -la").expect("Failed to write to stdin");
    //     // ... You can add more commands here
    // }
    // // match xterm.stdin {
    // //     Some(ref mut g)=>{
    // //         g.write("echo Hello, World!!!!".as_bytes());
    // //         writeln!(g, "echo Hello, World!").expect("Failed to write to stdin");
    // //         writeln!(g, "ls -la").expect("Failed to write to stdin");
    // //         // g.write("echo \"asdf\"".as_bytes());
    // //     }
    // //     None =>{
    // //         println!("gnome terminal stdin result is none");
    // //     }
    // // }
    // thread::sleep(Duration::from_secs(5));
    println!("just test message");
    // loop{
    //     match xterm.stdin {
    //         Some(mut g)=>{
    //             writeln!(g, "echo \"Hello, World!\"").expect("Failed to write to stdin");
    //             // g.write("echo \"asdf\"".as_bytes());
    //         }
    //         None =>{
    //             println!("gnome terminal stdin result is none");
    //         }
    //     }
    //     thread::spawn(Duration::from_secs(1));
    // }
     // println!("gnome terminal id {:?}", gnome_terminal.id());
}

/*
0x00007ffd68f91400
*/