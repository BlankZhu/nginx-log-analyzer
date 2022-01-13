mod analyzer;
mod config;
mod error;
mod item;
mod option;

use analyzer::Analyzer;
use clap::Clap;
use config::Config;
use log::error;
use option::Option;
use std::error::Error;

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
    env_logger::init();

    if let Err(err) = run() {
        error!("ryna failed, detail: {}", &err)
    }
}
