/* 
// use a function defined in a module
mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

// moving the scope of a function

/*
mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


mod customer {
    use crate::front_of_house::hosting; // move the use statement here, else the customer will not understand
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
*/

/* 
// importing the entire function - this is not the way to go
// defining the function's parent module first shows that this is a foreign function
mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
*/


/* 
// This is not the case with enums, structs and other items however
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
*/

/*
// This however will confuse the rust

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/

/* 
// This can be mitigates using the As keyword

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
*/

/* 
// re-exporting a crate

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

/*
// nested paths
// instead of this:
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// Do this:
// --snip--
use std::{cmp::Ordering,io};
// --snip--

//and you also can combine a module and an argument in module by turning this
use std::io;
use std::io::Write;
//into this
use std::io;
use std::io{self,Write};

*/

/*
//or use glob operator (do not recommend)
use std::collections::*;
*/