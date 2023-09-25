/* 
// Some is a refutable pattern so this will not work
fn main() {
    let mut a = Vec::new();

    a.push(1);
    let some_option_value = Some(a);

    let Some(x) = some_option_value;
}
 */

//fix this as follows

// Some is a refutable pattern so this will not work
fn main() {
    let mut a = Vec::new();

    a.push(1);
    let some_option_value = Some(&a);

    if let Some(x) = some_option_value {
        println!("{:?}", x);
    }
    a.pop();
    let some_option_value = Some(&a);
    if let Some(x) = some_option_value {
        println!("{:?}", x);
    }
}