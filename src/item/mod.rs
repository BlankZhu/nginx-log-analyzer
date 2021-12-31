mod f64;
mod hour;
mod isize;
mod minute;
mod noop;
mod str;

use crate::error;
use serde::{Deserialize, Serialize};

use self::f64::F64;
use self::hour::Hour;
use self::isize::Isize;
use self::minute::Minute;
use self::noop::Noop;
use self::str::Str;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Type {
    F64,
    Hour,
    Isize,
    Minute,
    Noop,
    Str,
}

pub trait Item {
    fn add(&mut self, datum: &String) -> Result<(), error::InvalidItemTypeError>;
    fn get_count(&self) -> usize;
    fn get_title(&self) -> String;
    fn get_result(&self) -> String;
}

pub struct Factory;

impl Factory {
    pub fn create_item(typ: Type, title: String) -> Box<dyn Item> {
        match typ {
            Type::F64 => Box::new(F64::new(title)),
            Type::Hour => Box::new(Hour::new(title)),
            Type::Isize => Box::new(Isize::new(title)),
            Type::Minute => Box::new(Minute::new(title)),
            Type::Noop => Box::new(Noop::new(title)),
            Type::Str => Box::new(Str::new(title)),
        }
    }
}

pub fn welford_step(
    old_mean: &f64,
    old_variance: &f64,
    curr_index: usize,
    curr_value: &f64,
) -> (f64, f64) {
    let mut mean = old_mean.clone();
    let mut variance = old_variance.clone();

    mean += (curr_value - mean) / (curr_index as f64 + 1.0);
    variance += (curr_value - mean) * (curr_value - old_mean);

    (mean, variance)
}
