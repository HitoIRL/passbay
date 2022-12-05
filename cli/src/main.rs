mod commands;

use std::{io::{stdin, stdout, Write}, collections::HashMap, str::SplitAsciiWhitespace};

use commands::{echo::Echo, Command};

fn main() {
    let mut commands: HashMap<&str, Box<dyn Fn(&mut SplitAsciiWhitespace)>> = HashMap::new();
    commands.insert(Echo::get_name(), Box::new(Echo::on_run));

    loop {
        print!("\n hito@passbay ~ ");

        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.split_ascii_whitespace();

        let cmd = input.next().unwrap();

        match cmd {
            "exit" => break,
            c => if commands.contains_key(c) {
                let x = commands.get(c).unwrap();
                (*x)(&mut input);
            } else {
                println!("Command '{c}' not found");
            }
        }
    }
}
