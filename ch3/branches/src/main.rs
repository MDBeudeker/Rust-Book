/*
fn main() {
    let number = 7;

    // we have 2 expressions within the IF statement, these can also be called arms
    if number < 5 {
        println!("Condition was true");
    } else{
        println!("Condition was false");
    }
}
*/

/*
// else if statements
fn main() {
    let number = 6;

    // we have 2 expressions within the IF statement, these can also be called arms
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number %3 == 0 {
        println!("Number is divisible by 3");
    } else if number %2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    } 
    // note; using too many else if statements can clutter your code, consider using matches (ch6)
}
*/

/*
// if in a let statement
fn main(){
    let condition = true;
    let number = if condition { 5 } else { 6 } ; // if condition is true, make number 5, if false, make number 6

    println!("The value of the number is {}", number);
}
*/

/*
//  looping time :D !
fn main(){
    let mut counter = 0;
    loop{
        println!("again!");
        counter = counter+1;
        if counter == 100{break}
    }
}

*/

/*
// Returning values from loops

fn main(){
    let mut counter = 0;
    let result = loop{
        counter = counter+1;

        if counter == 10 {break counter * 2}
    };
    println!("the result = {}",result);
}
*/

/*
// Loop Labeling

fn main(){
    let mut count = 0;
    'counting_up: loop{
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1; // this will execute only once because remaining == 9 will break off the when remaining equals 9
        }
        count += 1;
    }
    println!("End count = {count}")
}
*/

/*
// while loop
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}",number);

        number -= 1;
    }
    println!("liftoff!");
}
*/

/*
// loop through a collection of len 5

fn main(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
}

*/

/*
// for loop: through an array

fn main(){
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is {}", element);
    }
}

*/

// reverse for loop

fn main(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("liftoff!!!!");
}
