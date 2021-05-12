use std::collections::HashMap;
use super::stat::Stat;

pub struct StrStat {
    title: String,
    count: usize,
    map: HashMap<String, usize>,
}

impl Stat for StrStat {
    fn add(&mut self, elem: String) {
        let value = self.map.get_mut(&elem);
        match value {
            None => {
                self.map.insert(elem, 1);
            },
            Some(value ) => {
                *value += 1;
            }
        }
        self.count += 1;
    }

    fn count(&self) -> usize {
        self.count
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn get_result(&mut self) -> String {
        let mut ret = format!("{}\ncount:\t{}\ndetail:\n", self.title(), self.count());
        let mut tmp: Vec<String> = Vec::new();
        for (k, v) in self.percentage() {
            let line = format!("- {}\t{:.4}%", k, v*100.0);
            tmp.push(line);
        }
        ret.push_str(tmp.join("\n").as_str());
        ret
    }
}

impl StrStat {
    pub fn new(title: String) -> StrStat {
        StrStat {
            title,
            count: 0,
            map: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn percentage(&self) -> Vec<(String, f64)> {
        let mut ret: Vec<(String, f64)> = Vec::new();
        for (key, value) in &self.map {
            let v = *value as f64;
            let c = self.count as f64;
            ret.push((String::clone(key), v / c));
        }
        ret
    }
}