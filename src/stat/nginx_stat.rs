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
    pub fn read_line(&mut self, line: String) -> Result<(), String> {
        let splited = line.trim().split('\t').collect::<Vec<&str>>();
        if splited.len() != self.stats.len() {
            let err_msg = format!(
                "NginxStat title({}) not fit in with line({})",
                self.stats.len(),
                splited.len()
            );
            return Err(err_msg);
        }

        let mut index: usize = 0;
        for token in splited {
            self.stats[index].add(String::from(token));
            index += 1;
        }
        Ok(())
    }
    pub fn get_result(&mut self) -> String {
        let mut res: Vec<String> = Vec::new();
        for b in self.stats.iter_mut() {
            res.push(b.get_result());
        }
        res.join("\n==========\n")
    }
}
