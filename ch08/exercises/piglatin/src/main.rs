// For giggles I asked chatgpt to optimize my code to see if i would learn something:
// this is the optimized code , unoptimized code below, and the real ChatGPT code is below that again, funny to see that one does not work at all


fn main() {
    
    let words = vec!{"worst", "apple", "cheese", "battle", "rope", "snap", "eagle", "strong", "yacht"}; // change the list of words into a vector containing words
    for word in words{ // iterate over all the words instead of calling the function separately each time
        println!("{} - in pig latin - {}", word, pig_latin(word.to_string()));
    }
}

fn pig_latin(word: String) -> String{
    
    let mut counter = 0;
    let mut s = word.clone();
    let mut checker = 0;
    for c in word.chars() {


        if c == 'a' || c == 'e' || c == 'i' || c == 'u' || c == 'o'{
            checker = 1;
            if counter == 0{
                s.push('h');
            }
        }
        else if c == 'y' {
            let (_,t) = s.split_at(1);
            s = t.to_string();
            s.push('h');
    }
        else if checker == 0 {
                let (_,t) = s.split_at(1);
                s = t.to_string();
                s.push(c);
        }

        counter +=1

    }
    s.push('a');
    s.push('y');
    s
}


// this is the unoptimized code
/* 
fn main() {
    
    pig_latin("worst".to_string());
    pig_latin("apple".to_string());
    pig_latin("cheese".to_string());
    pig_latin("battle".to_string());
    pig_latin("rope".to_string()); // a = 97, y = 121
    pig_latin("snap".to_string()); // 97, 101, 117, 105, 111
    pig_latin("eagle".to_string());  // 121
    pig_latin("strong".to_string());  // 121
    pig_latin("yacht".to_string());  // 121
    
}

fn pig_latin(word: String) -> String{
    
    let mut counter = 0;
    let mut s = word.clone();
    let mut checker = 0;
    for c in word.chars() {


        if c == 'a' || c == 'e' || c == 'i' || c == 'u' || c == 'o'{
            checker = 1;
            if counter == 0{
                s.push('y');
            }
        }
        else if c == 'y' {
            let (_,t) = s.split_at(1);
            s = t.to_string();
            s.push('y');
    }
        else if checker == 0 {
                let (_,t) = s.split_at(1);
                s = t.to_string();
                s.push(c);
        }

        counter +=1

    }
    s.push('a');
    s.push('y');
    println!("{} - in pig latin - {}", word, s);
    s
}
 */
/* 
 fn main() {
    let words = vec!["worst", "apple", "cheese", "battle", "rope", "snap", "eagle", "strong", "yacht"];
    for word in words {
        println!("{} - in pig latin - {}", word, pig_latin(word));
    }
}

fn pig_latin(word: &str) -> String {
    let mut s = String::with_capacity(word.len() + 2);
    let mut chars = word.chars().peekable();

    match chars.peek() {
        Some(c @ 'a' | c @ 'e' | c @ 'i' | c @ 'o' | c @ 'u') => {
            s.push(*c);
            chars.next();
        }
        Some(&'y') => {
            s.push('y');
            chars.next();
        }
        _ => {}
    }

    while let Some(c) = chars.next() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                s.push(c);
                while let Some(&'y') = chars.peek() {
                    s.push(chars.next().unwrap());
                }
                break;
            }
            'y' => {
                s.push(c);
                while let Some(&'y') = chars.peek() {
                    s.push(chars.next().unwrap());
                }
                break;
            }
            _ => s.push(c),
        }
    }

    s.push_str("ay");
    s
}
 */
