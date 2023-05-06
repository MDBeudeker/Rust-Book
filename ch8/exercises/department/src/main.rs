fn main() {

    use std::io;
    use std::collections::HashMap;

    let mut input = String::new();
    let mut company = HashMap::new();
    company.insert(String::from("Engineering"),Vec::<String>::new());
    company.insert(String::from("Sales"),Vec::<String>::new());
    company.insert(String::from("Support"),Vec::<String>::new());
    company.insert(String::from("Management"),Vec::<String>::new());
    company.insert(String::from("Cleaning"),Vec::<String>::new());


    if let Some(_engineering_vec) = company.get_mut("Engineering") {
        company.entry(String::from("Engineering"))
            .or_insert_with(Vec::new)
            .push(String::from("Vicky"));
    }

    if let Some(_engineering_vec) = company.get_mut("Engineering") {
        company.entry(String::from("Engineering"))
            .or_insert_with(Vec::new)
            .push(String::from("Arend"));
    }

    if let Some(_engineering_vec) = company.get_mut("Kaas") {
        company.entry(String::from("Kaas"))
        .or_insert_with(Vec::new)
        .push(String::from("Arend"));
    }



    println!{"{:?}",company};
    loop{
        input.clear();
        println!("What do you want to do?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input: Vec<&str> = input.split_whitespace().collect();
        
        
        match input[0]{
            "ls" =>     for (key, value) in &company {println!("{}: {:?}",key,value);},
            "add" | "Add"=> if input.len() == 4 {
                company.entry(String::from(input[3]))
                    .or_insert_with(Vec::new)
                    .push(String::from(input[1])); //Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
            },
            "users" | "Users" => if input.len() == 3 { if input[1]=="in" {println!("{:?}",company.entry(String::from(input[2])));}} // Then print users in a department
            else if input.len()== 1 {println!("{:?}", company);},
            "quit" | "Quit" => break(),
            
            _ => println!("invalid command"),
        }

    }
}

