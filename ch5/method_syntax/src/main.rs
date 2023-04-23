/* 
// adding a method to a struct
fn main() {
        let rectangle1 = Rectangle{
        width: 30,
        height: 40,
    };
    println!("the area of the rectangle is {} square pixels",rectangle1.area())

}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}
*/

/*
// Add a different method for width
fn main() {
    let rectangle1 = Rectangle{
        width: 30,
        height: 40,
    };

    if rectangle1.width(){      // note that Rust recognizes the fact that width is a method here, not the 'width' field of the struct
        println!("the rectangle has a nonzero width which is {}",rectangle1.width);
    }
}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}

impl Rectangle{
    fn width(&self) -> bool{
        self.width > 0
    }
}
*/

/*
// More parameters
fn main() {
    let rectangle1 = Rectangle{
        width: 30,
        height: 40,
    };

    let rectangle2 = Rectangle{
        width: 10,
        height: 20,
    };

    let rectangle3 = Rectangle{
        width: 60,
        height: 5,
    };

    println!("Cant rectangle1 hold rectangle2? {}",rectangle1.can_hold(&rectangle2));
    println!("Cant rectangle1 hold rectangle3? {}",rectangle1.can_hold(&rectangle3));

}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        
        (self.width > other.width) && (self.height > other.height)

    }
}
*/

/* 
// associated functions
fn main() {
    let sq = Rectangle::square(3);
    println!("{} {}",sq.width, sq.height);
}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}

impl Rectangle{
    fn square(size: u32) -> Self{
        Self{
            width: size,
            height: size
        }
    }
}
*/




