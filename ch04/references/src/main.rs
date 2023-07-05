/*

// borrowing a string
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we reference s1 to the function, but we also still keep it in the current scope

    println!("{s1} has length {len}");
}


fn calculate_length(s: &String) -> usize { //s is a reference to a string
    s.len()
} // here s goes out of scope, but because it does not have ownership of what it refers to, it is not dropped

*/

/*
// attempt to change a borrowed string -- this will not compile

fn main() {
    let s = String::from("hello");

    change(&s); 
    
}


fn change(some_string: &String) {
    some_string.push_str(", world")
}

*/

/*
// changing a borrowed string -- this will compile
fn main() {
    let mut s = String::from("hello");

    change(&mut s); 

}


fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

*/


/*
// lending a string to 2 variables

fn main(){
    let mut s = String::from("Hello");

    let r1 = &mut s;
    let r2 = &mut s; // you cant borrow a mutable var 2 times, sadly


    println!("{}{}",r1, r2);

}
*/

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s= String::from("hello");

    &s //this will crash since we are returning the borrow value, and the variable 's' is out of scope when this function ends
}

