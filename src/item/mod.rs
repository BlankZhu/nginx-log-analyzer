mod f64;
mod hour;
mod isize;
mod minute;
mod noop;
mod str;

use crate::error;
use serde::{Deserialize, Serialize};

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
        todo!()
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
