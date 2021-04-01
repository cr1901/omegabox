use phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
        "pub const COMMANDS: phf::Map<&'static str, &dyn Cmd> = \n{};\n",
        phf_codegen::Map::new().entry("hello", "&Hello").build()
    )
    .unwrap();
}
