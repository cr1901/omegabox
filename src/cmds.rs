use crate::common::Cmd;
use phf;

mod hello;

use hello::Hello;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
