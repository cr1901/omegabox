use crate::common::{Arguments, Cmd, Result};

pub(super) struct Hello;

struct HelloArgs {
    name: Option<String>,
}

const HELP: &str = "\
Hello

DESCRIPTION:
    Basic \"Hello World\"-style app for testing purposes

USAGE:
    hello [OPTIONS]

FLAGS:
    -h          Prints help information

OPTIONS:
    -n          Print a name instead of \"World\"

ARGS:
    None

INVALID ARG BEHAVIOR:
    Ignore
";

impl Cmd for Hello {
    fn print_help(&self) {
        print!("{}", HELP);
    }

    fn run(&self, mut args: Arguments) -> Result<()> {
        let pargs = HelloArgs {
            name: args.opt_value_from_str("-n")?,
        };

        if let Some(n) = pargs.name {
            println!("Hello, {}!", n);
        } else {
            println!("Hello, world!");
        }

        Ok(())
    }
}
