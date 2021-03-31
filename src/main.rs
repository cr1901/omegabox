mod common;
mod cmds;

use common::{Result};
use cmds::*;

fn main() -> Result<()> {
    let args : Vec<_> = std::env::args().collect();

    let cmd = *COMMANDS.get(&*args[1]).unwrap();
    cmd.run(&args)?;

    Ok(())
}
