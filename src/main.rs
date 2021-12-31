mod analyzer;
pub mod config;
mod error;
mod item;
mod option;

use clap::Clap;
use option::Option;

fn main() {
    let _cli = Option::parse();

    // let mut analyzer = Analyzer::new();

    todo!();
}
