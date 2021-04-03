pub use eyre::Result;
pub use pico_args::Arguments;

pub trait Cmd {
    fn run(&self, args: pico_args::Arguments) -> Result<()>;
}
