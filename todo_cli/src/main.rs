use std::io::stdin;

struct SingleTodo {
    description: String,
    done: bool,
}

fn main() {
    let mut todos: Vec<SingleTodo> = Vec::new();
    loop {
        println!("What do you want to do?");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Unable to read line");

        let input = input.trim();

        match input {
            "addTodo" => add_todo(&mut todos),
            "viewTodo" => view_todo(&mut todos),
            "removeTodo" => remove_todo(&mut todos),
            "break" => break,
            _ => println!("This command is unknown!"),
        }
    }
}

fn add_todo(todos: &mut Vec<SingleTodo>) {
    println!("Add description: ");

    let mut desc = String::new();
    stdin().read_line(&mut desc).expect("Unable to read line!");

    desc = desc.trim().to_string();

    let todo = SingleTodo {
        description: desc,
        done: false,
    };

    todos.push(todo);
}

fn view_todo(todos: &mut Vec<SingleTodo>) {
    println!("Your current todos are:");

    for (i, todo) in todos.iter().enumerate() {
        println!("{}", i+1);
        print!(
            "{} and it is {}",
            todo.description,
            if todo.done { "done" } else { "not done" }
        );
    }
}

fn remove_todo(todos: &mut Vec<SingleTodo>) {
    println!("Which todo you want to remove");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read line");

    let number: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return;
        }
    };

    if number < todos.len() {
        todos.remove(number - 1 as usize);
    }
}
