use super::{
    enum_item::StrItem,
    noop_item::NoopItem,
    num_item::{F64Item, IsizeItem},
    time_item::{TimeHourItem, TimeMinuteItem},
    Item,
};

pub enum ItemType {
    F64ItemType,
    IsizeItemType,
    NoopItemType,
    StrItemType,
    TimeHourItemType,
    TimeMinuteType,
}

pub struct ItemFactory {
}

impl ItemFactory {
    pub fn create_item(typ: ItemType, title: &str) -> Box<dyn Item> {
        match typ {
            ItemType::F64ItemType => Box::new(F64Item::new(title)),
            ItemType::IsizeItemType => Box::new(IsizeItem::new(title)),
            ItemType::NoopItemType => Box::new(NoopItem::new(title)),
            ItemType::StrItemType => Box::new(StrItem::new(title)),
            ItemType::TimeHourItemType => Box::new(TimeHourItem::new(title)),
            ItemType::TimeMinuteType => Box::new(TimeMinuteItem::new(title)),
        }
    }
}