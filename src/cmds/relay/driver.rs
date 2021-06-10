use crate::common::*;

use hal_traits::blocking::i2c::{Read, Write, WriteRead};
use std::error;
use std::fmt;

struct Driver<T>
where
    T: Read + Write,
{
    ctx: T,
}

enum Error<T>
where
    T: Read + Write,
{
    ReadError(<T as Read>::Error),
    WriteError(<T as Write>::Error),
}

impl<T> fmt::Display for Error<T>
where
    T: Read + Write,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadError(_) => write!(f, "HAL read error"),
            Error::WriteError(_) => write!(f, "HAL write error"),
        }
    }
}

impl<T> fmt::Debug for Error<T>
where
    T: Read + Write,
    <T as Read>::Error: fmt::Debug,
    <T as Write>::Error: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadError(r) => f.debug_tuple("ReadError").field(r).finish(),
            Error::WriteError(w) => f.debug_tuple("WriteError").field(w).finish(),
        }
    }
}

impl<T> error::Error for Error<T>
where
    T: Read + Write,
    <T as Read>::Error: fmt::Debug,
    <T as Write>::Error: fmt::Debug,
{
}

impl<T> Driver<T>
where
    T: Read + Write,
{
    fn new(ctx: T) -> Self {
        Driver { ctx }
    }

    fn init(&mut self, mask: u16) -> Result<(), Error<T>> {
        unimplemented!()
    }
}
