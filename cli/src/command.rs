use std::str::SplitAsciiWhitespace;

pub trait Command { // TODO: implement description
    fn get_name<'a>() -> &'a str;
    fn on_run(args: &mut SplitAsciiWhitespace);
}
