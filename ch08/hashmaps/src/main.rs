
/*
// hashmaps, just like vectors, store their data on the Heap, all of the keys must have the same type
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // get a specific value from a hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    // Iterating over the values in a hashmap

    for (key, value) in &scores {
        println!("{key}: {value}")
    }
}
*/

/*
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // now we are not able to use field_name and field_value, since the hash map now owns the strings (unlike the previous example, which used i32s)
}
*/

/*
//overwrite a hashmap value
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // this will print Blue: 25
}
*/

/*
// inserting a hashmap value if it doesn't exist
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("Yellow")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores)
}
*/

// count words

fn main(){
    use std::collections::HashMap;

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}