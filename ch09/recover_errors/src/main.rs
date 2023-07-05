/* 
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("Hello.txt"); // get a result whether the file exists or not

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },

        }
    };
} */

// The same ode using closures
/* 
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
 */

// Unwrap
/* 
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
*/

/* 
// using expect
use std::fs::File;

fn main(){
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in in this project.\n");
}
*/

/* 
// Propagating errors in a function

use std::fs::File;
use std::io::{self, Read};

fn main(){
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result{
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
*/
/* 
// the ? operator

use std::fs::File;
use std::io::{self, Read};

fn main(){
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
*/

/* 
// or more concise:
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?..read_to_string(&mut username)?;
    Ok(username)
}
*/

/*
// Even more concise:

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
*/

/* use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
*/

// Letting fn Main return an error
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}