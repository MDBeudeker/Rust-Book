use std::collections::HashMap;

fn main() {

    use std::io;

    let mut user_input = String::new();
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let departments = ["Engineering","Sales","Support","Management", "Cleaning"];
    for department in departments{
        add_department(&mut company,department);
    }

    add_employee(& mut company,"Engineering","Vicky");
    add_employee(& mut company,"Engineering","Arend");
    add_employee(& mut company,"Sales","Peter");
    add_employee(& mut company,"Engineering","Joseph");
    add_employee(& mut company,"Cleaning","Erik");
    add_employee(& mut company,"Management","Partyboi69");

    // if let Some(_engineering_vec) = company.get_mut("Engineering") { // Make this into a function
    //     add_employee(& mut company,"Engineering","Vicky");
    // }

    // if let Some(_engineering_vec) = company.get_mut("Engineering") {
    //     company.entry(String::from("Engineering"))
    //         .or_insert_with(Vec::new)
    //         .push(String::from("Arend"));
    // }

    help_message();

    println!("  To quit - type 'quit'");
    loop{
        user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        let user_input: Vec<&str> = user_input.split_whitespace().collect();
        
        
        match user_input[0]{
            "ls" =>     for (key, _value) in &company {println!("  {}",key);},

            "add" | "Add"=> if user_input.len() == 4 {add_employee(& mut company,user_input[3],user_input[1]);} else {println!("invalid command")},

            "users" | "Users" => if user_input.len() == 3 {list_employees_department(& company,user_input[1], user_input[2])}
                                 else if user_input.len()== 1 {list_all_employees(& company)}, //Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically. ,

            "help" | "Help" => help_message(),
            "quit" | "Quit" => break(),
            
            _ => println!("invalid command"),
        }

    }
}

fn add_department (company: &mut HashMap<String, Vec<String>>, department: &str) {
    company.insert(String::from(department),Vec::new());
}

fn add_employee<'a> (company: &'a mut HashMap<String, Vec<String>>, department: &str, employee: &str) -> &'a mut HashMap<String, Vec<String>> {
    match company.get_mut(department){
        Some(_dept_employees) => {
            company.entry(String::from(department))
                .or_insert_with(Vec::new)
                .push(String::from(employee));

            company
        }
        None => {
            println!("Non existent department {}, consider adding a department named {}!", department, department);
            company
        }
    }
}

fn list_employees_department<'a> (company: &'a HashMap<String, Vec<String>>, word: &str, department: &str) {
    if word =="in" {
        if let Some(dept_employees) = company.get(department) {
            let mut sorted_employees = dept_employees.clone();
            sorted_employees.sort();
            let employee_list = sorted_employees
                .iter()
                .map(|employee| format!("\n  - {}", employee))
                .collect::<Vec<_>>()
                .join("");
            println!("Employees in {}: {}", department, employee_list);
        }
    }
    else {
        println!("Invalid command, please type: Users in [department_name]")
    }
}


// Below code was written with help from chatGPT
fn list_all_employees(company: &HashMap<String, Vec<String>>){
    let mut employees: Vec<(&String, &String)> = vec![];

    for (department, names) in company {
        for name in names {
            employees.push((name, department));
        }
    }

    employees.sort_by_key(|k| k.0);

    for (name, department) in employees {
        println!("  {} - {}", name, department);
    }
    
}

fn help_message() {
    println!("\nWhat do you want to do?");
    println!("  To add an employee to a department, type 'add [Employeename] to [department]'");
    println!("  To view all departments, type 'ls'");
    println!("  To view all users and their department, type 'users'");
    println!("  To view all users in a specific department, type 'users in [department]'");
    println!("  To view this message again, type 'help'");
}
