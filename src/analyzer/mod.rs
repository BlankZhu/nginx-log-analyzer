mod calculator;

use std::{
    fs::File,
    io::{self, BufRead},
};

use self::calculator::Calculator;
use crate::{config::Config, error, item};
use lazy_static::lazy_static;
use regex::Regex;

const STR: &str = "str";
const ISIZE: &str = "isize";
const F64: &str = "f64";
const NOOP: &str = "noop";
const HOUR: &str = "hour";
const MIN: &str = "min";

lazy_static! {
    static ref RE_TITLE: Regex = Regex::new(r"(\$[a-zA-Z_]+)").unwrap();
}

pub struct Analyzer {
    calculator: Calculator,
    extract_regex: Regex,
    access_log_filename: String,
    log_title_width: usize,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            calculator: Calculator::new(),
            extract_regex: Regex::new("").unwrap(),
            access_log_filename: String::new(),
            log_title_width: 0,
        }
    }

    pub fn register_config(&mut self, config: Config) -> Result<(), error::LogConfigError> {
        // set access log filename
        self.access_log_filename = config.access_log;

        // parse log_format into titles
        let titles = self.parse_titles(&config.log_format);

        // parse log_types into types
        let types = self.parse_types(&config.log_types);
        let types = match types {
            Ok(t) => t,
            Err(err) => {
                return Err(error::LogConfigError {
                    detail: format!("invalid type: {:?}", err.raw_type),
                });
            }
        };

        // validate titles & types
        if titles.len() != types.len() {
            return Err(error::LogConfigError {
                detail: format!("`{:?}` doesn't fit `{:?}`", titles, types),
            });
        }

        // creat item by factory and add them into calculator
        let factory = item::Factory::new();
        let mut title_iter = titles.iter();
        let mut type_iter = types.iter();
        loop {
            match (title_iter.next(), type_iter.next()) {
                (Some(title), Some(typ)) => {
                    let item = factory.create_item(typ.clone(), title.clone());
                    self.calculator.register_item(item);
                }
                _ => break,
            }
        }
        self.log_title_width = titles.len();

        // generate access log extract regex
        let extracted = self.generate_extract_regex(&config.log_format, &titles);
        match extracted {
            Ok(e) => self.extract_regex = e,
            Err(e) => {
                return Err(error::LogConfigError {
                    detail: format!("{}", e),
                });
            }
        }

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), error::LoadAccessLogError> {
        let file = File::open(self.access_log_filename.clone());
        let file = match file {
            Ok(f) => f,
            Err(err) => {
                return Err(error::LoadAccessLogError {
                    filename: self.access_log_filename.clone(),
                    detail: format!("{}", &err),
                });
            }
        };

        let scanner = io::BufReader::new(file).lines();
        for line in scanner {
            match line {
                Ok(l) => match self.parse_access_log_line(l) {
                    Ok(data) => match self.calculator.add_data(data) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(error::LoadAccessLogError {
                                detail: format!("{}", err),
                                filename: self.access_log_filename.clone(),
                            });
                        }
                    },
                    Err(err) => {
                        return Err(error::LoadAccessLogError {
                            detail: format!("{}", err),
                            filename: self.access_log_filename.clone(),
                        });
                    }
                },
                Err(err) => {
                    return Err(error::LoadAccessLogError {
                        detail: format!("{}", err),
                        filename: self.access_log_filename.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn get_result(&self) -> String {
        let res = self.calculator.get_results();
        let serialized = serde_json::to_string(&res).unwrap();
        serialized
    }

    pub fn debug_print_detail(&self) {
        let mut sb = String::new();

        sb.push_str("Titles:\n");
        sb.push_str(&self.calculator.get_titles());
        sb.push_str("\n");

        sb.push_str("Types:\n");
        sb.push_str(&self.calculator.get_types());
        sb.push_str("\n");

        sb.push_str("Extract regex: ");
        sb.push_str(&self.extract_regex.to_string());
        sb.push_str("\n");
    }

    fn parse_titles(&self, log_format: &String) -> Vec<String> {
        let lf = self.trim_nginx_log_format_str(log_format);
        self.extract_titles(&lf)
    }

    fn parse_types(
        &self,
        log_types: &Vec<String>,
    ) -> Result<Vec<item::Type>, error::InvalidLogTypeError> {
        let mut ret = Vec::new();

        for i in log_types.iter() {
            match i.as_str() {
                STR => {
                    ret.push(item::Type::Str);
                }
                ISIZE => {
                    ret.push(item::Type::Isize);
                }
                F64 => {
                    ret.push(item::Type::F64);
                }
                NOOP => {
                    ret.push(item::Type::Noop);
                }
                HOUR => {
                    ret.push(item::Type::Hour);
                }
                MIN => {
                    ret.push(item::Type::Minute);
                }
                _ => {
                    return Err(error::InvalidLogTypeError {
                        raw_type: i.to_string(),
                    })
                }
            }
        }

        Ok(ret)
    }

    fn trim_nginx_log_format_str(&self, nginx_log_format: &String) -> String {
        let mut lf = nginx_log_format.clone();
        lf = lf.replace("\n", "");
        lf = lf.replace("'", "");
        lf
    }

    fn extract_titles(&self, nginx_log_format: &String) -> Vec<String> {
        let mut ret = Vec::new();
        for caps in RE_TITLE.captures_iter(nginx_log_format) {
            ret.push(String::from(caps.get(1).unwrap().as_str()));
        }
        ret
    }

    fn generate_extract_regex(
        &self,
        nginx_log_format: &String,
        titles: &Vec<String>,
    ) -> Result<Regex, error::ExtractRegexError> {
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
                return Err(error::ExtractRegexError {});
            }
        }

        // search postfix
        if to_search.len() != 0 {
            re.push_str(&to_search.to_string());
        }
        re.push_str("$");

        // return Regex::new(re.as_str()).unwrap();
        let ret = match Regex::new(re.as_str()) {
            Ok(r) => r,
            Err(_) => {
                return Err(error::ExtractRegexError {});
            }
        };
        Ok(ret)
    }

    fn parse_access_log_line(
        &self,
        line: String,
    ) -> Result<Vec<String>, error::InvalidLogLineError> {
        let mut ret = Vec::new();

        for caps in self.extract_regex.captures_iter(&line) {
            if caps.len() != self.log_title_width + 1 {
                return Err(error::InvalidLogLineError { line });
            }

            let mut i = 1;
            while i < caps.len() {
                let cap = &caps[i];
                ret.push(String::from(cap));
                i += 1;
            }
        }

        Ok(ret)
    }
}
