use crate::common::*;

use gpio_cdev::{Chip, LineRequestFlags};

pub(super) struct Led;

struct LedArgs {
    color: Color,
    cmd: LedCmd,
}

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

    fn run(&self, mut args: Arguments) -> Result<()> {
        let pargs = LedArgs {
            color: args.free_from_fn(parse_color)?,
            cmd: args.free_from_fn(parse_cmd)?,
        };

        let mut chip = Chip::new("/dev/gpiochip0")?;
        let handle = chip.get_line(pargs.color as u32)?.request(
            LineRequestFlags::OUTPUT,
            0,
            "set-output",
        )?;

        match pargs.cmd {
            LedCmd::On => {
                handle.set_value(0)?;
            }
            LedCmd::Off => {
                handle.set_value(1)?;
            }
        }

        Ok(())
    }
}
