use crate::error;

use super::{welford_step, Item, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Isize {
    title: String,
    total: isize,
    count: usize,
    mean: f64,
    variance: f64,
    max: isize,
    min: isize,

    #[serde(skip_serializing)]
    typ: Type,
}

impl Item for Isize {
    fn add(&mut self, datum: &String) -> Result<(), error::InvalidItemTypeError> {
        let parsed_isize = datum.parse::<isize>();
        match parsed_isize {
            Ok(value) => {
                self.total += value;
                self.count += 1;

                let old_mean = self.mean.clone();
                let old_variance = self.variance.clone();
                let (mean, variance) =
                    welford_step(&old_mean, &old_variance, self.count - 1, &(value as f64));
                self.mean = mean;
                self.variance = variance;

                if value > self.max {
                    self.max = value;
                }
                if value < self.min {
                    self.min = value;
                }

                Ok(())
            }
            Err(_) => Err(error::InvalidItemTypeError {
                item_title: self.title.clone(),
                typ: self.typ.clone(),
                data: datum.clone(),
            }),
        }
    }

    fn get_count(&self) -> usize {
        self.count.clone()
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl Isize {
    pub fn new(title: String) -> Isize {
        Isize {
            title,
            total: 0,
            count: 0,
            mean: 0.0,
            variance: 0.0,
            max: isize::MIN,
            min: isize::MAX,
            typ: Type::Isize,
        }
    }
}
