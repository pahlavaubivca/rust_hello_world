use std::fmt::Display;

/**
 * This is a function that shows how to use shadowing
 * shadowing allow change value and type of a variable
 * shadowing is not the same as mut because in shadowing you create new variable in heap
 */
pub fn example_of_shadowing() {
    let asd = 89;
    let asd = 4;
    {
        let asd = asd.to_string();
    }
}

/// This is a function that shows how to use mut
/// can't change the type of a variable
/// can't change the value of a variable
pub fn example_of_mut() {
    let mut asd = 89;
    asd = 4;
}

pub fn example_of_converting_from_string() {
    let str_number = String::from("12");
    let num: i32 = str_number.parse().expect("");
}

pub fn example_of_assigning() {
    let asd = {
        let i = 0;
        i + 1
    };

    let qwe = if asd == 1 { -1 } else { 0 };

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
}

pub fn example_of_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn loop_through_collection() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // or for
    for element in a {
        println!("the value is :{element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}