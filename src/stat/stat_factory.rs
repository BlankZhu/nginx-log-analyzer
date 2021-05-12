use super::{
    enum_stat::StrStat,
    noop_stat::NoopStat,
    num_stat::{F64Stat, IsizeStat},
    stat::Stat,
};

pub enum StatType {
    NoopStatType,
    StrStatType,
    F64StatType,
    IsizeStatType,
}

pub struct StatFactory {}

impl StatFactory {
    pub fn create_stat(typ: StatType, title: &str) -> Box<dyn Stat> {
        match typ {
            StatType::StrStatType => Box::new(StrStat::new(title.to_string())),
            StatType::F64StatType => Box::new(F64Stat::new(title.to_string())),
            StatType::IsizeStatType => Box::new(IsizeStat::new(title.to_string())),
            _ => Box::new(NoopStat::new(title.to_string())),
        }
    }
}
