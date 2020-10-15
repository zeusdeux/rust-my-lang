use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut multiline_mode: bool = false;
    let mut running: bool = true;

    while running {
        let mut input: String = String::new();

        if !multiline_mode {
            print!("> ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let trimmed_input = input.trim_matches('\n');

            match trimmed_input {
                "+m" => multiline_mode = true,
                ".exit" => running = false,
                _ => print!("{}", input),
            }
        } else {
            print!("");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read input");

            print!("{}", input);
            multiline_mode = false
        }
        io::stdout().flush().unwrap();
    }
    println!("Bye 👋🏼!")
}
