use crate::common::*;

use gpio_cdev::{Chip, LineRequestFlags};
use hal::CdevPin;
use hal_traits::digital::v2::OutputPin;

pub(super) struct Led;

struct LedParams {
    color: Color,
    cmd: LedCmd,
}

impl CmdParameters for LedParams {}

#[derive(Clone, Copy)]
enum Color {
    Red = 17,
    Green = 16,
    Blue = 15,
}

enum LedCmd {
    On,
    Off,
}

const HELP: &str = "\
Led

DESCRIPTION:
    Toggle the Omega2 Expansion Dock LEDs on or off

USAGE:
    led [FLAGS] [ARGS]

FLAGS:
    -h          Prints help information

OPTIONS:
    None

ARGS:
    color      \"red\", \"green\", or \"blue\"
    cmd        \"on\" or \"off\"

INVALID ARG BEHAVIOR:
    Return error
";

fn parse_color(s: &str) -> Result<Color, &'static str> {
    match s {
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "blue" => Ok(Color::Blue),
        _ => Err("Expected \"red\", \"green\", or \"blue\""),
    }
}

fn parse_cmd(s: &str) -> Result<LedCmd, &'static str> {
    match s {
        "on" => Ok(LedCmd::On),
        "off" => Ok(LedCmd::Off),
        _ => Err("Expected \"on\" or \"off\""),
    }
}

impl Cmd for Led {
    fn print_help(&self) {
        print!("{}", HELP);
    }

    fn parse_args(&self, mut args: Arguments) -> Result<Box<dyn CmdParameters>> {
        let pargs = LedParams {
            color: args.free_from_fn(parse_color)?,
            cmd: args.free_from_fn(parse_cmd)?,
        };

        Ok(Box::new(pargs))
    }

    fn run(&self, params: Box<dyn CmdParameters>) -> Result<()> {
        let dparams: &LedParams = params.downcast_ref().ok_or(eyre!("Led Cmd didn't receive LedParams"))?;

        let mut chip = Chip::new("/dev/gpiochip0")?;
        let handle = chip.get_line(dparams.color as u32)?.request(
            LineRequestFlags::OUTPUT,
            0,
            "set-output",
        )?;

        let mut led = CdevPin::new(handle)?;

        match dparams.cmd {
            LedCmd::On => {
                led.set_low()?;
            }
            LedCmd::Off => {
                led.set_high()?;
            }
        }

        Ok(())
    }
}
