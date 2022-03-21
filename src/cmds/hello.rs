use crate::common::*;

pub(super) struct Hello;

struct HelloParams {
    name: Option<String>,
}

impl CmdParameters for HelloParams {}

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

    fn parse_args(&self, mut args: Arguments) -> Result<Box<dyn CmdParameters>> {
        let pargs = HelloParams {
            name: args.opt_value_from_str("-n")?,
        };

        Ok(Box::new(pargs))
    }

    fn run(&self, params: Box<dyn CmdParameters>) -> Result<()> {
        let dparams: &HelloParams = params.downcast_ref().ok_or(eyre!("Hello Cmd didn't receive HelloParams"))?;

        if let Some(n) = &dparams.name {
            println!("Hello, {}!", n);
        } else {
            println!("Hello, world!");
        }

        Ok(())
    }
}
