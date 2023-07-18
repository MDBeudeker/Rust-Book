use std::io;

fn main() {

    /*
    // exercise overflow
    let a : u8 = 20;
    println!("the value of a is {a}");
    println!("the value of a+ 10000 is {}", a+235);
    println!("the value of a+ 10000 is {}", a+236);
    */

    // floats
    /*
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    */

    // tuples
    /*
    let tup = (500, 6.4, 1);

    let (x,y,z) = tup;

    println!("The value of y is {}",y);
    */

    /* 
    // accessing specific tuple elements
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_for = x.1;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_for, one);
    */

    // arrays
    /*

    let a = [1,2,3,4,5]
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // adding the number of elements explicitly in an array
    let a: [i32; 5] // initialize an empty array of size 5

    //accessing array elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    */ 
    // excercise arrays

    /*
    let a = [1,2,3,4,5];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered wat not a number");

    let element = a[index];

    println!("The value of element at index {} is {}", element, index);

    */


    
    
    // Vectors 
    // Similar to arrays,only they can grow in size
    /*

    let a = [1,2,3,4,5]

    */
}
