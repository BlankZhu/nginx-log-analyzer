use super::ItemResult;
use num::traits::Pow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IsizeResult {
    title: String,
    total: isize,
    count: usize,
    average: f64,
    variance: f64,
    std_variance: f64,
    max: isize,
    min: isize,
}

impl ItemResult for IsizeResult {
    fn get_readable_result(&self) -> String {
        format!(
            "{}\ncount:\t{}\nmax:\t{}\nmin:\t{}\ntotal:\t{}\naverage:\t{}\nvariance:\t{}\nSTD:\t{}",
            self.title,
            self.count,
            self.max,
            self.min,
            self.total,
            self.average,
            self.variance,
            self.std_variance
        )
    }

    fn get_json_result(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl IsizeResult {
    pub fn new(title: &str, data: &Vec<isize>) -> IsizeResult {
        if data.is_empty() {
            return IsizeResult {
                title: title.to_string(),
                count: 0,
                total: 0,
                average: f64::NEG_INFINITY,
                variance: f64::NEG_INFINITY,
                std_variance: f64::NEG_INFINITY,
                max: isize::MIN,
                min: isize::MAX,
            };
        }

        let tmp_count = data.len();
        let tmp_total = data.iter().fold(0isize, |r, s| r + s);
        let tmp_avg = (tmp_total as f64) / (tmp_count as f64);
        let tmp_dev = data
            .iter()
            .map(|v| ((*v as f64) - tmp_avg).pow(2))
            .sum::<f64>()
            / (tmp_count as f64);
        let tmp_std = tmp_dev.pow(0.5);
        let tmp_max = data.iter().cloned().fold(isize::MIN, isize::max);
        let tmp_min = data.iter().cloned().fold(isize::MAX, isize::min);

        IsizeResult {
            title: title.to_string(),
            count: tmp_count,
            total: tmp_total,
            average: tmp_avg,
            variance: tmp_dev,
            std_variance: tmp_std,
            max: tmp_max,
            min: tmp_min,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct F64Result {
    title: String,
    total: f64,
    count: usize,
    average: f64,
    variance: f64,
    std_variance: f64,
    max: f64,
    min: f64,
}

impl ItemResult for F64Result {
    fn get_readable_result(&self) -> String {
        format!(
            "{}\ncount:\t{}\nmax:\t{}\nmin:\t{}\ntotal:\t{}\naverage:\t{}\nvariance:\t{}\nSTD:\t{}",
            self.title,
            self.count,
            self.max,
            self.min,
            self.total,
            self.average,
            self.variance,
            self.std_variance
        )
    }

    fn get_json_result(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl F64Result {
    pub fn new(title: &str, data: &Vec<f64>) -> F64Result {
        if data.is_empty() {
            return F64Result {
                title: title.to_string(),
                count: 0,
                total: 0.0,
                average: f64::NEG_INFINITY,
                variance: f64::NEG_INFINITY,
                std_variance: f64::NEG_INFINITY,
                max: f64::MIN,
                min: f64::MAX,
            };
        }

        let tmp_count = data.len();
        let tmp_total = data.iter().fold(0f64, |r, s| r + s);
        let tmp_avg = (tmp_total as f64) / (tmp_count as f64);
        let tmp_dev = data
            .iter()
            .map(|v| ((*v as f64) - tmp_avg).pow(2))
            .sum::<f64>()
            / (tmp_count as f64);
        let tmp_std = tmp_dev.pow(0.5);
        let tmp_max = data.iter().cloned().fold(f64::MIN, f64::max);
        let tmp_min = data.iter().cloned().fold(f64::MAX, f64::min);

        F64Result {
            title: title.to_string(),
            count: tmp_count,
            total: tmp_total,
            average: tmp_avg,
            variance: tmp_dev,
            std_variance: tmp_std,
            max: tmp_max,
            min: tmp_min,
        }
    }
}
