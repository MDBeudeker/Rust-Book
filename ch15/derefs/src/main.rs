/* //referencing using * and &
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}
 */

/* 
// using a box to do a similar thing
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}
*/

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let m = MyBox::new(String::from("Rust"));

    hello(&m);

    //// deref coercion makes that we did not have to write the following:
    //let m = MyBox::new(String::from("Rust"));
    //hello(&(*m)[..]);

}

fn hello(name: &str) {
    println!("Hello, {name}");
}