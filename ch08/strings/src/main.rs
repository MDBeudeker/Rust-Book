/*
// initiating a string is similar to initiating a vector
fn main() {
    //let mut s = String::new();

    let data = "initial contents"; // String Literal
    let s = data.to_string();      // String literal can be converted to string, since it has the Display trait

    let s = "initial contents".to_string(); // also works directly

    let s = String::from("initial contents"); // equivalent to the examples above, still converting a string literal to a string
}
*/


/*
// Appending to a string with push_str and push
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");   //push_str takes a string slice
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}"); //note that push_str does not take ownership of s2

    let mut s = String::from("lo");
    s.push('l');   // push can take 1 char, not multiple characters
    println!("{s}");
    
}
*/

/*
// Concatenating using + or format!

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;  // we have to borrow s2, since the signature of the add function is: add(self, s: &str) -> String
                        // note that the 'add' function can coerce a String into a &str using deref coerscion [ch15]
                        // it looks like the add function is making a lot of copies, but actually it is taking the value of s1, changing it and returning it as s3
    println!("{s3}");
    
}
*/

/*
// Concatenating multiple strings quickly gets unwieldy
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    //let s = s1 + "-" + &s2 + "-" + &s3;     // it is difficult to see what is going on here
    //println!("{s}");
    
    // Better use the format! macro

    let s = format!("{s1}-{s2}-{s3}");   // similar to println, however this function returns a string instead of displaying text on the screen
    println!("{s}");
}
*/

/*
// Indexing strings
fn main() {
    let s1 = String::from("Hello");
    let hello = "Здравствуйте";
    // let h = s1[0];  // this breaks the code
    // rust just does not want to support indexing from Strings,since some text have utf-8 encoding, where each letter could have 2 bytes

    // Rather than indexing using [], you can use [] with a range to create a string slice containing particular bytes

    let s = &s1[0..4];  // prints hell
    let ss = &hello[0..4];  // prints Зд  (since each character has 2 bytes) // hello[0..3] will crash the program (3 is not a char boundary)
    println!("{s}");
    println!("{ss}");
    // use string slices with indexes with caution, this can crash your program
}
*/

//Methods for iterating over Strings

fn main(){

    // the best way to iterate ofer strings is using the chars method
    for c in "Зд".chars() {
        println!("{c}");
    }

    // or, alternatively over bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // You can also use grapheme clusters from strings but this is complex, and not included in std lib

}

