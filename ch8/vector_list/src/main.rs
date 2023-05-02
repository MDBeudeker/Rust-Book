
/*
// Create a new vector

fn main() {
    let v: Vec<i32> = Vec::new();
}
*/

/*
// Vector w initial values

fn main() {
    let v = vec![1,2,3];
}
*/

/*
// Update a vector
fn main() {
   let mut v = Vec::new();
   v.push(5);
   v.push(6);
   v.push(7);
   v.push(8);
}
*/

/*
// Reading vector elements
fn main() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("{}",third);

    let third: Option<&i32> = v.get(2);
    match third{
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    //let does_not_exist = &v[100];  // this makes the code panice
    let does_not_exist = v.get(100); // the output of get() is an option, returning some/none
}
*/

/*
// Borrowing
fn main() {
    let v = vec![1,2,3,4,5];
    let first = &v[2];
    v.push(6);

    println!("The first element is: {first}"); // no can do since v is immutable
    
}
*/

/*
// iterate over a vector
fn main() {
    let v = vec![100, 32, 57];
    for i in &v{
    println!("{i}");
    }
}
*/

/*
// iterate and multiply over a vector
fn main() {
    let v = vec![100, 32, 57];
    for i in &v{
    *i +=50;
    }
}
*/


//Vectors can only be used to store 1 type
// However, vectors can inherently also store multiple enums which in themselves have multiple types
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

