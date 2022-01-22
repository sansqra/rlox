mod helpers;

use std::{ env, io::{ self, Write }, fs, process::exit };
use helpers::get_functions::get_extension_from_filename;

fn main() {
    let mut _had_error: &'static String;
    let args: Vec<String> = env::args().collect();

    if &args[1..].len() > &1 {
        println!("Usage rlox [script]");
        exit(64);
    } else if &args[1..].len() == &1 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &str) {
    if get_extension_from_filename(file_path) == Some("rlx") {
        let contents = fs::read_to_string(file_path).expect("Error in file read");
        println!("{}", contents);
    } else {
        println!("Invalid file extension");
    }
}

fn run_prompt() {
    loop {
        let mut formatted_code = String::new();
        let mut code_line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut code_line).expect("Could not interpret code");
        let code_chars = code_line.to_string();

        for c in code_chars.chars() {
            if c.to_string() != "\r"  {
                formatted_code.push(c);
            }
        }
        println!("{:?}", formatted_code);
    }
}
