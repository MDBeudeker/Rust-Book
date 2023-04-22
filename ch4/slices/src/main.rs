/*
// inefficient way to obtain the location of a space in a string
fn main() {
    let s = String::from("Het is feest");
    let a = first_word(&s);
    println!("{a}");
}

fn first_word(s : &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i
            }
    }
    s.len()
}
*/

/* 
// slicing example
fn main(){
    let s = String::from("Hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}{world}");
}

*/

/*
// slicing 
fn main() {
    let s = String::from("Het is feest");
    let a = first_word(&s);
    println!("{a}");
}

fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
    }
    &s[..]
}
*/

// even more better slicing
// slicing 
fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string)
    
    let my_string_literal = "hello_world";
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    

    let word = first_word(my_string_literal);
}

fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
    }
    &s[..]
}