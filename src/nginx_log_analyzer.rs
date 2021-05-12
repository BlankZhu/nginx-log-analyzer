use crate::nginx_log_analyze_error::*;
use crate::stat::{
    nginx_stat::NginxStat,
    stat_factory::{StatFactory, StatType::*},
};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const STR_S: &str = "str";
const ISIZE_S: &str = "isize";
const F64_S: &str = "f64";
const NOOP_S: &str = "noop";

pub struct NginxLogAnalyzer {
    nginx_stat: NginxStat,
}

impl NginxLogAnalyzer {
    pub fn new() -> NginxLogAnalyzer {
        NginxLogAnalyzer {
            nginx_stat: NginxStat::new(),
        }
    }

    pub fn apply_log_format(&mut self, file_path: &str) -> Result<(), InvalidLogFormatError> {
        // pub fn apply_log_format(&mut self, file_path: &str) {
        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                match line {
                    Ok(line) => {
                        // trim
                        let line = line.trim();
                        // split by ','
                        let splited = line.split(',').collect::<Vec<&str>>();
                        // check splited length
                        if splited.len() != 2 {
                            let err_msg =
                                format!("NginxLogAnalyzer got invalid log format line: {}", line);
                            return Err(InvalidLogFormatError::new(err_msg));
                        }
                        // match to generate Stat
                        let typ = splited[0].to_ascii_lowercase();
                        let title = splited[1];
                        match typ.as_str() {
                            STR_S => {
                                self.nginx_stat
                                    .add_title(StatFactory::create_stat(StrStatType, title));
                            }
                            ISIZE_S => {
                                self.nginx_stat
                                    .add_title(StatFactory::create_stat(IsizeStatType, title));
                            }
                            F64_S => {
                                self.nginx_stat
                                    .add_title(StatFactory::create_stat(F64StatType, title));
                            }
                            NOOP_S => {
                                self.nginx_stat
                                    .add_title(StatFactory::create_stat(NoopStatType, title));
                            }
                            _ => {
                                let err_msg = format!("failed to parse type from {}", typ);
                                return Err(InvalidLogFormatError::new(err_msg));
                            }
                        }
                    }
                    Err(err) => {
                        return Err(InvalidLogFormatError::new(err.to_string()));
                    }
                }
            }
        } else {
            let err_msg = format!(
                "NginxLogAnalyzer failed to read log format at {}",
                file_path
            );
            return Err(InvalidLogFormatError::new(err_msg));
        }
        Ok(())
    }

    pub fn apply_access_log(&mut self, file_path: &str) -> Result<(), InvalidAccessLogError> {
        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                match line {
                    Ok(line) => {
                        let res = self.nginx_stat.read_line(line);
                        match res {
                            Err(err_msg) => {
                                return Err(InvalidAccessLogError::new(err_msg));
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        eprintln!("failed to read line, detail: {}", err);
                        continue;
                    }
                }
            }
        } else {
            let err_msg = format!("cannot read file at {}", file_path);
            return Err(InvalidAccessLogError::new(err_msg));
        }
        Ok(())
    }

    pub fn get_result(&mut self) -> String {
        self.nginx_stat.get_result()
    }
}
