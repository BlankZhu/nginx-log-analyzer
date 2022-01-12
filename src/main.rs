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

    // debug
    // println!("log_format: `{}`", conf.log_format);

    let mut analyzer = Analyzer::new();
    if let Err(err) = analyzer.register_config(conf, cli.access_log_filename) {
        eprintln!("failed to apply config, detail: {}", err);
        return;
    }

    // debug
    // analyzer.debug_print_detail();
    if let Err(err) = analyzer.start() {
        eprintln!("failed to load access log, detail: {}", err);
        return;
    }
    println!("{}", analyzer.get_result());

    return;
}
