#[macro_use]
mod rustGlyph;
extern crate clap;

use clap::App;
use rustGlyph::rust_glyph::*;

fn main() {
    let config_yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(config_yaml).get_matches();
}
