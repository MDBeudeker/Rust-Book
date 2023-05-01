/* 
// defining enums
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

enum IpAddrKind{
    V4,
    V6,
}
*/

/*
// adding data to enums
fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}

enum IpAddrKind{
    V4(String),
    V6(String),
}
*/

/*
// restricting enums input
fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

*/

//some
fn main() {

    let some_number = Some(5);      // type is automatically Option<i32>
    let some_char = Some('e');      // type is automatically Option<char>
    let absent_number: Option<i32> = None;  //Explicitly set to none

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x+y;                  //Cannot add i8 and Option<i8> because they're different types

}

