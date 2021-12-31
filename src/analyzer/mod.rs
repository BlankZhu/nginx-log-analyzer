mod calculator;

use self::calculator::Calculator;
use crate::{config::Config, error};
use regex::Regex;
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

const STR: &str = "str";
const ISIZE: &str = "isize";
const F64: &str = "f64";
const NOOP: &str = "noop";
const HOUR: &str = "hour";
const MIN: &str = "min";

pub struct Analyzer {
    calculator: Calculator,
    extract_regex: Regex,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            calculator: Calculator::new(),
            extract_regex: Regex::new("").unwrap(),
        }
    }

    pub fn register_config(config: Config) -> Result<(), error::LogConfigError> {
        todo!()
    }

    pub fn get_result(&self) -> String {
        todo!()
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
}
