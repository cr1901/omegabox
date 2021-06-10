use crate::common::Cmd;
use phf;

mod hello;
mod led;
mod relay;

use hello::Hello;
use led::Led;
use relay::Relay;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
