use super::stat::Stat;
use num::traits::Pow;

pub struct IsizeStat {
    title: String,
    total: isize,
    average: f64,
    variance: f64,
    max: isize,
    min: isize,
    elem: Vec<isize>,
}

impl Stat for IsizeStat {
    fn add(&mut self, num: String) {
        let to_push = num.parse::<isize>();
        match to_push {
            Ok(value) => {
                self.elem.push(value);
                if value > self.max {
                    self.max = value;
                }
                if value < self.min {
                    self.min = value
                }
            }
            Err(err) => {
                eprintln!(
                    "IsizeStat {} failed to parse string {} to isize, detail: {}",
                    self.title(),
                    num,
                    err
                );
                return;
            }
        }
    }

    fn count(&self) -> usize {
        self.elem.len()
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn get_result(&mut self) -> String {
        self.calculate();
        let ret = format!(
            "{}\ncount:\t{}\nmax:\t{}\nmin:\t{}\ntotal:\t{}\naverage:\t{}\nvariance:\t{}\nSTD:\t{}",
            self.title(),
            self.elem.len(),
            self.max(),
            self.min(),
            self.total(),
            self.average(),
            self.variance(),
            self.standard_deviation()
        );
        ret
    }
}

impl IsizeStat {
    pub fn new(title: String) -> IsizeStat {
        IsizeStat {
            title,
            total: 0,
            average: 0.0,
            variance: 0.0,
            max: isize::MIN,
            min: isize::MAX,
            elem: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        // calculate total
        self.total = 0;
        for i in &self.elem {
            self.total += i;
        }

        // calculate average
        let t = self.total as f64;
        let l = self.elem.len() as f64;
        self.average = t / l;

        // calculate variance
        let mut sum: f64 = 0.0;
        for i in &self.elem {
            let f = *i as f64;
            sum += (f - self.average).pow(2);
        }
        self.variance = sum / l;
    }

    pub fn total(&self) -> isize {
        self.total
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn variance(&self) -> f64 {
        self.variance
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance.pow(0.5)
    }

    pub fn max(&self) -> isize {
        self.max
    }

    pub fn min(&self) -> isize {
        self.min
    }
}

pub struct F64Stat {
    title: String,
    total: f64,
    average: f64,
    variance: f64,
    max: f64,
    min: f64,
    elem: Vec<f64>,
}

impl Stat for F64Stat {
    fn add(&mut self, num: String) {
        let to_push = num.parse::<f64>();
        match to_push {
            Ok(value) => {
                self.elem.push(value);
                if value > self.max {
                    self.max = value;
                }
                if value < self.min {
                    self.min = value
                }
            }
            Err(err) => {
                eprintln!(
                    "F64Stat {} failed to parse string {} to f64, detail: {}",
                    self.title(),
                    num,
                    err
                );
                return;
            }
        }
    }

    fn count(&self) -> usize {
        self.elem.len()
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn get_result(&mut self) -> String {
        self.calculate();
        let ret = format!(
            "{}\ncount:\t{}\nmax:\t{}\nmin:\t{}\ntotal:\t{}\naverage:\t{}\nvariance:\t{}\nSTD:\t{}",
            self.title(),
            self.elem.len(),
            self.max(),
            self.min(),
            self.total(),
            self.average(),
            self.variance(),
            self.standard_deviation()
        );
        ret
    }
}

impl F64Stat {
    pub fn new(title: String) -> F64Stat {
        F64Stat {
            title,
            total: 0.0,
            average: 0.0,
            variance: 0.0,
            max: f64::MIN,
            min: f64::MAX,
            elem: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        // calculate total
        self.total = 0.0;
        for i in &self.elem {
            self.total += i;
        }

        // calculate average
        let t = self.total as f64;
        let l = self.elem.len() as f64;
        self.average = t / l;

        // calculate variance
        let mut sum: f64 = 0.0;
        for i in &self.elem {
            let f = *i as f64;
            sum += (f - self.average).pow(2);
        }
        self.variance = sum / l;
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn variance(&self) -> f64 {
        self.variance
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance.pow(0.5)
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }
}
