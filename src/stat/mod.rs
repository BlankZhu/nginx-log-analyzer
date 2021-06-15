// mod stat;
// pub mod enum_stat;
// pub mod num_stat;
// pub mod nginx_stat;
// pub mod noop_stat;
// pub mod stat_factory;

pub mod item;

use item::Item;
use item::result::ItemResult;

pub struct NginxStat {
    items: Vec<Box<dyn Item>>,
}

impl NginxStat {
    pub fn new() -> NginxStat {
        NginxStat { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn get_results(&self) -> Vec<Box<dyn ItemResult>> {
        let mut ret = Vec::new();

        for item in &self.items {
            ret.push(item.get_result());
        }

        ret
    }

    pub fn add_data(&mut self, data: Vec<String>) {
        if data.len() != self.items.len() {
            let titles = self.get_titles();
            panic!("cannot load access log data to Nginx stat analyzer, with title: \n`{}`\nline: \n`{}`\n", titles, data.join(","));
        }

        let mut index: usize = 0;
        for elem in data {
            self.items[index].add(&elem);
            index += 1;
        }
    }

    fn get_titles(&self) -> String {
        let mut sb: Vec<String> = Vec::new();
        for s in &self.items {
            sb.push(s.title().clone());
        }
        return sb.join(",");
    }
}