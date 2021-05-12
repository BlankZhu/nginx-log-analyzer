use super::stat::Stat;

pub struct NoopStat {
    title: String,
}

impl Stat for NoopStat {
    fn add(&mut self, _: String) {
    }

    fn count(&self) -> usize {
        0
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn get_result(&mut self) -> String {
        format!("{}\nno avaliable information", self.title())
    }
}

impl NoopStat {
    pub fn new(title: String) -> NoopStat {
        let t = format!("{}(Noop)", title);
        NoopStat {
            title: t,
        }
    }
}