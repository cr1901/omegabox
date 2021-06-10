use crate::common::*;

use bitvec::prelude::*;
use hal_traits::blocking::i2c::{Read, Write, WriteRead};
use std::error;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

pub struct Driver<T>
where
    T: Read + Write + WriteRead,
{
    ctx: T,
}

pub enum Error<T>
where
    T: Read + Write + WriteRead,
{
    #[allow(unused)]
    ReadError(<T as Read>::Error),
    WriteError(<T as Write>::Error),
    WriteReadError(<T as WriteRead>::Error),
}

impl<T> fmt::Display for Error<T>
where
    T: Read + Write + WriteRead,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadError(_) => write!(f, "HAL read error"),
            Error::WriteError(_) => write!(f, "HAL write error"),
            Error::WriteReadError(_) => write!(f, "HAL write-read error"),
        }
    }
}

impl<T> fmt::Debug for Error<T>
where
    T: Read + Write + WriteRead,
    <T as Read>::Error: fmt::Debug,
    <T as Write>::Error: fmt::Debug,
    <T as WriteRead>::Error: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadError(r) => f.debug_tuple("ReadError").field(r).finish(),
            Error::WriteError(w) => f.debug_tuple("WriteError").field(w).finish(),
            Error::WriteReadError(wr) => f.debug_tuple("WriteReadError").field(wr).finish(),
        }
    }
}

impl<T> error::Error for Error<T>
where
    T: Read + Write + WriteRead,
    <T as Read>::Error: fmt::Debug,
    <T as Write>::Error: fmt::Debug,
    <T as WriteRead>::Error: fmt::Debug,
{
}

impl<T> Driver<T>
where
    T: Read + Write + WriteRead,
{
    const IODIR: u8 = 0;
    const GPIO: u8 = 0x9;

    pub fn new(ctx: T) -> Self {
        Driver { ctx }
    }

    pub fn init(&mut self, mask: u16) -> Result<(), Error<T>> {
        Self::iterate_relays(mask, |adr, _| {
            // Initialize both relays to outputs avoid confusion.
            self.ctx
                .write(adr, &[Self::IODIR, !0x3])
                .map_err(Error::WriteError)?;

            Ok(())
        })
    }

    pub fn on(&mut self, mask: u16) -> Result<(), Error<T>> {
        Self::iterate_relays(mask, |adr, expander_mask| {
            let mut buf: [u8; 1] = [0; 1];

            self.ctx
                .write_read(adr, &[Self::GPIO], &mut buf)
                .map_err(Error::WriteReadError)?;

            let turn_on = buf[0] | expander_mask;

            self.ctx
                .write(adr, &[Self::GPIO, turn_on])
                .map_err(Error::WriteError)?;

            Ok(())
        })
    }

    pub fn off(&mut self, mask: u16) -> Result<(), Error<T>> {
        Self::iterate_relays(mask, |adr, expander_mask| {
            let mut buf: [u8; 1] = [0; 1];

            self.ctx
                .write_read(adr, &[Self::GPIO], &mut buf)
                .map_err(Error::WriteReadError)?;

            let turn_off = buf[0] & !expander_mask;

            self.ctx
                .write(adr, &[Self::GPIO, turn_off])
                .map_err(Error::WriteError)?;

            Ok(())
        })
    }

    pub fn toggle(&mut self, mask: u16) -> Result<(), Error<T>> {
        Self::iterate_relays(mask, |adr, expander_mask| {
            let mut buf: [u8; 1] = [0; 1];

            self.ctx
                .write_read(adr, &[Self::GPIO], &mut buf)
                .map_err(Error::WriteReadError)?;

            let toggled = buf[0] ^ expander_mask;

            self.ctx
                .write(adr, &[Self::GPIO, toggled])
                .map_err(Error::WriteError)?;

            Ok(())
        })
    }

    pub fn pulse(&mut self, millis: u16, mask: u16) -> Result<(), Error<T>> {
        let duration = Duration::from_millis(millis as u64);

        self.toggle(mask)?;
        sleep(duration);
        self.toggle(mask)?;

        Ok(())
    }

    fn iterate_relays<F>(mask: u16, mut f: F) -> Result<(), Error<T>> where F: FnMut(u8, u8) -> Result<(), Error<T>> {
        let bits = BitSlice::<Msb0, _>::from_element(&mask);

        for (adr, b) in (0x20..=0x27).rev().zip(bits.chunks(2)) {
            let expander_mask: u8 = b.load_le::<u8>();

            if expander_mask != 0 {
                f(adr, expander_mask)?;
            }
        }

        Ok(())
    }
}
