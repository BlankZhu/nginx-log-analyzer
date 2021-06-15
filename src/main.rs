mod analyzer;
mod cli;
mod error;
mod stat;

use analyzer::NginxLogAnalyzer;
use clap::Clap;
use cli::NginxLogAnalyzerCli;

fn main() {
    let cli = NginxLogAnalyzerCli::parse();
    println!(
        "using log fmt file `{}`, type fmt file `{}`, access log `{}`",
        cli.logfmt, cli.typfmt, cli.acclog
    );

    let mut analyzer = NginxLogAnalyzer::new();

    let apply_result = analyzer.apply_log_format_files(&cli.logfmt, &cli.typfmt);
    match apply_result {
        Ok(()) => {}
        Err(err) => {
            println!("failed to load file, detail: {}", err);
            return;
        }
    }

    let analyze_result = analyzer.apply_access_log_file(&cli.acclog);
    match analyze_result {
        Ok(()) => {}
        Err(err) => {
            println!("failed to analyze access log, detail: {}", err);
            return;
        }
    }

    if cli.json {
        println!("{}", analyzer.get_json_result());
    } else {
        println!("{}", analyzer.get_readable_result());
    }
}
