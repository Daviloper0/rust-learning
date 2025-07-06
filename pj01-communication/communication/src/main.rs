use std::io::{self, Write};

fn greetings() {
    println!("Welcome to communication!");
    print!("Please type your username: ");
    io::stdout().flush().expect("Deu pau de novo");
}
fn get_username_input() {
    let mut username:String = String::new();

    match io::stdin()
            .read_line(&mut username) {
            Ok(_) => {},
            Err(error) => {print!("There was an error while trying to get your input. I can't help you. Error: {error}")},
    }
}
fn sanitize_username(username: String) -> String {
    let mut sanitized_username = String::new();
    
    
    return String::new();
}

fn main() {
    greetings();
    get_username_input();
}
