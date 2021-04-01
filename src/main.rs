mod common;
mod cmds;

use common::{Result};
use cmds::*;

use eyre::eyre;
use std::convert::TryInto;
use std::path::Path;

fn main() -> Result<()> {
    let args : Vec<_> = std::env::args().collect();

    if args[0].contains("omegabox") {
        if args.len() < 2 {
            return Err(eyre!("No command supplied"));
        }

        if let Some(cmd) = COMMANDS.get(&*args[1]) {
            cmd.run(&args)?;
        } else {
            return Err(eyre!("Invalid command"));
        }
    }

    let path = Path::new(&args[0]);
    if let Some(stripped_path) = path.file_name().and_then(|p| p.to_str()) {
        if let Some(cmd_as_bin) = COMMANDS.get(stripped_path) {
            cmd_as_bin.run(&args[1..])?;
        } else {
            return Err(eyre!("Invalid command"));
        }
    } else {
        return Err(eyre!("Binary name could not be stripped from path"));
    }

    Ok(())
}
