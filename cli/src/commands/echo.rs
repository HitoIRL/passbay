use std::str::SplitAsciiWhitespace;

use itertools::Itertools;

use super::Command;

pub struct Echo;

impl Command for Echo {
    fn get_name<'a>() -> &'a str {
        "echo"
    }

    fn on_run(args: &mut SplitAsciiWhitespace) {
        let out = args.join(" ");
        println!("{out}");
    }
}
