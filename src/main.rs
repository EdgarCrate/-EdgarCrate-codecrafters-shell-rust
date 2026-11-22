use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        // we flush the buffer after the print because if not, the next print will be in the same line
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        // when pressing enter the readline add "\n" to the string, thus we need to remove it
        stdin().read_line(&mut user_input).unwrap();
        println!("{}: command not found", user_input.trim());
    }
}
