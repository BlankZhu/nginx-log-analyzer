use crate::error::{
    invalid_access_log_error::*, invalid_log_format_error::*, invalid_stat_type_error::*,
    load_file_error::*,
};
use crate::stat::{
    item::item_factory::{ItemFactory, ItemType},
    NginxStat,
};
use regex::Regex;
use std::{
    fs::{read_to_string, File},
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
const THOUR_S: &str = "thour";

pub struct NginxLogAnalyzer {
    nginx_stat: NginxStat, // stores the data processor
    titles: Vec<String>,   // stores titles in string
    types: Vec<String>,    // stores the types in string
    extract_regex: Regex,  // stores the generated extract regex for access log
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

    pub fn get_readable_result(&mut self) -> String {
        let mut tmp = Vec::new();
        for item in self.nginx_stat.get_results() {
            tmp.push(item.get_readable_result());
        }
        tmp.join("\n==========\n")
    }

    pub fn get_json_result(&mut self) -> String {
        let mut tmp = Vec::new();
        for item in self.nginx_stat.get_results() {
            tmp.push(item.get_json_result());
        }
        let jsons = tmp.join(",");
        format!("[{}]", jsons)
    }

    pub fn debug_print_detail(&self) {
        let mut sb = String::new();

        sb.push_str("Titles:\n");
        sb.push_str(&self.titles.join(","));
        sb.push_str("\n");

        sb.push_str("Types:\n");
        sb.push_str(&self.types.join(","));
        sb.push_str("\n");

        sb.push_str("Extract regex: ");
        sb.push_str(&self.extract_regex.to_string());
        sb.push_str("\n");

        println!("{}", sb);
    }

    pub fn apply_access_log_file(&mut self, file_path: &str) -> Result<(), InvalidAccessLogError> {
        let lines = read_lines(file_path);
        match lines {
            Ok(lines) => {
                for line in lines {
                    match line {
                        Ok(line) => {
                            let datas = self.match_access_log_line(&line, &self.extract_regex);
                            self.nginx_stat.add_data(datas);
                        }
                        Err(err) => {
                            eprintln!("failed to read line, detail: {}", err);
                            continue;
                        }
                    }
                }
            }
            Err(err) => {
                let err_msg = format!("cannot read file at {}, detail: {}", file_path, err);
                return Err(InvalidAccessLogError::new(err_msg));
            }
        }
        Ok(())
    }

    pub fn apply_log_format_files(
        &mut self,
        log_fmt_file: &str,
        stat_type_file: &str,
    ) -> Result<(), LoadFileError> {
        let apply_res = self.apply_log_format_file(log_fmt_file);
        match apply_res {
            Ok(()) => {}
            Err(err) => {
                let err_msg = format!("invalid log format, detail: {}", err);
                return Err(LoadFileError::new(err_msg));
            }
        }

        let apply_res = self.apply_stat_type_file(stat_type_file);
        match apply_res {
            Ok(()) => {}
            Err(err) => {
                let err_msg = format!("invalid stat type error, detail: {}", err);
                return Err(LoadFileError::new(err_msg));
            }
        }

        if self.titles.len() != self.types.len() {
            let err_msg = format!(
                "titles length [{}] incapable with types length [{}]",
                self.titles.len(),
                self.types.len()
            );
            self.debug_print_detail();
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
                        .add_item(ItemFactory::create_item(ItemType::StrItemType, title));
                }
                ISIZE_S => {
                    self.nginx_stat
                        .add_item(ItemFactory::create_item(ItemType::IsizeItemType, title));
                }
                F64_S => {
                    self.nginx_stat
                        .add_item(ItemFactory::create_item(ItemType::F64ItemType, title));
                }
                NOOP_S => {
                    self.nginx_stat
                        .add_item(ItemFactory::create_item(ItemType::NoopItemType, title));
                }
                THOUR_S => {
                    self.nginx_stat
                        .add_item(ItemFactory::create_item(ItemType::TimeHourItemType, title));
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
                let mut sb = String::new();
                for line in lines {
                    match line {
                        Ok(line) => {
                            sb.push_str(&line);
                        }
                        Err(err) => {
                            let err_msg = format!(
                                "failed to read line in file {}, detail: {}",
                                file_path, err
                            );
                            return Err(InvalidLogFormatError::new(err_msg));
                        }
                    }
                }
                let lf = self.trim_nginx_log_format_str(&sb);
                self.titles = self.extract_titles(&lf);
                self.extract_regex = self.generate_extract_regex(&lf, &self.titles);
            }
            Err(err) => {
                let err_msg = format!(
                    "cannot read log format file at {}, detail: {}",
                    file_path, err
                );
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
            }
            Err(err) => {
                let err_msg = format!(
                    "cannot read stat type file at {}, detail: {}",
                    file_path, err
                );
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
        let mut re = String::from("^");

        // search prefix
        if let Some(first) = titles.first() {
            if let Some(first_pos) = to_search.find(first) {
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
        re.push_str("$");

        return Regex::new(re.as_str()).unwrap();
    }

    fn match_access_log_line(&self, line: &str, re: &Regex) -> Vec<String> {
        let mut ret = Vec::new();

        for caps in re.captures_iter(line) {
            if caps.len() != self.titles.len() + 1 {
                eprintln!(
                    "invalid matching, only {} elems were matched on line: `{}`",
                    caps.len(),
                    line
                );
                continue;
            }

            let mut i = 1;
            while i < caps.len() {
                let cap = &caps[i];
                ret.push(String::from(cap));
                i += 1;
            }
        }

        ret
    }
}
