/*
// Matches 1
fn main() {
    let Test = Coin::Penny;
    println!("{}",value_in_cents(Test));
}

enum Coin { 
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

*/


/*

//Matches with State parameter in quarter
// Tests whether the value of an enum usstate in type Coin is from some state

fn main() {
    let Test = Coin::Quarter(UsState::Alabama);
    println!("{}",value_in_cents(Test));
}

enum Coin { 
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        }
    }
}
#[derive(Debug)] // to inspect the state
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    MontanaNebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}
*/


/*
// Write a function that uses the enum Option and Match
// Note that matches are usually used within enums
fn main() {
 let five = Some(5);
 let six = plus_one(five);
 let none = plus_one(None);
}

fn plus_one(x: Option:<i32>) -> Option<i32> {
    match x {
        None => None,
        Some => Some(i + 1),
    }
    // You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.
}
*/


// catch_all match function

fn main(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),
        _ => reroll(), // execute the reroll function
        // _ => () // do nothing at all (tuple ())
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
//fn move_player(num_spaces: u8) {}
fn reroll() {}