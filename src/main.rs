mod analyzer;
mod config;
mod error;
mod item;
mod option;

use analyzer::Analyzer;
use clap::Clap;
use config::Config;
use option::Option;

fn main() {
    let cli = Option::parse();
    let conf = match Config::load_from_yaml_file(&cli.config_filename) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("load config error, detail: {}", err);
            return;
        }
    };

    let mut analyzer = Analyzer::new();
    match analyzer.register_config(conf) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to apply config, detail: {}", err);
            return;
        }
    }
    analyzer.debug_print_detail();
    match analyzer.start() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to load access log, detail: {}", err);
            return;
        }
    }
    println!("{}", analyzer.get_result());

    return;
}
