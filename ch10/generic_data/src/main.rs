/* 
fn largest_i32(list: &[i32]) -> &i32{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }    
    largest
}

fn largest_char(list: &[char]) -> &char{ //these functions are almost the same, so let's combine them into 1 function
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

 */


 /*
 fn largest<T>(list: &[T]) -> &T{ //change <i32> to <T> (any type)
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/

/* 
struct Point<T> {
    x: T,
    y: T,
}

fn main () {
    let integer = Point { x: 5, y:10 };
    let float =  Point { x: 1.0, y: 4.0 };
}
 */

/*
struct Point<T> {
    x: T,
    y: T,
}

fn main () {
    let wont_work = Point { x: 5, y:4.5 };
}
*/

/* 
struct Point<T,U> {
    x: T,
    y: U,
}

fn main () {
    let integer = Point { x: 5, y:10 };
    let float = Point { x: 6.8, y:3.4 };
    let integer_and_float = Point { x: 6, y:3.4 };
}
*/
/* 
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}


fn main() {
    let p = Point { x:5, y: 10};

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
}

 */

/* 
struct Point<T>{
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}


fn main() {
    let p = Point { x:5.1, y: 10.8};

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
    println!("p.distance_from_origin = {}", p.distance_from_origin());
}
*/


struct Point <X1,Y1>{
    x : X1,
    y : Y1,
}

impl <X1, Y1> Point<X1,Y1> {
    fn mixup<X2,Y2>(self, other: Point<X2,Y2>) -> Point<X1,Y2>{
        Point {
            x: self.x,
            y: other.y,
        }
    }

}

fn main() {
    let p1 = Point { x:5, y: 10.4};
    let p2 = Point { x:"Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
