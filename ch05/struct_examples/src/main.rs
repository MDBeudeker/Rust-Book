/*
// calculating the surface area of a square without structs

fn main() {
    let width1 = 30;
    let height1 = 40;

    println! ("the area of the square is {} pixels", area(width1, height1));
}

fn area (w : u32, h: u32 ) -> u32{
    w * h
}
*/


/*
// calculating the surface area of a square with a tuple struct

fn main() {
    let rectangle1 = (30,50); // now it is one object but now it's unclear what each value means


    println!("the area of the square is {} pixels", area(rectangle1));
}


fn area (dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}
*/

/*
// calculating the surface area of a square with a struct

fn main() {
    let rectangle1 = Rectangle{
        width: 30,
        height: 40,
    };
    println!("Rect1 is {:?}", rectangle1);
    println!("Rect1 is {:#?}", rectangle1);

    println! ("the area of the square is {} pixels", area(&rectangle1));
}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}

fn area (rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
*/

// debugging the width

fn main() {
    let scale = 2;
    let rectangle1 = Rectangle{
        width: dbg!(30* scale),
        height: 40,
    };

    dbg!(&rectangle1);
}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}
