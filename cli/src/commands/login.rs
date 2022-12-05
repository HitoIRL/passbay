use std::str::SplitAsciiWhitespace;

use super::Command;

pub struct Login;

impl Command for Login {
    fn get_name<'a>() -> &'a str {
        "login"
    }

    fn on_run(args: &mut SplitAsciiWhitespace) {
        if let (Some(username), Some(password)) = (args.next(), args.next()) {
            println!("{username}:{password}");
        } else {
            println!("Usage: login <username> <password>");
        }
    }
}
