mod stat;
mod nginx_log_analyzer;
mod nginx_log_analyze_error;

use nginx_log_analyzer::NginxLogAnalyzer;


fn main() {
    let log_fmt = "res/log.fmt";
    let typ_fmt = "res/type.fmt";
    let access_log = "res/access.log";
    let mut analyzer = NginxLogAnalyzer::new();

    let apply_result = analyzer.apply_log_format_files(log_fmt, typ_fmt);
    match apply_result {
        Ok(()) => {},
        Err(err) => {
            println!("failed to load file, detail: {}", err);
            return;
        }
    }

    let analyze_result = analyzer.apply_access_log_file(access_log);
    match analyze_result {
        Ok(()) => {},
        Err(err) => {
            println!("failed to analyze access log, detail: {}", err);
            return;
        }
    }
    println!("{}", analyzer.get_result());
}
