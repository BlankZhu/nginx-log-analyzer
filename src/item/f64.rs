use crate::error;

use super::{welford_step, Item, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct F64 {
    title: String,
    total: f64,
    count: usize,
    mean: f64,
    variance: f64,
    max: f64,
    min: f64,

    #[serde(skip_serializing)]
    typ: Type,
}

impl Item for F64 {
    fn add(&mut self, datum: &String) -> Result<(), error::InvalidItemDataError> {
        let parsed_f64 = datum.parse::<f64>();
        match parsed_f64 {
            Ok(value) => {
                self.total += value;
                self.count += 1;

                let old_mean = self.mean.clone();
                let old_variance = self.variance.clone();
                let (mean, variance) =
                    welford_step(&old_mean, &old_variance, self.count - 1, &value);
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
            Err(_) => Err(error::InvalidItemDataError {
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

    fn get_type(&self) -> String {
        format!("{:?}", self.typ)
    }
}

impl F64 {
    pub fn new(title: String) -> F64 {
        F64 {
            title,
            total: 0.0,
            count: 0,
            mean: 0.0,
            variance: 0.0,
            max: f64::MIN,
            min: f64::MAX,
            typ: Type::F64,
        }
    }
}
