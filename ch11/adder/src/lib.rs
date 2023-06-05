pub fn add(left: usize, right: usize) -> usize {
    left + right
}
/* 
// basic test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */

/* 
// explore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
*/

/* 
// make a test fail on purpose
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail!");
    }
}
*/

/* 
//assert macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self,other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,    
        };
        let smaller = Rectangle {
            width: 5,
            height:1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn larger_cannot_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,    
        };
        let smaller = Rectangle {
            width: 5,
            height:1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
*/

/* 
//assert_eq gives some more context
pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
*/

/* 
//custom error message
pub fn greeting(name: &str) -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
    }
}
*/
/* 
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be between 1 and 100, got {}.", 
                value
            );
        } else if value > 100 {
            "Guess value must be less than or equal to 100, got {}.",
            panic!(
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
*/

// using Result<T,E>
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}


