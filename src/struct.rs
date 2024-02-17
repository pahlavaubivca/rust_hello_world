#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn work_with_struct() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
    let asd = vec![1,2,3];
}