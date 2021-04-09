use crate::common::Cmd;
use phf;

mod hello;
mod led;

use hello::Hello;
use led::Led;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
