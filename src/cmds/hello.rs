use crate::common::{Cmd, Result};

pub struct Hello;

impl Cmd for Hello {
    fn run(&self, _args: &[String]) -> Result<()> {
        println!("Hello, world!");
        Ok(())
    }
}
