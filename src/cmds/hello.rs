use crate::common::{Arguments, Cmd, Result};

pub struct Hello;

impl Cmd for Hello {
    fn run(&self, _args: Arguments) -> Result<()> {
        println!("Hello, world!");
        Ok(())
    }
}
