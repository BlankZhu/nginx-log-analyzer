mod analyzer;
mod config;
mod error;
mod item;
mod option;

use std::error::Error;

use analyzer::Analyzer;
use clap::Clap;
use config::Config;
use option::Option;

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Option::parse();
    Config::load_from_yaml_file(&cli.config_filename)
        .map_err(|err| Into::<Box<dyn Error>>::into(err))
        .and_then(|config| {
            let mut analyzer = Analyzer::new();
            analyzer
                .register_config(config, cli.access_log_filename)
                .map_err(|err| Into::<Box<dyn Error>>::into(err))
                .and_then(|_| {
                    analyzer
                        .start()
                        .map_err(|err| Into::<Box<dyn Error>>::into(err))
                        .map(|_| println!("{}", analyzer.get_result()))
                })
        })
}

fn main() {
    if let Err(err) = run() {
        eprintln!("[ryna failed], detail: {}", &err)
    }
}
