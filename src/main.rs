use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut multiline_mode: bool = false;
    let mut running: bool = true;
    // let mut enable_multiline_mode = || multiline_mode = true;
    // let mut exit_repl = || running = false;

    while running {
        let mut input: String = String::new();

        if multiline_mode {
            print!("");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read input");
            multiline_mode = false;
        } else {
            print!("> ");
            io::stdout().flush().unwrap();
            // read
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }

        let output = eval(input, || multiline_mode = true, || running = false);
        print!("{}", output);

        io::stdout().flush().unwrap();
    }
    println!("Bye 👋🏼!")
}

fn eval<F, G>(input: String, enable_multiline_mode: F, exit_repl: G) -> String
where
    F: FnOnce(),
    G: FnOnce(),
{
    let input = input.trim();
    let empty_string = String::new();

    match input {
        ".m" => {
            enable_multiline_mode();
            empty_string
        }
        ".exit" => {
            exit_repl();
            empty_string
        }
        _ => format!("{}\n", input),
    }
}
