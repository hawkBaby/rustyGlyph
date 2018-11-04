mod rustGlyph;
use rustGlyph::rust_glyph::*;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let config_yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(config_yaml).get_matches();
}
