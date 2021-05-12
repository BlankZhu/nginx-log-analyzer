mod stat;
mod nginx_log_analyzer;
mod nginx_log_analyze_error;

use nginx_log_analyzer::NginxLogAnalyzer;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("not enough arguments, use the command like: mc_process /path/to/log_format /path/to/access_log");
        return;
    }
    let log_format = args[1].as_str();
    let access_log = args[2].as_str();

    let mut analyzer = NginxLogAnalyzer::new();

    let apply_res = analyzer.apply_log_format(log_format);
    match apply_res {
        Ok(()) => {},
        Err(err) => {
            eprintln!("failed to apply log format, detail:\n\t{}\n", err);
            return;
        }
    }

    let apply_res = analyzer.apply_access_log(access_log);
    match apply_res {
        Ok(()) => {},
        Err(err) => {
            eprintln!("failed to apply access log, detail:\n\t{}\n", err);
            return;
        }
    }
    
    println!("{}", analyzer.get_result());
}
