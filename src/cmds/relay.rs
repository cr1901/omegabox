mod driver;

use crate::common::*;
use driver::Driver;

use hal::I2cdev;

pub(super) struct Relay;

struct RelayParams {
    no_init: bool,
    mask: u16,
    cmd: RelayCmd,
    duration: Option<u16>,
}

impl CmdParameters for RelayParams {}

enum RelayCmd {
    On,
    Off,
    Toggle,
    Pulse,
}

const HELP: &str = "\
Relay

DESCRIPTION:
    Toggle the Omega2 Relay Expansion on or off

USAGE:
    relay [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -h          Prints help information
    -n          Don't run initialization for each port expander (with nonzero mask).

OPTIONS:
    -d ms       Pulse duration (milliseconds)

ARGS:
    cmd         \"on\", \"off\", \"toggle\", or \"pulse\"
    mask        16-bit hex mask of relays across I2C addresses 0x20 to 0x27:
                Bit:  15 14 13 12 11 10   9 8   7 6   5 4   3 2   1 0
                Relay: 1  0  1  0  1  0   1 0   1 0   1 0   1 0   1 0
                Addr:  0x27  0x26  0x25  0x24  0x23  0x22  0x21  0x20

INVALID ARG BEHAVIOR:
    Return error on parse error
";

fn parse_cmd(s: &str) -> Result<RelayCmd, &'static str> {
    match s {
        "on" => Ok(RelayCmd::On),
        "off" => Ok(RelayCmd::Off),
        "toggle" => Ok(RelayCmd::Toggle),
        "pulse" => Ok(RelayCmd::Pulse),
        _ => Err("Expected \"on\", \"off\", \"toggle\", or \"pulse\""),
    }
}

fn parse_mask(s: &str) -> Result<u16, &'static str> {
    let no_prefix = s.trim_start_matches("0x");
    u16::from_str_radix(no_prefix, 16).map_err(|_| "Unable to convert mask to u16")
}

fn parse_ms(s: &str) -> Result<u16, &'static str> {
    s.parse().map_err(|_| "Unable to convert duration to u16")
}

impl Cmd for Relay {
    fn print_help(&self) {
        print!("{}", HELP);
    }

    fn parse_args(&self, mut args: Arguments) -> Result<Box<dyn CmdParameters>> {
        let pargs = RelayParams {
            no_init: args.contains("-n"),
            duration: args.opt_value_from_fn("-d", parse_ms)?,
            cmd: args.free_from_fn(parse_cmd)?,
            mask: args.free_from_fn(parse_mask)?,
        };

        Ok(Box::new(pargs))
    }

    fn run(&self, params: Box<dyn CmdParameters>) -> Result<()> {
        let dparams: &RelayParams = downcast_params(&params)?;

        let i2c = I2cdev::new("/dev/i2c-0")?;
        let mut driver = Driver::new(i2c);

        if !dparams.no_init {
            driver.init(dparams.mask)?;
        }

        match dparams.cmd {
            RelayCmd::On => {
                driver.on(dparams.mask)?;
            }
            RelayCmd::Off => {
                driver.off(dparams.mask)?;
            }
            RelayCmd::Toggle => {
                driver.toggle(dparams.mask)?;
            }
            RelayCmd::Pulse => {
                let dur = dparams
                    .duration
                    .ok_or(eyre!("duration value must be specified"))?;
                driver.pulse(dur, dparams.mask)?;
            }
        }

        Ok(())
    }
}
