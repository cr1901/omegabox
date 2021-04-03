use crate::common::{Arguments, Cmd, Result};

pub(super) struct Hello;

const HELP: &str = "\
Hello

DESCRIPTION:
    Basic \"Hello World\"-style app for testing purposes

USAGE:
    hello [OPTIONS]

FLAGS:
    -h          Prints help information

OPTIONS:
    None

ARGS:
    None

INVALID ARG BEHAVIOR:
    Ignore
";

impl Cmd for Hello {
    fn print_help(&self) {
        print!("{}", HELP);
    }

    fn run(&self, _args: Arguments) -> Result<()> {
        println!("Hello, world!");
        Ok(())
    }
}
