use super::stat::Stat;

pub struct NginxStat {
    stats: Vec<Box<dyn Stat>>,
}

impl NginxStat {
    pub fn new() -> NginxStat {
        NginxStat { stats: Vec::new() }
    }

    pub fn add_title(&mut self, stat: Box<dyn Stat>) {
        self.stats.push(stat);
    }

    pub fn get_result(&mut self) -> String {
        let mut res: Vec<String> = Vec::new();
        for b in self.stats.iter_mut() {
            res.push(b.get_result());
        }
        res.join("\n==========\n")
    }

    pub fn add_data(&mut self, datas: Vec<String>) {
        if datas.len() != self.stats.len() {
            let titles = self.get_titles();
            panic!("cannot load access log data to Nginx stat analyzer, with title: \n`{}`\nline: \n`{}`\n", titles, datas.join(","));
        }

        let mut index: usize = 0;
        for data in datas {
            self.stats[index].add(data);
            index += 1;
        }
    }

    fn get_titles(&self) -> String {
        let mut sb: Vec<String> = Vec::new();
        for s in &self.stats {
            sb.push(s.title().clone());
        }
        return sb.join(",");
    }
}
