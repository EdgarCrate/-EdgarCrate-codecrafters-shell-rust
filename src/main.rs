use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        // we flush the buffer after the print because if not, the next print will be in the same line
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        // when pressing enter the readline add "\n" to the string, thus we need to remove it
        stdin()
            .read_line(&mut user_input)
            .expect("An error ocurred while reading the input");

        let user_input = user_input.trim();
        let mut input_parts = user_input.split_whitespace();
        let command = input_parts.next();
        if let Some(c) = command {
            let args: Vec<&str> = input_parts.collect();
            let msg = args.join(" ");
            match c {
                "exit" => break,
                "echo" => {
                    println!("{}", msg);
                }
                _ => println!("{}: command not found", c),
            };
        } else {
            continue;
        }
    }
}
