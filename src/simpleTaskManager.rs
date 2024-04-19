use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    println!("Hello and welcome to our great task Manager!!");

    loop {
        let user_choice = get_user_choice();

        match user_choice.as_str().trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => remove_task(&mut tasks),
            _ => println!("Please enter a valid option!"),
        }

        println!("Press enter to continue, or enter any other key to exit.");
        let mut exit = String::new();
        io::stdin().read_line(&mut exit).expect("Failed to read input");

        if !exit.trim().is_empty() {
            break;
        }
    }
}

fn get_user_choice() -> String {
    loop {
        println!("Please enter the number corresponding to what you want to do next:");
        println!("1 - Add task");
        println!("2 - List tasks");
        println!("3 - Remove a task");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let user_input = user_input.trim();

        if user_input == "1" || user_input == "2" || user_input == "3" {
            return user_input.to_string();
        } else {
            println!("Please enter a valid option!");
        }
    }
}

fn add_task(tasks: &mut Vec<String>) {
    println!("Enter the task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read input");
    tasks.push(task.trim().to_string());
}

fn list_tasks(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("There are no saved tasks!");
    } else {
        println!("Tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }
}

fn remove_task(tasks: &mut Vec<String>) {
    if tasks.is_empty() {
        println!("There are no saved tasks!");
        return;
    }

    println!("Enter the number corresponding to the task to be removed:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");

    match user_input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= tasks.len() => {
            tasks.remove(index - 1);
        }
        _ => println!("Invalid task number!"),
    }
}
