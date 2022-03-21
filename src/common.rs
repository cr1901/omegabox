use downcast_rs::{impl_downcast, Downcast};
pub use embedded_hal as hal_traits;
pub use linux_embedded_hal as hal;
pub use pico_args::Arguments;
pub use simple_eyre::eyre::{eyre, Result};

pub trait Cmd {
    fn print_help(&self);
    fn parse_args(&self, args: Arguments) -> Result<Box<dyn CmdParameters>>;
    fn run(&self, params: Box<dyn CmdParameters>) -> Result<()>;
}

pub trait CmdParameters: Downcast {}
impl_downcast!(CmdParameters);

pub fn downcast_params<T>(params: &Box<dyn CmdParameters>) -> Result<&T>
where
    T: CmdParameters,
{
    Ok(params.downcast_ref::<T>().ok_or(eyre!(format!(
        "could not extract parameters, expected type {}",
        std::any::type_name::<T>()
    )))?)
}
