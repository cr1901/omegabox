pub use embedded_hal as hal_traits;
pub use simple_eyre::eyre::{eyre, Result};
pub use linux_embedded_hal as hal;
pub use pico_args::Arguments;

pub trait Cmd {
    fn print_help(&self);
    fn run(&self, args: Arguments) -> Result<()>;
}
