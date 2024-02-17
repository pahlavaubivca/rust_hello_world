pub fn borrow_checker() {
    let mut a = String::from("asd");
    let b = &a; //borrow
    println!("b is {:?}", b);
    let c = &a; //borrow
    println!("c is {:?}", c);
    let d = &mut a; //borrow mut
    println!("d is {:?}", d);
    println!("a is {:?}", a);
    // println!("b is {:?}", b); // error because b was borrowed as inmutable and a was borrowed as mutable
}
