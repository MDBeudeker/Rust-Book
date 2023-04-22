
//basic function
/*
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function(){
    println!("another function!");
}
*/

/*
// function with Parameter
fn main(){
    another_function(5, 'h');
}

fn another_function(x : i32, unit_label: char){
    println!("The measurement is {}{}", x, unit_label);
}
*/

// expression within a statement
/*
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
*/

// function with return value

/*
fn main(){
    let x = five();
    println!("the value of x is {}", x)
}

fn five() -> i32{
    5 // the value 5 as just the return value is a valid function
}
*/


// function with input and return value

fn main(){
    let x = plus_one(5);
    println!("the value of x is {}", x)
}

fn plus_one(x : i32) -> i32 {
    x + 1                       // this is an expression, if we would add a semicolon to this line it would becomes a statement
}

