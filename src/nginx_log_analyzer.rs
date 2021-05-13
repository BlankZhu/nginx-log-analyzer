use crate::nginx_log_analyze_error::*;
use crate::stat::{
    nginx_stat::NginxStat,
    stat_factory::{StatFactory, StatType::*},
};
use regex::Regex;
use std::{fmt::format, fs::{File, read_to_string}, io::{self, BufRead}, path::Path};

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
    titles: Vec<String>,
    types: Vec<String>,
    extract_regex: Regex,
}

impl NginxLogAnalyzer {
    pub fn new() -> NginxLogAnalyzer {
        NginxLogAnalyzer {
            nginx_stat: NginxStat::new(),
            titles: Vec::new(),
            types: Vec::new(),
            extract_regex: Regex::new("").unwrap(),
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

    pub fn apply_access_log_file(&mut self, file_path: &str) -> Result<(), InvalidAccessLogError> {
        let lines = read_lines(file_path);
        match lines {
            Ok(lines) => {
                for line in lines {
                    match line {
                        Ok(line) => {
                            let datas = self.match_access_log_line(&line,&self.extract_regex);
                            self.nginx_stat.add_data(datas);
                        }
                        Err(err) => {
                            eprintln!("failed to read line, detail: {}", err);
                            continue;
                        }
                    }
                }
            },
            Err(err) => {
                let err_msg = format!("cannot read file at {}, detail: {}", file_path, err);
                return Err(InvalidAccessLogError::new(err_msg));
            }
        }
        Ok(())
    }

    pub fn apply_log_format_files(&mut self, log_fmt_file: &str, stat_type_file: &str) -> Result<(), LoadFileError> {
        let apply_res = self.apply_log_format_file(log_fmt_file);
        match apply_res {
            Ok(()) => {},
            Err(err) => {
                let err_msg = format!("invalid log format, detail: {}", err);
                return Err(LoadFileError::new(err_msg));
            }
        }
        
        let apply_res = self.apply_stat_type_file(stat_type_file);
        match apply_res {
            Ok(()) => {},
            Err(err) => {
                let err_msg = format!("invalid stat type error, detail: {}", err);
                return Err(LoadFileError::new(err_msg));
            }
        }

        if self.titles.len() != self.types.len() {
            let err_msg = format!("titles length incapable with types length");
            return Err(LoadFileError::new(err_msg));
        }

        // generate nginx_stat
        let mut index = 0;
        while index < self.titles.len() {
            let title = self.titles[index].as_str();
            let typ = self.types[index].as_str();
            match typ {
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
                    panic!("invalid stat type")
                }
            }
            index += 1;
        }

        Ok(())
    }

    fn apply_log_format_file(&mut self, file_path: &str) -> Result<(), InvalidLogFormatError> {
        let lines = read_lines(file_path);
        match lines {
            Ok(lines) => {
                let mut content = String::new();
                for line in lines {
                    match line {
                        Ok(line) => {
                            content.push_str(&line);
                        }
                        Err(err) => {
                            let err_msg = format!("failed to read line in file {}, detail: {}", file_path, err);
                            return Err(InvalidLogFormatError::new(err_msg));
                        }
                    }
                }
                let lf = self.trim_nginx_log_format_str(&content);
                self.titles = self.extract_titles(&lf);
                self.extract_regex = self.generate_extract_regex(&lf, &self.titles);
            },
            Err(err) => {
                let err_msg = format!("cannot read log format file at {}, detail: {}", file_path, err);
                return Err(InvalidLogFormatError::new(err_msg));
            }
        }

        Ok(())
    }

    fn apply_stat_type_file(&mut self, file_path: &str) -> Result<(), InvalidStatTypeError> {
        let line = read_to_string(file_path);
        match line {
            Ok(line) => {
                let line = line.trim();
                let elems = line.split(",").collect::<Vec<&str>>();
                for elem in elems {
                    self.types.push(elem.to_string());
                }
            },
            Err(err) => {
                let err_msg = format!("cannot read stat type file at {}, detail: {}", file_path, err);
                return Err(InvalidStatTypeError::new(err_msg));
            }
        }
        Ok(())
    }

    fn trim_nginx_log_format_str(&self, nginx_log_format: &String) -> String {
        // organize in one line please...
        let mut lf = nginx_log_format.clone();
        lf = lf.replace("\n", "");
        lf = lf.replace("'", "");
        lf
    }

    fn extract_titles(&self, nginx_log_format: &String) -> Vec<String> {
        let mut ret = Vec::new();
        let re = Regex::new(r"(\$[a-zA-Z_]+)").unwrap();
        for caps in re.captures_iter(nginx_log_format) {
            ret.push(String::from(caps.get(1).unwrap().as_str()));
        }
        ret
    }

    fn generate_extract_regex(&self, nginx_log_format: &String, titles: &Vec<String>) -> Regex {
        let mut to_search = &nginx_log_format[..];
        let mut re = String::new();

        // search prefix
        if let Some(first) = titles.first() {
            println!("first title: {}", first);
            if let Some(first_pos) = to_search.find(first) {
                println!("first title found at: {}", first_pos);
                if first_pos != 0 {
                    println!("prefix exist");
                    // prefix exist
                    re.push_str(&to_search[0..first_pos].to_string());
                    to_search = &to_search[first_pos..];
                }
            }
        }

        // main search
        for title in titles {
            if let Some(found) = to_search.find(title) {
                // re.push_str(&to_search[0..found].to_string());
                re.push_str(&regex::escape(&to_search[0..found]));
                to_search = &to_search[(found + title.len())..];
                re.push_str("(.+?)");
            } else {
                eprintln!("failed to find {} in {}", title, to_search);
            }
        }

        // search postfix
        if to_search.len() != 0 {
            re.push_str(&to_search.to_string());
        }

        return Regex::new(re.as_str()).unwrap();
    }

    fn match_access_log_line(&self, line: &str, re: &Regex) -> Vec<String> {
        let mut ret = Vec::new();

        for caps in re.captures_iter(line) {
            for cap in caps.iter() {
                match cap {
                    Some(cap) => {
                        ret.push(String::from(cap.as_str()));
                    }
                    None => continue,
                }
            }
        }

        ret
    }
}
