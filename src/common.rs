pub use eyre::{eyre, Result};
pub use pico_args::Arguments;

pub trait Cmd {
    fn print_help(&self);
    fn run(&self, args: Arguments) -> Result<()>;
}
