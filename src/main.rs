use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        let user_input = stdin().read_line(&mut user_input).unwrap();
        print!("{} command not found", user_input);
    }
}
