mod command;

use std::{io::{stdin, stdout, Write}, collections::HashMap, str::SplitAsciiWhitespace, fmt::format};
use itertools::Itertools;

use command::Command;

struct Echo;

impl Command for Echo {
    fn get_name<'a>() -> &'a str {
        "echo"
    }

    fn on_run(args: &mut SplitAsciiWhitespace) {
        let out = args.join(" ");
        println!("{out}");
    }
}

struct Test;

impl Command for Test {
    fn get_name<'a>() -> &'a str {
        "test"
    }

    fn on_run(_args: &mut SplitAsciiWhitespace) {
        println!("test");
    }
}

fn main() {
    let mut commands: HashMap<&str, Box<dyn Fn(&mut SplitAsciiWhitespace)>> = HashMap::new();
    commands.insert(Echo::get_name(), Box::new(Echo::on_run));
    commands.insert(Test::get_name(), Box::new(Test::on_run));

    loop {
        print!("\n >> ");

        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.split_ascii_whitespace();

        let cmd = input.next().unwrap();
        if cmd == "exit" {
            break
        }

        if commands.contains_key(cmd) {
            let x = commands.get(cmd).unwrap();
            (*x)(&mut input);
        } else {
            println!("Command not found")
        }
    }
}
