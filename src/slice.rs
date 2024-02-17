/**
Slices is a kind of reference, so it doesn't have ownership.
 */


pub fn slice() {}

fn first_word_index_of_end(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

///String slices. example for "hello world"
fn string_slices(s: &String) -> &str {
    // let hello = &s[..5];
    // let world = &s[6..11];
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}