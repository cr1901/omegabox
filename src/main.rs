mod cmds;
mod common;

use cmds::*;
use common::{Cmd, Result};

use eyre::eyre;
use pico_args::Arguments;
use std::ffi::OsString;
use std::path::Path;

fn main() -> Result<()> {
    let mut args: Vec<_> = std::env::args_os().collect();

    // If called via "omegabox", parse top-level args and look for a subcommand.
    strip_omegabox_arg(&mut args)?;
    let (subcmd, args) = find_subcommand(args)?;

    match subcmd {
        Some(s) => s.run(args)?,
        None => println!("omegabox v0.1.0")
    }

    Ok(())
}

// A global args take priority. The command must be the first or second argument.
// If called via symlink, subcommand will return the first command line arg or error.
fn find_subcommand(args_in: Vec<OsString>) -> Result<(Option<&'static dyn Cmd>, Arguments)> {
    let mut args = Arguments::from_vec(args_in);
    let cmd: Option<&'static dyn Cmd>;

    match args.subcommand()? {
        Some(s) => {
            cmd = Some(*COMMANDS.get(&*s.as_str()).ok_or(eyre!("Invalid command"))?);
        }
        None => {
            cmd = None;
        },
    }

    Ok((cmd, args))
}

fn strip_omegabox_arg(args: &mut Vec<OsString>) -> Result<()> {
    let cmd = Path::new(&args[0])
        .file_name()
        .and_then(|p| p.to_str())
        .ok_or(eyre!("Binary name could not be stripped from path"))?;

    // Command extracted, we don't need it anymore.
    if cmd.eq("omegabox") {
        args.remove(0);
    }

    Ok(())
}
