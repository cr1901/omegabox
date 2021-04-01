pub use eyre::Result;

pub trait Cmd {
    fn run(&self, args: &[String]) -> Result<()>;
}
