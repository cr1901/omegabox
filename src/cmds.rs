use crate::common::{Cmd};
use phf;

pub mod hello;

pub use hello::*;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
